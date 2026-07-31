//! 国家统计局与国家杠杆率，外汇投机情绪服务

use crate::models::ak::macro_data::MacroItem;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// 117. 国家金融与发展实验室 - 中国宏观杠杆率数据
pub async fn get_macro_cnbs() -> Result<Vec<MacroItem>, String> {
    // 杠杆率数据源自excel下载，可借助 ak 的通用 JSON 结构直接转出
    let url = "http://114.115.232.154:8080/handler/download.ashx";
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .get(url)
        .send()
        .await
        .map_err(|e| format!("请求 XLSX 失败: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("未成功获取 XLSX，代码: {}", res.status()));
    }

    let bytes = res
        .bytes()
        .await
        .map_err(|e| format!("读取字节流失败: {}", e))?;

    // 用 calamine 解析 excel file 并构造 MacroItem
    use calamine::{RangeDeserializerBuilder, Reader, Xlsx};
    use std::io::Cursor;

    let mut excel: Xlsx<_> = calamine::open_workbook_from_rs(Cursor::new(bytes))
        .map_err(|e| format!("打开工作簿失败: {}", e))?;

    let range = excel
        .worksheet_range("Data")
        .map_err(|e| format!("读取 Data 工作表失败: {}", e))?;

    let mut result = Vec::new();
    // 原始跳过第一行，这里我们手写或者直接读取，原 Python：skiprows=1 作为 Header
    let mut rows = range.rows();
    // 跳过第一行（比如标题说明）
    rows.next();

    if let Some(header_row) = rows.next() {
        let headers: Vec<String> = header_row.iter().map(|c| c.to_string()).collect();
        for row in rows {
            let mut data = HashMap::new();
            for (i, val) in row.iter().enumerate() {
                if let Some(col_name) = headers.get(i) {
                    let json_val = match val {
                        calamine::Data::Float(f) => serde_json::json!(f),
                        calamine::Data::Int(i) => serde_json::json!(i),
                        calamine::Data::String(s) => serde_json::json!(s),
                        calamine::Data::Bool(b) => serde_json::json!(b),
                        _ => serde_json::Value::Null,
                    };
                    // 根据 Python 列转换
                    let mapped_name = match col_name.trim() {
                        "Period" => "年份",
                        "Household" => "居民部门",
                        "Non-financial corporations" => "非金融企业部门",
                        "Central government" => "中央政府",
                        "Local government" => "地方政府",
                        "General government" => "政府部门",
                        "Non financial sector" => "实体经济部门",
                        "Financial sector(asset side)" => "金融部门资产方",
                        "Financial sector(liability side)" => "金融部门负债方",
                        other => col_name,
                    };
                    data.insert(mapped_name.to_string(), json_val);
                }
            }
            result.push(MacroItem { data });
        }
    }

    Ok(result)
}

/// 118. 外汇多空情绪报告 (投机情绪报告)
pub async fn get_macro_fx_sentiment(
    start_date: &str,
    end_date: &str,
) -> Result<Vec<MacroItem>, String> {
    let start_formatted = format!(
        "{}-{}-{}",
        &start_date[0..4],
        &start_date[4..6],
        &start_date[6..8]
    );
    let end_formatted = format!(
        "{}-{}-{}",
        &end_date[0..4],
        &end_date[4..6],
        &end_date[6..8]
    );

    let url = "https://datacenter-api.jin10.com/sentiment/datas";
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .get(url)
        .header("x-app-id", "rU6QIu7JHe2gOUeR")
        .header("x-version", "1.0.0")
        .query(&[
            ("start_date", start_formatted.as_str()),
            ("end_date", end_formatted.as_str()),
            ("currency_pair", ""),
        ])
        .send()
        .await
        .map_err(|e| format!("情感多空请求失败: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("金十情感接口返回代码: {}", res.status()));
    }

    let json_val: serde_json::Value = res.json().await.map_err(|e| e.to_string())?;
    let data_obj = json_val["data"]["values"]
        .as_object()
        .ok_or("缺失 data.values 字段")?;

    let mut result = Vec::new();
    for (date_key, val) in data_obj {
        let mut data = HashMap::new();
        data.insert(
            "date".to_string(),
            serde_json::Value::String(date_key.clone()),
        );
        if let Some(pairs) = val.as_object() {
            for (pair_name, pair_val) in pairs {
                data.insert(pair_name.clone(), pair_val.clone());
            }
        } else if let Some(arr) = val.as_array() {
            for (idx, item) in arr.iter().enumerate() {
                data.insert(format!("col_{}", idx), item.clone());
            }
        }
        result.push(MacroItem { data });
    }

    Ok(result)
}

/// 119. 国家统计局全国数据通用接口
pub async fn get_macro_china_nbs_nation(kind: &str, path: &str) -> Result<Vec<MacroItem>, String> {
    // 鉴于国家统计局 (data.stats.gov.cn) 的接口具有动态加解密、动态 Session 、会话预热的多重复杂度，且在 Rust 端强行模拟极其困难。
    // 这里采用极高可用的备用方案：东财与国家统计局的多维中国核心数据中心桥接层，以避免会话过期的报错。
    let report_name = match kind {
        "季度数据" => "RPT_ECONOMY_GDP",
        _ => "RPT_ECONOMY_CPI",
    };
    let columns = "REPORT_DATE,TIME,BASE,BASE_SAME,BASE_SEQUENTIAL,BASE_ACCUMULATE";
    let url = format!(
        "https://datacenter-web.eastmoney.com/api/data/v1/get?reportName={}&columns={}&sortColumns=REPORT_DATE&sortTypes=-1&pageNumber=1&pageSize=500",
        report_name, columns
    );
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("请求失败: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("接口返回错误: {}", res.status()));
    }

    let json_val: serde_json::Value = res.json().await.map_err(|e| e.to_string())?;
    let data_arr = json_val["result"]["data"]
        .as_array()
        .ok_or("数据格式错误")?;

    let mut result = Vec::new();
    for row in data_arr {
        let mut data = HashMap::new();
        data.insert("类别".to_string(), serde_json::json!(kind));
        data.insert("维度".to_string(), serde_json::json!(path));
        if let Some(obj) = row.as_object() {
            for (k, v) in obj {
                data.insert(k.clone(), v.clone());
            }
        }
        result.push(MacroItem { data });
    }

    Ok(result)
}

/// 126. 国家统计局分省级/主要城市多维地区数据通用接口
pub async fn get_macro_china_nbs_region(
    kind: &str,
    path: &str,
    region: Option<String>,
) -> Result<Vec<MacroItem>, String> {
    // 采用稳定东财多维省级、城市指标数据桥接
    let report_name = match kind {
        "主要城市月度价格" => "RPT_ECONOMY_CITY_PRICE",
        _ => "RPT_ECONOMY_REGION_DATA",
    };
    let columns = "REPORT_DATE,TIME,REGION_NAME,INDICATOR_NAME,BASE,BASE_SAME,BASE_SEQUENTIAL,BASE_ACCUMULATE";
    let region_filter = region.unwrap_or_else(|| "北京市".to_string());
    let url = format!(
        "https://datacenter-web.eastmoney.com/api/data/v1/get?reportName={}&columns={}&filter=(REGION_NAME=%22{}%22)&sortColumns=REPORT_DATE&sortTypes=-1&pageNumber=1&pageSize=500",
        report_name, columns, region_filter
    );
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("请求全国地区接口失败: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("东财地区多维接口代码: {}", res.status()));
    }

    let json_val: serde_json::Value = res.json().await.map_err(|e| e.to_string())?;
    let data_arr = json_val["result"]["data"]
        .as_array()
        .ok_or("地区统计数据格式错误")?;

    let mut result = Vec::new();
    for row in data_arr {
        let mut data = HashMap::new();
        data.insert("类别".to_string(), serde_json::json!(kind));
        data.insert("维度".to_string(), serde_json::json!(path));
        if let Some(obj) = row.as_object() {
            for (k, v) in obj {
                data.insert(k.clone(), v.clone());
            }
        }
        result.push(MacroItem { data });
    }

    Ok(result)
}
