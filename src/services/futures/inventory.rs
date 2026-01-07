//! 99期货网库存数据

use crate::models::{Futures99Symbol, FuturesInventory99};
use anyhow::{anyhow, Result};
use reqwest::Client;

use super::common::QH99_STOCK_URL;

/// 获取99期货网品种映射表
/// 对应 akshare 的 __get_99_symbol_map() 函数
pub async fn get_99_symbol_map() -> Result<Vec<Futures99Symbol>> {
    let client = Client::builder()
        .danger_accept_invalid_certs(true)
        .build()?;

    println!("📡 请求99期货网品种映射 URL: {}", QH99_STOCK_URL);

    let response = client
        .get(QH99_STOCK_URL)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(anyhow!("获取99期货网品种映射失败: {}", response.status()));
    }

    let text = response.text().await?;

    use scraper::{Html, Selector};
    let document = Html::parse_document(&text);
    let script_selector = Selector::parse("script#__NEXT_DATA__").unwrap();

    let script = document
        .select(&script_selector)
        .next()
        .ok_or_else(|| anyhow!("未找到__NEXT_DATA__脚本标签"))?;

    let json_text = script.text().collect::<String>();
    let json_data: serde_json::Value =
        serde_json::from_str(&json_text).map_err(|e| anyhow!("解析JSON失败: {}", e))?;

    let mut symbols = Vec::new();

    if let Some(variety_list) = json_data["props"]["pageProps"]["data"]["varietyListData"].as_array() {
        for variety in variety_list {
            if let Some(product_list) = variety["productList"].as_array() {
                for product in product_list {
                    let product_id = product["productId"].as_i64().unwrap_or(0);
                    let name = product["name"].as_str().unwrap_or("").to_string();
                    let code = product["code"].as_str().unwrap_or("").to_string();

                    if product_id > 0 && !name.is_empty() {
                        symbols.push(Futures99Symbol { product_id, name, code });
                    }
                }
            }
        }
    }

    println!("📊 解析到 {} 个品种映射", symbols.len());
    Ok(symbols)
}

/// 获取99期货网库存数据
/// 对应 akshare 的 futures_inventory_99() 函数
/// symbol: 品种名称（如"豆一"）或代码（如"A"）
pub async fn get_futures_inventory_99(symbol: &str) -> Result<Vec<FuturesInventory99>> {
    let symbols = get_99_symbol_map().await?;

    let product_id = symbols
        .iter()
        .find(|s| s.name == symbol || s.code.eq_ignore_ascii_case(symbol))
        .map(|s| s.product_id)
        .ok_or_else(|| anyhow!("未找到品种 {} 对应的编号", symbol))?;

    println!("📡 品种 {} 对应的ID: {}", symbol, product_id);

    let client = Client::builder()
        .danger_accept_invalid_certs(true)
        .build()?;

    let url = format!("{}?productId={}", QH99_STOCK_URL, product_id);
    println!("📡 请求99期货网库存数据 URL: {}", url);

    let response = client
        .get(&url)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(anyhow!("获取99期货网库存数据失败: {}", response.status()));
    }

    let text = response.text().await?;

    use scraper::{Html, Selector};
    let document = Html::parse_document(&text);
    let script_selector = Selector::parse("script#__NEXT_DATA__").unwrap();

    let script = document
        .select(&script_selector)
        .next()
        .ok_or_else(|| anyhow!("未找到__NEXT_DATA__脚本标签"))?;

    let json_text = script.text().collect::<String>();
    let json_data: serde_json::Value =
        serde_json::from_str(&json_text).map_err(|e| anyhow!("解析JSON失败: {}", e))?;

    let mut inventory_list = Vec::new();

    if let Some(list) = json_data["props"]["pageProps"]["data"]["positionTrendChartListData"]["list"].as_array() {
        for item in list {
            if let Some(arr) = item.as_array() {
                let date = arr.get(0).and_then(|v| v.as_str()).unwrap_or("").to_string();

                let close_price = arr.get(1).and_then(|v| {
                    if v.is_null() {
                        None
                    } else if let Some(s) = v.as_str() {
                        s.parse::<f64>().ok()
                    } else {
                        v.as_f64()
                    }
                });

                let inventory = arr.get(2).and_then(|v| {
                    if v.is_null() {
                        None
                    } else if let Some(n) = v.as_i64() {
                        Some(n as f64)
                    } else if let Some(n) = v.as_f64() {
                        Some(n)
                    } else {
                        None
                    }
                });

                if !date.is_empty() {
                    inventory_list.push(FuturesInventory99 { date, close_price, inventory });
                }
            }
        }
    }

    inventory_list.sort_by(|a, b| a.date.cmp(&b.date));

    println!("📊 解析到 {} 条库存数据", inventory_list.len());
    Ok(inventory_list)
}
