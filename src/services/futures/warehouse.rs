//! 期货仓单日报数据模块
//!
//! 提供各交易所仓单日报数据的获取和处理

use anyhow::{anyhow, Result};
use reqwest::Client;
use std::collections::{HashMap, HashSet};

use crate::models::{
    CzceWarehouseReceipt, CzceWarehouseReceiptResponse, DceWarehouseReceipt,
    GfexWarehouseReceipt, GfexWarehouseReceiptResponse, ShfeWarehouseReceipt,
    ShfeWarehouseReceiptResponse,
};

/// 郑州商品交易所-交易数据-仓单日报
/// 对应 akshare 的 futures_warehouse_receipt_czce() 函数
/// 数据来源: http://www.czce.com.cn/cn/jysj/cdrb/H770310index_1.htm
///
/// date: 交易日期，格式 YYYYMMDD
pub async fn futures_warehouse_receipt_czce(
    date: &str,
) -> Result<Vec<CzceWarehouseReceiptResponse>> {
    let client = Client::builder()
        .danger_accept_invalid_certs(true)
        .timeout(std::time::Duration::from_secs(30))
        .build()?;

    let date_num: i32 = date.parse().unwrap_or(0);
    let url = if date_num > 20251101 {
        format!(
            "http://www.czce.com.cn/cn/DFSStaticFiles/Future/{}/{}/FutureDataWhsheet.xlsx",
            &date[0..4],
            date
        )
    } else {
        format!(
            "http://www.czce.com.cn/cn/DFSStaticFiles/Future/{}/{}/FutureDataWhsheet.xls",
            &date[0..4],
            date
        )
    };

    println!("📡 请求郑商所仓单日报数据 URL: {}", url);

    let response = client
        .get(&url)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(anyhow!(
            "获取郑商所仓单日报数据失败: {}，可能是非交易日",
            response.status()
        ));
    }

    let bytes = response.bytes().await?;

    use calamine::{open_workbook_auto_from_rs, Reader};
    use std::io::Cursor;

    let cursor = Cursor::new(bytes.as_ref());
    let mut workbook =
        open_workbook_auto_from_rs(cursor).map_err(|e| anyhow!("打开Excel文件失败: {}", e))?;

    let sheet_names = workbook.sheet_names().to_vec();
    if sheet_names.is_empty() {
        return Err(anyhow!("Excel文件没有工作表"));
    }

    let range = workbook
        .worksheet_range(&sheet_names[0])
        .map_err(|e| anyhow!("读取工作表失败: {}", e))?;

    let mut rows: Vec<Vec<String>> = Vec::new();
    for row in range.rows() {
        let row_data: Vec<String> = row
            .iter()
            .map(|cell| match cell {
                calamine::Data::String(s) => s.clone(),
                calamine::Data::Float(f) => format!("{}", f),
                calamine::Data::Int(i) => format!("{}", i),
                calamine::Data::Bool(b) => format!("{}", b),
                calamine::Data::DateTime(dt) => format!("{}", dt),
                calamine::Data::Error(e) => format!("{:?}", e),
                calamine::Data::Empty => String::new(),
                _ => String::new(),
            })
            .collect();
        rows.push(row_data);
    }

    let mut index_list: Vec<usize> = Vec::new();
    for (i, row) in rows.iter().enumerate() {
        if !row.is_empty() && row[0].starts_with("品种") {
            index_list.push(i);
        }
    }
    index_list.push(rows.len());

    let mut result: Vec<CzceWarehouseReceiptResponse> = Vec::new();

    for i in 0..index_list.len() - 1 {
        let start_idx = index_list[i];
        let end_idx = index_list[i + 1];

        if start_idx >= rows.len() {
            continue;
        }

        let first_cell = &rows[start_idx][0];
        let symbol = extract_letters(first_cell);

        if symbol.is_empty() {
            continue;
        }

        let mut header_idx = start_idx + 1;
        while header_idx < end_idx {
            if !rows[header_idx].is_empty()
                && (rows[header_idx][0].contains("仓库") || rows[header_idx][0].contains("简称"))
            {
                break;
            }
            header_idx += 1;
        }

        if header_idx >= end_idx {
            continue;
        }

        let mut data: Vec<CzceWarehouseReceipt> = Vec::new();
        for row in rows.iter().take(end_idx).skip(header_idx + 1) {
            if row.is_empty()
                || row[0].is_empty()
                || row[0].contains("合计")
                || row[0].contains("小计")
            {
                continue;
            }

            let warehouse = row.first().cloned().unwrap_or_default().trim().to_string();
            if warehouse.is_empty() {
                continue;
            }

            let parse_num = |s: &str| -> Option<i64> {
                let cleaned = s.trim().replace(",", "").replace("-", "");
                if cleaned.is_empty() {
                    None
                } else {
                    cleaned.parse().ok()
                }
            };

            let warehouse_receipt = row.get(1).and_then(|s| parse_num(s));
            let valid_forecast = row.get(2).and_then(|s| parse_num(s));
            let change = row.get(3).and_then(|s| parse_num(s));

            data.push(CzceWarehouseReceipt {
                warehouse,
                warehouse_receipt,
                valid_forecast,
                change,
            });
        }

        if !data.is_empty() {
            result.push(CzceWarehouseReceiptResponse { symbol, data });
        }
    }

    result.sort_by(|a, b| a.symbol.cmp(&b.symbol));

    println!("📊 解析到 {} 个品种的仓单日报数据", result.len());
    Ok(result)
}

/// 从字符串中提取字母部分
fn extract_letters(s: &str) -> String {
    s.chars()
        .filter(|c| c.is_ascii_alphabetic())
        .collect::<String>()
        .to_uppercase()
}

/// 大连商品交易所-行情数据-统计数据-日统计-仓单日报
/// 对应 akshare 的 futures_warehouse_receipt_dce() 函数
/// 数据来源: http://www.dce.com.cn/dalianshangpin/xqsj/tjsj26/rtj/cdrb/index.html
///
/// date: 交易日期，格式 YYYYMMDD
pub async fn futures_warehouse_receipt_dce(date: &str) -> Result<Vec<DceWarehouseReceipt>> {
    let client = Client::builder()
        .cookie_store(true)
        .timeout(std::time::Duration::from_secs(30))
        .build()?;

    let _home_resp = client
        .get("http://www.dce.com.cn/dalianshangpin/xqsj/tjsj26/rtj/cdrb/index.html")
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
        .send()
        .await;

    let url = "http://www.dce.com.cn/dcereport/publicweb/dailystat/wbillWeeklyQuotes";

    let payload = serde_json::json!({
        "tradeDate": date,
        "varietyId": "all"
    });

    println!("📡 请求大商所仓单日报数据 URL: {}", url);

    let response = client
        .post(url)
        .json(&payload)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
        .header("Accept", "application/json, text/plain, */*")
        .header("Accept-Language", "zh-CN,zh;q=0.9,en;q=0.8")
        .header("Origin", "http://www.dce.com.cn")
        .header("Referer", "http://www.dce.com.cn/dalianshangpin/xqsj/tjsj26/rtj/cdrb/index.html")
        .send()
        .await?;

    if !response.status().is_success() {
        if response.status().as_u16() == 412 {
            return Err(anyhow!(
                "大商所API访问被拒绝(412)，该交易所有反爬虫机制。\n\
                建议: 1) 稍后重试 2) 使用浏览器手动查看数据"
            ));
        }
        return Err(anyhow!(
            "获取大商所仓单日报数据失败: {}，可能是非交易日",
            response.status()
        ));
    }

    let json_data: serde_json::Value = response.json().await?;

    let entity_list = json_data["data"]["entityList"]
        .as_array()
        .ok_or_else(|| anyhow!("未找到entityList数据"))?;

    let mut result: Vec<DceWarehouseReceipt> = Vec::new();

    for item in entity_list {
        let variety_code = item["varietyOrder"].as_str().unwrap_or("").to_uppercase();
        let variety_name = item["variety"].as_str().unwrap_or("").to_string();
        let warehouse = item["whAbbr"].as_str().unwrap_or("").to_string();
        let delivery_location = item["deliveryAbbr"]
            .as_str()
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string());

        let last_receipt = item["lastWbillQty"]
            .as_i64()
            .or_else(|| item["lastWbillQty"].as_str().and_then(|s| s.parse().ok()))
            .unwrap_or(0);
        let today_receipt = item["wbillQty"]
            .as_i64()
            .or_else(|| item["wbillQty"].as_str().and_then(|s| s.parse().ok()))
            .unwrap_or(0);
        let change = item["diff"]
            .as_i64()
            .or_else(|| item["diff"].as_str().and_then(|s| s.parse().ok()))
            .unwrap_or(0);

        result.push(DceWarehouseReceipt {
            variety_code,
            variety_name,
            warehouse,
            delivery_location,
            last_receipt,
            today_receipt,
            change,
        });
    }

    println!("📊 解析到 {} 条仓单日报数据", result.len());
    Ok(result)
}

/// 上海期货交易所-指定交割仓库期货仓单日报
/// 对应 akshare 的 futures_shfe_warehouse_receipt() 函数
/// 数据来源: https://www.shfe.com.cn/data/tradedata/future/dailydata/{date}dailystock.dat
///
/// date: 交易日期，格式 YYYYMMDD（数据从 20140519 开始）
pub async fn futures_shfe_warehouse_receipt(
    date: &str,
) -> Result<Vec<ShfeWarehouseReceiptResponse>> {
    let client = Client::new();

    let url = format!(
        "https://www.shfe.com.cn/data/tradedata/future/dailydata/{}dailystock.dat",
        date
    );

    println!("📡 请求上期所仓单日报 URL: {}", url);

    let response = client
        .get(&url)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
        .header("Referer", "https://www.shfe.com.cn/")
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(anyhow!(
            "获取上期所仓单日报数据失败: {}，可能是非交易日或日期格式错误",
            response.status()
        ));
    }

    let json_data: serde_json::Value = response.json().await?;

    let o_cursor = json_data["o_cursor"]
        .as_array()
        .ok_or_else(|| anyhow!("未找到o_cursor数据"))?;

    let mut grouped: HashMap<String, Vec<ShfeWarehouseReceipt>> = HashMap::new();

    for item in o_cursor {
        let var_name = item["VARNAME"]
            .as_str()
            .unwrap_or("")
            .split('$')
            .next()
            .unwrap_or("")
            .trim()
            .to_string();

        if var_name.is_empty() {
            continue;
        }

        let reg_name = item["REGNAME"]
            .as_str()
            .unwrap_or("")
            .split('$')
            .next()
            .unwrap_or("")
            .trim()
            .to_string();

        let wh_name = item["WHABBRNAME"]
            .as_str()
            .unwrap_or("")
            .split('$')
            .next()
            .unwrap_or("")
            .trim()
            .to_string();

        let last_receipt = item["WRTWGHTS"]
            .as_i64()
            .or_else(|| item["WRTWGHTS"].as_str().and_then(|s| s.parse().ok()))
            .unwrap_or(0);
        let today_receipt = item["WRTQTY"]
            .as_i64()
            .or_else(|| item["WRTQTY"].as_str().and_then(|s| s.parse().ok()))
            .unwrap_or(0);
        let change = item["WRTCHANGE"]
            .as_i64()
            .or_else(|| item["WRTCHANGE"].as_str().and_then(|s| s.parse().ok()))
            .unwrap_or(0);

        let unit = item["UNIT"].as_str().unwrap_or("").to_string();

        let receipt = ShfeWarehouseReceipt {
            variety: var_name.clone(),
            region: reg_name,
            warehouse: wh_name,
            last_receipt,
            today_receipt,
            change,
            unit,
        };

        grouped.entry(var_name).or_default().push(receipt);
    }

    let mut result: Vec<ShfeWarehouseReceiptResponse> = grouped
        .into_iter()
        .map(|(symbol, data)| ShfeWarehouseReceiptResponse { symbol, data })
        .collect();

    result.sort_by(|a, b| a.symbol.cmp(&b.symbol));

    println!("📊 解析到 {} 个品种的仓单日报数据", result.len());
    Ok(result)
}

/// 广州期货交易所-行情数据-仓单日报
/// 对应 akshare 的 futures_gfex_warehouse_receipt() 函数
/// 数据来源: http://www.gfex.com.cn/gfex/cdrb/hqsj_tjsj.shtml
///
/// date: 交易日期，格式 YYYYMMDD
pub async fn futures_gfex_warehouse_receipt(
    date: &str,
) -> Result<Vec<GfexWarehouseReceiptResponse>> {
    let client = Client::new();

    let url = "http://www.gfex.com.cn/u/interfacesWebTdWbillWeeklyQuotes/loadList";

    let payload = [("gen_date", date)];

    println!("📡 请求广期所仓单日报数据 URL: {}", url);

    let response = client
        .post(url)
        .form(&payload)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(anyhow!(
            "获取广期所仓单日报数据失败: {}，可能是非交易日",
            response.status()
        ));
    }

    let json_data: serde_json::Value = response.json().await?;

    let data_array = json_data["data"]
        .as_array()
        .ok_or_else(|| anyhow!("未找到data数组"))?;

    let mut symbol_set: HashSet<String> = HashSet::new();
    for item in data_array {
        if let Some(symbol) = item["varietyOrder"].as_str() {
            if !symbol.is_empty() {
                symbol_set.insert(symbol.to_uppercase());
            }
        }
    }

    let mut result: Vec<GfexWarehouseReceiptResponse> = Vec::new();

    for symbol in symbol_set {
        let mut data: Vec<GfexWarehouseReceipt> = Vec::new();

        for item in data_array {
            let item_symbol = item["varietyOrder"].as_str().unwrap_or("").to_uppercase();
            if item_symbol != symbol {
                continue;
            }

            let wh_type = item["whType"]
                .as_str()
                .or_else(|| item["whType"].as_i64().map(|_| ""))
                .unwrap_or("");
            if wh_type.is_empty() && item["whType"].is_null() {
                continue;
            }

            let variety = item["variety"].as_str().unwrap_or("").to_string();
            let warehouse = item["whAbbr"].as_str().unwrap_or("").to_string();

            let last_receipt = item["lastWbillQty"]
                .as_i64()
                .or_else(|| item["lastWbillQty"].as_str().and_then(|s| s.parse().ok()))
                .unwrap_or(0);
            let today_receipt = item["wbillQty"]
                .as_i64()
                .or_else(|| item["wbillQty"].as_str().and_then(|s| s.parse().ok()))
                .unwrap_or(0);
            let change = item["regWbillQty"]
                .as_i64()
                .or_else(|| item["regWbillQty"].as_str().and_then(|s| s.parse().ok()))
                .unwrap_or(0);

            data.push(GfexWarehouseReceipt {
                variety,
                warehouse,
                last_receipt,
                today_receipt,
                change,
            });
        }

        if !data.is_empty() {
            result.push(GfexWarehouseReceiptResponse { symbol, data });
        }
    }

    result.sort_by(|a, b| a.symbol.cmp(&b.symbol));

    println!("📊 解析到 {} 个品种的仓单日报数据", result.len());
    Ok(result)
}
