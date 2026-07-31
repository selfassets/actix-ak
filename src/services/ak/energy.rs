//! 能源 (Energy) 相关数据服务

use serde_json::Value;

/// 全国汽柴油历史调价信息
pub async fn energy_oil_hist() -> Result<Vec<Value>, String> {
    let url = "https://datacenter-web.eastmoney.com/api/data/v1/get";
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .build()
        .map_err(|e| format!("构建HTTP客户端失败: {}", e))?;

    let res = client
        .get(url)
        .query(&[
            ("reportName", "RPTA_WEB_YJ_BD"),
            ("columns", "ALL"),
            ("sortColumns", "dim_date"),
            ("sortTypes", "-1"),
            ("token", "894050c76af8597a853f5b408b759f5d"),
            ("pageNumber", "1"),
            ("pageSize", "1000"),
            ("source", "WEB"),
        ])
        .send()
        .await
        .map_err(|e| format!("请求失败: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("接口返回状态码错误: {}", res.status()));
    }

    let json: Value = res.json().await.map_err(|e| e.to_string())?;
    if let Some(arr) = json["result"]["data"].as_array() {
        Ok(arr.clone())
    } else {
        Ok(vec![])
    }
}

/// 全国各地区汽油和柴油油价明细
pub async fn energy_oil_detail(date: &str) -> Result<Vec<Value>, String> {
    let url = "https://datacenter-web.eastmoney.com/api/data/v1/get";
    let filter = format!("(dim_date='{}')", date);

    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0")
        .build()
        .map_err(|e| format!("构建HTTP客户端失败: {}", e))?;

    let res = client
        .get(url)
        .query(&[
            ("reportName", "RPTA_WEB_YJ_JH"),
            ("columns", "ALL"),
            ("filter", &filter),
            ("sortColumns", "cityname"),
            ("sortTypes", "1"),
            ("token", "894050c76af8597a853f5b408b759f5d"),
            ("pageNumber", "1"),
            ("pageSize", "1000"),
            ("source", "WEB"),
        ])
        .send()
        .await
        .map_err(|e| format!("请求失败: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("接口状态码: {}", res.status()));
    }

    let json: Value = res.json().await.map_err(|e| e.to_string())?;
    Ok(json["result"]["data"]
        .as_array()
        .cloned()
        .unwrap_or_default())
}

/// 国内碳排放交易行情 (代理/通用方法实现)
pub async fn energy_carbon_domestic(symbol: &str) -> Result<Vec<Value>, String> {
    let url = "http://k.tanjiaoyi.com:8080/KDataController/getHouseDatasInAverage.do";
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0")
        .build()
        .map_err(|e| format!("Error Client: {}", e))?;

    let res = client
        .get(url)
        .query(&[
            ("brand", "TAN"),
            ("lcnK", "53f75bfcefff58e4046ccfa42171636c"),
        ])
        .send()
        .await
        .map_err(|e| format!("请求失败: {}", e))?;

    let text = res.text().await.map_err(|e| e.to_string())?;
    let start_idx = text.find('(').ok_or("格式错误")?;
    let end_idx = text.rfind(')').ok_or("格式错误")?;
    let json_text = &text[start_idx + 1..end_idx];

    let json: Value = serde_json::from_str(json_text).map_err(|e| e.to_string())?;
    if let Some(arr) = json[symbol].as_array() {
        Ok(arr.clone())
    } else {
        Err(format!("找不到交易地点 {}", symbol))
    }
}

/// 4. 北京市碳排放交易公开交易行情
pub async fn energy_carbon_bj() -> Result<Vec<Value>, String> {
    let url = "https://www.bjets.com.cn/article/jyxx/";
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0")
        .build()
        .map_err(|e| format!("Error Client: {}", e))?;

    let res = client
        .get(url)
        .send()
        .await
        .map_err(|e| format!("请求北京碳交易市场失败: {}", e))?;

    let text = res.text().await.map_err(|e| e.to_string())?;
    let mut result = Vec::new();

    // 采用 scraper 高效提取 table 行
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
            let mut data = serde_json::Map::new();
            data.insert("日期".to_string(), Value::String(cells[0].clone()));
            data.insert("成交量".to_string(), Value::String(cells[1].clone()));
            data.insert("成交均价".to_string(), Value::String(cells[2].clone()));
            data.insert("成交额".to_string(), Value::String(cells[3].clone()));
            result.push(Value::Object(data));
        }
    }

    Ok(result)
}

/// 5. 深圳碳交易中心 - 国内碳排行情数据
pub async fn energy_carbon_sz() -> Result<Vec<Value>, String> {
    let url = "http://www.cerx.cn/dailynewsCN/index.htm";
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0")
        .build()
        .map_err(|e| format!("Error Client: {}", e))?;

    let res = client
        .get(url)
        .send()
        .await
        .map_err(|e| format!("请求深圳碳排放行情失败: {}", e))?;

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
            let mut data = serde_json::Map::new();
            data.insert("交易日期".to_string(), Value::String(cells[0].clone()));
            data.insert("收盘价".to_string(), Value::String(cells[4].clone()));
            data.insert("成交量".to_string(), Value::String(cells[6].clone()));
            data.insert("成交额".to_string(), Value::String(cells[7].clone()));
            result.push(Value::Object(data));
        }
    }

    Ok(result)
}

/// 6. 深圳碳交易中心 - 国际碳情 (欧洲能源等)
pub async fn energy_carbon_eu() -> Result<Vec<Value>, String> {
    let url = "http://www.cerx.cn/dailynewsOuter/index.htm";
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0")
        .build()
        .map_err(|e| format!("Error Client: {}", e))?;

    let res = client
        .get(url)
        .send()
        .await
        .map_err(|e| format!("请求欧盟国际碳情失败: {}", e))?;

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
            let mut data = serde_json::Map::new();
            data.insert("交易日期".to_string(), Value::String(cells[0].clone()));
            data.insert("收盘价".to_string(), Value::String(cells[4].clone()));
            data.insert("成交量".to_string(), Value::String(cells[6].clone()));
            data.insert("成交额".to_string(), Value::String(cells[7].clone()));
            result.push(Value::Object(data));
        }
    }

    Ok(result)
}
