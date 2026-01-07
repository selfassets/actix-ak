//! 期货持仓排名数据模块
//!
//! 提供各交易所持仓排名数据的获取和处理

use anyhow::{anyhow, Result};
use regex::Regex;
use reqwest::Client;
use std::collections::HashMap;

use crate::models::{
    PositionRankData, RankSum, RankTableResponse, SinaHoldPosType, SinaHoldPosition,
};

/// 上海期货交易所会员成交及持仓排名表API
const SHFE_VOL_RANK_URL: &str = "https://www.shfe.com.cn/data/tradedata/future/dailydata/pm";

/// 中国金融期货交易所持仓排名API
const CFFEX_VOL_RANK_URL: &str = "http://www.cffex.com.cn/sj/ccpm";

/// 大连商品交易所持仓排名API
const DCE_VOL_RANK_URL: &str =
    "http://www.dce.com.cn/dcereport/publicweb/dailystat/memberDealPosi/batchDownload";

/// 从合约代码中提取品种代码
fn extract_variety(symbol: &str) -> String {
    let re = Regex::new(r"^([A-Za-z]+)").unwrap();
    re.captures(symbol)
        .and_then(|c| c.get(1))
        .map(|m| m.as_str().to_uppercase())
        .unwrap_or_default()
}

// ==================== 新浪期货持仓排名 ====================

/// 新浪财经-期货-成交持仓排名
/// 对应 akshare 的 futures_hold_pos_sina() 函数
/// 数据来源: https://vip.stock.finance.sina.com.cn/q/view/vFutures_Positions_cjcc.php
///
/// symbol: 数据类型，可选 "成交量"/"多单持仓"/"空单持仓" 或 "volume"/"long"/"short"
/// contract: 期货合约代码，如 "OI2501", "IC2403"
/// date: 查询日期，格式 YYYYMMDD
pub async fn futures_hold_pos_sina(
    symbol: &str,
    contract: &str,
    date: &str,
) -> Result<Vec<SinaHoldPosition>> {
    let pos_type = SinaHoldPosType::from_str(symbol).ok_or_else(|| {
        anyhow!(
            "无效的symbol参数: {}，可选: 成交量/多单持仓/空单持仓",
            symbol
        )
    })?;

    let client = Client::new();

    // 格式化日期为 YYYY-MM-DD
    let formatted_date = format!("{}-{}-{}", &date[0..4], &date[4..6], &date[6..8]);

    let url = "https://vip.stock.finance.sina.com.cn/q/view/vFutures_Positions_cjcc.php";

    println!(
        "📡 请求新浪期货持仓数据 URL: {}?t_breed={}&t_date={}",
        url, contract, formatted_date
    );

    let response = client
        .get(url)
        .query(&[("t_breed", contract), ("t_date", &formatted_date)])
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
        .header("Referer", "https://vip.stock.finance.sina.com.cn/")
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(anyhow!("获取新浪期货持仓数据失败: {}", response.status()));
    }

    // 使用GBK编码读取
    let bytes = response.bytes().await?;
    let html = encoding_rs::GBK.decode(&bytes).0.to_string();

    // 解析HTML表格
    let document = scraper::Html::parse_document(&html);
    let table_selector = scraper::Selector::parse("table").unwrap();
    let tables: Vec<_> = document.select(&table_selector).collect();

    let table_index = pos_type.table_index();
    if tables.len() <= table_index {
        return Err(anyhow!("未找到数据表格，可能是非交易日或合约不存在"));
    }

    let target_table = tables[table_index];
    let row_selector = scraper::Selector::parse("tr").unwrap();
    let cell_selector = scraper::Selector::parse("td").unwrap();

    let mut result: Vec<SinaHoldPosition> = Vec::new();

    for row in target_table.select(&row_selector) {
        let cells: Vec<_> = row.select(&cell_selector).collect();

        if cells.len() < 3 {
            continue;
        }

        let rank_text = cells[0].text().collect::<String>().trim().to_string();
        let company_text = cells[1].text().collect::<String>().trim().to_string();
        let value_text = cells[2].text().collect::<String>().trim().replace(",", "");
        let change_text = if cells.len() > 3 {
            cells[3].text().collect::<String>().trim().replace(",", "")
        } else {
            "0".to_string()
        };

        let rank: i32 = match rank_text.parse() {
            Ok(r) => r,
            Err(_) => continue,
        };

        if rank <= 0 {
            continue;
        }

        let value: i64 = value_text.parse().unwrap_or(0);
        let change: i64 = change_text.parse().unwrap_or(0);

        result.push(SinaHoldPosition {
            rank,
            company: company_text,
            value,
            change,
        });
    }

    println!("📊 解析到 {} 条持仓排名数据", result.len());
    Ok(result)
}

// ==================== 上期所持仓排名 ====================

/// 获取上海期货交易所会员成交及持仓排名表
/// 对应 akshare 的 get_shfe_rank_table() 函数
/// 数据来源: https://www.shfe.com.cn/
/// date: 交易日期，格式 YYYYMMDD，数据从 20020107 开始
/// vars_list: 品种代码列表，如 ["CU", "AL"]，为空时返回所有品种
pub async fn get_shfe_rank_table(
    date: &str,
    vars_list: Option<Vec<&str>>,
) -> Result<Vec<RankTableResponse>> {
    let client = Client::new();

    let url = format!("{}{}.dat", SHFE_VOL_RANK_URL, date);
    println!("📡 请求上期所持仓排名数据 URL: {}", url);

    let response = client
        .get(&url)
        .header(
            "User-Agent",
            "Mozilla/4.0 (compatible; MSIE 5.5; Windows NT)",
        )
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(anyhow!("获取上期所持仓排名数据失败: {}", response.status()));
    }

    let text = response.text().await?;

    let json_data: serde_json::Value =
        serde_json::from_str(&text).map_err(|e| anyhow!("解析JSON失败: {}", e))?;

    let cursor = json_data["o_cursor"]
        .as_array()
        .ok_or_else(|| anyhow!("未找到o_cursor数据"))?;

    let mut symbol_data: HashMap<String, Vec<PositionRankData>> = HashMap::new();

    for item in cursor {
        let rank = item["RANK"].as_i64().unwrap_or(0) as i32;
        if rank <= 0 {
            continue;
        }

        let symbol = item["INSTRUMENTID"]
            .as_str()
            .unwrap_or("")
            .trim()
            .to_uppercase();
        if symbol.is_empty() {
            continue;
        }

        let variety = extract_variety(&symbol);

        if let Some(ref vars) = vars_list {
            if !vars.iter().any(|v| v.eq_ignore_ascii_case(&variety)) {
                continue;
            }
        }

        let data = PositionRankData {
            rank,
            vol_party_name: item["PARTICIPANTABBR1"]
                .as_str()
                .unwrap_or("")
                .trim()
                .to_string(),
            vol: item["CJ1"].as_i64().unwrap_or(0),
            vol_chg: item["CJ1_CHG"].as_i64().unwrap_or(0),
            long_party_name: item["PARTICIPANTABBR2"]
                .as_str()
                .unwrap_or("")
                .trim()
                .to_string(),
            long_open_interest: item["CJ2"].as_i64().unwrap_or(0),
            long_open_interest_chg: item["CJ2_CHG"].as_i64().unwrap_or(0),
            short_party_name: item["PARTICIPANTABBR3"]
                .as_str()
                .unwrap_or("")
                .trim()
                .to_string(),
            short_open_interest: item["CJ3"].as_i64().unwrap_or(0),
            short_open_interest_chg: item["CJ3_CHG"].as_i64().unwrap_or(0),
            symbol: symbol.clone(),
            variety,
        };

        symbol_data.entry(symbol).or_default().push(data);
    }

    let mut result: Vec<RankTableResponse> = symbol_data
        .into_iter()
        .map(|(symbol, data)| RankTableResponse { symbol, data })
        .collect();

    result.sort_by(|a, b| a.symbol.cmp(&b.symbol));

    println!("📊 解析到 {} 个合约的持仓排名数据", result.len());
    Ok(result)
}

// ==================== 中金所持仓排名 ====================

/// 获取中国金融期货交易所前20会员持仓排名数据
/// 对应 akshare 的 get_cffex_rank_table() 函数
/// 数据来源: http://www.cffex.com.cn/ccpm/
/// date: 交易日期，格式 YYYYMMDD，数据从 20100416 开始
/// vars_list: 品种代码列表，如 ["IF", "IC"]，为空时返回所有品种
pub async fn get_cffex_rank_table(
    date: &str,
    vars_list: Option<Vec<&str>>,
) -> Result<Vec<RankTableResponse>> {
    let client = Client::new();

    let cffex_vars = vec!["IF", "IC", "IM", "IH", "T", "TF", "TS", "TL"];

    let target_vars: Vec<&str> = match vars_list {
        Some(vars) => vars
            .into_iter()
            .filter(|v| cffex_vars.iter().any(|cv| cv.eq_ignore_ascii_case(v)))
            .collect(),
        None => cffex_vars.clone(),
    };

    let mut all_results: Vec<RankTableResponse> = Vec::new();

    let year_month = &date[..6];
    let day = &date[6..8];

    for var in target_vars {
        let url = format!(
            "{}/{}/{}/{}_1.csv",
            CFFEX_VOL_RANK_URL, year_month, day, var
        );
        println!("📡 请求中金所 {} 持仓排名数据 URL: {}", var, url);

        let response = client
            .get(&url)
            .header(
                "User-Agent",
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36",
            )
            .send()
            .await;

        let response = match response {
            Ok(r) => r,
            Err(e) => {
                log::warn!("获取 {} 数据失败: {}", var, e);
                continue;
            }
        };

        if !response.status().is_success() {
            log::warn!("获取 {} 数据失败: {}", var, response.status());
            continue;
        }

        let bytes = response.bytes().await?;
        let text = encoding_rs::GBK.decode(&bytes).0.to_string();

        let mut symbol_data: HashMap<String, Vec<PositionRankData>> = HashMap::new();

        let lines: Vec<&str> = text.lines().collect();

        for line in lines {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            if line.contains("交易日") || line.contains("合约") || line.contains("名次") {
                continue;
            }

            let fields: Vec<&str> = line.split(',').collect();
            if fields.len() < 12 {
                continue;
            }

            let symbol = fields[1].trim().to_string();
            if symbol.is_empty() {
                continue;
            }

            let rank = fields[2].trim().parse::<i32>().unwrap_or(0);
            if rank <= 0 {
                continue;
            }

            let variety = extract_variety(&symbol);

            let data = PositionRankData {
                rank,
                vol_party_name: fields[3].trim().to_string(),
                vol: fields[4].trim().replace(",", "").parse().unwrap_or(0),
                vol_chg: fields[5].trim().replace(",", "").parse().unwrap_or(0),
                long_party_name: fields[6].trim().to_string(),
                long_open_interest: fields[7].trim().replace(",", "").parse().unwrap_or(0),
                long_open_interest_chg: fields[8].trim().replace(",", "").parse().unwrap_or(0),
                short_party_name: fields[9].trim().to_string(),
                short_open_interest: fields[10].trim().replace(",", "").parse().unwrap_or(0),
                short_open_interest_chg: fields[11].trim().replace(",", "").parse().unwrap_or(0),
                symbol: symbol.clone(),
                variety,
            };

            symbol_data.entry(symbol).or_default().push(data);
        }

        for (symbol, data) in symbol_data {
            all_results.push(RankTableResponse { symbol, data });
        }
    }

    all_results.sort_by(|a, b| a.symbol.cmp(&b.symbol));

    println!("📊 解析到 {} 个合约的持仓排名数据", all_results.len());
    Ok(all_results)
}


// ==================== 郑商所持仓排名 ====================

/// 获取郑州商品交易所前20会员持仓排名数据
/// 对应 akshare 的 get_rank_table_czce() 函数
/// 数据来源: https://www.czce.com.cn/cn/jysj/ccpm/H077003004index_1.htm
/// date: 交易日期，格式 YYYYMMDD，数据从 20151008 开始
pub async fn get_rank_table_czce(date: &str) -> Result<Vec<RankTableResponse>> {
    use calamine::{open_workbook_auto_from_rs, Reader};

    let client = Client::new();

    let year = &date[..4];
    let url = if date >= "20251102" {
        format!(
            "https://www.czce.com.cn/cn/DFSStaticFiles/Future/{}/{}/FutureDataHolding.xlsx",
            year, date
        )
    } else {
        format!(
            "https://www.czce.com.cn/cn/DFSStaticFiles/Future/{}/{}/FutureDataHolding.xls",
            year, date
        )
    };

    println!("📡 请求郑商所持仓排名数据 URL: {}", url);

    let response = client
        .get(&url)
        .header(
            "User-Agent",
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36",
        )
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(anyhow!("获取郑商所持仓排名数据失败: {}", response.status()));
    }

    let bytes = response.bytes().await?;

    use std::io::Cursor;
    let cursor = Cursor::new(bytes.as_ref());

    let mut workbook =
        open_workbook_auto_from_rs(cursor).map_err(|e| anyhow!("打开Excel文件失败: {}", e))?;

    let sheet_names = workbook.sheet_names();
    if sheet_names.is_empty() {
        return Err(anyhow!("Excel文件没有工作表"));
    }
    let first_sheet = sheet_names[0].clone();

    let range = workbook
        .worksheet_range(&first_sheet)
        .map_err(|e| anyhow!("读取工作表失败: {}", e))?;

    let mut symbol_data: HashMap<String, Vec<PositionRankData>> = HashMap::new();
    let mut current_symbol = String::new();
    let symbol_re = Regex::new(r"([A-Za-z]+\d+)").unwrap();

    for row in range.rows() {
        if row.is_empty() {
            continue;
        }

        let first_cell = row[0].to_string();

        if first_cell.contains("品种") || first_cell.contains("合约") {
            if let Some(cap) = symbol_re.captures(&first_cell) {
                current_symbol = cap
                    .get(1)
                    .map(|m| m.as_str().to_uppercase())
                    .unwrap_or_default();
            }
            continue;
        }

        if first_cell.contains("名次") || first_cell.contains("合计") || first_cell.is_empty() {
            continue;
        }

        if row.len() >= 10 && !current_symbol.is_empty() {
            let rank = row[0].to_string().parse::<i32>().unwrap_or(0);
            if rank <= 0 {
                continue;
            }

            let variety = extract_variety(&current_symbol);

            let parse_num = |s: &str| -> i64 {
                s.replace(",", "")
                    .replace("-", "0")
                    .trim()
                    .parse()
                    .unwrap_or(0)
            };

            let data = PositionRankData {
                rank,
                vol_party_name: row[1].to_string(),
                vol: parse_num(&row[2].to_string()),
                vol_chg: parse_num(&row[3].to_string()),
                long_party_name: row[4].to_string(),
                long_open_interest: parse_num(&row[5].to_string()),
                long_open_interest_chg: parse_num(&row[6].to_string()),
                short_party_name: row[7].to_string(),
                short_open_interest: parse_num(&row[8].to_string()),
                short_open_interest_chg: parse_num(&row[9].to_string()),
                symbol: current_symbol.clone(),
                variety,
            };

            symbol_data.entry(current_symbol.clone()).or_default().push(data);
        }
    }

    let mut result: Vec<RankTableResponse> = symbol_data
        .into_iter()
        .map(|(symbol, data)| RankTableResponse { symbol, data })
        .collect();

    result.sort_by(|a, b| a.symbol.cmp(&b.symbol));

    println!("📊 解析到 {} 个合约的持仓排名数据", result.len());
    Ok(result)
}

// ==================== 大商所持仓排名 ====================

/// 获取大连商品交易所前20会员持仓排名数据
/// 对应 akshare 的 get_dce_rank_table() 函数
/// 数据来源: http://www.dce.com.cn/dalianshangpin/xqsj/tjsj26/rtj/rcjccpm/index.html
/// date: 交易日期，格式 YYYYMMDD，数据从 20060104 开始
/// vars_list: 品种代码列表，如 ["M", "Y"]，为空时返回所有品种
pub async fn get_dce_rank_table(
    date: &str,
    vars_list: Option<Vec<&str>>,
) -> Result<Vec<RankTableResponse>> {
    let client = Client::builder().cookie_store(true).build()?;

    let _home_resp = client
        .get("http://www.dce.com.cn/dalianshangpin/xqsj/tjsj26/rtj/rcjccpm/index.html")
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
        .send()
        .await;

    let payload = serde_json::json!({
        "tradeDate": date,
        "varietyId": "a",
        "contractId": "a2601",
        "tradeType": "1",
        "lang": "zh"
    });

    println!("📡 请求大商所持仓排名数据 URL: {}", DCE_VOL_RANK_URL);

    let response = client
        .post(DCE_VOL_RANK_URL)
        .json(&payload)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
        .header("Accept", "application/json, text/plain, */*")
        .header("Accept-Language", "zh-CN,zh;q=0.9,en;q=0.8")
        .header("Accept-Encoding", "gzip, deflate")
        .header("Origin", "http://www.dce.com.cn")
        .header("Referer", "http://www.dce.com.cn/dalianshangpin/xqsj/tjsj26/rtj/rcjccpm/index.html")
        .header("Connection", "keep-alive")
        .send()
        .await?;

    if !response.status().is_success() {
        if response.status().as_u16() == 412 {
            return Err(anyhow!(
                "大商所API访问被拒绝(412)，该交易所有反爬虫机制。\n\
                建议: 1) 稍后重试 2) 使用浏览器手动下载数据 3) 使用akshare的futures_dce_position_rank()接口"
            ));
        }
        return Err(anyhow!("获取大商所持仓排名数据失败: {}", response.status()));
    }

    let bytes = response.bytes().await?;

    use std::io::{Cursor, Read};
    let cursor = Cursor::new(bytes.as_ref());
    let mut archive =
        zip::ZipArchive::new(cursor).map_err(|e| anyhow!("打开ZIP文件失败: {}", e))?;

    let mut symbol_data: HashMap<String, Vec<PositionRankData>> = HashMap::new();

    for i in 0..archive.len() {
        let mut file = archive
            .by_index(i)
            .map_err(|e| anyhow!("读取ZIP文件失败: {}", e))?;

        let file_name = file.name().to_string();

        if !file_name.starts_with(date) {
            continue;
        }

        let parts: Vec<&str> = file_name.split('_').collect();
        if parts.len() < 2 {
            continue;
        }
        let symbol = parts[1].to_uppercase();
        let variety = extract_variety(&symbol);

        if let Some(ref vars) = vars_list {
            if !vars.iter().any(|v| v.eq_ignore_ascii_case(&variety)) {
                continue;
            }
        }

        let mut content = Vec::new();
        file.read_to_end(&mut content)?;

        let text = match String::from_utf8(content.clone()) {
            Ok(s) => s,
            Err(_) => encoding_rs::GBK.decode(&content).0.to_string(),
        };

        let lines: Vec<&str> = text.lines().collect();

        let mut vol_start = None;
        let mut long_start = None;
        let mut short_start = None;

        for (i, line) in lines.iter().enumerate() {
            if line.contains("名次") {
                if vol_start.is_none() {
                    vol_start = Some(i);
                } else if long_start.is_none() {
                    long_start = Some(i);
                } else if short_start.is_none() {
                    short_start = Some(i);
                }
            }
        }

        if vol_start.is_none() || long_start.is_none() || short_start.is_none() {
            continue;
        }

        let vol_data = parse_dce_table_section(&lines, vol_start.unwrap(), long_start.unwrap());
        let long_data = parse_dce_table_section(&lines, long_start.unwrap(), short_start.unwrap());
        let short_data = parse_dce_table_section(&lines, short_start.unwrap(), lines.len());

        let max_len = vol_data.len().max(long_data.len()).max(short_data.len());
        let mut rank_data = Vec::new();

        for i in 0..max_len {
            let (vol_name, vol, vol_chg) = vol_data.get(i).cloned().unwrap_or_default();
            let (long_name, long_oi, long_chg) = long_data.get(i).cloned().unwrap_or_default();
            let (short_name, short_oi, short_chg) = short_data.get(i).cloned().unwrap_or_default();

            rank_data.push(PositionRankData {
                rank: (i + 1) as i32,
                vol_party_name: vol_name,
                vol,
                vol_chg,
                long_party_name: long_name,
                long_open_interest: long_oi,
                long_open_interest_chg: long_chg,
                short_party_name: short_name,
                short_open_interest: short_oi,
                short_open_interest_chg: short_chg,
                symbol: symbol.clone(),
                variety: variety.clone(),
            });
        }

        if !rank_data.is_empty() {
            symbol_data.insert(symbol, rank_data);
        }
    }

    let mut result: Vec<RankTableResponse> = symbol_data
        .into_iter()
        .map(|(symbol, data)| RankTableResponse { symbol, data })
        .collect();

    result.sort_by(|a, b| a.symbol.cmp(&b.symbol));

    println!("📊 解析到 {} 个合约的持仓排名数据", result.len());
    Ok(result)
}

/// 解析大商所表格数据段
fn parse_dce_table_section(lines: &[&str], start: usize, end: usize) -> Vec<(String, i64, i64)> {
    let mut result = Vec::new();

    for line in lines.iter().take(end).skip(start + 1) {
        let line = line.trim();
        if line.is_empty() || line.contains("总计") || line.contains("合计") {
            continue;
        }

        let fields: Vec<&str> = line
            .split(['\t', ' '])
            .filter(|s| !s.is_empty())
            .collect();

        if fields.len() >= 4 {
            let name = fields[1].trim().to_string();
            let value: i64 = fields[2].trim().replace(",", "").parse().unwrap_or(0);
            let change: i64 = fields[3].trim().replace(",", "").parse().unwrap_or(0);

            result.push((name, value, change));
        }
    }

    result
}


// ==================== 大商所持仓排名（备用接口） ====================

/// 大连商品交易所-每日持仓排名-具体合约
/// 对应 akshare 的 futures_dce_position_rank() 函数
pub async fn futures_dce_position_rank(
    date: &str,
    vars_list: Option<Vec<&str>>,
) -> Result<Vec<RankTableResponse>> {
    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()?;

    let url = "http://www.dce.com.cn/dcereport/publicweb/dailystat/memberDealPosi/batchDownload";

    let payload = serde_json::json!({
        "tradeDate": date,
        "varietyId": "a",
        "contractId": "a2601",
        "tradeType": "1",
        "lang": "zh"
    });

    println!("📡 请求大商所持仓排名数据(ZIP) URL: {}", url);

    let response = client
        .post(url)
        .json(&payload)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
        .header("Accept", "*/*")
        .header("Accept-Language", "zh-CN,zh;q=0.9,en;q=0.8")
        .header("Accept-Encoding", "gzip, deflate")
        .header("Origin", "http://www.dce.com.cn")
        .header("Referer", "http://www.dce.com.cn/dalianshangpin/xqsj/tjsj26/rtj/rcjccpm/index.html")
        .send()
        .await?;

    if !response.status().is_success() {
        if response.status().as_u16() == 412 {
            return Err(anyhow!(
                "大商所API访问被拒绝(412)，该交易所有反爬虫机制。\n\
                建议: 1) 稍后重试 2) 使用浏览器手动下载数据 3) 尝试 futures_dce_position_rank_other() 接口"
            ));
        }
        return Err(anyhow!("获取大商所持仓排名数据失败: {}", response.status()));
    }

    let bytes = response.bytes().await?;

    use std::io::{Cursor, Read};
    let cursor = Cursor::new(bytes.as_ref());
    let mut archive = match zip::ZipArchive::new(cursor) {
        Ok(a) => a,
        Err(e) => {
            return Err(anyhow!(
                "打开ZIP文件失败: {}，可能是非交易日或数据不存在",
                e
            ))
        }
    };

    let mut symbol_data: HashMap<String, Vec<PositionRankData>> = HashMap::new();

    for i in 0..archive.len() {
        let mut file = archive
            .by_index(i)
            .map_err(|e| anyhow!("读取ZIP文件失败: {}", e))?;

        let file_name = file.name().to_string();

        if !file_name.starts_with(date) {
            continue;
        }

        let parts: Vec<&str> = file_name.split('_').collect();
        if parts.len() < 2 {
            continue;
        }
        let symbol = parts[1].to_uppercase();
        let variety = extract_variety(&symbol);

        if let Some(ref vars) = vars_list {
            if !vars.iter().any(|v| v.eq_ignore_ascii_case(&variety)) {
                continue;
            }
        }

        let mut content = Vec::new();
        file.read_to_end(&mut content)?;

        let text = match String::from_utf8(content.clone()) {
            Ok(s) => s,
            Err(_) => encoding_rs::GBK.decode(&content).0.to_string(),
        };

        match parse_dce_position_file(&text, &symbol, &variety) {
            Ok(data) => {
                if !data.is_empty() {
                    symbol_data.insert(symbol, data);
                }
            }
            Err(e) => {
                log::warn!("解析 {} 数据失败: {}", symbol, e);
            }
        }
    }

    let mut result: Vec<RankTableResponse> = symbol_data
        .into_iter()
        .map(|(symbol, data)| RankTableResponse { symbol, data })
        .collect();

    result.sort_by(|a, b| a.symbol.cmp(&b.symbol));

    println!("📊 解析到 {} 个合约的持仓排名数据", result.len());
    Ok(result)
}

/// 解析大商所持仓排名文件内容
fn parse_dce_position_file(
    text: &str,
    symbol: &str,
    variety: &str,
) -> Result<Vec<PositionRankData>> {
    let lines: Vec<&str> = text.lines().collect();

    let has_member_type = lines.iter().any(|l| l.contains("会员类别"));
    let effective_lines: Vec<&str> = if has_member_type {
        lines[..lines.len().saturating_sub(6)].to_vec()
    } else {
        lines.clone()
    };

    let mut start_indices: Vec<usize> = Vec::new();
    for (i, line) in effective_lines.iter().enumerate() {
        if line.starts_with("名次") || line.contains("\t名次") {
            start_indices.push(i);
        }
    }

    if start_indices.len() < 3 {
        return Err(anyhow!("未找到完整的三个表格"));
    }

    if start_indices.len() >= 2 && start_indices[1] - start_indices[0] < 5 {
        return Ok(Vec::new());
    }

    let mut end_indices: Vec<usize> = Vec::new();
    for (i, line) in effective_lines.iter().enumerate() {
        if line.contains("总计") || line.contains("合计") {
            end_indices.push(i);
        }
    }

    if end_indices.len() < 3 {
        return Err(anyhow!("未找到完整的三个表格结束标记"));
    }

    let vol_data = parse_dce_rank_section(&effective_lines, start_indices[0] + 1, end_indices[0]);
    let long_data = parse_dce_rank_section(&effective_lines, start_indices[1] + 1, end_indices[1]);
    let short_data = parse_dce_rank_section(&effective_lines, start_indices[2] + 1, end_indices[2]);

    let max_len = vol_data.len().max(long_data.len()).max(short_data.len());
    let mut result = Vec::new();

    for i in 0..max_len {
        let (vol_name, vol, vol_chg) = vol_data.get(i).cloned().unwrap_or_default();
        let (long_name, long_oi, long_chg) = long_data.get(i).cloned().unwrap_or_default();
        let (short_name, short_oi, short_chg) = short_data.get(i).cloned().unwrap_or_default();

        result.push(PositionRankData {
            rank: (i + 1) as i32,
            vol_party_name: vol_name,
            vol,
            vol_chg,
            long_party_name: long_name,
            long_open_interest: long_oi,
            long_open_interest_chg: long_chg,
            short_party_name: short_name,
            short_open_interest: short_oi,
            short_open_interest_chg: short_chg,
            symbol: symbol.to_string(),
            variety: variety.to_string(),
        });
    }

    Ok(result)
}

/// 解析大商所排名表格段落
fn parse_dce_rank_section(lines: &[&str], start: usize, end: usize) -> Vec<(String, i64, i64)> {
    let mut result = Vec::new();

    for i in start..end {
        if i >= lines.len() {
            break;
        }
        let line = lines[i].trim();
        if line.is_empty() {
            continue;
        }

        let fields: Vec<&str> = line.split('\t').filter(|s| !s.is_empty()).collect();

        let fields = if fields.len() < 4 {
            line.split_whitespace().collect::<Vec<&str>>()
        } else {
            fields
        };

        if fields.len() >= 4 {
            let name = fields[1].trim().replace(",", "").replace("-", "");
            let value: i64 = fields[2]
                .trim()
                .replace(",", "")
                .replace("-", "0")
                .parse()
                .unwrap_or(0);
            let change: i64 = fields[3]
                .trim()
                .replace(",", "")
                .replace("-", "0")
                .parse()
                .unwrap_or(0);

            if !name.is_empty() {
                result.push((name, value, change));
            }
        }
    }

    result
}

/// 大连商品交易所-每日持仓排名-具体合约-补充接口
/// 对应 akshare 的 futures_dce_position_rank_other() 函数
pub async fn futures_dce_position_rank_other(date: &str) -> Result<Vec<RankTableResponse>> {
    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()?;

    let url = "http://www.dce.com.cn/publicweb/quotesdata/memberDealPosiQuotes.html";

    let year: i32 = date[0..4].parse().map_err(|_| anyhow!("无效的日期格式"))?;
    let month: i32 = date[4..6].parse().map_err(|_| anyhow!("无效的日期格式"))?;
    let day: i32 = date[6..8].parse().map_err(|_| anyhow!("无效的日期格式"))?;

    println!("📡 请求大商所持仓排名数据(HTML) URL: {}", url);

    let payload = [
        ("memberDealPosiQuotes.variety", "c"),
        ("memberDealPosiQuotes.trade_type", "0"),
        ("year", &year.to_string()),
        ("month", &(month - 1).to_string()),
        ("day", &day.to_string()),
        ("contract.contract_id", "all"),
        ("contract.variety_id", "c"),
        ("contract", ""),
    ];

    let response = client
        .post(url)
        .form(&payload)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
        .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8")
        .header("Accept-Language", "zh-CN,zh;q=0.9,en;q=0.8")
        .header("Origin", "http://www.dce.com.cn")
        .header("Referer", "http://www.dce.com.cn/publicweb/quotesdata/memberDealPosiQuotes.html")
        .send()
        .await?;

    if !response.status().is_success() {
        if response.status().as_u16() == 412 {
            return Err(anyhow!(
                "大商所API访问被拒绝(412)，该交易所有反爬虫机制。\n\
                建议: 1) 稍后重试 2) 使用浏览器手动下载数据"
            ));
        }
        return Err(anyhow!("获取大商所品种列表失败: {}", response.status()));
    }

    let html = response.text().await?;

    let symbol_list = parse_dce_symbol_list(&html)?;

    if symbol_list.is_empty() {
        return Err(anyhow!("未找到品种列表，可能是非交易日"));
    }

    println!("📊 找到 {} 个品种", symbol_list.len());

    let mut all_results: Vec<RankTableResponse> = Vec::new();

    for symbol in &symbol_list {
        let payload = [
            ("memberDealPosiQuotes.variety", symbol.as_str()),
            ("memberDealPosiQuotes.trade_type", "0"),
            ("year", &year.to_string()),
            ("month", &(month - 1).to_string()),
            ("day", &day.to_string()),
            ("contract.contract_id", "all"),
            ("contract.variety_id", symbol.as_str()),
            ("contract", ""),
        ];

        let response = match client
            .post(url)
            .form(&payload)
            .header(
                "User-Agent",
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36",
            )
            .send()
            .await
        {
            Ok(r) => r,
            Err(e) => {
                log::warn!("获取 {} 合约列表失败: {}", symbol, e);
                continue;
            }
        };

        if !response.status().is_success() {
            continue;
        }

        let html = response.text().await?;
        let contract_list = parse_dce_contract_list(&html, symbol);

        for contract in &contract_list {
            let payload = [
                ("memberDealPosiQuotes.variety", symbol.as_str()),
                ("memberDealPosiQuotes.trade_type", "0"),
                ("year", &year.to_string()),
                ("month", &(month - 1).to_string()),
                ("day", &format!("{:02}", day)),
                ("contract.contract_id", contract.as_str()),
                ("contract.variety_id", symbol.as_str()),
                ("contract", ""),
            ];

            let response = match client
                .post(url)
                .form(&payload)
                .header(
                    "User-Agent",
                    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36",
                )
                .send()
                .await
            {
                Ok(r) => r,
                Err(e) => {
                    log::warn!("获取 {} 数据失败: {}", contract, e);
                    continue;
                }
            };

            if !response.status().is_success() {
                continue;
            }

            let html = response.text().await?;

            match parse_dce_html_table(&html, contract, symbol) {
                Ok(data) => {
                    if !data.is_empty() {
                        all_results.push(RankTableResponse {
                            symbol: contract.to_uppercase(),
                            data,
                        });
                    }
                }
                Err(e) => {
                    log::warn!("解析 {} 数据失败: {}", contract, e);
                }
            }
        }
    }

    all_results.sort_by(|a, b| a.symbol.cmp(&b.symbol));

    println!("📊 解析到 {} 个合约的持仓排名数据", all_results.len());
    Ok(all_results)
}

/// 解析大商所品种列表
fn parse_dce_symbol_list(html: &str) -> Result<Vec<String>> {
    let document = scraper::Html::parse_document(html);
    let selector = scraper::Selector::parse("input.selBox").unwrap();

    let mut symbols = Vec::new();

    for element in document.select(&selector) {
        if let Some(onclick) = element.value().attr("onclick") {
            if let Some(start) = onclick.find("setVariety('") {
                let rest = &onclick[start + 12..];
                if let Some(end) = rest.find("'") {
                    let symbol = &rest[..end];
                    if !symbol.is_empty() {
                        symbols.push(symbol.to_string());
                    }
                }
            }
        }
    }

    if symbols.is_empty() {
        let selector = scraper::Selector::parse(".selBox input").unwrap();
        for element in document.select(&selector) {
            if let Some(onclick) = element.value().attr("onclick") {
                if let Some(start) = onclick.find("setVariety(") {
                    let rest = &onclick[start + 11..];
                    if let Some(end) = rest.find(")") {
                        let symbol = rest[..end].trim_matches(|c| c == '\'' || c == '"');
                        if !symbol.is_empty() {
                            symbols.push(symbol.to_string());
                        }
                    }
                }
            }
        }
    }

    Ok(symbols)
}

/// 解析大商所合约列表
fn parse_dce_contract_list(html: &str, symbol: &str) -> Vec<String> {
    let document = scraper::Html::parse_document(html);
    let selector = scraper::Selector::parse("input[name='contract']").unwrap();

    let mut contracts = Vec::new();

    for element in document.select(&selector) {
        if let Some(onclick) = element.value().attr("onclick") {
            if let Some(start) = onclick.find("setContract_id('") {
                let rest = &onclick[start + 16..];
                if let Some(end) = rest.find("'") {
                    let contract_suffix = &rest[..end];
                    let contract = if contract_suffix.len() == 4
                        && contract_suffix.chars().all(|c| c.is_ascii_digit())
                    {
                        format!("{}{}", symbol, contract_suffix)
                    } else {
                        contract_suffix.to_string()
                    };
                    if !contract.is_empty() {
                        contracts.push(contract);
                    }
                }
            }
        }
    }

    contracts
}

/// 解析大商所HTML表格数据
fn parse_dce_html_table(
    html: &str,
    contract: &str,
    variety: &str,
) -> Result<Vec<PositionRankData>> {
    let document = scraper::Html::parse_document(html);

    let table_selector = scraper::Selector::parse("table").unwrap();
    let tables: Vec<_> = document.select(&table_selector).collect();

    if tables.len() < 2 {
        return Err(anyhow!("未找到数据表格"));
    }

    let data_table = tables[1];
    let row_selector = scraper::Selector::parse("tr").unwrap();
    let cell_selector = scraper::Selector::parse("td").unwrap();

    let mut result = Vec::new();

    for row in data_table.select(&row_selector) {
        let cells: Vec<_> = row.select(&cell_selector).collect();

        if cells.len() < 12 {
            continue;
        }

        let first_cell = cells[0].text().collect::<String>().trim().to_string();
        if first_cell.is_empty()
            || first_cell.contains("名次")
            || first_cell.contains("合计")
            || first_cell.contains("总计")
        {
            continue;
        }

        let rank: i32 = first_cell.parse().unwrap_or(0);
        if rank == 0 {
            continue;
        }

        let get_text = |idx: usize| -> String {
            cells
                .get(idx)
                .map(|c| {
                    c.text()
                        .collect::<String>()
                        .trim()
                        .replace(",", "")
                        .replace("-", "0")
                })
                .unwrap_or_default()
        };

        let get_num = |idx: usize| -> i64 { get_text(idx).parse().unwrap_or(0) };

        result.push(PositionRankData {
            rank,
            vol_party_name: get_text(1),
            vol: get_num(2),
            vol_chg: get_num(3),
            long_party_name: get_text(5),
            long_open_interest: get_num(6),
            long_open_interest_chg: get_num(7),
            short_party_name: get_text(9),
            short_open_interest: get_num(10),
            short_open_interest_chg: get_num(11),
            symbol: contract.to_uppercase(),
            variety: variety.to_uppercase(),
        });
    }

    Ok(result)
}


// ==================== 广期所持仓排名 ====================

/// 获取广州期货交易所品种列表
/// 对应 akshare 的 __futures_gfex_vars_list() 函数
pub async fn get_gfex_vars_list() -> Result<Vec<String>> {
    let client = Client::new();
    let url = "http://www.gfex.com.cn/u/interfacesWebVariety/loadList";

    println!("📡 请求广期所品种列表 URL: {}", url);

    let response = client
        .post(url)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/119.0.0.0 Safari/537.36")
        .header("Content-Length", "0")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(anyhow!("获取广期所品种列表失败: {}", response.status()));
    }

    let json_data: serde_json::Value = response.json().await?;

    let data = json_data["data"]
        .as_array()
        .ok_or_else(|| anyhow!("未找到data数组"))?;

    let vars: Vec<String> = data
        .iter()
        .filter_map(|item| item["varietyId"].as_str())
        .map(|s| s.to_string())
        .collect();

    println!("📊 获取到 {} 个品种", vars.len());
    Ok(vars)
}

/// 获取广期所合约列表
async fn get_gfex_contract_list(client: &Client, symbol: &str, date: &str) -> Result<Vec<String>> {
    let url = "http://www.gfex.com.cn/u/interfacesWebTiMemberDealPosiQuotes/loadListContract_id";

    let payload = [("variety", symbol), ("trade_date", date)];

    let response = client
        .post(url)
        .form(&payload)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/119.0.0.0 Safari/537.36")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(anyhow!("获取广期所合约列表失败: {}", response.status()));
    }

    let json_data: serde_json::Value = response.json().await?;

    let contracts: Vec<String> = if let Some(data) = json_data["data"].as_array() {
        data.iter()
            .filter_map(|item| {
                if let Some(arr) = item.as_array() {
                    arr.first().and_then(|v| v.as_str()).map(|s| s.to_string())
                } else if let Some(obj) = item.as_object() {
                    obj.values()
                        .next()
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string())
                } else {
                    item.as_str().map(|s| s.to_string())
                }
            })
            .collect()
    } else {
        Vec::new()
    };

    Ok(contracts)
}

/// 获取广期所合约持仓排名数据
async fn get_gfex_contract_data(
    client: &Client,
    symbol: &str,
    contract_id: &str,
    date: &str,
) -> Result<Vec<PositionRankData>> {
    let url = "http://www.gfex.com.cn/u/interfacesWebTiMemberDealPosiQuotes/loadList";

    let mut vol_data: Vec<(String, i64, i64)> = Vec::new();
    let mut long_data: Vec<(String, i64, i64)> = Vec::new();
    let mut short_data: Vec<(String, i64, i64)> = Vec::new();

    for data_type in 1..=3 {
        let payload = [
            ("trade_date", date),
            ("trade_type", "0"),
            ("variety", symbol),
            ("contract_id", contract_id),
            ("data_type", &data_type.to_string()),
        ];

        let response = client
            .post(url)
            .form(&payload)
            .header(
                "User-Agent",
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36",
            )
            .send()
            .await?;

        if !response.status().is_success() {
            continue;
        }

        let json_data: serde_json::Value = response.json().await?;

        if let Some(data) = json_data["data"].as_array() {
            let parsed: Vec<(String, i64, i64)> = data
                .iter()
                .filter_map(|item| {
                    let name = item["abbr"].as_str().unwrap_or("").to_string();
                    let qty = item["todayQty"]
                        .as_str()
                        .or_else(|| item["todayQty"].as_i64().map(|_| ""))
                        .unwrap_or("0")
                        .parse::<i64>()
                        .or_else(|_| item["todayQty"].as_i64().ok_or(()))
                        .unwrap_or(0);
                    let chg = item["qtySub"]
                        .as_str()
                        .or_else(|| item["todayQtyChg"].as_str())
                        .unwrap_or("0")
                        .parse::<i64>()
                        .or_else(|_| {
                            item["qtySub"]
                                .as_i64()
                                .or_else(|| item["todayQtyChg"].as_i64())
                                .ok_or(())
                        })
                        .unwrap_or(0);

                    if name.is_empty() || name == "合计" {
                        None
                    } else {
                        Some((name, qty, chg))
                    }
                })
                .collect();

            match data_type {
                1 => vol_data = parsed,
                2 => long_data = parsed,
                3 => short_data = parsed,
                _ => {}
            }
        }
    }

    let max_len = vol_data.len().max(long_data.len()).max(short_data.len());
    let mut result = Vec::new();

    for i in 0..max_len {
        let (vol_name, vol, vol_chg) = vol_data.get(i).cloned().unwrap_or_default();
        let (long_name, long_oi, long_chg) = long_data.get(i).cloned().unwrap_or_default();
        let (short_name, short_oi, short_chg) = short_data.get(i).cloned().unwrap_or_default();

        result.push(PositionRankData {
            rank: (i + 1) as i32,
            vol_party_name: vol_name,
            vol,
            vol_chg,
            long_party_name: long_name,
            long_open_interest: long_oi,
            long_open_interest_chg: long_chg,
            short_party_name: short_name,
            short_open_interest: short_oi,
            short_open_interest_chg: short_chg,
            symbol: contract_id.to_uppercase(),
            variety: symbol.to_uppercase(),
        });
    }

    Ok(result)
}

/// 获取广州期货交易所前20会员持仓排名数据
/// 对应 akshare 的 futures_gfex_position_rank() 函数
pub async fn get_gfex_rank_table(
    date: &str,
    vars_list: Option<Vec<&str>>,
) -> Result<Vec<RankTableResponse>> {
    let client = Client::new();

    let gfex_vars = ["SI", "LC", "PS"];

    let target_vars: Vec<String> = match vars_list {
        Some(vars) => vars
            .into_iter()
            .filter(|v| gfex_vars.iter().any(|gv| gv.eq_ignore_ascii_case(v)))
            .map(|v| v.to_lowercase())
            .collect(),
        None => gfex_vars.iter().map(|v| v.to_lowercase()).collect(),
    };

    let mut all_results: Vec<RankTableResponse> = Vec::new();

    for var in target_vars {
        let contract_list = match get_gfex_contract_list(&client, &var, date).await {
            Ok(list) => list,
            Err(e) => {
                log::warn!("获取广期所 {} 合约列表失败: {}", var, e);
                continue;
            }
        };

        for contract in contract_list {
            match get_gfex_contract_data(&client, &var, &contract, date).await {
                Ok(data) => {
                    if !data.is_empty() {
                        all_results.push(RankTableResponse {
                            symbol: contract.to_uppercase(),
                            data,
                        });
                    }
                }
                Err(e) => {
                    log::warn!("获取广期所 {} 合约数据失败: {}", contract, e);
                }
            }
        }
    }

    all_results.sort_by(|a, b| a.symbol.cmp(&b.symbol));

    println!("📊 解析到 {} 个合约的持仓排名数据", all_results.len());
    Ok(all_results)
}

/// 广州期货交易所-日成交持仓排名
/// 对应 akshare 的 futures_gfex_position_rank() 函数
pub async fn futures_gfex_position_rank(
    date: &str,
    vars_list: Option<Vec<&str>>,
) -> Result<Vec<RankTableResponse>> {
    let client = Client::new();

    let target_vars: Vec<String> = match vars_list {
        Some(vars) => vars.into_iter().map(|v| v.to_lowercase()).collect(),
        None => match get_gfex_vars_list().await {
            Ok(vars) => vars,
            Err(e) => {
                log::warn!("获取广期所品种列表失败: {}，使用默认品种列表", e);
                vec!["si".to_string(), "lc".to_string(), "ps".to_string()]
            }
        },
    };

    println!("📡 请求广期所持仓排名数据，品种: {:?}", target_vars);

    let mut all_results: Vec<RankTableResponse> = Vec::new();

    for var in target_vars {
        let contract_list = match get_gfex_contract_list(&client, &var, date).await {
            Ok(list) => list,
            Err(e) => {
                log::warn!("获取广期所 {} 合约列表失败: {}", var, e);
                continue;
            }
        };

        if contract_list.is_empty() {
            log::warn!("广期所 {} 在 {} 无合约数据", var, date);
            continue;
        }

        println!(
            "  品种 {} 有 {} 个合约",
            var.to_uppercase(),
            contract_list.len()
        );

        for contract in contract_list {
            match get_gfex_contract_data(&client, &var, &contract, date).await {
                Ok(data) => {
                    if !data.is_empty() {
                        all_results.push(RankTableResponse {
                            symbol: contract.to_uppercase(),
                            data,
                        });
                    }
                }
                Err(e) => {
                    log::warn!("获取广期所 {} 合约数据失败: {}", contract, e);
                }
            }
        }
    }

    all_results.sort_by(|a, b| a.symbol.cmp(&b.symbol));

    println!("📊 解析到 {} 个合约的持仓排名数据", all_results.len());
    Ok(all_results)
}


// ==================== 持仓排名汇总 ====================

/// 获取单日期货持仓排名汇总数据
/// 对应 akshare 的 get_rank_sum() 函数
/// 采集五个期货交易所前5、前10、前15、前20会员持仓排名数据
pub async fn get_rank_sum(date: &str, vars_list: Option<Vec<String>>) -> Result<Vec<RankSum>> {
    let dce_vars: Vec<&str> = vec![
        "C", "CS", "A", "B", "M", "Y", "P", "FB", "BB", "JD", "L", "V", "PP", "J", "JM", "I", "EG",
        "RR", "EB", "PG", "LH", "LG", "BZ",
    ];
    let shfe_vars: Vec<&str> = vec![
        "CU", "AL", "ZN", "PB", "NI", "SN", "AU", "AG", "RB", "WR", "HC", "FU", "BU", "RU", "SC",
        "NR", "SP", "SS", "LU", "BC", "AO", "BR", "EC", "AD",
    ];
    let czce_vars: Vec<&str> = vec![
        "WH", "PM", "CF", "SR", "TA", "OI", "RI", "MA", "ME", "FG", "RS", "RM", "ZC", "JR", "LR",
        "SF", "SM", "WT", "TC", "GN", "RO", "ER", "SRX", "SRY", "WSX", "WSY", "CY", "AP", "UR",
        "CJ", "SA", "PK", "PF", "PX", "SH", "PR",
    ];
    let cffex_vars: Vec<&str> = vec!["IF", "IC", "IM", "IH", "T", "TF", "TS", "TL"];
    let gfex_vars: Vec<&str> = vec!["SI", "LC", "PS"];

    let filter_vars = |exchange_vars: &[&str], target: &Option<Vec<String>>| -> Vec<String> {
        match target {
            Some(vars) => exchange_vars
                .iter()
                .filter(|v| vars.iter().any(|tv| tv.eq_ignore_ascii_case(v)))
                .map(|v| v.to_string())
                .collect(),
            None => exchange_vars.iter().map(|v| v.to_string()).collect(),
        }
    };

    let dce_target = filter_vars(&dce_vars, &vars_list);
    let shfe_target = filter_vars(&shfe_vars, &vars_list);
    let czce_target = filter_vars(&czce_vars, &vars_list);
    let cffex_target = filter_vars(&cffex_vars, &vars_list);
    let gfex_target = filter_vars(&gfex_vars, &vars_list);

    let mut all_rank_data: HashMap<String, Vec<PositionRankData>> = HashMap::new();

    // 获取大商所数据
    if !dce_target.is_empty() {
        let dce_refs: Vec<&str> = dce_target.iter().map(|s| s.as_str()).collect();
        match get_dce_rank_table(date, Some(dce_refs)).await {
            Ok(data) => {
                for item in data {
                    all_rank_data.insert(item.symbol.clone(), item.data);
                }
            }
            Err(e) => log::warn!("获取大商所数据失败: {}", e),
        }
    }

    // 获取上期所数据
    if !shfe_target.is_empty() {
        let shfe_refs: Vec<&str> = shfe_target.iter().map(|s| s.as_str()).collect();
        match get_shfe_rank_table(date, Some(shfe_refs)).await {
            Ok(data) => {
                for item in data {
                    all_rank_data.insert(item.symbol.clone(), item.data);
                }
            }
            Err(e) => log::warn!("获取上期所数据失败: {}", e),
        }
    }

    // 获取郑商所数据
    if !czce_target.is_empty() {
        match get_rank_table_czce(date).await {
            Ok(data) => {
                for item in data {
                    let variety = extract_variety(&item.symbol);
                    if czce_target.iter().any(|v| v.eq_ignore_ascii_case(&variety)) {
                        all_rank_data.insert(item.symbol.clone(), item.data);
                    }
                }
            }
            Err(e) => log::warn!("获取郑商所数据失败: {}", e),
        }
    }

    // 获取中金所数据
    if !cffex_target.is_empty() {
        let cffex_refs: Vec<&str> = cffex_target.iter().map(|s| s.as_str()).collect();
        match get_cffex_rank_table(date, Some(cffex_refs)).await {
            Ok(data) => {
                for item in data {
                    all_rank_data.insert(item.symbol.clone(), item.data);
                }
            }
            Err(e) => log::warn!("获取中金所数据失败: {}", e),
        }
    }

    // 获取广期所数据
    if !gfex_target.is_empty() {
        let gfex_refs: Vec<&str> = gfex_target.iter().map(|s| s.as_str()).collect();
        match get_gfex_rank_table(date, Some(gfex_refs)).await {
            Ok(data) => {
                for item in data {
                    all_rank_data.insert(item.symbol.clone(), item.data);
                }
            }
            Err(e) => log::warn!("获取广期所数据失败: {}", e),
        }
    }

    // 计算汇总数据
    let mut results: Vec<RankSum> = Vec::new();

    for (symbol, data) in &all_rank_data {
        let variety = extract_variety(symbol);

        let top5: Vec<&PositionRankData> = data.iter().filter(|d| d.rank <= 5).collect();
        let top10: Vec<&PositionRankData> = data.iter().filter(|d| d.rank <= 10).collect();
        let top15: Vec<&PositionRankData> = data.iter().filter(|d| d.rank <= 15).collect();
        let top20: Vec<&PositionRankData> = data.iter().filter(|d| d.rank <= 20).collect();

        let rank_sum = RankSum {
            symbol: symbol.clone(),
            variety: variety.clone(),
            vol_top5: top5.iter().map(|d| d.vol).sum(),
            vol_chg_top5: top5.iter().map(|d| d.vol_chg).sum(),
            long_open_interest_top5: top5.iter().map(|d| d.long_open_interest).sum(),
            long_open_interest_chg_top5: top5.iter().map(|d| d.long_open_interest_chg).sum(),
            short_open_interest_top5: top5.iter().map(|d| d.short_open_interest).sum(),
            short_open_interest_chg_top5: top5.iter().map(|d| d.short_open_interest_chg).sum(),
            vol_top10: top10.iter().map(|d| d.vol).sum(),
            vol_chg_top10: top10.iter().map(|d| d.vol_chg).sum(),
            long_open_interest_top10: top10.iter().map(|d| d.long_open_interest).sum(),
            long_open_interest_chg_top10: top10.iter().map(|d| d.long_open_interest_chg).sum(),
            short_open_interest_top10: top10.iter().map(|d| d.short_open_interest).sum(),
            short_open_interest_chg_top10: top10.iter().map(|d| d.short_open_interest_chg).sum(),
            vol_top15: top15.iter().map(|d| d.vol).sum(),
            vol_chg_top15: top15.iter().map(|d| d.vol_chg).sum(),
            long_open_interest_top15: top15.iter().map(|d| d.long_open_interest).sum(),
            long_open_interest_chg_top15: top15.iter().map(|d| d.long_open_interest_chg).sum(),
            short_open_interest_top15: top15.iter().map(|d| d.short_open_interest).sum(),
            short_open_interest_chg_top15: top15.iter().map(|d| d.short_open_interest_chg).sum(),
            vol_top20: top20.iter().map(|d| d.vol).sum(),
            vol_chg_top20: top20.iter().map(|d| d.vol_chg).sum(),
            long_open_interest_top20: top20.iter().map(|d| d.long_open_interest).sum(),
            long_open_interest_chg_top20: top20.iter().map(|d| d.long_open_interest_chg).sum(),
            short_open_interest_top20: top20.iter().map(|d| d.short_open_interest).sum(),
            short_open_interest_chg_top20: top20.iter().map(|d| d.short_open_interest_chg).sum(),
            date: date.to_string(),
        };

        results.push(rank_sum);
    }

    // 添加品种汇总
    let mut variety_sums: HashMap<String, RankSum> = HashMap::new();

    for result in &results {
        let variety = &result.variety;

        let should_sum = shfe_vars.iter().any(|v| v.eq_ignore_ascii_case(variety))
            || dce_vars.iter().any(|v| v.eq_ignore_ascii_case(variety))
            || cffex_vars.iter().any(|v| v.eq_ignore_ascii_case(variety));

        if should_sum {
            variety_sums
                .entry(variety.clone())
                .and_modify(|sum| {
                    sum.vol_top5 += result.vol_top5;
                    sum.vol_chg_top5 += result.vol_chg_top5;
                    sum.long_open_interest_top5 += result.long_open_interest_top5;
                    sum.long_open_interest_chg_top5 += result.long_open_interest_chg_top5;
                    sum.short_open_interest_top5 += result.short_open_interest_top5;
                    sum.short_open_interest_chg_top5 += result.short_open_interest_chg_top5;
                    sum.vol_top10 += result.vol_top10;
                    sum.vol_chg_top10 += result.vol_chg_top10;
                    sum.long_open_interest_top10 += result.long_open_interest_top10;
                    sum.long_open_interest_chg_top10 += result.long_open_interest_chg_top10;
                    sum.short_open_interest_top10 += result.short_open_interest_top10;
                    sum.short_open_interest_chg_top10 += result.short_open_interest_chg_top10;
                    sum.vol_top15 += result.vol_top15;
                    sum.vol_chg_top15 += result.vol_chg_top15;
                    sum.long_open_interest_top15 += result.long_open_interest_top15;
                    sum.long_open_interest_chg_top15 += result.long_open_interest_chg_top15;
                    sum.short_open_interest_top15 += result.short_open_interest_top15;
                    sum.short_open_interest_chg_top15 += result.short_open_interest_chg_top15;
                    sum.vol_top20 += result.vol_top20;
                    sum.vol_chg_top20 += result.vol_chg_top20;
                    sum.long_open_interest_top20 += result.long_open_interest_top20;
                    sum.long_open_interest_chg_top20 += result.long_open_interest_chg_top20;
                    sum.short_open_interest_top20 += result.short_open_interest_top20;
                    sum.short_open_interest_chg_top20 += result.short_open_interest_chg_top20;
                })
                .or_insert_with(|| RankSum {
                    symbol: variety.clone(),
                    variety: variety.clone(),
                    date: date.to_string(),
                    ..*result
                });
        }
    }

    for (_, sum) in variety_sums {
        results.push(sum);
    }

    results.sort_by(|a, b| a.symbol.cmp(&b.symbol));

    println!("📊 计算得到 {} 条持仓排名汇总数据", results.len());
    Ok(results)
}

/// 获取日期范围内的期货持仓排名汇总数据
/// 对应 akshare 的 get_rank_sum_daily() 函数
pub async fn get_rank_sum_daily(
    start_day: &str,
    end_day: &str,
    vars_list: Option<Vec<String>>,
) -> Result<Vec<RankSum>> {
    use chrono::NaiveDate;

    let start = NaiveDate::parse_from_str(start_day, "%Y%m%d")
        .map_err(|e| anyhow!("解析开始日期失败: {}", e))?;
    let end = NaiveDate::parse_from_str(end_day, "%Y%m%d")
        .map_err(|e| anyhow!("解析结束日期失败: {}", e))?;

    if start > end {
        return Err(anyhow!("开始日期不能大于结束日期"));
    }

    let mut all_results: Vec<RankSum> = Vec::new();
    let mut current = start;

    while current <= end {
        let date_str = current.format("%Y%m%d").to_string();
        println!("📅 正在获取 {} 的持仓排名数据...", date_str);

        let vars_clone: Option<Vec<String>> = vars_list.clone();

        match get_rank_sum(&date_str, vars_clone).await {
            Ok(mut data) => {
                if !data.is_empty() {
                    println!("  ✅ 获取到 {} 条数据", data.len());
                    all_results.append(&mut data);
                } else {
                    println!("  ⚠️ {} 无数据（可能是非交易日）", date_str);
                }
            }
            Err(e) => {
                println!("  ❌ {} 获取失败: {}", date_str, e);
            }
        }

        current = current.succ_opt().unwrap_or(current);
    }

    println!("📊 共获取 {} 条持仓排名汇总数据", all_results.len());
    Ok(all_results)
}
