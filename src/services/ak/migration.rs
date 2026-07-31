//! 百度地图迁移与百度慧眼迁徙 API 服务

use serde_json::Value;
use std::collections::HashMap;
use crate::models::ak::macro_data::MacroItem;
use super::baidu_cons::get_baidu_migration_code;

/// 127. 百度地图慧眼-百度迁徙-XXX迁入地/迁出地 Top100 详情
pub async fn get_migration_area_baidu(area: &str, indicator: &str, date: &str) -> Result<Vec<MacroItem>, String> {
    let (id, dt_flag) = get_baidu_migration_code(area)
        .ok_or_else(|| format!("无法识别的百度迁徙城市或省份: {}", area))?;

    let url = "https://huiyan.baidu.com/migration/cityrank.jsonp";
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client.get(url)
        .query(&[
            ("dt", dt_flag),
            ("id", id),
            ("type", indicator),
            ("date", date),
        ])
        .send().await
        .map_err(|e| format!("请求百度迁徙接口失败: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("百度接口故障, 状态码: {}", res.status()));
    }

    let text = res.text().await.map_err(|e| e.to_string())?;
    let start_idx = text.find("({").ok_or("百度返回 JSONP 边界丢失")?;
    let end_idx = text.rfind(");").ok_or("百度返回 JSONP 边界丢失")?;
    let json_text = &text[start_idx + 1..end_idx];

    let json_val: Value = serde_json::from_str(json_text).map_err(|e| e.to_string())?;
    let list_arr = json_val["data"]["list"].as_array().ok_or("缺失 data.list 字段")?;

    let mut result = Vec::new();
    for row in list_arr {
        let mut data = HashMap::new();
        if let Some(obj) = row.as_object() {
            for (k, v) in obj {
                let name = match k.as_str() {
                    "city_name" => "城市名称",
                    "province_name" => "省份名称",
                    "value" => "比例",
                    _ => k,
                };
                data.insert(name.to_string(), v.clone());
            }
        }
        result.push(MacroItem { data });
    }

    Ok(result)
}

/// 128. 百度地图慧眼-百度迁徙-迁徙规模历史曲线指数
pub async fn get_migration_scale_baidu(area: &str, indicator: &str) -> Result<Vec<MacroItem>, String> {
    let (id, dt_flag) = get_baidu_migration_code(area)
        .ok_or_else(|| format!("无法识别的百度迁徙城市或省份: {}", area))?;

    let url = "https://huiyan.baidu.com/migration/historycurve.jsonp";
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client.get(url)
        .query(&[
            ("dt", dt_flag),
            ("id", id),
            ("type", indicator),
        ])
        .send().await
        .map_err(|e| format!("请求百度历史规模曲线失败: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("百度接口故障, 状态码: {}", res.status()));
    }

    let text = res.text().await.map_err(|e| e.to_string())?;
    let start_idx = text.find("({").ok_or("百度返回 JSONP 边界丢失")?;
    let end_idx = text.rfind(");").ok_or("百度返回 JSONP 边界丢失")?;
    let json_text = &text[start_idx + 1..end_idx];

    let json_val: Value = serde_json::from_str(json_text).map_err(|e| e.to_string())?;
    let list_obj = json_val["data"]["list"].as_object().ok_or("缺失 data.list 对象")?;

    let mut result = Vec::new();
    for (date_key, val) in list_obj {
        let mut data = HashMap::new();
        data.insert("日期".to_string(), Value::String(date_key.clone()));
        data.insert("迁徙规模指数".to_string(), val.clone());
        result.push(MacroItem { data });
    }

    // 排序
    result.sort_by_key(|item| {
        item.data.get("日期")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string()
    });

    Ok(result)
}
