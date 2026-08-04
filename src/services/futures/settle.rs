//! 各大期货交易所结算价与期转现服务

use crate::models::ak::macro_data::MacroItem;
use serde_json::Value;
use std::collections::HashMap;

/// 中金所 (CFFEX) 日终结算价
pub async fn futures_settle_cffex(date: &str) -> Result<Vec<MacroItem>, String> {
    let year = &date[0..4];
    let month = &date[4..6];
    let day = &date[6..8];
    let url = format!(
        "http://www.cffex.com.cn/sj/hqsj/rtj/{}/{}/index.xml",
        year, month
    );

    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("请求 CFFEX 结算数据失败: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("CFFEX 接口代码: {}", res.status()));
    }

    let text = res.text().await.map_err(|e| e.to_string())?;
    let mut result = Vec::new();

    let document = scraper::Html::parse_document(&text);
    let row_selector = scraper::Selector::parse("sjsj tr, item").unwrap();
    let cell_selector =
        scraper::Selector::parse("td, th, instrumentid, presettlementprice, settlementprice")
            .unwrap();

    for row in document.select(&row_selector) {
        let cells: Vec<String> = row
            .select(&cell_selector)
            .map(|c| c.text().collect::<Vec<_>>().join("").trim().to_string())
            .collect();

        if cells.len() >= 3 {
            let mut data = HashMap::new();
            data.insert("合约代码".to_string(), Value::String(cells[0].clone()));
            data.insert("前结算价".to_string(), Value::String(cells[1].clone()));
            data.insert("结算价".to_string(), Value::String(cells[2].clone()));
            data.insert(
                "日期".to_string(),
                Value::String(format!("{}-{}-{}", year, month, day)),
            );
            result.push(MacroItem { data });
        }
    }

    Ok(result)
}

/// 郑商所 (CZCE) 日终结算价
pub async fn futures_settle_czce(date: &str) -> Result<Vec<MacroItem>, String> {
    let year = &date[0..4];
    let url = format!(
        "http://www.czce.com.cn/cn/DFS/history/{}/CZCE_{}.htm",
        year, date
    );

    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("请求 CZCE 结算数据失败: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("CZCE 接口代码: {}", res.status()));
    }

    let text = res.text().await.map_err(|e| e.to_string())?;
    let mut result = Vec::new();

    let document = scraper::Html::parse_document(&text);
    let row_selector = scraper::Selector::parse("table tr").unwrap();
    let cell_selector = scraper::Selector::parse("td, th").unwrap();

    let mut is_header = true;
    for row in document.select(&row_selector) {
        let cells: Vec<String> = row
            .select(&cell_selector)
            .map(|c| c.text().collect::<Vec<_>>().join("").trim().to_string())
            .collect();

        if is_header || cells.is_empty() {
            is_header = false;
            continue;
        }

        if cells.len() >= 6 {
            let mut data = HashMap::new();
            data.insert("品种/合约".to_string(), Value::String(cells[0].clone()));
            data.insert("今结算".to_string(), Value::String(cells[5].clone()));
            data.insert("日期".to_string(), Value::String(date.to_string()));
            result.push(MacroItem { data });
        }
    }

    Ok(result)
}

/// 大商所 (DCE) 日终结算价
pub async fn futures_settle_dce(date: &str) -> Result<Vec<MacroItem>, String> {
    let year = &date[0..4];
    let month = &date[4..6];
    let day = &date[6..8];
    let url = "http://www.dce.com.cn/publicweb/quotesdata/dayQuotes.html";

    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .post(url)
        .form(&[
            ("year", year),
            (
                "month",
                &format!("{}", month.parse::<i32>().unwrap_or(1) - 1),
            ),
            ("day", day),
            ("dayQuotes.start_year", year),
            ("dayQuotes.end_year", year),
            ("dayQuotes.param", "0"),
        ])
        .send()
        .await
        .map_err(|e| format!("请求 DCE 结算数据失败: {}", e))?;

    let text = res.text().await.map_err(|e| e.to_string())?;
    let mut result = Vec::new();

    let document = scraper::Html::parse_document(&text);
    let row_selector = scraper::Selector::parse("table tr").unwrap();
    let cell_selector = scraper::Selector::parse("td, th").unwrap();

    let mut is_header = true;
    for row in document.select(&row_selector) {
        let cells: Vec<String> = row
            .select(&cell_selector)
            .map(|c| c.text().collect::<Vec<_>>().join("").trim().to_string())
            .collect();

        if is_header || cells.is_empty() {
            is_header = false;
            continue;
        }

        if cells.len() >= 8 {
            let mut data = HashMap::new();
            data.insert("合约名称".to_string(), Value::String(cells[0].clone()));
            data.insert("结算价".to_string(), Value::String(cells[7].clone()));
            data.insert("日期".to_string(), Value::String(date.to_string()));
            result.push(MacroItem { data });
        }
    }

    Ok(result)
}

/// 上期所 (SHFE) 日终结算价
pub async fn futures_settle_shfe(date: &str) -> Result<Vec<MacroItem>, String> {
    let url = format!("http://www.shfe.com.cn/data/dailydata/kx/kx{}.dat", date);

    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("请求 SHFE 结算数据失败: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("SHFE 接口代码: {}", res.status()));
    }

    let json_val: Value = res.json().await.map_err(|e| e.to_string())?;
    let arr = json_val["o_curinstrument"]
        .as_array()
        .ok_or("缺失 o_curinstrument 数组")?;

    let mut result = Vec::new();
    for row in arr {
        let mut data = HashMap::new();
        if let Some(obj) = row.as_object() {
            for (k, v) in obj {
                let name = match k.as_str() {
                    "INSTRUMENTID" => "合约代码",
                    "SETTLEMENTPRICE" => "结算价",
                    "PRESETTLEMENTPRICE" => "前结算价",
                    _ => k,
                };
                data.insert(name.to_string(), v.clone());
            }
        }
        result.push(MacroItem { data });
    }

    Ok(result)
}

/// 广期所 (GFEX) 日终结算价
pub async fn futures_settle_gfex(date: &str) -> Result<Vec<MacroItem>, String> {
    let year = &date[0..4];
    let month = &date[4..6];
    let day = &date[6..8];
    let url = format!(
        "http://www.gfex.com.cn/u/s/dailyQuotations_{}{}{}.json",
        year, month, day
    );

    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("请求 GFEX 结算数据失败: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("GFEX 接口代码: {}", res.status()));
    }

    let json_val: Value = res.json().await.map_err(|e| e.to_string())?;
    let arr = json_val["data"].as_array().ok_or("缺失 data 数组")?;

    let mut result = Vec::new();
    for row in arr {
        let mut data = HashMap::new();
        if let Some(obj) = row.as_object() {
            for (k, v) in obj {
                let name = match k.as_str() {
                    "varietyId" => "品种代码",
                    "clearPrice" => "结算价",
                    _ => k,
                };
                data.insert(name.to_string(), v.clone());
            }
        }
        result.push(MacroItem { data });
    }

    Ok(result)
}

/// 上期能源 (INE) 日终结算价
pub async fn futures_settle_ine(date: &str) -> Result<Vec<MacroItem>, String> {
    let url = format!("http://www.ine.cn/data/dailydata/kx/kx{}.dat", date);

    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("请求 INE 结算数据失败: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("INE 接口代码: {}", res.status()));
    }

    let json_val: Value = res.json().await.map_err(|e| e.to_string())?;
    let arr = json_val["o_curinstrument"]
        .as_array()
        .ok_or("缺失 o_curinstrument 数组")?;

    let mut result = Vec::new();
    for row in arr {
        let mut data = HashMap::new();
        if let Some(obj) = row.as_object() {
            for (k, v) in obj {
                let name = match k.as_str() {
                    "INSTRUMENTID" => "合约代码",
                    "SETTLEMENTPRICE" => "结算价",
                    "PRESETTLEMENTPRICE" => "前结算价",
                    _ => k,
                };
                data.insert(name.to_string(), v.clone());
            }
        }
        result.push(MacroItem { data });
    }

    Ok(result)
}

/// 上期所 (SHFE) 期转现明细
pub async fn futures_to_spot_shfe(date: &str) -> Result<Vec<MacroItem>, String> {
    let url = format!(
        "http://www.shfe.com.cn/data/dailydata/option/{}qzx.dat",
        date
    );

    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("请求上期所期转现失败: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("接口代码: {}", res.status()));
    }

    let json_val: Value = res.json().await.map_err(|e| e.to_string())?;
    let arr = json_val["o_curinstrument"]
        .as_array()
        .ok_or("缺失数据数组")?;

    let mut result = Vec::new();
    for row in arr {
        let mut data = HashMap::new();
        if let Some(obj) = row.as_object() {
            for (k, v) in obj {
                let name = match k.as_str() {
                    "INSTRUMENTID" => "合约代码",
                    "SPOTPRICE" => "期转现价格",
                    _ => k,
                };
                data.insert(name.to_string(), v.clone());
            }
        }
        result.push(MacroItem { data });
    }

    Ok(result)
}

/// 大商所 (DCE) 期转现明细
pub async fn futures_to_spot_dce(date: &str) -> Result<Vec<MacroItem>, String> {
    let year = &date[0..4];
    let month = &date[4..6];
    let url = "http://www.dce.com.cn/publicweb/quotesdata/delivery.html";

    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .post(url)
        .form(&[
            ("year", year),
            (
                "month",
                &format!("{}", month.parse::<i32>().unwrap_or(1) - 1),
            ),
            ("deliveryData.param", "0"),
        ])
        .send()
        .await
        .map_err(|e| format!("请求大商所期转现失败: {}", e))?;

    let text = res.text().await.map_err(|e| e.to_string())?;
    let mut result = Vec::new();

    let document = scraper::Html::parse_document(&text);
    let row_selector = scraper::Selector::parse("table tr").unwrap();
    let cell_selector = scraper::Selector::parse("td, th").unwrap();

    let mut is_header = true;
    for row in document.select(&row_selector) {
        let cells: Vec<String> = row
            .select(&cell_selector)
            .map(|c| c.text().collect::<Vec<_>>().join("").trim().to_string())
            .collect();

        if is_header || cells.is_empty() {
            is_header = false;
            continue;
        }

        if cells.len() >= 4 {
            let mut data = HashMap::new();
            data.insert("合约名称".to_string(), Value::String(cells[0].clone()));
            data.insert("期转现数量".to_string(), Value::String(cells[1].clone()));
            data.insert("交割价格".to_string(), Value::String(cells[2].clone()));
            data.insert("配对日期".to_string(), Value::String(cells[3].clone()));
            result.push(MacroItem { data });
        }
    }

    Ok(result)
}

/// 郑商所 (CZCE) 期转现明细
pub async fn futures_to_spot_czce(date: &str) -> Result<Vec<MacroItem>, String> {
    let year = &date[0..4];
    let url = format!(
        "http://www.czce.com.cn/cn/DFS/history/{}/CZCE_qzx_{}.htm",
        year, date
    );

    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("请求郑商所期转现失败: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("接口代码: {}", res.status()));
    }

    let text = res.text().await.map_err(|e| e.to_string())?;
    let mut result = Vec::new();

    let document = scraper::Html::parse_document(&text);
    let row_selector = scraper::Selector::parse("table tr").unwrap();
    let cell_selector = scraper::Selector::parse("td, th").unwrap();

    let mut is_header = true;
    for row in document.select(&row_selector) {
        let cells: Vec<String> = row
            .select(&cell_selector)
            .map(|c| c.text().collect::<Vec<_>>().join("").trim().to_string())
            .collect();

        if is_header || cells.is_empty() {
            is_header = false;
            continue;
        }

        if cells.len() >= 3 {
            let mut data = HashMap::new();
            data.insert("品种/合约".to_string(), Value::String(cells[0].clone()));
            data.insert("期转现数量".to_string(), Value::String(cells[1].clone()));
            data.insert("日期".to_string(), Value::String(date.to_string()));
            result.push(MacroItem { data });
        }
    }

    Ok(result)
}
