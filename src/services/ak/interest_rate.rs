//! 拆借利率与利率数据服务

use crate::models::ak::interest_rate::{InterbankRateItem, InterbankRateQuery};
use std::collections::HashMap;

/// 东方财富-拆借利率一览
pub async fn get_rate_interbank(
    query: InterbankRateQuery,
) -> Result<Vec<InterbankRateItem>, String> {
    let market = query
        .market
        .unwrap_or_else(|| "上海银行同业拆借市场".to_string());
    let symbol = query.symbol.unwrap_or_else(|| "Shibor人民币".to_string());
    let indicator = query.indicator.unwrap_or_else(|| "隔夜".to_string());

    let market_code = match market.as_str() {
        "上海银行同业拆借市场" => "001",
        "中国银行同业拆借市场" => "002",
        "伦敦银行同业拆借市场" => "003",
        "欧洲银行同业拆借市场" => "004",
        "香港银行同业拆借市场" => "005",
        "新加坡银行同业拆借市场" => "006",
        _ => "001",
    };

    let currency_code = match symbol.as_str() {
        "Shibor人民币" | "Chibor人民币" => "CNY",
        "Libor英镑" => "GBP",
        "Libor欧元" | "Euribor欧元" => "EUR",
        "Libor美元" | "Hibor美元" | "Sibor美元" => "USD",
        "Libor日元" => "JPY",
        "Hibor人民币" => "CNH",
        "Hibor港币" => "HKD",
        "Sibor星元" => "SGD",
        _ => "CNY",
    };

    let indicator_id = match indicator.as_str() {
        "隔夜" => "001",
        "1周" => "101",
        "2周" => "102",
        "3周" => "103",
        "1月" => "201",
        "2月" => "202",
        "3月" => "203",
        "4月" => "204",
        "5月" => "205",
        "6月" => "206",
        "7月" => "207",
        "8月" => "208",
        "9月" => "209",
        "10月" => "210",
        "11月" => "211",
        "1年" => "301",
        _ => "001",
    };

    let filter_param = format!(
        "(MARKET_CODE=\"{}\")(CURRENCY_CODE=\"{}\")(INDICATOR_ID=\"{}\")",
        market_code, currency_code, indicator_id
    );

    let url = "https://datacenter-web.eastmoney.com/api/data/v1/get";
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64)")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .get(url)
        .query(&[
            ("reportName", "RPT_IMP_INTRESTRATEN"),
            (
                "columns",
                "REPORT_DATE,REPORT_PERIOD,IR_RATE,CHANGE_RATE,INDICATOR_ID,LATEST_RECORD,MARKET,MARKET_CODE,CURRENCY,CURRENCY_CODE",
            ),
            ("quoteColumns", ""),
            ("filter", filter_param.as_str()),
            ("pageNumber", "1"),
            ("pageSize", "500"),
            ("sortTypes", "-1"),
            ("sortColumns", "REPORT_DATE"),
        ])
        .send()
        .await
        .map_err(|e| format!("请求东方财富拆借利率失败: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("东方财富接口响应状态码: {}", res.status()));
    }

    let json_val: serde_json::Value = res
        .json()
        .await
        .map_err(|e| format!("解析 JSON 失败: {}", e))?;
    let data_arr = json_val["result"]["data"]
        .as_array()
        .ok_or_else(|| "响应结果中缺失 result.data 列表".to_string())?;

    let mut result = Vec::new();
    for row in data_arr {
        let date = row["REPORT_DATE"]
            .as_str()
            .map(|s| s.chars().take(10).collect());
        let rate = row["IR_RATE"].as_f64();
        let change_rate = row["CHANGE_RATE"].as_f64();
        let mkt_name = row["MARKET"].as_str().map(|s| s.to_string());
        let curr_code = row["CURRENCY_CODE"].as_str().map(|s| s.to_string());

        let mut extra = HashMap::new();
        if let Some(obj) = row.as_object() {
            for (k, v) in obj {
                extra.insert(k.clone(), v.clone());
            }
        }

        result.push(InterbankRateItem {
            date,
            rate,
            change_rate,
            market: mkt_name,
            currency: curr_code,
            extra,
        });
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_rate_interbank() {
        let query = InterbankRateQuery {
            market: Some("上海银行同业拆借市场".into()),
            symbol: Some("Shibor人民币".into()),
            indicator: Some("隔夜".into()),
        };
        let res = get_rate_interbank(query).await;
        assert!(res.is_ok());
    }
}
