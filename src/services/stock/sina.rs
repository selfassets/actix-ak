//! 新浪财经股票接口实现
//!
//! 提供实时行情、历史K线、股票列表等数据
//! 对接 https://hq.sinajs.cn 和 https://quotes.sina.cn

use anyhow::{anyhow, Result};
use chrono::Utc;
use chrono_tz::Asia::Shanghai;
use reqwest::Client;
use crate::models::{StockInfo, StockHistoryData, StockQuery};

/// 获取北京时间字符串（ISO 8601 格式，带+08:00时区）
fn get_beijing_time() -> String {
    Utc::now().with_timezone(&Shanghai).to_rfc3339()
}

/// 获取单只股票信息
///
/// 对接新浪财经实时行情 API: https://hq.sinajs.cn/list=<symbol>
pub async fn get_stock_info(symbol: &str) -> Result<StockInfo> {
    let client = Client::new();
    let url = format!("https://hq.sinajs.cn/list={}", symbol);

    let response = client
        .get(&url)
        .header("Referer", "https://finance.sina.com.cn/")
        .header(
            "User-Agent",
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36",
        )
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(anyhow!("获取股票数据失败: {}", response.status()));
    }

    let bytes = response.bytes().await?;
    let text = encoding_rs::GBK.decode(&bytes).0.to_string();

    parse_sina_stock_info(&text, symbol)
}

/// 解析新浪股票实时数据
fn parse_sina_stock_info(data: &str, symbol: &str) -> Result<StockInfo> {
    // 格式: var hq_str_sh600000="浦发银行,10.00,10.01,10.05,10.07,9.98,10.05,10.06,123456,123456789,...";
    let start = data.find('"').ok_or_else(|| anyhow!("无法解析响应数据"))?;
    let end = data.rfind('"').ok_or_else(|| anyhow!("无法解析响应数据"))?;
    let content = &data[start + 1..end];

    if content.is_empty() {
        return Err(anyhow!("股票代码 {} 可能无效或已退市", symbol));
    }

    let fields: Vec<&str> = content.split(',').collect();
    if fields.len() < 32 {
        return Err(anyhow!("数据字段不足"));
    }

    let name = fields[0].to_string();
    let open = fields[1].parse::<f64>().unwrap_or(0.0);
    let prev_close = fields[2].parse::<f64>().unwrap_or(0.0);
    let current_price = fields[3].parse::<f64>().unwrap_or(0.0);
    let high = fields[4].parse::<f64>().unwrap_or(0.0);
    let low = fields[5].parse::<f64>().unwrap_or(0.0);
    let volume = fields[8].parse::<u64>().unwrap_or(0);
    let amount = fields[9].parse::<f64>().unwrap_or(0.0);

    let change = if prev_close > 0.0 { current_price - prev_close } else { 0.0 };
    let change_percent = if prev_close > 0.0 { (change / prev_close) * 100.0 } else { 0.0 };

    Ok(StockInfo {
        symbol: symbol.to_uppercase(),
        name,
        current_price,
        change,
        change_percent,
        volume,
        amount,
        open,
        high,
        low,
        prev_close,
        market_cap: None, // 实时接口不直接提供市值，需另行计算或从列表接口获取
        updated_at: format!("{} {}", fields[30], fields[31]),
    })
}

/// 获取股票历史K线数据
pub async fn get_stock_history(symbol: &str, query: &StockQuery) -> Result<Vec<StockHistoryData>> {
    let client = Client::new();
    let limit = query.limit.unwrap_or(30);

    // 使用新浪财经分钟线/日线接口 (JSON 格式比较容易解析)
    // scale=240 表示日线
    let url = "https://quotes.sina.cn/cn/api/jsonp_v2.php/=/CN_MarketDataService.getKLineData";

    let response = client
        .get(url)
        .query(&[
            ("symbol", symbol),
            ("scale", "240"),
            ("ma", "no"),
            ("datalen", &limit.to_string()),
        ])
        .header("Referer", "https://finance.sina.com.cn/")
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(anyhow!("获取历史数据失败: {}", response.status()));
    }

    let text = response.text().await?;
    parse_sina_stock_history(&text, symbol)
}

fn parse_sina_stock_history(data: &str, symbol: &str) -> Result<Vec<StockHistoryData>> {
    // 格式: =([{day:"2024-01-01",open:"10.00",high:"10.50",low:"9.80",close:"10.20",volume:"123456"},...]);
    let start = data.find("([").ok_or_else(|| anyhow!("解析历史数据失败"))?;
    let end = data.rfind("])").ok_or_else(|| anyhow!("解析历史数据失败"))?;
    let json_str = &data[start + 1..end + 1];

    let json_data: serde_json::Value = serde_json::from_str(json_str)?;
    let mut history = Vec::new();

    if let Some(arr) = json_data.as_array() {
        for item in arr {
            history.push(StockHistoryData {
                symbol: symbol.to_uppercase(),
                date: item["day"].as_str().unwrap_or("").to_string(),
                open: item["open"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                high: item["high"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                low: item["low"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                close: item["close"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                volume: item["volume"].as_str().unwrap_or("0").parse().unwrap_or(0),
            });
        }
    }

    Ok(history)
}

/// 获取股票列表（实时行情）
/// 对应 akshare 的 stock_zh_a_spot
pub async fn list_stocks(query: &StockQuery) -> Result<Vec<StockInfo>> {
    let client = Client::new();
    let limit = query.limit.unwrap_or(20);

    let url = "http://vip.stock.finance.sina.com.cn/quotes_service/api/json_v2.php/Market_Center.getHQNodeData";

    let response = client
        .get(url)
        .query(&[
            ("node", "hs_a"),
            ("page", "1"),
            ("num", &limit.to_string()),
            ("sort", "symbol"),
            ("asc", "1"),
        ])
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(anyhow!("获取股票列表失败: {}", response.status()));
    }

    let json_data: serde_json::Value = response.json().await?;
    let mut stocks = Vec::new();

    if let Some(arr) = json_data.as_array() {
        for item in arr {
            stocks.push(StockInfo {
                symbol: item["symbol"].as_str().unwrap_or("").to_string(),
                name: item["name"].as_str().unwrap_or("").to_string(),
                current_price: item["trade"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                change: item["pricechange"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                change_percent: item["changepercent"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                volume: item["volume"].as_str().unwrap_or("0").parse().unwrap_or(0),
                amount: item["amount"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                open: item["open"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                high: item["high"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                low: item["low"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                prev_close: item["settlement"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                market_cap: Some(item["mktcap"].as_f64().unwrap_or(0.0) * 10000.0), // 新浪列表单位通常是万元
                updated_at: get_beijing_time(),
            });
        }
    }

    Ok(stocks)
}
