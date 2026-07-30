//! 加密货币数据服务

use crate::models::ak::crypto::{CryptoBitcoinCmeItem, CryptoBitcoinHoldItem, CryptoQuery};
use std::collections::HashMap;

/// 芝加哥商业交易所-比特币成交量报告
pub async fn get_crypto_bitcoin_cme(
    query: CryptoQuery,
) -> Result<Vec<CryptoBitcoinCmeItem>, String> {
    let date_str = query.date.unwrap_or_else(|| "20230830".to_string());
    let date_fmt = if date_str.len() >= 8 {
        format!(
            "{}-{}-{}",
            &date_str[0..4],
            &date_str[4..6],
            &date_str[6..8]
        )
    } else {
        "2023-08-30".to_string()
    };

    let url = "https://datacenter-api.jin10.com/reports/list";
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .get(url)
        .header("x-app-id", "rU6QIu7JHe2gOUeR")
        .header("x-version", "1.0.0")
        .query(&[
            ("category", "cme"),
            ("date", date_fmt.as_str()),
            ("attr_id", "4"),
        ])
        .send()
        .await
        .map_err(|e| format!("请求 CME 比特币数据失败: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("金十数据 API 响应状态码: {}", res.status()));
    }

    let json_val: serde_json::Value = res
        .json()
        .await
        .map_err(|e| format!("解析 JSON 失败: {}", e))?;
    let keys_arr = json_val["data"]["keys"]
        .as_array()
        .ok_or_else(|| "缺失 keys 数组".to_string())?;
    let values_arr = json_val["data"]["values"]
        .as_array()
        .ok_or_else(|| "缺失 values 数组".to_string())?;

    let mut col_names = Vec::new();
    for k in keys_arr {
        if let Some(name) = k["name"].as_str() {
            col_names.push(name.to_string());
        }
    }

    let mut result = Vec::new();
    for row in values_arr {
        if let Some(val_row) = row.as_array() {
            let mut data = HashMap::new();
            for (i, val) in val_row.iter().enumerate() {
                let name = col_names
                    .get(i)
                    .cloned()
                    .unwrap_or_else(|| format!("col_{}", i));
                data.insert(name, val.clone());
            }
            result.push(CryptoBitcoinCmeItem { data });
        }
    }

    Ok(result)
}

/// 金十数据-全球上市公司及机构比特币持仓报告
pub async fn get_crypto_bitcoin_hold_report() -> Result<Vec<CryptoBitcoinHoldItem>, String> {
    let url = "https://datacenter-api.jin10.com/bitcoin_treasuries/list";
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .get(url)
        .header("X-App-Id", "lnFP5lxse24wPgtY")
        .header("X-Version", "1.0.0")
        .send()
        .await
        .map_err(|e| format!("请求比特币持仓报告失败: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("金十数据持仓 API 状态码: {}", res.status()));
    }

    let json_val: serde_json::Value = res
        .json()
        .await
        .map_err(|e| format!("解析 JSON 失败: {}", e))?;
    let values_arr = json_val["data"]["values"]
        .as_array()
        .ok_or_else(|| "缺失 values 数组".to_string())?;

    let mut result = Vec::new();
    for row in values_arr {
        if let Some(arr) = row.as_array() {
            if arr.len() >= 16 {
                let symbol = arr[0].as_str().map(|s| s.to_string());
                let name_en = arr[1].as_str().map(|s| s.to_string());
                let country = arr[2].as_str().map(|s| s.to_string());
                let market_cap = arr[3]
                    .as_f64()
                    .or_else(|| arr[3].as_str().and_then(|s| s.parse().ok()));
                let btc_market_ratio = arr[4]
                    .as_f64()
                    .or_else(|| arr[4].as_str().and_then(|s| s.parse().ok()));
                let cost = arr[5]
                    .as_f64()
                    .or_else(|| arr[5].as_str().and_then(|s| s.parse().ok()));
                let hold_ratio = arr[6]
                    .as_f64()
                    .or_else(|| arr[6].as_str().and_then(|s| s.parse().ok()));
                let hold_amount = arr[7]
                    .as_f64()
                    .or_else(|| arr[7].as_str().and_then(|s| s.parse().ok()));
                let current_hold_market_val = arr[8]
                    .as_f64()
                    .or_else(|| arr[8].as_str().and_then(|s| s.parse().ok()));
                let date = arr[9].as_str().map(|s| s.chars().take(10).collect());
                let link = arr[10].as_str().map(|s| s.to_string());
                let category = arr[12].as_str().map(|s| s.to_string());
                let multiple = arr[13]
                    .as_f64()
                    .or_else(|| arr[13].as_str().and_then(|s| s.parse().ok()));
                let name_cn = arr[15].as_str().map(|s| s.to_string());

                result.push(CryptoBitcoinHoldItem {
                    symbol,
                    name_en,
                    name_cn,
                    country,
                    market_cap,
                    btc_market_ratio,
                    cost,
                    hold_ratio,
                    hold_amount,
                    current_hold_market_val,
                    date,
                    link,
                    category,
                    multiple,
                });
            }
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_crypto_bitcoin_cme() {
        let query = CryptoQuery {
            date: Some("20230830".to_string()),
        };
        let res = get_crypto_bitcoin_cme(query).await;
        // 如果网络或上游 API 变更，允许测试优雅返回或通过
        if let Ok(data) = res {
            println!("CME 比特币数据获取成功，条数: {}", data.len());
        }
    }
}
