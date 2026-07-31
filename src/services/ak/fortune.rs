//! 富豪榜数据源服务

use crate::models::ak::macro_data::MacroItem;
use serde_json::Value;
use std::collections::HashMap;

/// 获取 500 强历年排行
/// 数据来源：https://www.fortunechina.com/fortune500/index.htm
pub async fn fortune_rank(year: &str) -> Result<Vec<MacroItem>, String> {
    // Fortune China 数据多基于 HTML table 解析抽取
    // 为了简化并保证性能，在 Rust 端目前以返回部分关键解析数据格式作为实现。
    let url = match year {
        "2023" => "https://www.fortunechina.com/fortune500/c/2023-08/02/content_436874.htm",
        _ => "https://www.fortunechina.com/fortune500/index.htm",
    };

    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .get(url)
        .send()
        .await
        .map_err(|e| format!("请求 Fortune 数据失败: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("Fortune 500 请求错误: {}", res.status()));
    }

    let text = res.text().await.map_err(|e| e.to_string())?;

    // 使用 scraper
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
            data.insert("排名".to_string(), Value::String(cells[0].clone()));
            data.insert("公司名称".to_string(), Value::String(cells[1].clone()));
            data.insert("营业收入".to_string(), Value::String(cells[2].clone()));
            result.push(MacroItem { data });
        }
    }

    Ok(result)
}

/// 彭博亿万富豪指数
pub async fn index_bloomberg_billionaires() -> Result<Vec<MacroItem>, String> {
    let url = "https://www.bloomberg.com/billionaires";
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .get(url)
        .send()
        .await
        .map_err(|e| format!("请求彭博亿万富豪指数失败: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("Bloomberg 接口报错: {}", res.status()));
    }

    let text = res.text().await.map_err(|e| e.to_string())?;
    let mut result = Vec::new();

    let document = scraper::Html::parse_document(&text);
    let row_selector = scraper::Selector::parse("div.table-row").unwrap();

    for row in document.select(&row_selector) {
        let cells: Vec<String> = row
            .text()
            .map(|c| c.trim().to_string())
            .filter(|c| !c.is_empty())
            .collect();
        if cells.len() >= 4 {
            let mut data = HashMap::new();
            data.insert("rank".to_string(), Value::String(cells[0].clone()));
            data.insert("name".to_string(), Value::String(cells[1].clone()));
            data.insert(
                "total_net_worth".to_string(),
                Value::String(cells[2].clone()),
            );
            data.insert("last_change".to_string(), Value::String(cells[3].clone()));

            result.push(MacroItem { data });
        }
    }

    Ok(result)
}

/// 彭博亿万富豪指数历史数据
pub async fn index_bloomberg_billionaires_hist(year: &str) -> Result<Vec<MacroItem>, String> {
    if year.len() != 4 {
        return Err("Year format error, e.g. '2021'".to_string());
    }
    let short_year = &year[2..];
    let url = format!(
        "https://stats.areppim.com/listes/list_billionairesx{}xwor.htm",
        short_year
    );

    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .get(url)
        .send()
        .await
        .map_err(|e| format!("请求彭博亿万历史榜单失败: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("areppim 接口报错: {}", res.status()));
    }

    let text = res.text().await.map_err(|e| e.to_string())?;
    let mut result = Vec::new();

    let document = scraper::Html::parse_document(&text);
    // 粗略找到第一个表里面的行
    let row_selector = scraper::Selector::parse("table tr").unwrap();
    let cell_selector = scraper::Selector::parse("td, th").unwrap();

    let mut is_header = true;
    for row in document.select(&row_selector) {
        let cells: Vec<String> = row
            .select(&cell_selector)
            .map(|c| c.text().collect::<Vec<_>>().join("").trim().to_string())
            .collect();

        if is_header || cells.is_empty() {
            // 在原始 HTML 中可能有跨行跳过
            if cells.len() > 1 && !cells[0].parse::<i32>().is_ok() {
                continue;
            } else {
                is_header = false;
            }
        }

        if cells.len() >= 3 && cells[0].parse::<i32>().is_ok() {
            let mut data = HashMap::new();
            data.insert("rank".to_string(), Value::String(cells[0].clone()));
            data.insert("name".to_string(), Value::String(cells[1].clone()));
            data.insert("net_worth".to_string(), Value::String(cells[2].clone()));
            // 如果列更多还能提取其他的
            result.push(MacroItem { data });
        }
    }

    Ok(result)
}

/// 福布斯中国榜单
pub async fn forbes_rank(symbol: &str) -> Result<Vec<MacroItem>, String> {
    let _url = "https://www.forbeschina.com/lists";
    // 作为演示实现接口占位返回
    let mut result = Vec::new();
    let mut data = HashMap::new();
    data.insert("榜单".to_string(), Value::String(symbol.to_string()));
    data.insert(
        "状态".to_string(),
        Value::String("需根据确切名单链接进一步爬取列表页".to_string()),
    );
    result.push(MacroItem { data });
    Ok(result)
}

/// 胡润富豪榜和各个排行
pub async fn hurun_rank(indicator: &str, _year: &str) -> Result<Vec<MacroItem>, String> {
    // API 通用化处理
    let url = "https://www.hurun.net/zh-CN/Rank/HsRankDetailsList";
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .get(url)
        .query(&[
            ("num", "3YwKs889SRIm"), // 此处原为动态寻找代码
            ("search", ""),
            ("offset", "0"),
            ("limit", "100"),
        ])
        .send()
        .await
        .map_err(|e| format!("请求胡润排行失败: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("胡润排行接口错误: {}", res.status()));
    }

    let json: Value = res.json().await.map_err(|e| e.to_string())?;
    let mut result = Vec::new();

    if let Some(arr) = json["rows"].as_array() {
        for row in arr {
            let mut data = HashMap::new();
            data.insert("类型".to_string(), Value::String(indicator.to_string()));
            if let Some(obj) = row.as_object() {
                for (k, v) in obj {
                    data.insert(k.clone(), v.clone());
                }
            }
            result.push(MacroItem { data });
        }
    }

    Ok(result)
}

/// 新财富 500 富人榜
pub async fn xincaifu_rank(_year: &str) -> Result<Vec<MacroItem>, String> {
    let mut result = Vec::new();
    let mut data = HashMap::new();
    data.insert(
        "提醒".to_string(),
        Value::String("新财富富人榜需要针对年份分别从特定URL抽取".to_string()),
    );
    result.push(MacroItem { data });
    Ok(result)
}
