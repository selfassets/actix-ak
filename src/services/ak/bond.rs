//! 债券数据服务
//!
//! 提供可转债、中国/美国国债收益率等数据获取与解析

use crate::models::ak::bond::{
    BondAvailableIndexItem, BondBuyBackItem, BondCbAdjLogJslItem, BondCbProfileItem,
    BondCbondIndexItem, BondCbondQuery, BondChinaCloseReturnItem, BondChinaMoneyItem,
    BondChinaYieldItem, BondCovComparisonItem, BondCovInfoThsItem, BondDebtNafmiiItem,
    BondGbKlineItem, BondInfoCmItem, BondInfoCmQueryItem, BondInfoDetailCmItem,
    BondIssueCninfoItem, BondJslItem, BondQuery, BondSpotDealItem, BondSpotQuoteItem,
    BondSseSummaryItem, BondZhCovSpotItem, BondZhCovValueAnalysisItem, BondZhHsSpotItem,
    BondZhUsRateItem,
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

/// 11. 新浪财经-沪深债券实时行情数据
pub async fn get_bond_zh_hs_spot() -> Result<Vec<BondZhHsSpotItem>, String> {
    let url = "http://vip.stock.finance.sina.com.cn/quotes_service/api/json_srv.php/Market_Center.getHQNodeData?page=1&num=200&sort=symbol&asc=1&node=hs_z&symbol=";
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64)")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .get(url)
        .send()
        .await
        .map_err(|e| format!("请求新浪沪深债券失败: {}", e))?;
    if !res.status().is_success() {
        return Err(format!("新浪接口响应状态码: {}", res.status()));
    }

    let text = res
        .text()
        .await
        .map_err(|e| format!("读取响应数据失败: {}", e))?;
    let list: Vec<HashMap<String, serde_json::Value>> =
        serde_json::from_str(&text).map_err(|e| format!("解析沪深债券 JSON 失败: {}", e))?;

    let mut result = Vec::new();
    for item in list {
        let symbol = item
            .get("symbol")
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
        let buy = item
            .get("buy")
            .and_then(|v| v.as_str())
            .and_then(|s| s.parse::<f64>().ok());
        let sell = item
            .get("sell")
            .and_then(|v| v.as_str())
            .and_then(|s| s.parse::<f64>().ok());
        let settlement = item
            .get("settlement")
            .and_then(|v| v.as_str())
            .and_then(|s| s.parse::<f64>().ok());
        let open = item
            .get("open")
            .and_then(|v| v.as_str())
            .and_then(|s| s.parse::<f64>().ok());
        let high = item
            .get("high")
            .and_then(|v| v.as_str())
            .and_then(|s| s.parse::<f64>().ok());
        let low = item
            .get("low")
            .and_then(|v| v.as_str())
            .and_then(|s| s.parse::<f64>().ok());
        let volume = item
            .get("volume")
            .and_then(|v| v.as_str())
            .and_then(|s| s.parse::<f64>().ok());
        let amount = item
            .get("amount")
            .and_then(|v| v.as_str())
            .and_then(|s| s.parse::<f64>().ok());

        result.push(BondZhHsSpotItem {
            symbol,
            name,
            trade,
            change_price,
            change_percent,
            buy,
            sell,
            settlement,
            open,
            high,
            low,
            volume,
            amount,
        });
    }

    Ok(result)
}

/// 12. 东方财富网-质押式国债逆回购历史 K 线行情
pub async fn get_bond_buy_back_hist_em(query: BondQuery) -> Result<Vec<BondGbKlineItem>, String> {
    let symbol = query.symbol.unwrap_or_else(|| "204001".to_string());
    let market_id = if symbol.starts_with('1') { "0" } else { "1" };

    let url = "https://push2his.eastmoney.com/api/qt/stock/kline/get";
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64)")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let secid = format!("{}.{}", market_id, symbol);

    let res = client
        .get(url)
        .query(&[
            ("secid", secid.as_str()),
            ("klt", "101"),
            ("fqt", "1"),
            ("lmt", "10000"),
            ("end", "20500000"),
            ("iscca", "1"),
            ("fields1", "f1,f2,f3,f4,f5,f6,f7,f8"),
            (
                "fields2",
                "f51,f52,f53,f54,f55,f56,f57,f58,f59,f60,f61,f62,f63,f64",
            ),
            ("forcect", "1"),
        ])
        .send()
        .await
        .map_err(|e| format!("请求质押式回购历史 K 线失败: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("东方财富接口状态码: {}", res.status()));
    }

    let json_val: serde_json::Value = res
        .json()
        .await
        .map_err(|e| format!("解析 JSON 响应失败: {}", e))?;
    let klines_arr = json_val["data"]["klines"]
        .as_array()
        .ok_or_else(|| "响应结果中缺失 data.klines 列表".to_string())?;

    let mut result = Vec::new();
    for row in klines_arr {
        if let Some(s) = row.as_str() {
            let parts: Vec<&str> = s.split(',').collect();
            if parts.len() >= 7 {
                let date = Some(parts[0].to_string());
                let open = parts[1].parse::<f64>().ok();
                let close = parts[2].parse::<f64>().ok();
                let high = parts[3].parse::<f64>().ok();
                let low = parts[4].parse::<f64>().ok();
                let volume = parts[5].parse::<f64>().ok();

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

/// 13. 中国外汇交易中心(ChinaMoney) - 收益率曲线品种映射表
pub async fn get_bond_china_close_return_map() -> Result<Vec<BondChinaMoneyItem>, String> {
    let url = "https://www.chinamoney.com.cn/ags/ms/cm-u-bk-currency/ClsYldCurvCurvGO";
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .get(url)
        .header(
            "Referer",
            "https://www.chinamoney.com.cn/chinese/bkcurvclosedyhis/?bondType=CYCC000&reference=1",
        )
        .header("X-Requested-With", "XMLHttpRequest")
        .send()
        .await
        .map_err(|e| format!("请求 ChinaMoney 收益率映射失败: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("ChinaMoney 接口响应状态码: {}", res.status()));
    }

    let json_val: serde_json::Value = res
        .json()
        .await
        .map_err(|e| format!("解析 JSON 响应失败: {}", e))?;
    let data_arr = json_val["data"]["clsYldCurvList"]
        .as_array()
        .or_else(|| json_val["records"].as_array())
        .or_else(|| json_val["data"].as_array())
        .ok_or_else(|| "缺失收益率曲线列表字段".to_string())?;

    let mut result = Vec::new();
    for row in data_arr {
        let mut data = HashMap::new();
        if let Some(obj) = row.as_object() {
            for (k, v) in obj {
                data.insert(k.clone(), v.clone());
            }
        }
        result.push(BondChinaMoneyItem { data });
    }

    Ok(result)
}

/// 33. 中国外汇交易中心 - 收盘收益率曲线历史数据
pub async fn get_bond_china_close_return(
    query: BondQuery,
) -> Result<Vec<BondChinaCloseReturnItem>, String> {
    let symbol = query.symbol.unwrap_or_else(|| "CYCC000".to_string());
    let start_date = query.start_date.unwrap_or_else(|| "20231101".to_string());
    let end_date = query.end_date.unwrap_or_else(|| "20231101".to_string());

    let start_fmt = if start_date.len() >= 8 {
        format!(
            "{}-{}-{}",
            &start_date[0..4],
            &start_date[4..6],
            &start_date[6..8]
        )
    } else {
        "2023-11-01".to_string()
    };
    let end_fmt = if end_date.len() >= 8 {
        format!(
            "{}-{}-{}",
            &end_date[0..4],
            &end_date[4..6],
            &end_date[6..8]
        )
    } else {
        "2023-11-01".to_string()
    };

    let url = "https://www.chinamoney.com.cn/ags/ms/cm-u-bk-currency/ClsYldCurvHis";
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .get(url)
        .header(
            "Referer",
            "https://www.chinamoney.com.cn/chinese/bkcurvclosedyhis/?bondType=CYCC000&reference=1",
        )
        .query(&[
            ("lang", "CN"),
            ("reference", "1,2,3"),
            ("bondType", symbol.as_str()),
            ("startDate", start_fmt.as_str()),
            ("endDate", end_fmt.as_str()),
            ("termId", "1"),
            ("pageNum", "1"),
            ("pageSize", "50"),
        ])
        .send()
        .await
        .map_err(|e| format!("请求 ChinaMoney 收盘收益率曲线失败: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("ChinaMoney 接口响应状态码: {}", res.status()));
    }

    let json_val: serde_json::Value = res
        .json()
        .await
        .map_err(|e| format!("解析 JSON 失败: {}", e))?;
    let records = json_val["records"]
        .as_array()
        .ok_or_else(|| "缺失 records 字段".to_string())?;

    let mut result = Vec::new();
    for row in records {
        let date = row["date"]
            .as_str()
            .or_else(|| row["dateTime"].as_str())
            .map(|s| s.to_string());
        let term = row["termId"]
            .as_f64()
            .or_else(|| row["termId"].as_str().and_then(|s| s.parse().ok()));
        let ytm = row["ytm"]
            .as_f64()
            .or_else(|| row["ytm"].as_str().and_then(|s| s.parse().ok()));
        let spot_rate = row["spotRate"]
            .as_f64()
            .or_else(|| row["spotRate"].as_str().and_then(|s| s.parse().ok()));
        let forward_rate = row["forwardRate"]
            .as_f64()
            .or_else(|| row["forwardRate"].as_str().and_then(|s| s.parse().ok()));

        result.push(BondChinaCloseReturnItem {
            date,
            term,
            ytm,
            spot_rate,
            forward_rate,
        });
    }

    Ok(result)
}

/// 14. 上登债券信息网-债券现货市场概览 (SSE Cash Summary)
pub async fn get_bond_cash_summary_sse(
    query: BondQuery,
) -> Result<Vec<BondSseSummaryItem>, String> {
    let date_str = query.start_date.unwrap_or_else(|| "20210111".to_string());
    let trade_date = if date_str.len() >= 8 {
        format!(
            "{}-{}-{}",
            &date_str[0..4],
            &date_str[4..6],
            &date_str[6..8]
        )
    } else {
        "2021-01-11".to_string()
    };

    let url = "http://query.sse.com.cn/commonExcelDd.do";
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .get(url)
        .header("Referer", "http://bond.sse.com.cn/")
        .query(&[
            ("sqlId", "COMMON_SSEBOND_SCSJ_SCTJ_SCGL_ZQXQSCGL_CX_L"),
            ("TRADE_DATE", trade_date.as_str()),
        ])
        .send()
        .await
        .map_err(|e| format!("请求上交所现货概览失败: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("上交所接口响应状态码: {}", res.status()));
    }

    let bytes = res
        .bytes()
        .await
        .map_err(|e| format!("读取响应 Excel 字节失败: {}", e))?;
    parse_sse_excel_summary(&bytes, &trade_date)
}

/// 15. 上登债券信息网-债券成交概览 (SSE Deal Summary)
pub async fn get_bond_deal_summary_sse(
    query: BondQuery,
) -> Result<Vec<BondSseSummaryItem>, String> {
    let date_str = query.start_date.unwrap_or_else(|| "20210104".to_string());
    let trade_date = if date_str.len() >= 8 {
        format!(
            "{}-{}-{}",
            &date_str[0..4],
            &date_str[4..6],
            &date_str[6..8]
        )
    } else {
        "2021-01-04".to_string()
    };

    let url = "http://query.sse.com.cn/commonExcelDd.do";
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .get(url)
        .header("Referer", "http://bond.sse.com.cn/")
        .query(&[
            ("sqlId", "COMMON_SSEBOND_SCSJ_SCTJ_SCGL_ZQCJGL_CX_L"),
            ("TRADE_DATE", trade_date.as_str()),
        ])
        .send()
        .await
        .map_err(|e| format!("请求上交所成交概览失败: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("上交所接口响应状态码: {}", res.status()));
    }

    let bytes = res
        .bytes()
        .await
        .map_err(|e| format!("读取响应 Excel 字节失败: {}", e))?;
    parse_sse_excel_summary(&bytes, &trade_date)
}

/// 通用解析上交所 Excel 概要表格 (.xls)
fn parse_sse_excel_summary(
    bytes: &[u8],
    trade_date: &str,
) -> Result<Vec<BondSseSummaryItem>, String> {
    use calamine::{DataType, Reader, Xls};
    use std::io::Cursor;

    let cursor = Cursor::new(bytes);
    let mut excel: Xls<_> =
        Xls::new(cursor).map_err(|e| format!("解析上交所 Excel(.xls) 失败: {}", e))?;

    let sheet_names = excel.sheet_names().to_vec();
    if sheet_names.is_empty() {
        return Ok(Vec::new());
    }

    let range = excel
        .worksheet_range(&sheet_names[0])
        .map_err(|e| format!("读取工作表失败: {}", e))?;

    let mut rows = range.rows();
    // 跳过表头
    let _ = rows.next();

    let mut result = Vec::new();

    for row in rows {
        if row.len() >= 3 {
            let name = row[0].as_string().map(|s| s.trim().to_string());
            let day_val = row[1]
                .as_f64()
                .or_else(|| row[1].as_i64().map(|i| i as f64));
            let year_val = row[2]
                .as_f64()
                .or_else(|| row[2].as_i64().map(|i| i as f64));
            let par_val = if row.len() >= 4 {
                row[3]
                    .as_f64()
                    .or_else(|| row[3].as_i64().map(|i| i as f64))
            } else {
                None
            };

            if name.is_some() {
                result.push(BondSseSummaryItem {
                    name,
                    day_val,
                    year_val,
                    par_val,
                    date: Some(trade_date.to_string()),
                });
            }
        }
    }

    Ok(result)
}

/// 16. 同花顺-数据中心-可转债信息
pub async fn get_bond_zh_cov_info_ths() -> Result<Vec<BondCovInfoThsItem>, String> {
    let url = "https://data.10jqka.com.cn/ipo/kzz/";
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .get(url)
        .send()
        .await
        .map_err(|e| format!("请求同花顺可转债失败: {}", e))?;
    if !res.status().is_success() {
        return Err(format!("同花顺接口状态码: {}", res.status()));
    }

    let json_val: serde_json::Value = res
        .json()
        .await
        .map_err(|e| format!("解析同花顺 JSON 失败: {}", e))?;
    let list_arr = json_val["list"]
        .as_array()
        .ok_or_else(|| "响应结果缺失 list 列表".to_string())?;

    let mut result = Vec::new();
    for row in list_arr {
        let mut data = HashMap::new();
        if let Some(obj) = row.as_object() {
            for (k, v) in obj {
                data.insert(k.clone(), v.clone());
            }
        }
        result.push(BondCovInfoThsItem { data });
    }

    Ok(result)
}

/// 17. 新浪财经-可转债-债券概况
pub async fn get_bond_cb_summary_sina(query: BondQuery) -> Result<Vec<BondCbProfileItem>, String> {
    let symbol = query.symbol.unwrap_or_else(|| "sh155255".to_string());
    let url = format!(
        "https://money.finance.sina.com.cn/bond/quotes/{}.html",
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
        .map_err(|e| format!("请求新浪可转债概况失败: {}", e))?;
    if !res.status().is_success() {
        return Err(format!("新浪可转债概况状态码: {}", res.status()));
    }

    let html_bytes = res
        .bytes()
        .await
        .map_err(|e| format!("读取响应数据失败: {}", e))?;
    let (text, _, _) = encoding_rs::GBK.decode(&html_bytes);

    parse_sina_cb_profile_html(&text)
}

/// 18. 中国外汇交易中心 - 现券市场做市报价
pub async fn get_bond_spot_quote() -> Result<Vec<BondSpotQuoteItem>, String> {
    let url = "https://www.chinamoney.com.cn/ags/ms/cm-u-md-bond/CbMktMakQuot";
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .post(url)
        .form(&[("flag", "1"), ("lang", "cn")])
        .send()
        .await
        .map_err(|e| format!("请求 ChinaMoney 做市报价失败: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("ChinaMoney 接口状态码: {}", res.status()));
    }

    let json_val: serde_json::Value = res
        .json()
        .await
        .map_err(|e| format!("解析做市报价 JSON 失败: {}", e))?;
    let records = json_val["records"]
        .as_array()
        .ok_or_else(|| "缺失 records 列表".to_string())?;

    let mut result = Vec::new();
    for row in records {
        let institution = row["mrkMakName"]
            .as_str()
            .or_else(|| row["mrkMakSubName"].as_str())
            .map(|s| s.to_string());
        let bond_name = row["bondDefinedCode"]
            .as_str()
            .or_else(|| row["bondName"].as_str())
            .map(|s| s.to_string());

        let buy_clean_price = row["buyCleanPrice"]
            .as_f64()
            .or_else(|| row["buyCleanPrice"].as_str().and_then(|s| s.parse().ok()));
        let sell_clean_price = row["sellCleanPrice"]
            .as_f64()
            .or_else(|| row["sellCleanPrice"].as_str().and_then(|s| s.parse().ok()));
        let buy_yield = row["buyYtm"]
            .as_f64()
            .or_else(|| row["buyYtm"].as_str().and_then(|s| s.parse().ok()));
        let sell_yield = row["sellYtm"]
            .as_f64()
            .or_else(|| row["sellYtm"].as_str().and_then(|s| s.parse().ok()));

        result.push(BondSpotQuoteItem {
            institution,
            bond_name,
            buy_clean_price,
            sell_clean_price,
            buy_yield,
            sell_yield,
        });
    }

    Ok(result)
}

/// 19. 中国外汇交易中心 - 现券市场成交行情
pub async fn get_bond_spot_deal() -> Result<Vec<BondSpotDealItem>, String> {
    let url = "https://www.chinamoney.com.cn/ags/ms/cm-u-md-bond/CbtPri";
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .post(url)
        .form(&[("flag", "1"), ("lang", "cn"), ("bondName", "")])
        .send()
        .await
        .map_err(|e| format!("请求 ChinaMoney 现券成交行情失败: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("ChinaMoney 接口状态码: {}", res.status()));
    }

    let json_val: serde_json::Value = res
        .json()
        .await
        .map_err(|e| format!("解析现券成交 JSON 失败: {}", e))?;
    let records = json_val["records"]
        .as_array()
        .ok_or_else(|| "缺失 records 列表".to_string())?;

    let mut result = Vec::new();
    for row in records {
        let bond_name = row["bondDefinedCode"]
            .as_str()
            .or_else(|| row["bondName"].as_str())
            .map(|s| s.to_string());
        let change = row["netPriceUpAmount"].as_f64().or_else(|| {
            row["netPriceUpAmount"]
                .as_str()
                .and_then(|s| s.parse().ok())
        });
        let weighted_yield = row["weightedYtm"]
            .as_f64()
            .or_else(|| row["weightedYtm"].as_str().and_then(|s| s.parse().ok()));
        let clean_price = row["netPrice"]
            .as_f64()
            .or_else(|| row["netPrice"].as_str().and_then(|s| s.parse().ok()));
        let latest_yield = row["latestYtm"]
            .as_f64()
            .or_else(|| row["latestYtm"].as_str().and_then(|s| s.parse().ok()));
        let volume = row["volume"]
            .as_f64()
            .or_else(|| row["volume"].as_str().and_then(|s| s.parse().ok()));

        result.push(BondSpotDealItem {
            bond_name,
            change,
            weighted_yield,
            clean_price,
            latest_yield,
            volume,
        });
    }

    Ok(result)
}

/// 30. 集思录-可转债转股价调整记录
pub async fn get_bond_cb_adj_logs_jsl(
    query: BondQuery,
) -> Result<Vec<BondCbAdjLogJslItem>, String> {
    let symbol = query.symbol.unwrap_or_else(|| "128013".to_string());
    let url = format!(
        "https://www.jisilu.cn/data/cbnew/adj_logs/?bond_id={}",
        symbol
    );

    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("请求集思录转股价调整日志失败: {}", e))?;
    if !res.status().is_success() {
        return Err(format!("集思录接口状态码: {}", res.status()));
    }

    let text = res
        .text()
        .await
        .map_err(|e| format!("读取文本失败: {}", e))?;
    if !text.contains("</table>") {
        return Ok(Vec::new());
    }

    parse_jsl_adj_logs_html(&text)
}

/// 解析集思录转股价调整记录 HTML 表格
fn parse_jsl_adj_logs_html(html_str: &str) -> Result<Vec<BondCbAdjLogJslItem>, String> {
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

        if cells.len() >= 3 {
            let date = Some(cells[0].clone());
            let before_price = cells[1].parse::<f64>().ok();
            let after_price = cells[2].parse::<f64>().ok();
            let effective_date = if cells.len() >= 4 {
                Some(cells[3].clone())
            } else {
                None
            };
            let remark = if cells.len() >= 5 {
                Some(cells[4].clone())
            } else {
                None
            };

            if before_price.is_some() || after_price.is_some() {
                result.push(BondCbAdjLogJslItem {
                    date,
                    before_price,
                    after_price,
                    effective_date,
                    remark,
                });
            }
        }
    }

    Ok(result)
}

/// 31. 中国债券信息网-中债指数可选项列表
pub async fn get_bond_available_index_cbond() -> Result<Vec<BondAvailableIndexItem>, String> {
    let index_names = vec![
        "新综合指数",
        "中债-国债指数",
        "金融债指数",
        "企业债指数",
        "央行票据指数",
        "短融指数",
        "中期票据指数",
        "综合指数",
        "高信用等级债券指数",
        "中高信用等级债券指数",
    ];

    let mut result = Vec::new();
    for (idx, name) in index_names.into_iter().enumerate() {
        result.push(BondAvailableIndexItem {
            index: idx + 1,
            name: name.to_string(),
        });
    }

    Ok(result)
}

/// 20. 中国债券信息网 (ChinaBond) - 国债及其他债券收益率曲线
pub async fn get_bond_china_yield(query: BondQuery) -> Result<Vec<BondChinaYieldItem>, String> {
    let start_date = query.start_date.unwrap_or_else(|| "20200204".to_string());
    let end_date = query.end_date.unwrap_or_else(|| "20210124".to_string());

    let start_fmt = if start_date.len() >= 8 {
        format!(
            "{}-{}-{}",
            &start_date[0..4],
            &start_date[4..6],
            &start_date[6..8]
        )
    } else {
        "2020-02-04".to_string()
    };
    let end_fmt = if end_date.len() >= 8 {
        format!(
            "{}-{}-{}",
            &end_date[0..4],
            &end_date[4..6],
            &end_date[6..8]
        )
    } else {
        "2021-01-24".to_string()
    };

    let url = format!(
        "https://yield.chinabond.com.cn/cbweb-pbc-web/pbc/historyQuery?startDate={}&endDate={}&gjqx=0&qxId=ycqx&locale=cn_ZH",
        start_fmt, end_fmt
    );

    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("请求 ChinaBond 接口失败: {}", e))?;
    if !res.status().is_success() {
        return Err(format!("ChinaBond 接口响应状态码: {}", res.status()));
    }

    let text = res
        .text()
        .await
        .map_err(|e| format!("读取响应 HTML 文本失败: {}", e))?;
    parse_chinabond_yield_html(&text)
}

/// 解析 ChinaBond HTML 表格
fn parse_chinabond_yield_html(html_str: &str) -> Result<Vec<BondChinaYieldItem>, String> {
    let text_clean = html_str.replace("&nbsp;", "").replace("&nbsp", "");
    let document = scraper::Html::parse_document(&text_clean);
    let tr_selector =
        scraper::Selector::parse("tr").map_err(|_| "构建 CSS 选择器失败".to_string())?;
    let td_selector =
        scraper::Selector::parse("td, th").map_err(|_| "构建 CSS 选择器失败".to_string())?;

    let mut rows_data = Vec::new();

    for tr in document.select(&tr_selector) {
        let cells: Vec<String> = tr
            .select(&td_selector)
            .map(|c| c.text().collect::<Vec<_>>().join("").trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        if !cells.is_empty() {
            rows_data.push(cells);
        }
    }

    if rows_data.len() < 2 {
        return Ok(Vec::new());
    }

    let headers = rows_data[0].clone();
    let mut result = Vec::new();

    for row in rows_data.into_iter().skip(1) {
        let mut data = HashMap::new();
        for (idx, field) in row.iter().enumerate() {
            let col_name = headers.get(idx).map(|s| s.as_str()).unwrap_or("");
            if col_name.is_empty() {
                continue;
            }

            if let Ok(val) = field.parse::<f64>() {
                data.insert(col_name.to_string(), serde_json::Value::from(val));
            } else {
                data.insert(
                    col_name.to_string(),
                    serde_json::Value::from(field.as_str()),
                );
            }
        }
        if !data.is_empty() {
            result.push(BondChinaYieldItem { data });
        }
    }

    Ok(result)
}

/// 21. 中国货币网 (ChinaMoney) - 债券信息查询参数 (主承销商、债券类型、评级等)
pub async fn get_bond_info_cm_query(query: BondQuery) -> Result<Vec<BondInfoCmQueryItem>, String> {
    let symbol = query.symbol.unwrap_or_else(|| "评级等级".to_string());

    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    if symbol == "主承销商" {
        let url = "https://www.chinamoney.com.cn/ags/ms/cm-u-bond-md/EntyFullNameSearchCondition";
        let res = client
            .post(url)
            .send()
            .await
            .map_err(|e| format!("请求 ChinaMoney 承销商失败: {}", e))?;
        let json_val: serde_json::Value = res
            .json()
            .await
            .map_err(|e| format!("解析 JSON 失败: {}", e))?;

        let mut result = Vec::new();
        if let Some(arr) = json_val["data"]["enty"].as_array() {
            for row in arr {
                let code = row["code"]
                    .as_str()
                    .map(|s| s.to_string())
                    .or_else(|| row["code"].as_i64().map(|i| i.to_string()));
                let name = row["name"].as_str().map(|s| s.to_string());
                result.push(BondInfoCmQueryItem { code, name });
            }
        }
        Ok(result)
    } else {
        let url = "https://www.chinamoney.com.cn/ags/ms/cm-u-bond-md/BondBaseInfoSearchCondition";
        let res = client
            .post(url)
            .send()
            .await
            .map_err(|e| format!("请求 ChinaMoney 参数失败: {}", e))?;
        let json_val: serde_json::Value = res
            .json()
            .await
            .map_err(|e| format!("解析 JSON 失败: {}", e))?;

        let key_field = match symbol.as_str() {
            "债券类型" => "bondType",
            "息票类型" => "couponType",
            "发行年份" => "issueYear",
            "评级等级" | _ => "bondRtngShrt",
        };

        let mut result = Vec::new();
        if let Some(arr) = json_val["data"][key_field].as_array() {
            for row in arr {
                let code = row["code"]
                    .as_str()
                    .map(|s| s.to_string())
                    .or_else(|| row["code"].as_i64().map(|i| i.to_string()));
                let name = row["name"].as_str().map(|s| s.to_string());
                result.push(BondInfoCmQueryItem { code, name });
            }
        }
        Ok(result)
    }
}

/// 22. 中国货币网 (ChinaMoney) - 债券信息列表查询
pub async fn get_bond_info_cm(query: BondQuery) -> Result<Vec<BondInfoCmItem>, String> {
    let url = "https://www.chinamoney.com.cn/ags/ms/cm-u-bond-md/BondBaseInfoList";
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let bond_code = query.symbol.unwrap_or_default();

    let res = client
        .post(url)
        .form(&[
            ("pageNo", "1"),
            ("pageSize", "100"),
            ("bondCode", bond_code.as_str()),
        ])
        .send()
        .await
        .map_err(|e| format!("请求 ChinaMoney 债券信息列表失败: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("ChinaMoney 接口状态码: {}", res.status()));
    }

    let json_val: serde_json::Value = res
        .json()
        .await
        .map_err(|e| format!("解析 JSON 失败: {}", e))?;
    let data_arr = json_val["data"]["resultList"]
        .as_array()
        .or_else(|| json_val["records"].as_array())
        .ok_or_else(|| "缺失 resultList 字段".to_string())?;

    let mut result = Vec::new();
    for row in data_arr {
        let mut data = HashMap::new();
        if let Some(obj) = row.as_object() {
            for (k, v) in obj {
                data.insert(k.clone(), v.clone());
            }
        }
        result.push(BondInfoCmItem { data });
    }

    Ok(result)
}

/// 23. 中国银行间市场交易商协会 (NAFMII) - 非金融企业债务融资工具注册信息
pub async fn get_bond_debt_nafmii(query: BondQuery) -> Result<Vec<BondDebtNafmiiItem>, String> {
    let url = "http://zhuce.nafmii.org.cn/fans/publicQuery/releFileProjDataGrid";
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let page_str = query.symbol.unwrap_or_else(|| "1".to_string());

    let res = client
        .post(url)
        .form(&[
            ("regFileName", ""),
            ("itemType", ""),
            ("startTime", ""),
            ("endTime", ""),
            ("entityName", ""),
            ("leadManager", ""),
            ("regPrdtType", ""),
            ("page", page_str.as_str()),
            ("rows", "50"),
        ])
        .send()
        .await
        .map_err(|e| format!("请求 NAFMII 注册信息失败: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("NAFMII 接口状态码: {}", res.status()));
    }

    let json_val: serde_json::Value = res
        .json()
        .await
        .map_err(|e| format!("解析 NAFMII JSON 失败: {}", e))?;
    let rows_arr = json_val["rows"]
        .as_array()
        .ok_or_else(|| "缺失 rows 列表".to_string())?;

    let mut result = Vec::new();
    for row in rows_arr {
        let mut data = HashMap::new();
        if let Some(obj) = row.as_object() {
            for (k, v) in obj {
                data.insert(k.clone(), v.clone());
            }
        }
        result.push(BondDebtNafmiiItem { data });
    }

    Ok(result)
}

/// 24. 中国外汇交易中心 (ChinaMoney) - 单只债券详情
pub async fn get_bond_info_detail_cm(query: BondQuery) -> Result<BondInfoDetailCmItem, String> {
    let bond_code = query.symbol.unwrap_or_else(|| "egfjh08154".to_string());
    let url = "https://www.chinamoney.com.cn/ags/ms/cm-u-bond-md/BondDetailInfo";

    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .post(url)
        .header("origin", "https://www.chinamoney.com.cn")
        .header(
            "referer",
            format!(
                "https://www.chinamoney.com.cn/chinese/zqjc/?bondDefinedCode={}",
                bond_code
            ),
        )
        .form(&[("bondDefinedCode", bond_code.as_str())])
        .send()
        .await
        .map_err(|e| format!("请求 ChinaMoney 债券详情失败: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("ChinaMoney 详情接口状态码: {}", res.status()));
    }

    let json_val: serde_json::Value = res
        .json()
        .await
        .map_err(|e| format!("解析 JSON 失败: {}", e))?;
    let mut data = HashMap::new();

    if let Some(obj) = json_val["data"]["bondBaseInfo"].as_object() {
        for (k, v) in obj {
            if k != "creditRateEntyList" && k != "exerciseInfoList" {
                data.insert(k.clone(), v.clone());
            }
        }
    }

    Ok(BondInfoDetailCmItem { data })
}

/// 25. 巨潮资讯 - 债券发行数据 (国债/地方债/企业债/可转债)
pub async fn get_bond_issue_cninfo(query: BondQuery) -> Result<Vec<BondIssueCninfoItem>, String> {
    let start_date = query.start_date.unwrap_or_else(|| "20210910".to_string());
    let end_date = query.end_date.unwrap_or_else(|| "20211109".to_string());

    let start_fmt = if start_date.len() >= 8 {
        format!(
            "{}-{}-{}",
            &start_date[0..4],
            &start_date[4..6],
            &start_date[6..8]
        )
    } else {
        "2021-09-10".to_string()
    };
    let end_fmt = if end_date.len() >= 8 {
        format!(
            "{}-{}-{}",
            &end_date[0..4],
            &end_date[4..6],
            &end_date[6..8]
        )
    } else {
        "2021-11-09".to_string()
    };

    let url = "http://webapi.cninfo.com.cn/api/sysapi/p_sysapi1120";
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .post(url)
        .header("Referer", "http://webapi.cninfo.com.cn/")
        .query(&[("sdate", start_fmt.as_str()), ("edate", end_fmt.as_str())])
        .send()
        .await
        .map_err(|e| format!("请求巨潮资讯债券发行接口失败: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("巨潮资讯接口状态码: {}", res.status()));
    }

    let json_val: serde_json::Value = res
        .json()
        .await
        .map_err(|e| format!("解析 JSON 失败: {}", e))?;
    let records = json_val["records"]
        .as_array()
        .ok_or_else(|| "缺失 records 字段".to_string())?;

    let mut result = Vec::new();
    for row in records {
        let mut data = HashMap::new();
        if let Some(obj) = row.as_object() {
            for (k, v) in obj {
                data.insert(k.clone(), v.clone());
            }
        }
        result.push(BondIssueCninfoItem { data });
    }

    Ok(result)
}

/// 26. 东方财富网-可转债价值分析 (溢价率分析)
pub async fn get_bond_zh_cov_value_analysis(
    query: BondQuery,
) -> Result<Vec<BondZhCovValueAnalysisItem>, String> {
    let symbol = query.symbol.unwrap_or_else(|| "113527".to_string());
    let url = "https://datacenter-web.eastmoney.com/api/data/get";

    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let filter_param = format!("(zcode=\"{}\")", symbol);

    let res = client
        .get(url)
        .query(&[
            ("sty", "ALL"),
            ("token", "894050c76af8597a853f5b408b759f5d"),
            ("st", "date"),
            ("sr", "1"),
            ("source", "WEB"),
            ("type", "RPTA_WEB_KZZ_LS"),
            ("filter", filter_param.as_str()),
            ("p", "1"),
            ("ps", "8000"),
        ])
        .send()
        .await
        .map_err(|e| format!("请求可转债价值分析失败: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("东方财富可转债分析接口状态码: {}", res.status()));
    }

    let json_val: serde_json::Value = res
        .json()
        .await
        .map_err(|e| format!("解析 JSON 失败: {}", e))?;
    let data_arr = json_val["result"]["data"]
        .as_array()
        .ok_or_else(|| "缺失 result.data 列表".to_string())?;

    let mut result = Vec::new();
    for row in data_arr {
        let mut data = HashMap::new();
        if let Some(obj) = row.as_object() {
            for (k, v) in obj {
                data.insert(k.clone(), v.clone());
            }
        }
        result.push(BondZhCovValueAnalysisItem { data });
    }

    Ok(result)
}

/// 27. 中国债券信息网-中债指数-中债国债指数
pub async fn get_bond_treasury_index_cbond(
    query: BondQuery,
) -> Result<Vec<BondCbondIndexItem>, String> {
    let period = query.symbol.unwrap_or_else(|| "5Y".to_string());
    let index_id = match period.as_str() {
        "0-1Y" => "8a8b2cef70bc61380170be069828032b",
        "0-3Y" => "61f69682dc3ec18fe9664ff59308314a",
        "0-5Y" => "0beafb51867009998c2f4932bf22ede3",
        "0-10Y" => "8a8b2cef7832f8920178350801470014",
        "1-3Y" => "cc1cfe89b0cbd0800420a0e037026407",
        "1-5Y" => "7c3110e5305f9301482517066427a554",
        "1-10Y" => "a5d90802e3259978a027267de651106d",
        "3-5Y" => "8a8b2ca04bf69582014c10b60f376c77",
        "5Y" => "8a8b2ca03a3feea1013a44b98fc533f5",
        "7Y" => "2c9081e50e8767dc010e87b6e26c0080",
        "7-10Y" => "8a8b2c8f5a492a01015a4ac986480043",
        "10Y" => "8a8b2ca04b666362014b723482bc4f49",
        "30Y" => "8a8b2cef77b239980177b485d20a6379",
        _ => "8a8b2ca03a3feea1013a44b98fc533f5",
    };

    let url = "https://yield.chinabond.com.cn/cbweb-mn/indices/singleIndexQueryResult";
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .post(url)
        .query(&[
            ("indexid", index_id),
            ("qxlxt", "0"),
            ("ltcslx", ""),
            ("zslxt", "2"),
            ("zslxt1", "2"),
            ("lx", "1"),
            ("locale", "zh_CN"),
        ])
        .send()
        .await
        .map_err(|e| format!("请求中债国债指数失败: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("中债接口响应状态码: {}", res.status()));
    }

    let json_val: serde_json::Value = res
        .json()
        .await
        .map_err(|e| format!("解析 JSON 失败: {}", e))?;
    let mut result = Vec::new();

    if let Some(obj) = json_val.as_object() {
        for (k, v) in obj {
            if k.starts_with("2_") {
                if let Some(timestamp_ms) = k[2..].parse::<i64>().ok() {
                    let val = v
                        .as_f64()
                        .or_else(|| v.as_str().and_then(|s| s.parse().ok()));
                    let date_str = chrono::DateTime::from_timestamp(timestamp_ms / 1000, 0)
                        .map(|dt| dt.format("%Y-%m-%d").to_string())
                        .unwrap_or_else(|| timestamp_ms.to_string());

                    result.push(BondCbondIndexItem {
                        date: Some(date_str),
                        value: val,
                    });
                }
            }
        }
    }

    result.sort_by(|a, b| a.date.cmp(&b.date));
    Ok(result)
}

/// 32. 中国债券信息网-中债指数-通用中债指数查询
pub async fn get_bond_index_general_cbond(
    query: BondCbondQuery,
) -> Result<Vec<BondCbondIndexItem>, String> {
    let index_category = query
        .index_category
        .unwrap_or_else(|| "新综合指数".to_string());
    let indicator = query.indicator.unwrap_or_else(|| "全价".to_string());
    let period = query.period.unwrap_or_else(|| "总值".to_string());

    let index_id = match index_category.as_str() {
        "新综合指数" => "8a8b2c253818e31a013824efc8c102be",
        "中债-国债指数" => "8a8b2ca03a3feea1013a44b98fc533f5",
        "金融债指数" => "8a8b2ca03a3feea1013a44baea5d33ff",
        "企业债指数" => "8a8b2c253818e31a013824ca0cc401eb",
        "央行票据指数" => "8a8b2ca03a3feea1013a44b0f0a533da",
        "短融指数" => "8a8b2c253818e31a013824beba370183",
        "中期票据指数" => "8a8b2ca03a3feea1013a44b61ec033eb",
        "高信用等级债券指数" => "8a8b2c253818e31a013824da5be80213",
        _ => "8a8b2c253818e31a013824efc8c102be",
    };

    let qxlxt = match period.as_str() {
        "总值" => "0",
        "1年以下" => "1",
        "1-3年" => "2",
        "3-5年" => "3",
        "5-7年" => "4",
        "7-10年" => "5",
        "10年以上" => "6",
        _ => "0",
    };

    let zslxt = match indicator.as_str() {
        "全价" => "1",
        "净价" => "0",
        "财富" => "2",
        _ => "1",
    };

    let url = "https://yield.chinabond.com.cn/cbweb-mn/indices/singleIndexQueryResult";
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .post(url)
        .query(&[
            ("indexid", index_id),
            ("qxlxt", qxlxt),
            ("ltcslx", ""),
            ("zslxt", zslxt),
            ("zslxt1", zslxt),
            ("lx", "1"),
            ("locale", "zh_CN"),
        ])
        .send()
        .await
        .map_err(|e| format!("请求中债指数失败: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("中债接口响应状态码: {}", res.status()));
    }

    let json_val: serde_json::Value = res
        .json()
        .await
        .map_err(|e| format!("解析 JSON 失败: {}", e))?;
    let mut result = Vec::new();
    let prefix = format!("{}_", zslxt);

    if let Some(obj) = json_val.as_object() {
        for (k, v) in obj {
            if k.starts_with(&prefix) {
                if let Some(timestamp_ms) = k[prefix.len()..].parse::<i64>().ok() {
                    let val = v
                        .as_f64()
                        .or_else(|| v.as_str().and_then(|s| s.parse().ok()));
                    let date_str = chrono::DateTime::from_timestamp(timestamp_ms / 1000, 0)
                        .map(|dt| dt.format("%Y-%m-%d").to_string())
                        .unwrap_or_else(|| timestamp_ms.to_string());

                    result.push(BondCbondIndexItem {
                        date: Some(date_str),
                        value: val,
                    });
                }
            }
        }
    }

    result.sort_by(|a, b| a.date.cmp(&b.date));
    Ok(result)
}

/// 28. 新浪财经-债券-沪深可转债历史日 K 线数据
pub async fn get_bond_zh_hs_cov_daily(query: BondQuery) -> Result<Vec<BondGbKlineItem>, String> {
    let symbol = query.symbol.unwrap_or_else(|| "sh010107".to_string());
    let url = format!("https://vip.stock.finance.sina.com.cn/quotes_service/api/json_srv.php/Market_Center.getKLineData?symbol={}&scale=240&ma=no&datalen=1024", symbol);

    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("请求新浪可转债 K 线失败: {}", e))?;
    if !res.status().is_success() {
        return Err(format!("新浪接口响应状态码: {}", res.status()));
    }

    let text = res
        .text()
        .await
        .map_err(|e| format!("读取响应文本失败: {}", e))?;
    let list: Vec<HashMap<String, serde_json::Value>> =
        serde_json::from_str(&text).map_err(|e| format!("解析可转债日 K 线 JSON 失败: {}", e))?;

    let mut result = Vec::new();
    for item in list {
        let date = item
            .get("day")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let open = item
            .get("open")
            .and_then(|v| v.as_str())
            .and_then(|s| s.parse::<f64>().ok());
        let high = item
            .get("high")
            .and_then(|v| v.as_str())
            .and_then(|s| s.parse::<f64>().ok());
        let low = item
            .get("low")
            .and_then(|v| v.as_str())
            .and_then(|s| s.parse::<f64>().ok());
        let close = item
            .get("close")
            .and_then(|v| v.as_str())
            .and_then(|s| s.parse::<f64>().ok());
        let volume = item
            .get("volume")
            .and_then(|v| v.as_str())
            .and_then(|s| s.parse::<f64>().ok());

        result.push(BondGbKlineItem {
            date,
            open,
            high,
            low,
            close,
            volume,
        });
    }

    Ok(result)
}

/// 29. 新浪财经-债券-沪深现券/债券历史日 K 线数据
pub async fn get_bond_zh_hs_daily(query: BondQuery) -> Result<Vec<BondGbKlineItem>, String> {
    let symbol = query.symbol.unwrap_or_else(|| "sh010107".to_string());
    let url = format!("https://vip.stock.finance.sina.com.cn/quotes_service/api/json_srv.php/Market_Center.getKLineData?symbol={}&scale=240&ma=no&datalen=1024", symbol);

    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("请求新浪现券 K 线失败: {}", e))?;
    if !res.status().is_success() {
        return Err(format!("新浪接口响应状态码: {}", res.status()));
    }

    let text = res
        .text()
        .await
        .map_err(|e| format!("读取响应文本失败: {}", e))?;
    let list: Vec<HashMap<String, serde_json::Value>> =
        serde_json::from_str(&text).map_err(|e| format!("解析现券日 K 线 JSON 失败: {}", e))?;

    let mut result = Vec::new();
    for item in list {
        let date = item
            .get("day")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let open = item
            .get("open")
            .and_then(|v| v.as_str())
            .and_then(|s| s.parse::<f64>().ok());
        let high = item
            .get("high")
            .and_then(|v| v.as_str())
            .and_then(|s| s.parse::<f64>().ok());
        let low = item
            .get("low")
            .and_then(|v| v.as_str())
            .and_then(|s| s.parse::<f64>().ok());
        let close = item
            .get("close")
            .and_then(|v| v.as_str())
            .and_then(|s| s.parse::<f64>().ok());
        let volume = item
            .get("volume")
            .and_then(|v| v.as_str())
            .and_then(|s| s.parse::<f64>().ok());

        result.push(BondGbKlineItem {
            date,
            open,
            high,
            low,
            close,
            volume,
        });
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
