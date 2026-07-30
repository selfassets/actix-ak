//! 波动率与量化计算服务 (Yang-Zhang Realized Volatility)

use crate::models::ak::cal::{OhlcItem, YangZhangVolatilityResult};

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
