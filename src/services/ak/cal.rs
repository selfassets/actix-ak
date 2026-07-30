//! 波动率与量化计算服务 (Yang-Zhang Realized Volatility)

use crate::models::ak::cal::{OhlcItem, RvMinuteQuery, YangZhangVolatilityResult};

/// 根据 OHLC 数据计算 Yang-Zhang (YZ) 已实现波动率
///
/// 公式:
/// RV^2 = Vo + k * Vc + (1 - k) * Vrs
/// - Vo: 隔夜波动率, Vo = 1/(n-1) * sum(oi - obar)^2, 其中 oi = ln(Open_i / Close_{i-1})
/// - Vc: 连续交易开收盘波动率, Vc = 1/(n-1) * sum(ci - cbar)^2, 其中 ci = ln(Close_i / Open_i)
/// - k: 权重, k = 0.34 / (1.34 + (n + 1) / (n - 1))
/// - Vrs: Rogers-Satchell 波动率, Vrs = 1/n * sum(u_i * (u_i - c_i) + d_i * (d_i - c_i))
///   其中 u_i = ln(High_i / Open_i), d_i = ln(Low_i / Open_i), c_i = ln(Close_i / Open_i)
pub fn calculate_yang_zhang_volatility(
    data: &[OhlcItem],
) -> Result<YangZhangVolatilityResult, String> {
    let n = data.len();
    if n < 2 {
        return Err("数据条数不足 2 条，无法计算 Yang-Zhang 波动率".to_string());
    }

    let n_f64 = n as f64;

    // 1. 计算 c_i, u_i, d_i
    let mut c_list = Vec::with_capacity(n);
    let mut u_list = Vec::with_capacity(n);
    let mut d_list = Vec::with_capacity(n);

    for item in data {
        if item.open <= 0.0 || item.high <= 0.0 || item.low <= 0.0 || item.close <= 0.0 {
            return Err("价格数据中存在 <= 0 的非法值".to_string());
        }
        c_list.push((item.close / item.open).ln());
        u_list.push((item.high / item.open).ln());
        d_list.push((item.low / item.open).ln());
    }

    // 2. 计算 o_i = ln(Open_i / Close_{i-1})，从第 1 条开始（索引从 1 到 n-1）
    let mut o_list = Vec::with_capacity(n - 1);
    for i in 1..n {
        o_list.push((data[i].open / data[i - 1].close).ln());
    }

    let n_o = o_list.len() as f64;

    // 3. 计算 Vo (隔夜波动率)
    let o_bar = o_list.iter().sum::<f64>() / n_o;
    let vo = o_list.iter().map(|o| (o - o_bar).powi(2)).sum::<f64>() / (n_o - 1.0);

    // 4. 计算 Vc (收盘价对开盘价波动率)
    let c_bar = c_list.iter().sum::<f64>() / n_f64;
    let vc = c_list.iter().map(|c| (c - c_bar).powi(2)).sum::<f64>() / (n_f64 - 1.0);

    // 5. 计算 Vrs (Rogers-Satchell 波动率)
    let mut vrs_sum = 0.0;
    for i in 0..n {
        let u = u_list[i];
        let c = c_list[i];
        let d = d_list[i];
        vrs_sum += u * (u - c) + d * (d - c);
    }
    let vrs = vrs_sum / n_f64;

    // 6. 计算权重 k
    let k = 0.34 / (1.34 + (n_f64 + 1.0) / (n_f64 - 1.0));

    // 7. 汇总 YZ 波动率
    let yang_zhang_var = vo + k * vc + (1.0 - k) * vrs;
    let yang_zhang_volatility = if yang_zhang_var > 0.0 {
        yang_zhang_var.sqrt()
    } else {
        0.0
    };

    Ok(YangZhangVolatilityResult {
        yang_zhang_volatility,
        count: n,
        vo,
        vc,
        vrs,
        k,
    })
}

/// 股票分钟级历史行情数据清洗格式化（支持东方财富）
pub async fn rv_from_stock_zh_a_hist_min_em(query: RvMinuteQuery) -> Result<Vec<OhlcItem>, String> {
    let symbol = query.symbol.unwrap_or_else(|| "000001".to_string());
    let period = query.period.unwrap_or_else(|| "5".to_string());
    let adjust = query.adjust.unwrap_or_else(|| "hfq".to_string());

    let market_id = if symbol.starts_with('6') || symbol.starts_with('9') {
        "1"
    } else {
        "0"
    };
    let secid = format!("{}.{}", market_id, symbol);

    let adjust_code = match adjust.as_str() {
        "qfq" => "1",
        "hfq" => "2",
        _ => "0",
    };

    let url = "https://push2his.eastmoney.com/api/qt/stock/kline/get";
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .get(url)
        .query(&[
            ("secid", secid.as_str()),
            ("klt", period.as_str()),
            ("fqt", adjust_code),
            ("lmt", "1000"),
            ("end", "20500000"),
            ("iscca", "1"),
            ("fields1", "f1,f2,f3,f4,f5,f6,f7,f8"),
            (
                "fields2",
                "f51,f52,f53,f54,f55,f56,f57,f58,f59,f60,f61,f62,f63,f64",
            ),
        ])
        .send()
        .await
        .map_err(|e| format!("请求股票分钟 K 线失败: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("东方财富接口状态码: {}", res.status()));
    }

    let json_val: serde_json::Value = res
        .json()
        .await
        .map_err(|e| format!("解析 JSON 失败: {}", e))?;
    let klines_arr = json_val["data"]["klines"]
        .as_array()
        .ok_or_else(|| "缺失 klines 数据".to_string())?;

    let mut result = Vec::new();
    for row in klines_arr {
        if let Some(s) = row.as_str() {
            let parts: Vec<&str> = s.split(',').collect();
            if parts.len() >= 6 {
                let date = parts[0].to_string();
                let open = parts[1].parse::<f64>().unwrap_or(0.0);
                let close = parts[2].parse::<f64>().unwrap_or(0.0);
                let high = parts[3].parse::<f64>().unwrap_or(0.0);
                let low = parts[4].parse::<f64>().unwrap_or(0.0);

                if open > 0.0 {
                    result.push(OhlcItem {
                        date,
                        open,
                        high,
                        low,
                        close,
                    });
                }
            }
        }
    }

    Ok(result)
}

/// 期货分钟级历史行情数据清洗格式化（新浪源）
pub async fn rv_from_futures_zh_minute_sina(query: RvMinuteQuery) -> Result<Vec<OhlcItem>, String> {
    let symbol = query.symbol.unwrap_or_else(|| "IF2008".to_string());
    let period = query.period.unwrap_or_else(|| "5".to_string());

    let url = format!(
        "https://stock2.finance.sina.com.cn/futures/api/jsonp.php/var%20_{}=/InnerFuturesNewService.getMinLine?symbol={}&type={}",
        symbol, symbol, period
    );

    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("请求新浪期货分钟 K 线失败: {}", e))?;
    if !res.status().is_success() {
        return Err(format!("新浪接口状态码: {}", res.status()));
    }

    let text = res
        .text()
        .await
        .map_err(|e| format!("读取响应失败: {}", e))?;
    let start_idx = text.find('[').ok_or_else(|| "数据格式不匹配".to_string())?;
    let end_idx = text
        .rfind(']')
        .ok_or_else(|| "数据格式不匹配".to_string())?;

    let json_str = &text[start_idx..=end_idx];
    let arr: Vec<serde_json::Value> =
        serde_json::from_str(json_str).map_err(|e| format!("解析 JSON 失败: {}", e))?;

    let mut result = Vec::new();
    for row in arr {
        let date = row[0].as_str().unwrap_or("").to_string();
        let open = row[1].as_str().and_then(|s| s.parse().ok()).unwrap_or(0.0);
        let high = row[2].as_str().and_then(|s| s.parse().ok()).unwrap_or(0.0);
        let low = row[3].as_str().and_then(|s| s.parse().ok()).unwrap_or(0.0);
        let close = row[4].as_str().and_then(|s| s.parse().ok()).unwrap_or(0.0);

        if open > 0.0 {
            result.push(OhlcItem {
                date,
                open,
                high,
                low,
                close,
            });
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_yang_zhang_volatility() {
        let sample_data = vec![
            OhlcItem {
                date: "2024-01-01".into(),
                open: 10.0,
                high: 10.5,
                low: 9.8,
                close: 10.2,
            },
            OhlcItem {
                date: "2024-01-02".into(),
                open: 10.3,
                high: 10.8,
                low: 10.1,
                close: 10.6,
            },
            OhlcItem {
                date: "2024-01-03".into(),
                open: 10.5,
                high: 10.9,
                low: 10.2,
                close: 10.4,
            },
        ];

        let res = calculate_yang_zhang_volatility(&sample_data);
        assert!(res.is_ok());
        let yz = res.unwrap();
        assert!(yz.yang_zhang_volatility > 0.0);
        assert_eq!(yz.count, 3);
    }
}
