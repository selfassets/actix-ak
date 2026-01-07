//! 主力连续合约相关

use crate::models::{FuturesHoldPosition, FuturesMainContract, FuturesMainDailyData};
use anyhow::{anyhow, Result};
use regex::Regex;
use reqwest::Client;

use super::common::{SINA_HOLD_POS_API, SINA_MAIN_DAILY_API};

/// 获取主力连续合约一览表
/// 对应 akshare 的 futures_display_main_sina() 函数
pub async fn get_futures_display_main_sina() -> Result<Vec<FuturesMainContract>> {
    let mut all_contracts = Vec::new();

    for exchange in &["dce", "czce", "shfe", "cffex", "gfex"] {
        match get_main_contracts_by_exchange(exchange).await {
            Ok(mut contracts) => all_contracts.append(&mut contracts),
            Err(e) => {
                log::warn!("获取 {} 主力连续合约失败: {}", exchange, e);
            }
        }
    }

    Ok(all_contracts)
}

/// 获取指定交易所的主力连续合约
async fn get_main_contracts_by_exchange(exchange: &str) -> Result<Vec<FuturesMainContract>> {
    let client = Client::new();
    let mut contracts = Vec::new();

    let symbol_url = "https://vip.stock.finance.sina.com.cn/quotes_service/view/js/qihuohangqing.js";
    let response = client
        .get(symbol_url)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .send()
        .await?;

    let bytes = response.bytes().await?;
    let text = encoding_rs::GBK.decode(&bytes).0.to_string();

    let nodes = parse_exchange_nodes(&text, exchange)?;

    for node in nodes {
        let list_url = "https://vip.stock.finance.sina.com.cn/quotes_service/api/json_v2.php/Market_Center.getHQFuturesData";

        let response = client
            .get(list_url)
            .query(&[
                ("page", "1"),
                ("sort", "position"),
                ("asc", "0"),
                ("node", &node),
                ("base", "futures"),
            ])
            .send()
            .await;

        if let Ok(resp) = response {
            if let Ok(text) = resp.text().await {
                if let Ok(json_data) = serde_json::from_str::<serde_json::Value>(&text) {
                    if let Some(arr) = json_data.as_array() {
                        for item in arr {
                            let name = item["name"].as_str().unwrap_or("");
                            let symbol = item["symbol"].as_str().unwrap_or("");

                            if name.contains("连续") && symbol.ends_with("0") {
                                contracts.push(FuturesMainContract {
                                    symbol: symbol.to_string(),
                                    name: name.to_string(),
                                    exchange: exchange.to_uppercase(),
                                });
                                break;
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(contracts)
}

/// 解析交易所的品种node列表
fn parse_exchange_nodes(js_text: &str, exchange: &str) -> Result<Vec<String>> {
    let mut nodes = Vec::new();

    let start = js_text.find("ARRFUTURESNODES = {");
    let end = js_text.find("};");

    if start.is_none() || end.is_none() {
        return Err(anyhow!("无法解析品种映射JS数据"));
    }

    let content = &js_text[start.unwrap()..end.unwrap() + 2];

    let pattern = format!(r"{}\s*:\s*\[", exchange);
    let re = Regex::new(&pattern).unwrap();

    if let Some(m) = re.find(content) {
        let start_pos = m.end();
        let remaining = &content[start_pos..];

        let item_re = Regex::new(r"\['[^']+',\s*'([^']+)',\s*'[^']*'").unwrap();

        for cap in item_re.captures_iter(remaining) {
            if let Some(node) = cap.get(1) {
                let node_str = node.as_str();
                if node_str.ends_with("_qh") {
                    nodes.push(node_str.to_string());
                }
            }
        }
    }

    Ok(nodes)
}

/// 获取主力连续合约日K线数据
/// 对应 akshare 的 futures_main_sina() 函数
pub async fn get_futures_main_sina(
    symbol: &str,
    start_date: Option<&str>,
    end_date: Option<&str>,
) -> Result<Vec<FuturesMainDailyData>> {
    let client = Client::new();

    let trade_date = "20210817";
    let trade_date_fmt = format!(
        "{}_{}_{}",
        &trade_date[..4],
        &trade_date[4..6],
        &trade_date[6..]
    );

    let url = format!(
        "{}/var%20_{}{}=/InnerFuturesNewService.getDailyKLine?symbol={}&_={}",
        SINA_MAIN_DAILY_API, symbol, trade_date_fmt, symbol, trade_date_fmt
    );

    println!("📡 请求主力连续日K线 URL: {}", url);

    let response = client
        .get(&url)
        .header("Referer", "https://finance.sina.com.cn/")
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(anyhow!("获取主力连续数据失败: {}", response.status()));
    }

    let text = response.text().await?;
    println!("📥 原始响应数据长度: {} 字节", text.len());

    let mut data = parse_main_daily_data(&text)?;

    if let Some(start) = start_date {
        data.retain(|d| d.date.replace("-", "").as_str() >= start);
    }
    if let Some(end) = end_date {
        data.retain(|d| d.date.replace("-", "").as_str() <= end);
    }

    Ok(data)
}

/// 解析主力连续日K线数据
fn parse_main_daily_data(data: &str) -> Result<Vec<FuturesMainDailyData>> {
    let mut history = Vec::new();

    let start = data.find("([");
    let end = data.rfind("])");

    if start.is_none() || end.is_none() {
        return Err(anyhow!("无效的主力连续数据格式"));
    }

    let json_str = &data[start.unwrap() + 1..end.unwrap() + 1];

    let json_data: serde_json::Value =
        serde_json::from_str(json_str).map_err(|e| anyhow!("解析JSON失败: {}", e))?;

    if let Some(arr) = json_data.as_array() {
        for item in arr {
            if item.is_object() {
                history.push(FuturesMainDailyData {
                    date: item["d"].as_str().unwrap_or("").to_string(),
                    open: item["o"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                    high: item["h"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                    low: item["l"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                    close: item["c"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                    volume: item["v"].as_str().unwrap_or("0").parse().unwrap_or(0),
                    hold: item["p"].as_str().unwrap_or("0").parse().unwrap_or(0),
                    settle: item["s"].as_str().and_then(|s| s.parse().ok()),
                });
            }
        }
    }

    Ok(history)
}

/// 获取期货持仓排名数据
/// 对应 akshare 的 futures_hold_pos_sina() 函数
pub async fn get_futures_hold_pos_sina(
    pos_type: &str,
    contract: &str,
    date: &str,
) -> Result<Vec<FuturesHoldPosition>> {
    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()?;

    let formatted_date = if date.len() == 8 {
        format!("{}-{}-{}", &date[..4], &date[4..6], &date[6..])
    } else {
        date.to_string()
    };

    let url = format!("{}?t_breed={}&t_date={}", SINA_HOLD_POS_API, contract, formatted_date);
    println!("📡 请求持仓排名 URL: {}", url);

    let response = client
        .get(&url)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
        .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8")
        .header("Accept-Language", "zh-CN,zh;q=0.9,en;q=0.8")
        .header("Accept-Encoding", "gzip, deflate")
        .header("Connection", "keep-alive")
        .header("Referer", "https://vip.stock.finance.sina.com.cn/")
        .header("Host", "vip.stock.finance.sina.com.cn")
        .send()
        .await?;

    let status = response.status();
    if !status.is_success() {
        if status.as_u16() == 456 || status.as_u16() == 403 {
            return Err(anyhow!("IP被新浪封禁，请稍后重试（5-60分钟后自动解封）"));
        }
        return Err(anyhow!("获取持仓排名失败: {}", status));
    }

    let bytes = response.bytes().await?;
    let text = encoding_rs::GBK.decode(&bytes).0.to_string();

    if text.contains("拒绝访问") || text.contains("IP 存在异常访问") {
        return Err(anyhow!("IP被新浪封禁，请稍后重试（5-60分钟后自动解封）"));
    }

    let table_index = match pos_type {
        "volume" => 2,
        "long" => 3,
        "short" => 4,
        _ => return Err(anyhow!("无效的持仓类型: {}, 应为 volume/long/short", pos_type)),
    };

    parse_hold_pos_html(&text, table_index, pos_type)
}

/// 解析持仓排名HTML数据
fn parse_hold_pos_html(
    html: &str,
    table_index: usize,
    pos_type: &str,
) -> Result<Vec<FuturesHoldPosition>> {
    let mut positions = Vec::new();

    let table_re = Regex::new(r"<table[^>]*>([\s\S]*?)</table>").unwrap();
    let tables: Vec<_> = table_re.captures_iter(html).collect();

    if tables.len() <= table_index {
        return Err(anyhow!("未找到持仓排名数据表格"));
    }

    let table_content = tables[table_index].get(1).map(|m| m.as_str()).unwrap_or("");

    let row_re = Regex::new(r"<tr[^>]*>([\s\S]*?)</tr>").unwrap();
    let cell_re = Regex::new(r"<td[^>]*>([\s\S]*?)</td>").unwrap();
    let tag_re = Regex::new(r"<[^>]+>").unwrap();

    let value_col_name = match pos_type {
        "volume" => "成交量",
        "long" => "多单持仓",
        "short" => "空单持仓",
        _ => "数值",
    };

    for (i, row_cap) in row_re.captures_iter(table_content).enumerate() {
        if i == 0 {
            continue;
        }

        let row_content = row_cap.get(1).map(|m| m.as_str()).unwrap_or("");
        let cells: Vec<_> = cell_re
            .captures_iter(row_content)
            .filter_map(|c| c.get(1).map(|m| m.as_str().trim()))
            .collect();

        if cells.len() >= 3 {
            let clean_text = |s: &str| -> String {
                tag_re.replace_all(s, "").trim().to_string()
            };

            let rank_str = clean_text(cells[0]);
            let company = clean_text(cells[1]);
            let value_str = clean_text(cells[2]);

            if rank_str.contains("合计") || company.contains("合计") {
                continue;
            }

            let rank = rank_str.parse::<u32>().unwrap_or(0);
            let value = value_str.replace(",", "").parse::<i64>().unwrap_or(0);

            let change = if cells.len() >= 4 {
                clean_text(cells[3]).replace(",", "").parse::<i64>().unwrap_or(0)
            } else {
                0
            };

            if rank > 0 {
                positions.push(FuturesHoldPosition { rank, company, value, change });
            }
        }
    }

    println!("📊 解析到 {} 条{}排名数据", positions.len(), value_col_name);
    Ok(positions)
}
