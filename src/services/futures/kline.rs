//! K线数据相关函数

use crate::models::{FuturesHistoryData, FuturesQuery};
use anyhow::{anyhow, Result};
use reqwest::Client;

use super::common::{SINA_FUTURES_DAILY_API, SINA_FUTURES_MINUTE_API};

/// 获取期货日K线历史数据
/// 对应 akshare 的 futures_zh_daily_sina() 函数
pub async fn get_futures_history(
    symbol: &str,
    query: &FuturesQuery,
) -> Result<Vec<FuturesHistoryData>> {
    let client = Client::new();
    let limit = query.limit.unwrap_or(30);

    let full_url = format!("{}?symbol={}", SINA_FUTURES_DAILY_API, symbol);
    println!("📡 请求日K线数据 URL: {}", full_url);

    let response = client
        .get(SINA_FUTURES_DAILY_API)
        .query(&[("symbol", symbol)])
        .header("Referer", "https://finance.sina.com.cn/")
        .header(
            "User-Agent",
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36",
        )
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(anyhow!("获取历史数据失败: {}", response.status()));
    }

    let text = response.text().await?;
    let preview: String = text.chars().take(300).collect();
    println!("📥 原始响应数据: {}", preview);
    parse_sina_history_data(&text, symbol, limit)
}

/// 获取期货分钟K线数据
/// 对应 akshare 的 futures_zh_minute_sina() 函数
/// period: "1", "5", "15", "30", "60" 分钟
pub async fn get_futures_minute_data(
    symbol: &str,
    period: &str,
) -> Result<Vec<FuturesHistoryData>> {
    let client = Client::new();

    let full_url = format!(
        "{}?symbol={}&type={}",
        SINA_FUTURES_MINUTE_API, symbol, period
    );
    println!("📡 请求分钟K线数据 URL: {}", full_url);

    let response = client
        .get(SINA_FUTURES_MINUTE_API)
        .query(&[("symbol", symbol), ("type", period)])
        .header("Referer", "https://finance.sina.com.cn/")
        .header(
            "User-Agent",
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36",
        )
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(anyhow!("获取分钟数据失败: {}", response.status()));
    }

    let text = response.text().await?;
    let preview: String = text.chars().take(300).collect();
    println!("📥 原始响应数据: {}", preview);
    parse_sina_minute_data(&text, symbol)
}

/// 解析新浪期货日K线历史数据
fn parse_sina_history_data(
    data: &str,
    symbol: &str,
    limit: usize,
) -> Result<Vec<FuturesHistoryData>> {
    let mut history = Vec::new();

    let start = data.find("([");
    let end = data.rfind("])");

    if start.is_none() || end.is_none() {
        println!("❌ 未找到有效的JSON数据边界");
        return Err(anyhow!("无效的历史数据格式"));
    }

    let json_str = &data[start.unwrap() + 1..end.unwrap() + 1];
    println!("📊 解析JSON数据，长度: {} 字节", json_str.len());

    let json_data: serde_json::Value =
        serde_json::from_str(json_str).map_err(|e| anyhow!("解析JSON失败: {}", e))?;

    if let Some(arr) = json_data.as_array() {
        println!("📈 解析到 {} 条K线数据", arr.len());

        let start_idx = if arr.len() > limit {
            arr.len() - limit
        } else {
            0
        };

        for item in arr.iter().skip(start_idx) {
            if item.is_object() {
                let date = item["d"].as_str().unwrap_or("").to_string();
                let open = item["o"].as_str().unwrap_or("0").parse().unwrap_or(0.0);
                let high = item["h"].as_str().unwrap_or("0").parse().unwrap_or(0.0);
                let low = item["l"].as_str().unwrap_or("0").parse().unwrap_or(0.0);
                let close = item["c"].as_str().unwrap_or("0").parse().unwrap_or(0.0);
                let volume = item["v"].as_str().unwrap_or("0").parse().unwrap_or(0);
                let open_interest = item["p"].as_str().unwrap_or("0").parse().ok();
                let settlement = item["s"].as_str().unwrap_or("0").parse().ok();

                history.push(FuturesHistoryData {
                    symbol: symbol.to_string(),
                    date,
                    open,
                    high,
                    low,
                    close,
                    volume,
                    open_interest,
                    settlement,
                });
            } else if let Some(fields) = item.as_array() {
                if fields.len() >= 8 {
                    history.push(FuturesHistoryData {
                        symbol: symbol.to_string(),
                        date: fields[0].as_str().unwrap_or("").to_string(),
                        open: fields[1].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                        high: fields[2].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                        low: fields[3].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                        close: fields[4].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                        volume: fields[5].as_str().unwrap_or("0").parse().unwrap_or(0),
                        open_interest: fields[6].as_str().unwrap_or("0").parse().ok(),
                        settlement: fields[7].as_str().unwrap_or("0").parse().ok(),
                    });
                }
            }
        }
    }

    Ok(history)
}

/// 解析新浪期货分钟K线数据
fn parse_sina_minute_data(data: &str, symbol: &str) -> Result<Vec<FuturesHistoryData>> {
    let mut history = Vec::new();

    let start = data.find("([");
    let end = data.rfind("])");

    if start.is_none() || end.is_none() {
        println!("❌ 未找到有效的JSON数据边界");
        return Err(anyhow!("无效的分钟数据格式"));
    }

    let json_str = &data[start.unwrap() + 1..end.unwrap() + 1];
    println!("📊 解析JSON数据，长度: {} 字节", json_str.len());

    let json_data: serde_json::Value =
        serde_json::from_str(json_str).map_err(|e| anyhow!("解析JSON失败: {}", e))?;

    if let Some(arr) = json_data.as_array() {
        println!("📈 解析到 {} 条K线数据", arr.len());

        for item in arr.iter() {
            if item.is_object() {
                history.push(FuturesHistoryData {
                    symbol: symbol.to_string(),
                    date: item["d"].as_str().unwrap_or("").to_string(),
                    open: item["o"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                    high: item["h"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                    low: item["l"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                    close: item["c"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                    volume: item["v"].as_str().unwrap_or("0").parse().unwrap_or(0),
                    open_interest: item["p"].as_str().unwrap_or("0").parse().ok(),
                    settlement: None,
                });
            } else if let Some(fields) = item.as_array() {
                if fields.len() >= 6 {
                    history.push(FuturesHistoryData {
                        symbol: symbol.to_string(),
                        date: fields[0].as_str().unwrap_or("").to_string(),
                        open: fields[1].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                        high: fields[2].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                        low: fields[3].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                        close: fields[4].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                        volume: fields[5].as_str().unwrap_or("0").parse().unwrap_or(0),
                        open_interest: fields
                            .get(6)
                            .and_then(|v| v.as_str())
                            .and_then(|s| s.parse().ok()),
                        settlement: None,
                    });
                }
            }
        }
    }

    Ok(history)
}
