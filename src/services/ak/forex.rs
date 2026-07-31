//! 东方财富外汇行情服务

use serde_json::Value;

/// 东方财富外汇实时行情
pub async fn forex_spot_em() -> Result<Vec<Value>, String> {
    let url = "https://push2.eastmoney.com/api/qt/clist/get";
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .get(url)
        .query(&[
            ("np", "1"),
            ("fltt", "2"),
            ("invt", "2"),
            ("fs", "m:119,m:120,m:133"),
            ("fields", "f12,f13,f14,f1,f2,f4,f3,f152,f17,f18,f15,f16"),
            ("fid", "f3"),
            ("pn", "1"),
            ("pz", "150"),
            ("po", "1"),
            ("dect", "1"),
            ("wbp2u", "|0|0|0|web"),
        ])
        .send()
        .await
        .map_err(|e| format!("请求东财外汇失败: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("东财接口故障, 代码: {}", res.status()));
    }

    let json: Value = res.json().await.map_err(|e| e.to_string())?;
    Ok(json["result"]["data"]
        .as_array()
        .cloned()
        .unwrap_or_default())
}

/// 东方财富外汇历史日线行情
pub async fn forex_hist_em(symbol: &str) -> Result<Vec<Value>, String> {
    // 自动判定市场域代号，缺省或非特型外汇统一走 119 或 120 / 133 等
    let market_code = match symbol {
        "EURCNYC" | "NZDCNYC" | "CNYRUBC" | "AUDCNYC" | "GBPCNYC" | "JPYCNYC" | "SGDCNYC"
        | "CADCNYC" | "CNYSARC" | "CNYAEDC" | "CNYTRYC" | "USDCNYC" | "HKDCNYC" | "CNYMOPC" => {
            "120"
        }
        "JPYCNH" | "CHFCNH" | "NZDCNH" | "USDCNH" | "HKDCNH" | "CADCNH" | "EURCNH" | "AUDCNH"
        | "CNHSGD" | "CNHGBP" | "CNHAUD" | "SGDCNH" | "GBPCNH" => "133",
        _ => "119",
    };

    let url = "https://push2his.eastmoney.com/api/qt/stock/kline/get";
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let secid = format!("{}.{}", market_code, symbol);
    let res = client
        .get(url)
        .query(&[
            ("secid", secid.as_str()),
            ("klt", "101"),
            ("fqt", "1"),
            ("lmt", "50000"),
            ("end", "20500000"),
            ("iscca", "1"),
            ("fields1", "f1,f2,f3,f4,f5,f6,f7,f8"),
            (
                "fields2",
                "f51,f52,f53,f54,f55,f56,f57,f58,f59,f60,f61,f62,f63,f64",
            ),
            ("ut", "f057cbcbce2a86e2866ab8877db1d059"),
            ("forcect", "1"),
        ])
        .send()
        .await
        .map_err(|e| format!("请求外汇K线失败: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("外汇K线接口状态码: {}", res.status()));
    }

    let json: Value = res.json().await.map_err(|e| e.to_string())?;
    Ok(json["data"]["klines"]
        .as_array()
        .cloned()
        .unwrap_or_default())
}
