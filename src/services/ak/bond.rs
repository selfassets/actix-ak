//! 债券数据服务
//!
//! 提供可转债、中国/美国国债收益率等数据获取与解析

use crate::models::ak::bond::{
    BondBuyBackItem, BondCbProfileItem, BondCovComparisonItem, BondGbKlineItem, BondJslItem,
    BondQuery, BondZhCovSpotItem, BondZhUsRateItem,
};
use std::collections::HashMap;

/// 1. 沪深可转债实时行情（新浪数据源）
pub async fn get_bond_zh_cov_spot() -> Result<Vec<BondZhCovSpotItem>, String> {
    let url = "http://vip.stock.finance.sina.com.cn/quotes_service/api/json_srv.php/Market_Center.getHQNodeData?page=1&num=500&sort=symbol&asc=1&node=hsb_z&symbol=";
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64)")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .get(url)
        .send()
        .await
        .map_err(|e| format!("请求新浪可转债行情失败: {}", e))?;
    if !res.status().is_success() {
        return Err(format!("请求可转债行情失败，HTTP 状态码: {}", res.status()));
    }

    let text = res
        .text()
        .await
        .map_err(|e| format!("读取响应数据失败: {}", e))?;
    parse_sina_cov_spot_json(&text)
}

/// 解析新浪可转债中心返回的 JSON 列表
fn parse_sina_cov_spot_json(json_str: &str) -> Result<Vec<BondZhCovSpotItem>, String> {
    let list: Vec<HashMap<String, serde_json::Value>> =
        serde_json::from_str(json_str).map_err(|e| format!("解析可转债 JSON 数据失败: {}", e))?;

    let mut result = Vec::new();
    for item in list {
        let code = item
            .get("code")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let name = item
            .get("name")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let trade = item
            .get("trade")
            .and_then(|v| v.as_str())
            .and_then(|s| s.parse::<f64>().ok());
        let change_price = item
            .get("pricechange")
            .and_then(|v| v.as_str())
            .and_then(|s| s.parse::<f64>().ok());
        let change_percent = item
            .get("changepercent")
            .and_then(|v| v.as_str())
            .and_then(|s| s.parse::<f64>().ok());

        result.push(BondZhCovSpotItem {
            code,
            name,
            trade,
            change_price,
            change_percent,
            extra: item,
        });
    }

    Ok(result)
}

/// 2. 新浪财经-中国国债收益率历史行情数据
pub async fn get_bond_gb_zh_sina(query: BondQuery) -> Result<Vec<BondGbKlineItem>, String> {
    let symbol = query.symbol.unwrap_or_else(|| "中国10年期国债".to_string());
    let symbol_code = match symbol.as_str() {
        "中国1年期国债" => "CN1YT",
        "中国2年期国债" => "CN2YT",
        "中国3年期国债" => "CN3YT",
        "中国5年期国债" => "CN5YT",
        "中国7年期国债" => "CN7YT",
        "中国10年期国债" => "CN10YT",
        "中国15年期国债" => "CN15YT",
        "中国20年期国债" => "CN20YT",
        "中国30年期国债" => "CN30YT",
        _ => "CN10YT",
    };

    fetch_sina_gb_daily(symbol_code).await
}

/// 3. 新浪财经-美国国债收益率历史行情数据
pub async fn get_bond_gb_us_sina(query: BondQuery) -> Result<Vec<BondGbKlineItem>, String> {
    let symbol = query.symbol.unwrap_or_else(|| "美国10年期国债".to_string());
    let symbol_code = match symbol.as_str() {
        "美国1月期国债" => "US1MT",
        "美国2月期国债" => "US2MT",
        "美国3月期国债" => "US3MT",
        "美国4月期国债" => "US4MT",
        "美国6月期国债" => "US6MT",
        "美国1年期国债" => "US1YT",
        "美国2年期国债" => "US2YT",
        "美国3年期国债" => "US3YT",
        "美国5年期国债" => "US5YT",
        "美国7年期国债" => "US7YT",
        "美国10年期国债" => "US10YT",
        "美国20年期国债" => "US20YT",
        "美国30年期国债" => "US30YT",
        _ => "US10YT",
    };

    fetch_sina_gb_daily(symbol_code).await
}

/// 通用请求新浪国债历史行情接口
async fn fetch_sina_gb_daily(symbol_code: &str) -> Result<Vec<BondGbKlineItem>, String> {
    let url = format!(
        "https://bond.finance.sina.com.cn/hq/gb/daily?symbol={}",
        symbol_code
    );
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64)")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("请求新浪国债接口失败: {}", e))?;
    if !res.status().is_success() {
        return Err(format!("新浪国债接口响应状态码: {}", res.status()));
    }

    let json_val: serde_json::Value = res
        .json()
        .await
        .map_err(|e| format!("解析 JSON 响应失败: {}", e))?;
    let data_arr = json_val["result"]["data"]
        .as_array()
        .ok_or_else(|| "响应结果中缺失 result.data 列表".to_string())?;

    let mut result = Vec::new();
    for row in data_arr {
        if let Some(arr) = row.as_array() {
            if arr.len() >= 6 {
                let date = arr[0].as_str().map(|s| s.to_string());
                let open = arr[1].as_str().and_then(|s| s.parse::<f64>().ok());
                let high = arr[2].as_str().and_then(|s| s.parse::<f64>().ok());
                let low = arr[3].as_str().and_then(|s| s.parse::<f64>().ok());
                let close = arr[4].as_str().and_then(|s| s.parse::<f64>().ok());
                let volume = arr[5].as_str().and_then(|s| s.parse::<f64>().ok());

                result.push(BondGbKlineItem {
                    date,
                    open,
                    high,
                    low,
                    close,
                    volume,
                });
            }
        }
    }

    Ok(result)
}

/// 4. 东方财富网-中美国债收益率
pub async fn get_bond_zh_us_rate() -> Result<Vec<BondZhUsRateItem>, String> {
    let url = "https://datacenter.eastmoney.com/api/data/get?type=RPTA_WEB_TREASURYYIELD&sty=ALL&st=SOLAR_DATE&sr=-1&token=894050c76af8597a853f5b408b759f5d&p=1&ps=500&pageNo=1&pageNum=1";
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64)")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .get(url)
        .send()
        .await
        .map_err(|e| format!("请求东方财富国债收益率失败: {}", e))?;
    if !res.status().is_success() {
        return Err(format!("请求东方财富接口失败，状态码: {}", res.status()));
    }

    let json_val: serde_json::Value = res
        .json()
        .await
        .map_err(|e| format!("解析 JSON 响应失败: {}", e))?;
    let data_arr = json_val["result"]["data"]
        .as_array()
        .ok_or_else(|| "响应中缺失 result.data 字段".to_string())?;

    let mut result = Vec::new();
    for row in data_arr {
        let date = row["SOLAR_DATE"]
            .as_str()
            .map(|s| s.chars().take(10).collect());
        let cn_1y = row["CHINA_1YEAR"].as_f64();
        let cn_10y = row["CHINA_10YEAR"].as_f64();
        let us_1y = row["USA_1YEAR"].as_f64();
        let us_10y = row["USA_10YEAR"].as_f64();
        let spread_10y = row["SPREAD_10YEAR"].as_f64();

        let mut extra = HashMap::new();
        if let Some(obj) = row.as_object() {
            for (k, v) in obj {
                extra.insert(k.clone(), v.clone());
            }
        }

        result.push(BondZhUsRateItem {
            date,
            cn_1y,
            cn_10y,
            us_1y,
            us_10y,
            spread_10y,
            extra,
        });
    }

    Ok(result)
}

/// 5. 东方财富网-上证质押式国债逆回购行情
pub async fn get_bond_sh_buy_back() -> Result<Vec<BondBuyBackItem>, String> {
    fetch_eastmoney_buy_back("m:1+b:MK0356").await
}

/// 6. 东方财富网-深证质押式国债逆回购行情
pub async fn get_bond_sz_buy_back() -> Result<Vec<BondBuyBackItem>, String> {
    fetch_eastmoney_buy_back("m:0+b:MK0356").await
}

/// 通用请求东方财富质押式国债逆回购行情
async fn fetch_eastmoney_buy_back(fs_param: &str) -> Result<Vec<BondBuyBackItem>, String> {
    let url = "https://push2.eastmoney.com/api/qt/clist/get";
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64)")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .get(url)
        .query(&[
            ("np", "1"),
            ("fltt", "1"),
            ("invt", "2"),
            ("fs", fs_param),
            (
                "fields",
                "f12,f13,f14,f1,f2,f4,f3,f152,f17,f18,f15,f16,f5,f6",
            ),
            ("fid", "f6"),
            ("pn", "1"),
            ("pz", "50"),
            ("po", "1"),
            ("dect", "1"),
        ])
        .send()
        .await
        .map_err(|e| format!("请求质押式回购行情失败: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("东方财富接口响应状态码: {}", res.status()));
    }

    let json_val: serde_json::Value = res
        .json()
        .await
        .map_err(|e| format!("解析 JSON 响应失败: {}", e))?;
    let diff_arr = json_val["data"]["diff"]
        .as_array()
        .ok_or_else(|| "响应结果中缺失 data.diff 列表".to_string())?;

    let mut result = Vec::new();
    for row in diff_arr {
        let code = row["f12"].as_str().map(|s| s.to_string());
        let name = row["f14"].as_str().map(|s| s.to_string());
        let price = row["f2"].as_f64().map(|v| v / 1000.0);
        let change_price = row["f4"].as_f64().map(|v| v / 1000.0);
        let change_percent = row["f3"].as_f64().map(|v| v / 100.0);
        let open = row["f17"].as_f64().map(|v| v / 1000.0);
        let high = row["f15"].as_f64().map(|v| v / 1000.0);
        let low = row["f16"].as_f64().map(|v| v / 1000.0);
        let close = row["f18"].as_f64().map(|v| v / 1000.0);
        let volume = row["f5"].as_f64();
        let amount = row["f6"].as_f64();

        result.push(BondBuyBackItem {
            code,
            name,
            price,
            change_price,
            change_percent,
            open,
            high,
            low,
            close,
            volume,
            amount,
        });
    }

    Ok(result)
}

/// 7. 集思录可转债等权指数历史
pub async fn get_bond_cb_index_jsl() -> Result<Vec<BondJslItem>, String> {
    let url = "https://www.jisilu.cn/webapi/cb/index_history/";
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64)")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .get(url)
        .send()
        .await
        .map_err(|e| format!("请求集思录指数失败: {}", e))?;
    if !res.status().is_success() {
        return Err(format!("集思录接口响应状态码: {}", res.status()));
    }

    let json_val: serde_json::Value = res
        .json()
        .await
        .map_err(|e| format!("解析 JSON 失败: {}", e))?;
    let data_arr = json_val["data"]
        .as_array()
        .ok_or_else(|| "响应结果中缺失 data 列表".to_string())?;

    let mut result = Vec::new();
    for row in data_arr {
        let mut data = HashMap::new();
        if let Some(obj) = row.as_object() {
            for (k, v) in obj {
                data.insert(k.clone(), v.clone());
            }
        }
        result.push(BondJslItem { data });
    }

    Ok(result)
}

/// 8. 集思录可转债-强赎信息列表
pub async fn get_bond_cb_redeem_jsl() -> Result<Vec<BondJslItem>, String> {
    let url = "https://www.jisilu.cn/data/cbnew/redeem_list/";
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .post(url)
        .header("Referer", "https://www.jisilu.cn/data/cbnew/")
        .header("X-Requested-With", "XMLHttpRequest")
        .form(&[("rp", "50")])
        .send()
        .await
        .map_err(|e| format!("请求集思录强赎数据失败: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("集思录强赎接口状态码: {}", res.status()));
    }

    let json_val: serde_json::Value = res
        .json()
        .await
        .map_err(|e| format!("解析强赎 JSON 失败: {}", e))?;
    let rows_arr = json_val["rows"]
        .as_array()
        .ok_or_else(|| "缺失 rows 列表".to_string())?;

    let mut result = Vec::new();
    for row in rows_arr {
        let mut data = HashMap::new();
        if let Some(cell) = row["cell"].as_object() {
            for (k, v) in cell {
                data.insert(k.clone(), v.clone());
            }
        } else if let Some(obj) = row.as_object() {
            for (k, v) in obj {
                data.insert(k.clone(), v.clone());
            }
        }
        result.push(BondJslItem { data });
    }

    Ok(result)
}

/// 9. 新浪财经-可转债-详情资料
pub async fn get_bond_cb_profile_sina(query: BondQuery) -> Result<Vec<BondCbProfileItem>, String> {
    let symbol = query.symbol.unwrap_or_else(|| "sz128039".to_string());
    let url = format!(
        "https://money.finance.sina.com.cn/bond/info/{}.html",
        symbol
    );

    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64)")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("请求新浪可转债详情失败: {}", e))?;
    if !res.status().is_success() {
        return Err(format!("新浪可转债详情响应状态码: {}", res.status()));
    }

    let html_bytes = res
        .bytes()
        .await
        .map_err(|e| format!("读取响应数据失败: {}", e))?;
    let (text, _, _) = encoding_rs::GBK.decode(&html_bytes);

    parse_sina_cb_profile_html(&text)
}

/// 解析新浪可转债详情网页 HTML 表格
fn parse_sina_cb_profile_html(html_str: &str) -> Result<Vec<BondCbProfileItem>, String> {
    let document = scraper::Html::parse_document(html_str);
    let tr_selector =
        scraper::Selector::parse("tr").map_err(|_| "构建 CSS 选择器失败".to_string())?;
    let td_selector =
        scraper::Selector::parse("td, th").map_err(|_| "构建 CSS 选择器失败".to_string())?;

    let mut result = Vec::new();

    for tr in document.select(&tr_selector) {
        let cells: Vec<String> = tr
            .select(&td_selector)
            .map(|c| c.text().collect::<Vec<_>>().join("").trim().to_string())
            .collect();

        if cells.len() >= 2 {
            let item = cells[0].clone();
            let value = cells[1].clone();
            if !item.is_empty() {
                result.push(BondCbProfileItem { item, value });
            }
        }
    }

    Ok(result)
}

/// 10. 东方财富网-可转债比价表
pub async fn get_bond_cov_comparison() -> Result<Vec<BondCovComparisonItem>, String> {
    let url = "https://16.push2.eastmoney.com/api/qt/clist/get";
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64)")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .get(url)
        .query(&[
            ("pn", "1"),
            ("pz", "100"),
            ("po", "1"),
            ("np", "1"),
            ("ut", "bd1d9ddb04089700cf9c27f6f7426281"),
            ("fltt", "2"),
            ("invt", "2"),
            ("fid", "f243"),
            ("fs", "b:MK0354"),
            (
                "fields",
                "f1,f152,f2,f3,f12,f13,f14,f227,f228,f229,f230,f231,f232,f233,f234,f235,f236,f237,f238,f239,f240,f241,f242,f26,f243",
            ),
        ])
        .send()
        .await
        .map_err(|e| format!("请求可转债比价表失败: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("东方财富接口响应状态码: {}", res.status()));
    }

    let json_val: serde_json::Value = res
        .json()
        .await
        .map_err(|e| format!("解析 JSON 失败: {}", e))?;
    let diff_arr = json_val["data"]["diff"]
        .as_array()
        .ok_or_else(|| "缺失 data.diff 列表".to_string())?;

    let mut result = Vec::new();
    for row in diff_arr {
        let mut data = HashMap::new();
        if let Some(obj) = row.as_object() {
            for (k, v) in obj {
                data.insert(k.clone(), v.clone());
            }
        }
        result.push(BondCovComparisonItem { data });
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_parse_sina_cov_spot() {
        let sample_json = r#"[{"code":"sh113527","name":"维格转债","trade":"112.5","pricechange":"0.5","changepercent":"0.44"}]"#;
        let res = parse_sina_cov_spot_json(sample_json);
        assert!(res.is_ok());
        let items = res.unwrap();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].code.as_deref(), Some("sh113527"));
        assert_eq!(items[0].trade, Some(112.5));
    }
}
