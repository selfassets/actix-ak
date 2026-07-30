//! 外汇与货币数据服务
//!
//! 提供中国银行人民币外汇牌价等数据获取与解析

use crate::models::ak::currency::{CurrencyBocItem, CurrencyBocQuery, CurrencySafeItem};

/// 货币代码映射
fn get_money_code(symbol: &str) -> &'static str {
    match symbol {
        "美元" => "USD",
        "英镑" => "GBP",
        "欧元" => "EUR",
        "日元" => "JPY",
        "港币" => "HKD",
        "澳大利亚元" | "澳元" => "AUD",
        "加拿大元" => "CAD",
        "新加坡元" => "SGD",
        "瑞士法郎" => "CHF",
        "新西兰元" => "NZD",
        "澳门元" => "MOP",
        "泰国铢" | "泰铢" => "THB",
        "韩国元" | "韩元" => "KRW",
        _ => "USD",
    }
}

/// 获取新浪财经-中国银行人民币牌价历史数据
pub async fn get_currency_boc_sina(
    query: CurrencyBocQuery,
) -> Result<Vec<CurrencyBocItem>, String> {
    let symbol = query.symbol.unwrap_or_else(|| "美元".to_string());
    let start_date = query.start_date.unwrap_or_else(|| "20230101".to_string());
    let end_date = query.end_date.unwrap_or_else(|| "20231231".to_string());

    let money_code = get_money_code(&symbol);
    let start_fmt = format!(
        "{}-{}-{}",
        &start_date[0..4],
        &start_date[4..6],
        &start_date[6..8]
    );
    let end_fmt = format!(
        "{}-{}-{}",
        &end_date[0..4],
        &end_date[4..6],
        &end_date[6..8]
    );

    let url = format!(
        "http://biz.finance.sina.com.cn/forex/forex.php?money_code={}&type=0&startdate={}&enddate={}&page=1&call_type=ajax",
        money_code, start_fmt, end_fmt
    );

    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64)")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("请求新浪外汇牌价失败: {}", e))?;
    if !res.status().is_success() {
        return Err(format!("新浪外汇牌价响应状态码: {}", res.status()));
    }

    let bytes = res
        .bytes()
        .await
        .map_err(|e| format!("读取响应数据失败: {}", e))?;
    let (text, _, _) = encoding_rs::GBK.decode(&bytes);

    parse_boc_sina_html(&text, &symbol)
}

/// 解析新浪外汇牌价 HTML 表格
fn parse_boc_sina_html(html_str: &str, symbol: &str) -> Result<Vec<CurrencyBocItem>, String> {
    let document = scraper::Html::parse_document(html_str);
    let tr_selector =
        scraper::Selector::parse("tr").map_err(|_| "构建 CSS 选择器失败".to_string())?;
    let td_selector =
        scraper::Selector::parse("td").map_err(|_| "构建 CSS 选择器失败".to_string())?;

    let mut result = Vec::new();

    for tr in document.select(&tr_selector) {
        let cells: Vec<String> = tr
            .select(&td_selector)
            .map(|c| c.text().collect::<Vec<_>>().join("").trim().to_string())
            .collect();

        if cells.len() >= 6 {
            let bank_foreign_buy_pri = cells[1].parse::<f64>().ok();
            let bank_cash_buy_pri = cells[2].parse::<f64>().ok();
            let bank_foreign_sell_pri = cells[3].parse::<f64>().ok();
            let bank_cash_sell_pri = cells[4].parse::<f64>().ok();
            let bank_conversion_pri = cells[5].parse::<f64>().ok();
            let publish_time = if cells.len() >= 7 {
                Some(cells[6].clone())
            } else {
                None
            };

            result.push(CurrencyBocItem {
                currency: Some(symbol.to_string()),
                date: publish_time.clone(),
                bank_conversion_pri,
                bank_cash_buy_pri,
                bank_foreign_buy_pri,
                bank_cash_sell_pri,
                bank_foreign_sell_pri,
                publish_time,
            });
        }
    }

    Ok(result)
}

/// 国家外汇管理局 (SAFE) - 人民币汇率中间价数据
pub async fn get_currency_boc_safe(
    query: CurrencyBocQuery,
) -> Result<Vec<CurrencySafeItem>, String> {
    let now = chrono::Local::now();
    let start_date = query.start_date.unwrap_or_else(|| "2023-01-01".to_string());
    let end_date = query
        .end_date
        .unwrap_or_else(|| now.format("%Y-%m-%d").to_string());

    let start_fmt = if start_date.contains('-') {
        start_date
    } else if start_date.len() >= 8 {
        format!(
            "{}-{}-{}",
            &start_date[0..4],
            &start_date[4..6],
            &start_date[6..8]
        )
    } else {
        "2023-01-01".to_string()
    };

    let end_fmt = if end_date.contains('-') {
        end_date
    } else if end_date.len() >= 8 {
        format!(
            "{}-{}-{}",
            &end_date[0..4],
            &end_date[4..6],
            &end_date[6..8]
        )
    } else {
        now.format("%Y-%m-%d").to_string()
    };

    let url = "https://www.safe.gov.cn/AppStructured/hlw/RMBQuery.do";
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .post(url)
        .form(&[
            ("startDate", start_fmt.as_str()),
            ("endDate", end_fmt.as_str()),
            ("queryYN", "true"),
        ])
        .send()
        .await
        .map_err(|e| format!("请求国家外汇管理局接口失败: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("国家外汇管理局接口响应状态码: {}", res.status()));
    }

    let text = res
        .text()
        .await
        .map_err(|e| format!("读取响应 HTML 失败: {}", e))?;
    parse_safe_rmb_query_html(&text)
}

/// 解析国家外汇管理局中间价查询 HTML 表格
fn parse_safe_rmb_query_html(html_str: &str) -> Result<Vec<CurrencySafeItem>, String> {
    let document = scraper::Html::parse_document(html_str);
    let tr_selector =
        scraper::Selector::parse("tr").map_err(|_| "构建 CSS 选择器失败".to_string())?;
    let td_selector =
        scraper::Selector::parse("td, th").map_err(|_| "构建 CSS 选择器失败".to_string())?;

    let mut rows_data = Vec::new();

    for tr in document.select(&tr_selector) {
        let cells: Vec<String> = tr
            .select(&td_selector)
            .map(|c| c.text().collect::<Vec<_>>().join("").trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        if !cells.is_empty() {
            rows_data.push(cells);
        }
    }

    if rows_data.len() < 2 {
        return Ok(Vec::new());
    }

    let headers = rows_data[0].clone();
    let mut result = Vec::new();

    for row in rows_data.into_iter().skip(1) {
        let mut data = std::collections::HashMap::new();
        for (idx, field) in row.iter().enumerate() {
            let col_name = headers.get(idx).map(|s| s.as_str()).unwrap_or("");
            if col_name.is_empty() {
                continue;
            }

            if let Ok(val) = field.parse::<f64>() {
                data.insert(col_name.to_string(), serde_json::Value::from(val));
            } else {
                data.insert(
                    col_name.to_string(),
                    serde_json::Value::from(field.as_str()),
                );
            }
        }
        if !data.is_empty() {
            result.push(CurrencySafeItem { data });
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_money_code() {
        assert_eq!(get_money_code("美元"), "USD");
        assert_eq!(get_money_code("日元"), "JPY");
        assert_eq!(get_money_code("欧元"), "EUR");
    }
}
