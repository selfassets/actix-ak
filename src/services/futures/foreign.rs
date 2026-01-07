//! 外盘期货相关

use crate::models::{ForeignFuturesDetail, ForeignFuturesDetailItem, ForeignFuturesHistData, ForeignFuturesSymbol, FuturesInfo};
use anyhow::{anyhow, Result};
use chrono::Utc;
use chrono_tz::Asia::Shanghai;
use regex::Regex;
use reqwest::Client;
use std::collections::HashMap;

use super::common::{get_beijing_time, SINA_FOREIGN_DAILY_API, SINA_FUTURES_REALTIME_API};

/// 获取外盘期货品种列表
/// 对应 akshare 的 futures_hq_subscribe_exchange_symbol() 函数
pub fn get_foreign_futures_symbols() -> Vec<ForeignFuturesSymbol> {
    vec![
        ForeignFuturesSymbol { symbol: "新加坡铁矿石".to_string(), code: "FEF".to_string() },
        ForeignFuturesSymbol { symbol: "马棕油".to_string(), code: "FCPO".to_string() },
        ForeignFuturesSymbol { symbol: "日橡胶".to_string(), code: "RSS3".to_string() },
        ForeignFuturesSymbol { symbol: "美国原糖".to_string(), code: "RS".to_string() },
        ForeignFuturesSymbol { symbol: "CME比特币期货".to_string(), code: "BTC".to_string() },
        ForeignFuturesSymbol { symbol: "NYBOT-棉花".to_string(), code: "CT".to_string() },
        ForeignFuturesSymbol { symbol: "LME镍3个月".to_string(), code: "NID".to_string() },
        ForeignFuturesSymbol { symbol: "LME铅3个月".to_string(), code: "PBD".to_string() },
        ForeignFuturesSymbol { symbol: "LME锡3个月".to_string(), code: "SND".to_string() },
        ForeignFuturesSymbol { symbol: "LME锌3个月".to_string(), code: "ZSD".to_string() },
        ForeignFuturesSymbol { symbol: "LME铝3个月".to_string(), code: "AHD".to_string() },
        ForeignFuturesSymbol { symbol: "LME铜3个月".to_string(), code: "CAD".to_string() },
        ForeignFuturesSymbol { symbol: "CBOT-黄豆".to_string(), code: "S".to_string() },
        ForeignFuturesSymbol { symbol: "CBOT-小麦".to_string(), code: "W".to_string() },
        ForeignFuturesSymbol { symbol: "CBOT-玉米".to_string(), code: "C".to_string() },
        ForeignFuturesSymbol { symbol: "CBOT-黄豆油".to_string(), code: "BO".to_string() },
        ForeignFuturesSymbol { symbol: "CBOT-黄豆粉".to_string(), code: "SM".to_string() },
        ForeignFuturesSymbol { symbol: "COMEX铜".to_string(), code: "HG".to_string() },
        ForeignFuturesSymbol { symbol: "NYMEX天然气".to_string(), code: "NG".to_string() },
        ForeignFuturesSymbol { symbol: "NYMEX原油".to_string(), code: "CL".to_string() },
        ForeignFuturesSymbol { symbol: "COMEX白银".to_string(), code: "SI".to_string() },
        ForeignFuturesSymbol { symbol: "COMEX黄金".to_string(), code: "GC".to_string() },
        ForeignFuturesSymbol { symbol: "布伦特原油".to_string(), code: "OIL".to_string() },
        ForeignFuturesSymbol { symbol: "伦敦金".to_string(), code: "XAU".to_string() },
        ForeignFuturesSymbol { symbol: "伦敦银".to_string(), code: "XAG".to_string() },
        ForeignFuturesSymbol { symbol: "伦敦铂金".to_string(), code: "XPT".to_string() },
        ForeignFuturesSymbol { symbol: "伦敦钯金".to_string(), code: "XPD".to_string() },
        ForeignFuturesSymbol { symbol: "欧洲碳排放".to_string(), code: "EUA".to_string() },
    ]
}

/// 获取外盘期货实时行情
/// 对应 akshare 的 futures_foreign_commodity_realtime() 函数
pub async fn get_foreign_futures_realtime(codes: &[String]) -> Result<Vec<FuturesInfo>> {
    use std::time::Duration;

    let client = Client::builder()
        .timeout(Duration::from_secs(30))
        .connect_timeout(Duration::from_secs(10))
        .build()?;

    let symbols_str = codes
        .iter()
        .map(|c| format!("hf_{}", c))
        .collect::<Vec<_>>()
        .join(",");

    let url = format!("{}?list={}", SINA_FUTURES_REALTIME_API, symbols_str);
    println!("📡 请求外盘期货行情 URL: {}", url);

    let response = client
        .get(&url)
        .header("Accept", "*/*")
        .header("Accept-Encoding", "gzip, deflate")
        .header("Accept-Language", "zh-CN,zh;q=0.9,en;q=0.8")
        .header("Cache-Control", "no-cache")
        .header("Host", "hq.sinajs.cn")
        .header("Pragma", "no-cache")
        .header("Referer", "https://finance.sina.com.cn/")
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(anyhow!("获取外盘期货数据失败: {}", response.status()));
    }

    let text = response.text().await?;
    let preview: String = text.chars().take(500).collect();
    println!("📥 原始响应数据: {}", preview);

    parse_foreign_futures_data(&text, codes)
}

/// 解析外盘期货数据
fn parse_foreign_futures_data(data: &str, codes: &[String]) -> Result<Vec<FuturesInfo>> {
    let mut results = Vec::new();
    let symbol_map = get_foreign_futures_symbols();
    let code_to_name: HashMap<String, String> = symbol_map
        .iter()
        .map(|s| (s.code.clone(), s.symbol.clone()))
        .collect();

    for (i, item) in data.split(';').filter(|s| !s.trim().is_empty()).enumerate() {
        if i >= codes.len() {
            break;
        }

        let parts: Vec<&str> = item.split('=').collect();
        if parts.len() < 2 {
            continue;
        }

        let data_part = parts[1].trim_matches('"').trim_matches('\'');
        if data_part.is_empty() {
            continue;
        }

        let fields: Vec<&str> = data_part.split(',').collect();
        if fields.len() < 13 {
            continue;
        }

        let code = &codes[i];
        let name = code_to_name.get(code).cloned().unwrap_or(code.clone());

        let current_price = fields[0].parse::<f64>().unwrap_or(0.0);
        let high = fields[4].parse::<f64>().unwrap_or(0.0);
        let low = fields[5].parse::<f64>().unwrap_or(0.0);
        let prev_settlement = fields[7].parse::<f64>().unwrap_or(0.0);
        let open = fields[8].parse::<f64>().unwrap_or(0.0);
        let open_interest = fields[9].parse::<u64>().ok();

        let change = current_price - prev_settlement;
        let change_percent = if prev_settlement != 0.0 {
            (change / prev_settlement) * 100.0
        } else {
            0.0
        };

        results.push(FuturesInfo {
            symbol: code.clone(),
            name,
            current_price,
            change,
            change_percent,
            volume: 0,
            open,
            high,
            low,
            settlement: None,
            prev_settlement: Some(prev_settlement),
            open_interest,
            updated_at: get_beijing_time(),
        });
    }

    Ok(results)
}

/// 获取外盘期货历史数据（日K线）
/// 对应 akshare 的 futures_foreign_hist() 函数
pub async fn get_futures_foreign_hist(symbol: &str) -> Result<Vec<ForeignFuturesHistData>> {
    let client = Client::new();

    let now = Utc::now().with_timezone(&Shanghai);
    let today = format!(
        "{}_{}_{}",
        now.format("%Y"),
        now.format("%-m"),
        now.format("%-d")
    );

    let url = format!(
        "{}/var%20_S{}=/GlobalFuturesService.getGlobalFuturesDailyKLine",
        SINA_FOREIGN_DAILY_API, today
    );

    println!("📡 请求外盘期货历史数据 URL: {}", url);

    let response = client
        .get(&url)
        .query(&[("symbol", symbol), ("_", &today), ("source", "web")])
        .header("Referer", "https://finance.sina.com.cn/")
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(anyhow!("获取外盘期货历史数据失败: {}", response.status()));
    }

    let text = response.text().await?;
    println!("📥 原始响应数据长度: {} 字节", text.len());

    parse_foreign_hist_data(&text)
}

/// 解析外盘期货历史数据
fn parse_foreign_hist_data(data: &str) -> Result<Vec<ForeignFuturesHistData>> {
    let mut history = Vec::new();

    let start = data.find('[');
    let end = data.rfind(']');

    if start.is_none() || end.is_none() {
        return Err(anyhow!("无效的外盘期货历史数据格式"));
    }

    let json_str = &data[start.unwrap()..end.unwrap() + 1];

    let json_data: serde_json::Value =
        serde_json::from_str(json_str).map_err(|e| anyhow!("解析JSON失败: {}", e))?;

    if let Some(arr) = json_data.as_array() {
        println!("📈 解析到 {} 条外盘期货历史数据", arr.len());

        for item in arr {
            if item.is_object() {
                history.push(ForeignFuturesHistData {
                    date: item["date"].as_str().unwrap_or("").to_string(),
                    open: item["open"]
                        .as_str()
                        .or_else(|| item["open"].as_f64().map(|_| ""))
                        .and_then(|s| {
                            if s.is_empty() { item["open"].as_f64() } else { s.parse().ok() }
                        })
                        .unwrap_or(0.0),
                    high: item["high"]
                        .as_str()
                        .or_else(|| item["high"].as_f64().map(|_| ""))
                        .and_then(|s| {
                            if s.is_empty() { item["high"].as_f64() } else { s.parse().ok() }
                        })
                        .unwrap_or(0.0),
                    low: item["low"]
                        .as_str()
                        .or_else(|| item["low"].as_f64().map(|_| ""))
                        .and_then(|s| {
                            if s.is_empty() { item["low"].as_f64() } else { s.parse().ok() }
                        })
                        .unwrap_or(0.0),
                    close: item["close"]
                        .as_str()
                        .or_else(|| item["close"].as_f64().map(|_| ""))
                        .and_then(|s| {
                            if s.is_empty() { item["close"].as_f64() } else { s.parse().ok() }
                        })
                        .unwrap_or(0.0),
                    volume: item["volume"]
                        .as_str()
                        .and_then(|s| s.parse().ok())
                        .or_else(|| item["volume"].as_u64())
                        .unwrap_or(0),
                });
            }
        }
    }

    Ok(history)
}

/// 获取外盘期货合约详情
/// 对应 akshare 的 futures_foreign_detail() 函数
pub async fn get_futures_foreign_detail(symbol: &str) -> Result<ForeignFuturesDetail> {
    let client = Client::new();

    let url = format!("https://finance.sina.com.cn/futures/quotes/{}.shtml", symbol);
    println!("📡 请求外盘期货合约详情 URL: {}", url);

    let response = client
        .get(&url)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(anyhow!("获取外盘期货合约详情失败: {}", response.status()));
    }

    let bytes = response.bytes().await?;
    let text = encoding_rs::GBK.decode(&bytes).0.to_string();

    parse_foreign_detail_html(&text)
}

/// 解析外盘期货合约详情HTML
fn parse_foreign_detail_html(html: &str) -> Result<ForeignFuturesDetail> {
    let mut items = Vec::new();

    let table_re = Regex::new(r"<table[^>]*>([\s\S]*?)</table>").unwrap();
    let tables: Vec<_> = table_re.captures_iter(html).collect();

    let target_table_index = if tables.len() > 6 { 6 } else { tables.len().saturating_sub(1) };

    if tables.is_empty() {
        return Err(anyhow!("未找到合约详情表格"));
    }

    let table_content = tables
        .get(target_table_index)
        .and_then(|cap| cap.get(1))
        .map(|m| m.as_str())
        .unwrap_or("");

    let row_re = Regex::new(r"<tr[^>]*>([\s\S]*?)</tr>").unwrap();
    let cell_re = Regex::new(r"<t[dh][^>]*>([\s\S]*?)</t[dh]>").unwrap();

    let clean_html = |s: &str| -> String {
        let tag_re = Regex::new(r"<[^>]+>").unwrap();
        tag_re.replace_all(s, "").trim().to_string()
    };

    for row_cap in row_re.captures_iter(table_content) {
        let row_content = row_cap.get(1).map(|m| m.as_str()).unwrap_or("");
        let cells: Vec<_> = cell_re
            .captures_iter(row_content)
            .filter_map(|c| c.get(1).map(|m| clean_html(m.as_str())))
            .collect();

        if cells.len() >= 2 {
            let name = cells[0].clone();
            let value = cells[1].clone();

            if !name.is_empty() && !value.is_empty() {
                items.push(ForeignFuturesDetailItem { name, value });
            }

            if cells.len() >= 4 {
                let name2 = cells[2].clone();
                let value2 = cells[3].clone();

                if !name2.is_empty() && !value2.is_empty() {
                    items.push(ForeignFuturesDetailItem { name: name2, value: value2 });
                }
            }
        }
    }

    println!("📊 解析到 {} 条合约详情项", items.len());
    Ok(ForeignFuturesDetail { items })
}
