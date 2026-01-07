//! 现货价格及基差数据

use crate::models::{FuturesSpotPrice, FuturesSpotPricePrevious};
use anyhow::{anyhow, Result};
use reqwest::Client;

use super::common::{
    chinese_to_english, extract_contract_month, parse_basis_string, SPOT_PRICE_PREVIOUS_URL,
    SPOT_PRICE_URL,
};

/// 获取期货现货价格及基差数据
/// 对应 akshare 的 futures_spot_price() 函数
/// 数据来源: https://www.100ppi.com/sf/
pub async fn get_futures_spot_price(
    date: &str,
    symbols: Option<Vec<&str>>,
) -> Result<Vec<FuturesSpotPrice>> {
    use scraper::{Html, Selector};

    let formatted_date = if date.len() == 8 {
        format!("{}-{}-{}", &date[0..4], &date[4..6], &date[6..8])
    } else {
        date.to_string()
    };

    let url = format!("{}/day-{}.html", SPOT_PRICE_URL, formatted_date);
    println!("📡 请求现货价格数据 URL: {}", url);

    let client = Client::new();
    let response = client
        .get(&url)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8")
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(anyhow!("获取现货价格数据失败: {}", response.status()));
    }

    let text = response.text().await?;
    let document = Html::parse_document(&text);

    let table_selector = Selector::parse("table#fdata").unwrap();
    let tr_selector = Selector::parse("tr").unwrap();
    let td_selector = Selector::parse("td").unwrap();

    let mut spot_prices = Vec::new();

    let main_table = document.select(&table_selector).next();
    if main_table.is_none() {
        return Err(anyhow!("未找到数据表格(#fdata)"));
    }

    let main_table = main_table.unwrap();
    let rows: Vec<_> = main_table.select(&tr_selector).collect();

    for row in rows {
        let cells: Vec<String> = row
            .select(&td_selector)
            .map(|cell| cell.text().collect::<Vec<_>>().join("").trim().to_string())
            .collect();

        if cells.len() < 10 {
            continue;
        }

        let first_cell = cells[0].replace('\u{a0}', "").trim().to_string();

        if first_cell.contains("交易所") || first_cell == "商品" || first_cell.is_empty() {
            continue;
        }

        let chinese_name = first_cell.trim();
        let symbol = match chinese_to_english(chinese_name) {
            Some(s) => s.to_string(),
            None => {
                if chinese_name.chars().all(|c| c.is_ascii_alphabetic()) {
                    chinese_name.to_uppercase()
                } else {
                    continue;
                }
            }
        };

        if let Some(ref filter_symbols) = symbols {
            if !filter_symbols.iter().any(|s| s.eq_ignore_ascii_case(&symbol)) {
                continue;
            }
        }

        let spot_price = cells
            .get(1)
            .map(|s| s.replace('\u{a0}', "").replace(",", ""))
            .and_then(|s| s.trim().parse::<f64>().ok())
            .unwrap_or(0.0);

        if spot_price == 0.0 {
            continue;
        }

        let near_contract_raw = cells.get(2).map(|s| s.replace('\u{a0}', "")).unwrap_or_default();
        let near_contract_price = cells
            .get(3)
            .map(|s| s.replace('\u{a0}', "").replace(",", ""))
            .and_then(|s| s.trim().parse::<f64>().ok())
            .unwrap_or(0.0);

        let dominant_contract_raw = cells.get(7).map(|s| s.replace('\u{a0}', "")).unwrap_or_default();
        let dominant_contract_price = cells
            .get(8)
            .map(|s| s.replace('\u{a0}', "").replace(",", ""))
            .and_then(|s| s.trim().parse::<f64>().ok())
            .unwrap_or(0.0);

        let near_month = extract_contract_month(&near_contract_raw);
        let dominant_month = extract_contract_month(&dominant_contract_raw);

        let near_contract = format!("{}{}", symbol.to_lowercase(), near_month);
        let dominant_contract = format!("{}{}", symbol.to_lowercase(), dominant_month);

        let near_basis = near_contract_price - spot_price;
        let dom_basis = dominant_contract_price - spot_price;

        let near_basis_rate = if spot_price != 0.0 {
            near_contract_price / spot_price - 1.0
        } else {
            0.0
        };

        let dom_basis_rate = if spot_price != 0.0 {
            dominant_contract_price / spot_price - 1.0
        } else {
            0.0
        };

        spot_prices.push(FuturesSpotPrice {
            date: date.replace("-", ""),
            symbol,
            spot_price,
            near_contract,
            near_contract_price,
            dominant_contract,
            dominant_contract_price,
            near_basis,
            dom_basis,
            near_basis_rate,
            dom_basis_rate,
        });
    }

    println!("📊 解析到 {} 条现货价格数据", spot_prices.len());
    Ok(spot_prices)
}

/// 获取期货现货价格及基差历史数据（包含180日统计）
/// 对应 akshare 的 futures_spot_price_previous() 函数
pub async fn get_futures_spot_price_previous(date: &str) -> Result<Vec<FuturesSpotPricePrevious>> {
    use scraper::{Html, Selector};

    let formatted_date = if date.len() == 8 {
        format!("{}-{}-{}", &date[0..4], &date[4..6], &date[6..8])
    } else {
        date.to_string()
    };

    let url = format!("{}/day-{}.html", SPOT_PRICE_PREVIOUS_URL, formatted_date);
    println!("📡 请求现货价格历史数据 URL: {}", url);

    let client = Client::new();
    let response = client
        .get(&url)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8")
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(anyhow!("获取现货价格历史数据失败: {}", response.status()));
    }

    let text = response.text().await?;
    let document = Html::parse_document(&text);

    let table_selector = Selector::parse("table#fdata").unwrap();
    let tr_selector = Selector::parse("tr").unwrap();
    let td_selector = Selector::parse("td").unwrap();

    let mut spot_prices = Vec::new();

    let main_table = document.select(&table_selector).next();
    if main_table.is_none() {
        return Err(anyhow!("未找到数据表格(#fdata)"));
    }

    let main_table = main_table.unwrap();
    let rows: Vec<_> = main_table.select(&tr_selector).collect();

    for row in rows {
        let cells: Vec<String> = row
            .select(&td_selector)
            .map(|cell| cell.text().collect::<Vec<_>>().join("").trim().to_string())
            .collect();

        if cells.len() < 8 {
            continue;
        }

        let first_cell = cells[0].replace('\u{a0}', "").trim().to_string();

        if first_cell.contains("交易所") || first_cell == "商品" || first_cell.is_empty() {
            continue;
        }

        let spot_price = cells
            .get(1)
            .map(|s| s.replace('\u{a0}', "").replace(",", ""))
            .and_then(|s| s.trim().parse::<f64>().ok())
            .unwrap_or(0.0);

        if spot_price == 0.0 {
            continue;
        }

        let dominant_contract = cells
            .get(2)
            .map(|s| s.replace('\u{a0}', "").trim().to_string())
            .unwrap_or_default();

        let dominant_price = cells
            .get(3)
            .map(|s| s.replace('\u{a0}', "").replace(",", ""))
            .and_then(|s| s.trim().parse::<f64>().ok())
            .unwrap_or(0.0);

        let basis_str = cells.get(4).map(|s| s.replace('\u{a0}', "")).unwrap_or_default();
        let (basis, basis_rate) = parse_basis_string(&basis_str);

        let basis_180d_high = cells
            .get(5)
            .map(|s| s.replace('\u{a0}', "").replace(",", ""))
            .and_then(|s| s.trim().parse::<f64>().ok());

        let basis_180d_low = cells
            .get(6)
            .map(|s| s.replace('\u{a0}', "").replace(",", ""))
            .and_then(|s| s.trim().parse::<f64>().ok());

        let basis_180d_avg = cells
            .get(7)
            .map(|s| s.replace('\u{a0}', "").replace(",", ""))
            .and_then(|s| s.trim().parse::<f64>().ok());

        spot_prices.push(FuturesSpotPricePrevious {
            commodity: first_cell,
            spot_price,
            dominant_contract,
            dominant_price,
            basis,
            basis_rate,
            basis_180d_high,
            basis_180d_low,
            basis_180d_avg,
        });
    }

    println!("📊 解析到 {} 条现货价格历史数据", spot_prices.len());
    Ok(spot_prices)
}

/// 获取期货现货价格日线数据（日期范围）
/// 对应 akshare 的 futures_spot_price_daily() 函数
pub async fn get_futures_spot_price_daily(
    start_date: &str,
    end_date: &str,
    symbols: Option<Vec<&str>>,
) -> Result<Vec<FuturesSpotPrice>> {
    use chrono::NaiveDate;

    let start = NaiveDate::parse_from_str(start_date, "%Y%m%d")
        .map_err(|e| anyhow!("无效的开始日期格式: {}", e))?;
    let end = NaiveDate::parse_from_str(end_date, "%Y%m%d")
        .map_err(|e| anyhow!("无效的结束日期格式: {}", e))?;

    if start > end {
        return Err(anyhow!("开始日期不能大于结束日期"));
    }

    println!("📡 获取现货价格日线数据: {} 至 {}", start_date, end_date);

    let mut all_data = Vec::new();
    let mut current = start;

    while current <= end {
        let date_str = current.format("%Y%m%d").to_string();

        match get_futures_spot_price(&date_str, symbols.clone()).await {
            Ok(data) => {
                if !data.is_empty() {
                    all_data.extend(data);
                }
            }
            Err(e) => {
                println!("  ⚠️ {} 数据获取失败（可能是非交易日）: {}", date_str, e);
            }
        }

        current = current.succ_opt().unwrap_or(current);
    }

    println!("📊 共获取 {} 条现货价格日线数据", all_data.len());
    Ok(all_data)
}
