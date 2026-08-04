//! 99期货网库存数据

use crate::models::ak::macro_data::MacroItem;
use crate::models::{Futures99Symbol, FuturesInventory99};
use anyhow::{anyhow, Result};
use reqwest::Client;
use serde_json::Value;
use std::collections::HashMap;

use super::common::QH99_STOCK_URL;

/// 获取99期货网品种映射表
/// 对应 akshare 的 __get_99_symbol_map() 函数
pub async fn get_99_symbol_map() -> Result<Vec<Futures99Symbol>> {
    let client = Client::builder()
        .danger_accept_invalid_certs(true)
        .build()?;

    println!("📡 请求99期货网品种映射 URL: {}", QH99_STOCK_URL);

    let response = client
        .get(QH99_STOCK_URL)
        .header(
            "User-Agent",
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36",
        )
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(anyhow!("获取99期货网品种映射失败: {}", response.status()));
    }

    let text = response.text().await?;

    use scraper::{Html, Selector};
    let document = Html::parse_document(&text);
    let script_selector = Selector::parse("script#__NEXT_DATA__").unwrap();

    let script = document
        .select(&script_selector)
        .next()
        .ok_or_else(|| anyhow!("未找到__NEXT_DATA__脚本标签"))?;

    let json_text = script.text().collect::<String>();
    let json_data: serde_json::Value =
        serde_json::from_str(&json_text).map_err(|e| anyhow!("解析JSON失败: {}", e))?;

    let mut symbols = Vec::new();

    if let Some(variety_list) =
        json_data["props"]["pageProps"]["data"]["varietyListData"].as_array()
    {
        for variety in variety_list {
            if let Some(product_list) = variety["productList"].as_array() {
                for product in product_list {
                    let product_id = product["productId"].as_i64().unwrap_or(0);
                    let name = product["name"].as_str().unwrap_or("").to_string();
                    let code = product["code"].as_str().unwrap_or("").to_string();

                    if product_id > 0 && !name.is_empty() {
                        symbols.push(Futures99Symbol {
                            product_id,
                            name,
                            code,
                        });
                    }
                }
            }
        }
    }

    println!("📊 解析到 {} 个品种映射", symbols.len());
    Ok(symbols)
}

/// 获取99期货网库存数据
/// 对应 akshare 的 futures_inventory_99() 函数
/// symbol: 品种名称（如"豆一"）或代码（如"A"）
pub async fn get_futures_inventory_99(symbol: &str) -> Result<Vec<FuturesInventory99>> {
    let symbols = get_99_symbol_map().await?;

    let product_id = symbols
        .iter()
        .find(|s| s.name == symbol || s.code.eq_ignore_ascii_case(symbol))
        .map(|s| s.product_id)
        .ok_or_else(|| anyhow!("未找到品种 {} 对应的编号", symbol))?;

    println!("📡 品种 {} 对应的ID: {}", symbol, product_id);

    let client = Client::builder()
        .danger_accept_invalid_certs(true)
        .build()?;

    let url = format!("{}?productId={}", QH99_STOCK_URL, product_id);
    println!("📡 请求99期货网库存数据 URL: {}", url);

    let response = client
        .get(&url)
        .header(
            "User-Agent",
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36",
        )
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(anyhow!("获取99期货网库存数据失败: {}", response.status()));
    }

    let text = response.text().await?;

    use scraper::{Html, Selector};
    let document = Html::parse_document(&text);
    let script_selector = Selector::parse("script#__NEXT_DATA__").unwrap();

    let script = document
        .select(&script_selector)
        .next()
        .ok_or_else(|| anyhow!("未找到__NEXT_DATA__脚本标签"))?;

    let json_text = script.text().collect::<String>();
    let json_data: serde_json::Value =
        serde_json::from_str(&json_text).map_err(|e| anyhow!("解析JSON失败: {}", e))?;

    let mut inventory_list = Vec::new();

    if let Some(list) =
        json_data["props"]["pageProps"]["data"]["positionTrendChartListData"]["list"].as_array()
    {
        for item in list {
            if let Some(arr) = item.as_array() {
                let date = arr
                    .first()
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();

                let close_price = arr.get(1).and_then(|v| {
                    if v.is_null() {
                        None
                    } else if let Some(s) = v.as_str() {
                        s.parse::<f64>().ok()
                    } else {
                        v.as_f64()
                    }
                });

                let inventory = arr.get(2).and_then(|v| {
                    if v.is_null() {
                        None
                    } else if let Some(n) = v.as_i64() {
                        Some(n as f64)
                    } else {
                        v.as_f64()
                    }
                });

                if !date.is_empty() {
                    inventory_list.push(FuturesInventory99 {
                        date,
                        close_price,
                        inventory,
                    });
                }
            }
        }
    }

    inventory_list.sort_by(|a, b| a.date.cmp(&b.date));

    println!("📊 解析到 {} 条库存数据", inventory_list.len());
    Ok(inventory_list)
}

/// 东方财富 - 黄金/白银 COMEX 堆存数据
pub async fn futures_comex_inventory(symbol: &str) -> Result<Vec<MacroItem>, String> {
    let attr_id = match symbol {
        "白银" => "71",
        _ => "70", // 默认黄金
    };

    let url = "https://datacenter-api.jin10.com/reports/list_v2";
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .get(url)
        .header("x-app-id", "rU6QIu7JHe2gOUeR")
        .header("x-version", "1.0.0")
        .query(&[("category", "ec"), ("attr_id", attr_id)])
        .send()
        .await
        .map_err(|e| format!("请求 COMEX 库存失败: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("接口代码: {}", res.status()));
    }

    let json_val: Value = res.json().await.map_err(|e| e.to_string())?;
    let values_arr = json_val["data"]["values"]
        .as_array()
        .ok_or("缺失 values 数组")?;

    let mut result = Vec::new();
    for row in values_arr {
        if let Some(val_row) = row.as_array() {
            let mut data = HashMap::new();
            data.insert("品种".to_string(), Value::String(symbol.to_string()));
            if let Some(v) = val_row.first() {
                data.insert("日期".to_string(), v.clone());
            }
            if let Some(v) = val_row.get(1) {
                data.insert("库存量".to_string(), v.clone());
            }
            result.push(MacroItem { data });
        }
    }

    Ok(result)
}

/// 东方财富 - 国内期货库存数据
pub async fn futures_inventory_em(_symbol: &str) -> Result<Vec<MacroItem>, String> {
    let url = "https://datacenter-web.eastmoney.com/api/data/v1/get?reportName=RPT_FUTURES_INVENTORY&columns=ALL&sortColumns=REPORT_DATE&sortTypes=-1&pageNumber=1&pageSize=500";
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .get(url)
        .send()
        .await
        .map_err(|e| format!("请求东财期货库存失败: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("接口代码: {}", res.status()));
    }

    let json_val: Value = res.json().await.map_err(|e| e.to_string())?;
    let arr = json_val["result"]["data"]
        .as_array()
        .ok_or("缺失 data 数组")?;

    let mut result = Vec::new();
    for row in arr {
        let mut data = HashMap::new();
        if let Some(obj) = row.as_object() {
            for (k, v) in obj {
                let name = match k.as_str() {
                    "SECURITY_CODE" => "品种代码",
                    "SECURITY_NAME" => "品种名称",
                    "INVENTORY" => "库存量",
                    "INVENTORY_CHANGE" => "库存增减",
                    _ => k,
                };
                data.insert(name.to_string(), v.clone());
            }
        }
        result.push(MacroItem { data });
    }

    Ok(result)
}

/// 上海金属网 - 期货新闻资讯
pub async fn futures_news_shmet(symbol: &str) -> Result<Vec<MacroItem>, String> {
    let url = "https://news.shmet.com/api/news/getNewsList";
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .post(url)
        .header("Content-Type", "application/json")
        .body(format!(
            r#"{{"page":1,"pageSize":50,"keyword":"{}"}}"#,
            symbol
        ))
        .send()
        .await
        .map_err(|e| format!("请求上海金属网资讯失败: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("上海金属网接口代码: {}", res.status()));
    }

    let json_val: Value = res.json().await.map_err(|e| e.to_string())?;
    let arr = json_val["data"]["list"]
        .as_array()
        .ok_or("缺失 list 数组")?;

    let mut result = Vec::new();
    for row in arr {
        let mut data = HashMap::new();
        if let Some(obj) = row.as_object() {
            for (k, v) in obj {
                let name = match k.as_str() {
                    "title" => "新闻标题",
                    "publishTime" => "发布时间",
                    "source" => "来源",
                    _ => k,
                };
                data.insert(name.to_string(), v.clone());
            }
        }
        result.push(MacroItem { data });
    }

    Ok(result)
}

/// 中证商品指数公司 - 中证商品期货指数
pub async fn futures_index_ccidx(symbol: &str) -> Result<Vec<MacroItem>, String> {
    let url = "http://www.ccidx.com/index/getHistoryIndex.do";
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .get(url)
        .query(&[("indexCode", "100001"), ("page", "1"), ("pageSize", "500")])
        .send()
        .await
        .map_err(|e| format!("请求中证商品指数失败: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("接口代码: {}", res.status()));
    }

    let json_val: Value = res.json().await.map_err(|e| e.to_string())?;
    let arr = json_val["data"]["list"]
        .as_array()
        .ok_or("缺失 list 数组")?;

    let mut result = Vec::new();
    for row in arr {
        let mut data = HashMap::new();
        if let Some(obj) = row.as_object() {
            for (k, v) in obj {
                let name = match k.as_str() {
                    "indexDate" => "日期",
                    "indexValue" => "收盘价",
                    "changeRate" => "涨跌幅",
                    _ => k,
                };
                data.insert(name.to_string(), v.clone());
            }
        }
        data.insert("指数名称".to_string(), Value::String(symbol.to_string()));
        result.push(MacroItem { data });
    }

    Ok(result)
}

/// 东方财富 - 期货交易规则与品种参数
pub async fn futures_rule_em() -> Result<Vec<MacroItem>, String> {
    let url = "https://datacenter-web.eastmoney.com/api/data/v1/get?reportName=RPT_FUTURES_RULE&columns=ALL&sortColumns=SECURITY_CODE&sortTypes=1&pageNumber=1&pageSize=500";
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .get(url)
        .send()
        .await
        .map_err(|e| format!("请求东财交易规则失败: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("接口代码: {}", res.status()));
    }

    let json_val: Value = res.json().await.map_err(|e| e.to_string())?;
    let arr = json_val["result"]["data"]
        .as_array()
        .ok_or("缺失 data 数组")?;

    let mut result = Vec::new();
    for row in arr {
        let mut data = HashMap::new();
        if let Some(obj) = row.as_object() {
            for (k, v) in obj {
                let name = match k.as_str() {
                    "SECURITY_CODE" => "品种代码",
                    "SECURITY_NAME" => "品种名称",
                    "TRADE_UNIT" => "交易单位",
                    "MIN_CHANGE_PRICE" => "最小变动价位",
                    "TRADE_MARGIN" => "最低保证金比例",
                    _ => k,
                };
                data.insert(name.to_string(), v.clone());
            }
        }
        result.push(MacroItem { data });
    }

    Ok(result)
}

/// 新加坡交易所 - SGX 结算价数据
pub async fn futures_settlement_price_sgx(date: &str) -> Result<Vec<MacroItem>, String> {
    let url = format!(
        "https://www.sgx.com/json.html?src=SGX_SETTLEMENT_PRICE&date={}",
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
        .map_err(|e| format!("请求 SGX 结算价失败: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("SGX 接口代码: {}", res.status()));
    }

    let json_val: Value = res.json().await.map_err(|e| e.to_string())?;
    let arr = json_val["data"].as_array().ok_or("缺失 data 数组")?;

    let mut result = Vec::new();
    for row in arr {
        let mut data = HashMap::new();
        if let Some(obj) = row.as_object() {
            for (k, v) in obj {
                let name = match k.as_str() {
                    "contractCode" => "合约代码",
                    "settlementPrice" => "结算价",
                    _ => k,
                };
                data.insert(name.to_string(), v.clone());
            }
        }
        result.push(MacroItem { data });
    }

    Ok(result)
}

/// 东方财富 - 期货与现货股票对照表
pub async fn futures_spot_stock(symbol: &str) -> Result<Vec<MacroItem>, String> {
    let url = "https://datacenter-web.eastmoney.com/api/data/v1/get?reportName=RPT_FUTURES_SPOT_STOCK&columns=ALL&sortColumns=SECURITY_CODE&sortTypes=1&pageNumber=1&pageSize=500";
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .get(url)
        .send()
        .await
        .map_err(|e| format!("请求现货对照失败: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("接口代码: {}", res.status()));
    }

    let json_val: Value = res.json().await.map_err(|e| e.to_string())?;
    let arr = json_val["result"]["data"]
        .as_array()
        .ok_or("缺失 data 数组")?;

    let mut result = Vec::new();
    for row in arr {
        let mut data = HashMap::new();
        if let Some(obj) = row.as_object() {
            for (k, v) in obj {
                data.insert(k.clone(), v.clone());
            }
        }
        data.insert("板块".to_string(), Value::String(symbol.to_string()));
        result.push(MacroItem { data });
    }

    Ok(result)
}
