//! 中国与全球宏观经济数据服务

use crate::models::ak::macro_data::MacroItem;
use std::collections::HashMap;

/// 金十 API 通用抓取函数
async fn fetch_jin10_macro_report(report_id: &str) -> Result<Vec<MacroItem>, String> {
    let url = "https://datacenter-api.jin10.com/reports/list_v2";
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .get(url)
        .header("x-app-id", "rU6QIu7JHe2gOUeR")
        .header("x-version", "1.0.0")
        .query(&[("report_id", report_id)])
        .send()
        .await
        .map_err(|e| format!("请求金十宏观接口失败: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("金十宏观接口状态码: {}", res.status()));
    }

    let json_val: serde_json::Value = res
        .json()
        .await
        .map_err(|e| format!("解析 JSON 失败: {}", e))?;
    let keys_arr = json_val["data"]["keys"]
        .as_array()
        .ok_or_else(|| "缺失 keys 字段".to_string())?;
    let values_arr = json_val["data"]["values"]
        .as_array()
        .ok_or_else(|| "缺失 values 字段".to_string())?;

    let mut col_names = Vec::new();
    for k in keys_arr {
        if let Some(name) = k["name"].as_str() {
            col_names.push(name.to_string());
        }
    }

    let mut result = Vec::new();
    for row in values_arr {
        if let Some(val_row) = row.as_array() {
            let mut data = HashMap::new();
            for (i, val) in val_row.iter().enumerate() {
                let name = col_names
                    .get(i)
                    .cloned()
                    .unwrap_or_else(|| format!("col_{}", i));
                data.insert(name, val.clone());
            }
            result.push(MacroItem { data });
        }
    }

    Ok(result)
}

/// 1. 中国 GDP 年率/季率
pub async fn get_macro_china_gdp() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_macro_report("61").await
}

/// 2. 中国 CPI 年率
pub async fn get_macro_china_cpi() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_macro_report("62").await
}

/// 3. 中国 PPI 年率
pub async fn get_macro_china_ppi() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_macro_report("63").await
}

/// 4. 中国官方 PMI 数据
pub async fn get_macro_china_pmi() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_macro_report("64").await
}

/// 5. 中国社会融资规模
pub async fn get_macro_china_shrzgm() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_macro_report("67").await
}

/// 6. 中国 M2 货币供应量
pub async fn get_macro_china_m2() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_macro_report("68").await
}

/// 7. 中国 LPR 贷款市场报价利率 (东方财富/新浪)
pub async fn get_macro_china_lpr() -> Result<Vec<MacroItem>, String> {
    let url = "https://datacenter-web.eastmoney.com/api/data/v1/get?reportName=RPT_LPR_IPC&columns=TRADE_DATE,LPR_1Y,LPR_5Y&sortColumns=TRADE_DATE&sortTypes=-1&pageNumber=1&pageSize=500";
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .get(url)
        .send()
        .await
        .map_err(|e| format!("请求 LPR 失败: {}", e))?;
    if !res.status().is_success() {
        return Err(format!("东方财富 LPR 接口状态码: {}", res.status()));
    }

    let json_val: serde_json::Value = res
        .json()
        .await
        .map_err(|e| format!("解析 JSON 失败: {}", e))?;
    let data_arr = json_val["result"]["data"]
        .as_array()
        .ok_or_else(|| "缺失 result.data 字段".to_string())?;

    let mut result = Vec::new();
    for row in data_arr {
        let mut data = HashMap::new();
        if let Some(obj) = row.as_object() {
            for (k, v) in obj {
                data.insert(k.clone(), v.clone());
            }
        }
        result.push(MacroItem { data });
    }

    Ok(result)
}

/// 8. 美国非农就业人口变动
pub async fn get_macro_usa_non_farm() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_macro_report("1").await
}

/// 9. 美国失业率
pub async fn get_macro_usa_unemployment() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_macro_report("2").await
}

/// 10. 美国 CPI 年率/月率
pub async fn get_macro_usa_cpi() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_macro_report("3").await
}

/// 11. 美国 GDP 年化季率
pub async fn get_macro_usa_gdp() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_macro_report("4").await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_macro_china_lpr() {
        let res = get_macro_china_lpr().await;
        if let Ok(data) = res {
            println!("LPR 数据获取成功，条数: {}", data.len());
        }
    }
}
