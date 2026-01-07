//! 期货交易费用和规则相关

use crate::models::{FuturesCommInfo, FuturesFeesInfo, FuturesRule};
use anyhow::{anyhow, Result};
use chrono::Utc;
use chrono_tz::Asia::Shanghai;
use regex::Regex;
use reqwest::Client;

use super::common::{GTJA_CALENDAR_URL, OPENCTP_FEES_URL, QIHUO_COMM_URL};

/// 获取期货交易费用参照表
/// 对应 akshare 的 futures_fees_info() 函数
/// 数据来源: http://openctp.cn/fees.html
pub async fn get_futures_fees_info() -> Result<Vec<FuturesFeesInfo>> {
    let client = Client::new();

    println!("📡 请求期货交易费用数据 URL: {}", OPENCTP_FEES_URL);

    let response = client
        .get(OPENCTP_FEES_URL)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(anyhow!("获取期货交易费用数据失败: {}", response.status()));
    }

    let text = response.text().await?;
    parse_fees_html(&text)
}

/// 解析期货交易费用HTML
fn parse_fees_html(html: &str) -> Result<Vec<FuturesFeesInfo>> {
    let mut fees_list = Vec::new();

    let time_re = Regex::new(r"Generated at ([^.]+)\.").unwrap();
    let updated_at = time_re
        .captures(html)
        .and_then(|c| c.get(1))
        .map(|m| m.as_str().trim().to_string())
        .unwrap_or_else(|| "未知".to_string());

    println!("📅 数据更新时间: {}", updated_at);

    let tbody_start = html.find("<tbody>");
    let tbody_end = html.find("</tbody>");

    if tbody_start.is_none() || tbody_end.is_none() {
        return Err(anyhow!("未找到费用数据表格"));
    }

    let tbody_content = &html[tbody_start.unwrap()..tbody_end.unwrap()];

    for row in tbody_content.split("<tr>").skip(1) {
        let cells: Vec<String> = row
            .split("<td")
            .skip(1)
            .filter_map(|cell| {
                let start = cell.find('>')?;
                let end = cell.find("</td>")?;
                let content = &cell[start + 1..end];
                let clean = content
                    .replace("style=\"background-color:yellow;\"", "")
                    .replace("style=\"background-color:red;\"", "")
                    .trim()
                    .to_string();
                Some(clean)
            })
            .collect();

        if cells.len() >= 16 {
            fees_list.push(FuturesFeesInfo {
                exchange: cells[0].clone(),
                contract_code: cells[1].clone(),
                contract_name: cells[2].clone(),
                product_code: cells[3].clone(),
                product_name: cells[4].clone(),
                contract_size: cells[5].clone(),
                price_tick: cells[6].clone(),
                open_fee_rate: cells[7].clone(),
                open_fee: cells[8].clone(),
                close_fee_rate: cells[9].clone(),
                close_fee: cells[10].clone(),
                close_today_fee_rate: cells[11].clone(),
                close_today_fee: cells[12].clone(),
                long_margin_rate: cells[13].clone(),
                short_margin_rate: cells[15].clone(),
                updated_at: updated_at.clone(),
            });
        }
    }

    println!("📊 解析到 {} 条期货费用数据", fees_list.len());
    Ok(fees_list)
}

/// 获取期货手续费信息
/// 对应 akshare 的 futures_comm_info() 函数
/// 数据来源: https://www.9qihuo.com/qihuoshouxufei
pub async fn get_futures_comm_info(exchange: Option<&str>) -> Result<Vec<FuturesCommInfo>> {
    use scraper::{Html, Selector};

    let client = Client::builder()
        .danger_accept_invalid_certs(true)
        .build()?;

    println!("📡 请求九期网期货手续费数据 URL: {}", QIHUO_COMM_URL);

    let response = client
        .get(QIHUO_COMM_URL)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8")
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(anyhow!("获取九期网数据失败: {}", response.status()));
    }

    let text = response.text().await?;
    let document = Html::parse_document(&text);

    let table_selector = Selector::parse("table").unwrap();
    let tr_selector = Selector::parse("tr").unwrap();
    let td_selector = Selector::parse("td").unwrap();

    let mut all_data: Vec<FuturesCommInfo> = Vec::new();

    let exchange_markers = [
        "上海期货交易所",
        "大连商品交易所",
        "郑州商品交易所",
        "上海国际能源交易中心",
        "广州期货交易所",
        "中国金融期货交易所",
    ];

    let mut current_exchange = String::new();
    let mut skip_rows = 0;

    if let Some(table) = document.select(&table_selector).next() {
        for row in table.select(&tr_selector) {
            let cells: Vec<String> = row
                .select(&td_selector)
                .map(|cell| cell.text().collect::<Vec<_>>().join("").trim().to_string())
                .collect();

            if cells.is_empty() {
                continue;
            }

            let first_cell = &cells[0];
            let mut is_exchange_header = false;
            for marker in &exchange_markers {
                if first_cell.contains(marker) {
                    current_exchange = marker.to_string();
                    skip_rows = 2;
                    is_exchange_header = true;
                    break;
                }
            }

            if is_exchange_header {
                continue;
            }

            if skip_rows > 0 {
                skip_rows -= 1;
                continue;
            }

            if current_exchange.is_empty() || cells.len() < 12 {
                continue;
            }

            if let Some(filter) = exchange {
                if filter != "所有" && current_exchange != filter {
                    continue;
                }
            }

            let contract_str = &cells[0];
            let (contract_name, contract_code) = if let Some(idx) = contract_str.find('(') {
                let name = contract_str[..idx].trim().to_string();
                let code = contract_str[idx + 1..].trim_end_matches(')').to_string();
                (name, code)
            } else {
                (contract_str.clone(), String::new())
            };

            let current_price = cells.get(1).and_then(|s| s.replace(",", "").parse::<f64>().ok());

            let (limit_up, limit_down) = if let Some(limit_str) = cells.get(2) {
                if let Some(idx) = limit_str.find('/') {
                    let up = limit_str[..idx].trim().replace(",", "").parse::<f64>().ok();
                    let down = limit_str[idx + 1..].trim().replace(",", "").parse::<f64>().ok();
                    (up, down)
                } else {
                    (None, None)
                }
            } else {
                (None, None)
            };

            let margin_buy = cells.get(3).and_then(|s| s.trim_end_matches('%').parse::<f64>().ok());
            let margin_sell = cells.get(4).and_then(|s| s.trim_end_matches('%').parse::<f64>().ok());
            let margin_per_lot = cells.get(5).and_then(|s| {
                s.trim_end_matches('元').replace(",", "").parse::<f64>().ok()
            });

            let parse_fee = |s: &str| -> (Option<f64>, Option<f64>) {
                let s = s.trim();
                if s.contains("万分之") {
                    let ratio = s
                        .replace("万分之", "")
                        .split('/')
                        .next()
                        .and_then(|v| v.trim().parse::<f64>().ok())
                        .map(|v| v / 10000.0);
                    (ratio, None)
                } else if s.contains("元") {
                    let yuan = s.replace("元", "").replace(",", "").trim().parse::<f64>().ok();
                    (None, yuan)
                } else {
                    (None, None)
                }
            };

            let (fee_open_ratio, fee_open_yuan) = cells.get(6).map(|s| parse_fee(s)).unwrap_or((None, None));
            let (fee_close_yesterday_ratio, fee_close_yesterday_yuan) = cells.get(7).map(|s| parse_fee(s)).unwrap_or((None, None));
            let (fee_close_today_ratio, fee_close_today_yuan) = cells.get(8).map(|s| parse_fee(s)).unwrap_or((None, None));

            let profit_per_tick = cells.get(9).and_then(|s| s.replace(",", "").parse::<f64>().ok());
            let fee_total = cells.get(10).and_then(|s| {
                s.trim_end_matches('元').replace(",", "").parse::<f64>().ok()
            });
            let net_profit_per_tick = cells.get(11).and_then(|s| s.replace(",", "").parse::<f64>().ok());
            let remark = cells.get(12).cloned();

            all_data.push(FuturesCommInfo {
                exchange: current_exchange.clone(),
                contract_name,
                contract_code,
                current_price,
                limit_up,
                limit_down,
                margin_buy,
                margin_sell,
                margin_per_lot,
                fee_open_ratio,
                fee_open_yuan,
                fee_close_yesterday_ratio,
                fee_close_yesterday_yuan,
                fee_close_today_ratio,
                fee_close_today_yuan,
                profit_per_tick,
                fee_total,
                net_profit_per_tick,
                remark,
            });
        }
    }

    if all_data.is_empty() {
        return Err(anyhow!("未能解析到期货手续费数据，请检查九期网是否可访问"));
    }

    println!("📊 解析到 {} 条期货手续费数据", all_data.len());
    Ok(all_data)
}

/// 获取期货交易规则
/// 对应 akshare 的 futures_rule() 函数
/// 数据来源: https://www.gtjaqh.com/pc/calendar.html
pub async fn get_futures_rule(date: Option<&str>) -> Result<Vec<FuturesRule>> {
    let client = Client::builder()
        .danger_accept_invalid_certs(true)
        .build()?;

    let query_date = date.unwrap_or_else(|| {
        let now = Utc::now().with_timezone(&Shanghai);
        Box::leak(now.format("%Y%m%d").to_string().into_boxed_str())
    });

    let url = format!("{}?date={}", GTJA_CALENDAR_URL, query_date);
    println!("📡 请求期货交易规则数据 URL: {}", url);

    let response = client
        .get(&url)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8")
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(anyhow!("获取期货交易规则数据失败: {}", response.status()));
    }

    let text = response.text().await?;
    parse_futures_rule_html(&text)
}

/// 解析期货交易规则HTML
fn parse_futures_rule_html(html: &str) -> Result<Vec<FuturesRule>> {
    use scraper::{Html, Selector};

    let mut rules = Vec::new();

    if !html.contains("交易保证金比例") && !html.contains("涨跌停板幅度") {
        return Err(anyhow!("未找到交易规则数据表格"));
    }

    let document = Html::parse_document(html);
    let tr_selector = Selector::parse("tr").unwrap();
    let td_selector = Selector::parse("td").unwrap();
    let th_selector = Selector::parse("th").unwrap();

    for row in document.select(&tr_selector) {
        let mut cells: Vec<String> = Vec::new();

        for cell in row.select(&td_selector) {
            let text = cell.text().collect::<Vec<_>>().join("").trim().to_string();
            cells.push(text);
        }

        if cells.is_empty() {
            for cell in row.select(&th_selector) {
                let text = cell.text().collect::<Vec<_>>().join("").trim().to_string();
                cells.push(text);
            }
        }

        if cells.len() <= 1 {
            continue;
        }

        let header_cells: Vec<&String> = cells.iter().take(4).collect();
        let is_header = header_cells.iter().any(|c| {
            c.contains("交易所") || c.contains("交易保证金比例") || *c == "品种" || c.contains("保证金收取标准")
        });

        if is_header {
            continue;
        }

        if cells.len() >= 6 {
            let exchange = cells.first().cloned().unwrap_or_default();
            let product = cells.get(1).cloned().unwrap_or_default();
            let code = cells.get(2).cloned().unwrap_or_default();

            if exchange.is_empty() && product.is_empty() {
                continue;
            }
            if exchange == "交易所" || product == "品种" {
                continue;
            }

            let margin_rate = cells.get(3).and_then(|s| {
                let s = s.trim_end_matches('%').trim();
                if s == "--" || s.is_empty() { None } else { s.parse::<f64>().ok() }
            });

            let price_limit = cells.get(4).and_then(|s| {
                let s = s.trim_end_matches('%').trim();
                if s == "--" || s.is_empty() { None } else { s.parse::<f64>().ok() }
            });

            let contract_size = cells.get(5).and_then(|s| s.parse::<f64>().ok());
            let price_tick = cells.get(6).and_then(|s| s.parse::<f64>().ok());
            let max_order_size = cells.get(7).and_then(|s| s.parse::<u64>().ok());
            let special_note = cells.get(8).cloned().filter(|s| !s.is_empty());
            let remark = cells.get(9).cloned().filter(|s| !s.is_empty());

            rules.push(FuturesRule {
                exchange,
                product,
                code,
                margin_rate,
                price_limit,
                contract_size,
                price_tick,
                max_order_size,
                special_note,
                remark,
            });
        }
    }

    println!("📊 解析到 {} 条期货交易规则数据", rules.len());
    Ok(rules)
}
