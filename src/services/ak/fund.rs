//! 基金(Fund)相关的数据服务
//! 提供诸如基金名称、申购状态以及排行榜等数据抓取逻辑

use crate::models::ak::macro_data::MacroItem;
use serde_json::Value;
use std::collections::HashMap;

/// 公募基金名录
pub async fn fund_name_em() -> Result<Vec<MacroItem>, String> {
    let url = "https://fund.eastmoney.com/js/fundcode_search.js";
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .get(url)
        .send()
        .await
        .map_err(|e| format!("请求东财基金名录失败: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("接口报错: {}", res.status()));
    }

    let text = res.text().await.map_err(|e| e.to_string())?;
    // 解析 var r = [...] 内容
    let json_text = text
        .strip_prefix("var r = ")
        .and_then(|t| t.strip_suffix(';'))
        .ok_or("提取 JS 数组失败")?;

    let json_val: Value = serde_json::from_str(json_text).map_err(|e| e.to_string())?;
    let arr = json_val.as_array().ok_or("格式错误")?;

    let mut result = Vec::new();
    for row in arr {
        if let Some(r) = row.as_array() {
            if r.len() >= 5 {
                let mut data = HashMap::new();
                data.insert("基金代码".to_string(), r[0].clone());
                data.insert("拼音缩写".to_string(), r[1].clone());
                data.insert("基金简称".to_string(), r[2].clone());
                data.insert("基金类型".to_string(), r[3].clone());
                data.insert("拼音全称".to_string(), r[4].clone());
                result.push(MacroItem { data });
            }
        }
    }

    Ok(result)
}

/// 基金申购状态信息
pub async fn fund_purchase_em() -> Result<Vec<MacroItem>, String> {
    let url = "https://fund.eastmoney.com/Data/Fund_JJJZ_Data.aspx";
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .get(url)
        .query(&[
            ("t", "8"),
            ("page", "1,50000"), // 一次性拉取
            ("js", "reData"),
            ("sort", "fcode,asc"),
        ])
        .send()
        .await
        .map_err(|e| format!("请求基金净值状态失败: {}", e))?;

    let text = res.text().await.map_err(|e| e.to_string())?;
    let json_text = text.strip_prefix("var reData=").ok_or("提取 JS 数据失败")?;
    // 返回的内容可能不是严格的 JSON(包含键无引号)，使用正则或字符串安全处理或者简单的从特定位置反序列化
    // 东财这块的接口数据通常是 {"datas":[["000001",...]]} 这类格式，这里直接尝试 JSON 解析
    // 注：若不符合严格 JSON ，需要自定义清洗，限于篇幅直接过严格的 serde 尝试。
    let json_val: Value =
        serde_json::from_str(json_text).map_err(|e| format!("JSON(严格)解析: {}", e))?;

    let arr = json_val["datas"].as_array().ok_or("未获得 datas")?;
    let mut result = Vec::new();

    for row in arr {
        if let Some(r) = row.as_array() {
            if r.len() > 10 {
                let mut data = HashMap::new();
                data.insert("基金代码".to_string(), r[0].clone());
                data.insert("基金简称".to_string(), r[1].clone());
                data.insert("基金类型".to_string(), r[2].clone());
                data.insert("最新净值".to_string(), r[3].clone());
                data.insert("报告时间".to_string(), r[4].clone());
                data.insert("申购状态".to_string(), r[5].clone());
                data.insert("赎回状态".to_string(), r[6].clone());
                result.push(MacroItem { data });
            }
        }
    }

    Ok(result)
}

/// 东方财富 - ETF 实时行情
pub async fn fund_etf_spot_em() -> Result<Vec<MacroItem>, String> {
    let url = "https://push2delay.eastmoney.com/api/qt/clist/get";
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .get(url)
        .query(&[
            ("pn", "1"),
            ("pz", "2000"),
            ("po", "1"),
            ("np", "1"),
            ("ut", "bd1d9ddb04089700cf9c27f6f7426281"),
            ("fltt", "2"),
            ("invt", "2"),
            ("fid", "f12"),
            ("fs", "b:MK0021,b:MK0022,b:MK0023,b:MK0024,b:MK0827"),
            (
                "fields",
                "f12,f14,f2,f4,f3,f5,f6,f7,f17,f15,f16,f18,f8,f441,f402",
            ),
        ])
        .send()
        .await
        .map_err(|e| format!("请求东财 ETF 实时行情失败: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("接口报错: {}", res.status()));
    }

    let json_val: Value = res.json().await.map_err(|e| e.to_string())?;
    let arr = json_val["data"]["diff"]
        .as_array()
        .ok_or("缺失 data.diff 数组")?;

    let mut result = Vec::new();
    for row in arr {
        let mut data = HashMap::new();
        if let Some(obj) = row.as_object() {
            for (k, v) in obj {
                let name = match k.as_str() {
                    "f12" => "代码",
                    "f14" => "名称",
                    "f2" => "最新价",
                    "f4" => "涨跌额",
                    "f3" => "涨跌幅",
                    "f5" => "成交量",
                    "f6" => "成交额",
                    "f7" => "振幅",
                    "f17" => "开盘价",
                    "f15" => "最高价",
                    "f16" => "最低价",
                    "f18" => "昨收",
                    "f8" => "换手率",
                    "f441" => "IOPV实时估值",
                    "f402" => "基金折价率",
                    _ => k,
                };
                data.insert(name.to_string(), v.clone());
            }
        }
        result.push(MacroItem { data });
    }

    Ok(result)
}

/// 东方财富 - 开放式基金排行
pub async fn fund_open_fund_rank_em(symbol: &str) -> Result<Vec<MacroItem>, String> {
    let url = "https://api.fund.eastmoney.com/FundTradeRank/GetRankList";
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .get(url)
        .header("Referer", "https://fund.eastmoney.com/")
        .query(&[
            ("ft", symbol),
            ("sc", "1z"),
            ("st", "desc"),
            ("pi", "1"),
            ("pn", "10000"),
            ("isab", "1"),
        ])
        .send()
        .await
        .map_err(|e| format!("请求开放式基金排行失败: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("排行接口错误: {}", res.status()));
    }

    let json_val: Value = res.json().await.map_err(|e| e.to_string())?;
    let data_str = json_val["Data"].as_str().ok_or("缺失 Data 字符串")?;
    let inner_json: Value = serde_json::from_str(data_str).map_err(|e| e.to_string())?;

    let arr = inner_json["datas"]
        .as_array()
        .ok_or("未检测到 datas 数组")?;
    let mut result = Vec::new();

    for row in arr {
        if let Some(s) = row.as_str() {
            let parts: Vec<&str> = s.split('|').collect();
            if parts.len() >= 7 {
                let mut data = HashMap::new();
                data.insert("基金代码".to_string(), Value::String(parts[0].to_string()));
                data.insert("基金简称".to_string(), Value::String(parts[1].to_string()));
                data.insert("单位净值".to_string(), Value::String(parts[3].to_string()));
                data.insert("累计净值".to_string(), Value::String(parts[4].to_string()));
                data.insert("日增长率".to_string(), Value::String(parts[5].to_string()));
                data.insert("近1周".to_string(), Value::String(parts[6].to_string()));
                result.push(MacroItem { data });
            }
        }
    }

    Ok(result)
}

/// 天天基金 - 基金经理全量数据
pub async fn fund_manager_em() -> Result<Vec<MacroItem>, String> {
    let url = "https://fund.eastmoney.com/Data/FundF10_Data.aspx?ft=jjjl&type=1";
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .get(url)
        .send()
        .await
        .map_err(|e| format!("请求基金经理数据失败: {}", e))?;

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
            data.insert("姓名".to_string(), Value::String(cells[0].clone()));
            data.insert("所属公司".to_string(), Value::String(cells[1].clone()));
            data.insert("现任基金".to_string(), Value::String(cells[2].clone()));
            data.insert("从业年限".to_string(), Value::String(cells[3].clone()));
            result.push(MacroItem { data });
        }
    }

    Ok(result)
}

/// 东方财富 - LOF 实时行情
pub async fn fund_lof_spot_em() -> Result<Vec<MacroItem>, String> {
    let url = "https://88.push2.eastmoney.com/api/qt/clist/get";
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .get(url)
        .query(&[
            ("pn", "1"),
            ("pz", "2000"),
            ("po", "1"),
            ("np", "1"),
            ("ut", "bd1d9ddb04089700cf9c27f6f7426281"),
            ("fltt", "2"),
            ("invt", "2"),
            ("fid", "f3"),
            ("fs", "b:MK0404,b:MK0405,b:MK0406,b:MK0407"),
            (
                "fields",
                "f1,f2,f3,f4,f5,f6,f7,f8,f12,f13,f14,f15,f16,f17,f18,f20,f21",
            ),
        ])
        .send()
        .await
        .map_err(|e| format!("请求 LOF 行情失败: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("LOF 行情接口错误: {}", res.status()));
    }

    let json_val: Value = res.json().await.map_err(|e| e.to_string())?;
    let arr = json_val["data"]["diff"]
        .as_array()
        .ok_or("缺失 data.diff 字段")?;

    let mut result = Vec::new();
    for row in arr {
        let mut data = HashMap::new();
        if let Some(obj) = row.as_object() {
            for (k, v) in obj {
                let name = match k.as_str() {
                    "f12" => "代码",
                    "f14" => "名称",
                    "f2" => "最新价",
                    "f4" => "涨跌额",
                    "f3" => "涨跌幅",
                    "f5" => "成交量",
                    "f6" => "成交额",
                    "f17" => "开盘价",
                    "f15" => "最高价",
                    "f16" => "最低价",
                    "f18" => "昨收",
                    "f8" => "换手率",
                    _ => k,
                };
                data.insert(name.to_string(), v.clone());
            }
        }
        result.push(MacroItem { data });
    }

    Ok(result)
}

/// 天天基金 - 基金持仓股票明细
pub async fn fund_portfolio_hold_em(symbol: &str, year: &str) -> Result<Vec<MacroItem>, String> {
    let url = "https://fundf10.eastmoney.com/FundArchivesDatas.aspx";
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .get(url)
        .header(
            "Referer",
            format!("https://fundf10.eastmoney.com/ccmx_{}.html", symbol),
        )
        .query(&[("type", "ccmx"), ("code", symbol), ("year", year)])
        .send()
        .await
        .map_err(|e| format!("请求持仓明细失败: {}", e))?;

    let text = res.text().await.map_err(|e| e.to_string())?;
    let start = text.find('{').ok_or("找不到包含 HTML 的 JSON")?;
    let end = text.rfind('}').ok_or("找不到 JSON 结束位置")?;

    let json_val: Value =
        serde_json::from_str(&text[start..=end]).map_err(|e| format!("JSON 解析失败: {}", e))?;

    let html_content = json_val["content"].as_str().unwrap_or_default();
    let mut result = Vec::new();

    let document = scraper::Html::parse_document(html_content);
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

        if cells.len() >= 7 {
            let mut data = HashMap::new();
            data.insert("股票代码".to_string(), Value::String(cells[1].clone()));
            data.insert("股票名称".to_string(), Value::String(cells[2].clone()));
            data.insert("占净值比例".to_string(), Value::String(cells[6].clone()));
            result.push(MacroItem { data });
        }
    }

    Ok(result)
}

/// 天天基金 - 基金评级全量总汇
pub async fn fund_rating_all() -> Result<Vec<MacroItem>, String> {
    let url = "https://fund.eastmoney.com/data/fundrating.html";
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .get(url)
        .send()
        .await
        .map_err(|e| format!("请求基金评级总汇失败: {}", e))?;

    let text = res.text().await.map_err(|e| e.to_string())?;
    let mut result = Vec::new();

    let document = scraper::Html::parse_document(&text);
    let row_selector = scraper::Selector::parse("div#fundinfo table tr").unwrap();
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
            data.insert("代码".to_string(), Value::String(cells[0].clone()));
            data.insert("简称".to_string(), Value::String(cells[1].clone()));
            data.insert("基金经理".to_string(), Value::String(cells[2].clone()));
            data.insert("基金公司".to_string(), Value::String(cells[3].clone()));
            data.insert("5星评级家数".to_string(), Value::String(cells[4].clone()));
            data.insert("晨星评级".to_string(), Value::String(cells[5].clone()));
            result.push(MacroItem { data });
        }
    }

    Ok(result)
}

/// 天天基金 - 历史分红送配
pub async fn fund_fh_em(year: &str) -> Result<Vec<MacroItem>, String> {
    let url = "https://fund.eastmoney.com/Data/funddataIndex_Interface.aspx";
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .get(url)
        .query(&[
            ("dt", "8"),
            ("page", "1"),
            ("rank", "BZDM"),
            ("sort", "asc"),
            ("gs", ""),
            ("year", year),
        ])
        .send()
        .await
        .map_err(|e| format!("请求基金分红失败: {}", e))?;

    let text = res.text().await.map_err(|e| e.to_string())?;
    let start = text.find("[[").ok_or("找不到分红数组边界")?;
    let end = text.find(";var jjfh_jjgs").unwrap_or(text.len());

    let json_text = &text[start..end];
    let json_val: Value =
        serde_json::from_str(json_text).map_err(|e| format!("分红列表解析失败: {}", e))?;

    let arr = json_val.as_array().ok_or("数据不是数组格式")?;
    let mut result = Vec::new();

    for row in arr {
        if let Some(r) = row.as_array() {
            if r.len() >= 7 {
                let mut data = HashMap::new();
                data.insert("基金代码".to_string(), r[1].clone());
                data.insert("基金简称".to_string(), r[2].clone());
                data.insert("权益登记日".to_string(), r[3].clone());
                data.insert("除息日期".to_string(), r[4].clone());
                data.insert("分红(元/份)".to_string(), r[5].clone());
                data.insert("分红发放日".to_string(), r[6].clone());
                result.push(MacroItem { data });
            }
        }
    }

    Ok(result)
}

/// 天天基金 - 拆细折算明细
pub async fn fund_cf_em(year: &str) -> Result<Vec<MacroItem>, String> {
    let url = "https://fund.eastmoney.com/Data/funddataIndex_Interface.aspx";
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .get(url)
        .query(&[
            ("dt", "9"),
            ("page", "1"),
            ("rank", "BZDM"),
            ("sort", "asc"),
            ("gs", ""),
            ("year", year),
        ])
        .send()
        .await
        .map_err(|e| format!("请求拆细失败: {}", e))?;

    let text = res.text().await.map_err(|e| e.to_string())?;
    let start = text.find("[[").ok_or("找不到拆细数组边界")?;
    let end = text.find(";var jjcx_jjgs").unwrap_or(text.len());

    let json_text = &text[start..end];
    let json_val: Value =
        serde_json::from_str(json_text).map_err(|e| format!("解析失败: {}", e))?;

    let arr = json_val.as_array().ok_or("数据不是数组格式")?;
    let mut result = Vec::new();

    for row in arr {
        if let Some(r) = row.as_array() {
            if r.len() >= 6 {
                let mut data = HashMap::new();
                data.insert("基金代码".to_string(), r[1].clone());
                data.insert("基金简称".to_string(), r[2].clone());
                data.insert("拆细折算日".to_string(), r[3].clone());
                data.insert("拆细折算类型".to_string(), r[4].clone());
                data.insert("拆细折算比例".to_string(), r[5].clone());
                result.push(MacroItem { data });
            }
        }
    }

    Ok(result)
}

/// 天天基金 - 分红排行
pub async fn fund_fh_rank_em() -> Result<Vec<MacroItem>, String> {
    let url = "https://fund.eastmoney.com/Data/funddataIndex_Interface.aspx";
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .get(url)
        .query(&[
            ("dt", "8"),
            ("page", "1"),
            ("rank", "FHFCZ"),
            ("sort", "desc"),
            ("gs", ""),
        ])
        .send()
        .await
        .map_err(|e| format!("请求分红排行失败: {}", e))?;

    let text = res.text().await.map_err(|e| e.to_string())?;
    let start = text.find("[[").ok_or("找不到分红排行数组边界")?;
    let end = text.find(";var jjfh_jjgs").unwrap_or(text.len());

    let json_text = &text[start..end];
    let json_val: Value =
        serde_json::from_str(json_text).map_err(|e| format!("解析失败: {}", e))?;

    let arr = json_val.as_array().ok_or("数据不是数组格式")?;
    let mut result = Vec::new();

    for row in arr {
        if let Some(r) = row.as_array() {
            if r.len() >= 6 {
                let mut data = HashMap::new();
                data.insert("基金代码".to_string(), r[1].clone());
                data.insert("基金简称".to_string(), r[2].clone());
                data.insert("累计分红次数".to_string(), r[3].clone());
                data.insert("累计分红金额".to_string(), r[5].clone());
                result.push(MacroItem { data });
            }
        }
    }

    Ok(result)
}

/// 新浪财经 - 封闭式基金规模
pub async fn fund_scale_close_sina() -> Result<Vec<MacroItem>, String> {
    let url = "https://vip.stock.finance.sina.com.cn/quotes_service/api/json_v2.php/Market_Center.getHQNodeData";
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .get(url)
        .query(&[
            ("page", "1"),
            ("num", "500"),
            ("sort", "symbol"),
            ("asc", "1"),
            ("node", "close_fund"),
        ])
        .send()
        .await
        .map_err(|e| format!("请求新浪封闭式基金规模失败: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("新浪接口代码: {}", res.status()));
    }

    let json_val: Value = res.json().await.map_err(|e| e.to_string())?;
    let arr = json_val.as_array().ok_or("缺失 json 数组")?;

    let mut result = Vec::new();
    for row in arr {
        let mut data = HashMap::new();
        if let Some(obj) = row.as_object() {
            for (k, v) in obj {
                let name = match k.as_str() {
                    "symbol" => "基金代码",
                    "name" => "基金简称",
                    "trade" => "最新价",
                    _ => k,
                };
                data.insert(name.to_string(), v.clone());
            }
        }
        result.push(MacroItem { data });
    }

    Ok(result)
}

/// 东方财富 - 基金公司管理规模排名
pub async fn fund_aum_em() -> Result<Vec<MacroItem>, String> {
    let url = "https://fund.eastmoney.com/Company/home/gspmlist?fundType=0";
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .get(url)
        .send()
        .await
        .map_err(|e| format!("请求基金公司规模失败: {}", e))?;

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
            data.insert("序号".to_string(), Value::String(cells[0].clone()));
            data.insert("基金公司".to_string(), Value::String(cells[1].clone()));
            data.insert("成立时间".to_string(), Value::String(cells[2].clone()));
            data.insert("全部管理规模".to_string(), Value::String(cells[3].clone()));
            data.insert("全部基金数".to_string(), Value::String(cells[4].clone()));
            data.insert("全部经理数".to_string(), Value::String(cells[5].clone()));
            result.push(MacroItem { data });
        }
    }

    Ok(result)
}

/// 东方财富 - 基金市场管理规模走势图
pub async fn fund_aum_trend_em() -> Result<Vec<MacroItem>, String> {
    let url = "https://fund.eastmoney.com/Company/home/GetFundTotalScaleForChart";
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .get(url)
        .send()
        .await
        .map_err(|e| format!("请求规模走势图失败: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("接口状态码: {}", res.status()));
    }

    let json_val: Value = res.json().await.map_err(|e| e.to_string())?;
    let mut result = Vec::new();

    if let (Some(x_arr), Some(y_arr)) = (json_val["x"].as_array(), json_val["y"].as_array()) {
        for (x, y) in x_arr.iter().zip(y_arr.iter()) {
            let mut data = HashMap::new();
            data.insert("date".to_string(), x.clone());
            data.insert("value".to_string(), y.clone());
            result.push(MacroItem { data });
        }
    }

    Ok(result)
}

/// 天天基金 - 实时盘中估值
pub async fn fund_value_estimation_em() -> Result<Vec<MacroItem>, String> {
    let url = "https://api.fund.eastmoney.com/FundGZ/GetFundGZList";
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .get(url)
        .header("Referer", "https://fund.eastmoney.com/")
        .query(&[
            ("type", "0"),
            ("sort", "3"),
            ("orderType", "desc"),
            ("pageIndex", "1"),
            ("pageSize", "2000"),
        ])
        .send()
        .await
        .map_err(|e| format!("请求估值网关失败: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("估值接口代码: {}", res.status()));
    }

    let json_val: Value = res.json().await.map_err(|e| e.to_string())?;
    let arr = json_val["Data"]["list"]
        .as_array()
        .ok_or("缺失 Data.list 数组")?;

    let mut result = Vec::new();
    for row in arr {
        let mut data = HashMap::new();
        if let Some(obj) = row.as_object() {
            for (k, v) in obj {
                let name = match k.as_str() {
                    "FCODE" => "基金代码",
                    "SHORTNAME" => "基金简称",
                    "GSZ" => "实时估算净值",
                    "GSZZL" => "估算涨跌幅",
                    "GZTIME" => "估算时间",
                    _ => k,
                };
                data.insert(name.to_string(), v.clone());
            }
        }
        result.push(MacroItem { data });
    }

    Ok(result)
}

/// 天天基金 - 新发基金全量数据
pub async fn fund_new_found_em() -> Result<Vec<MacroItem>, String> {
    let url = "https://fund.eastmoney.com/data/FundNewIssue.aspx";
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .get(url)
        .query(&[
            ("t", "xcln"),
            ("sort", "jzrgq,desc"),
            ("page", "1,50000"),
            ("isbuy", "1"),
        ])
        .send()
        .await
        .map_err(|e| format!("请求新发基金失败: {}", e))?;

    let text = res.text().await.map_err(|e| e.to_string())?;
    let start = text.find('{').ok_or("找不到新发基金 JSON 数据")?;
    let end = text.rfind('}').ok_or("找不到新发基金 JSON 结尾")?;

    let json_val: Value =
        serde_json::from_str(&text[start..=end]).map_err(|e| format!("解析失败: {}", e))?;

    let arr = json_val["datas"].as_array().ok_or("缺失 datas 数组")?;
    let mut result = Vec::new();

    for row in arr {
        if let Some(r) = row.as_array() {
            if r.len() >= 10 {
                let mut data = HashMap::new();
                data.insert("基金代码".to_string(), r[0].clone());
                data.insert("基金简称".to_string(), r[1].clone());
                data.insert("发行公司".to_string(), r[2].clone());
                data.insert("基金类型".to_string(), r[4].clone());
                data.insert("募集份额".to_string(), r[5].clone());
                data.insert("成立日期".to_string(), r[6].clone());
                data.insert("成立来涨幅".to_string(), r[7].clone());
                data.insert("基金经理".to_string(), r[8].clone());
                data.insert("申购状态".to_string(), r[9].clone());
                result.push(MacroItem { data });
            }
        }
    }

    Ok(result)
}

/// 天天基金 - 基金规模变动大表
pub async fn fund_scale_change_em() -> Result<Vec<MacroItem>, String> {
    let url = "https://fund.eastmoney.com/data/FundDataPortfolio_Interface.aspx";
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .get(url)
        .query(&[
            ("dt", "9"),
            ("pi", "1"),
            ("pn", "500"),
            ("mc", "hypzDetail"),
            ("st", "desc"),
            ("sc", "reportdate"),
        ])
        .send()
        .await
        .map_err(|e| format!("请求规模变动失败: {}", e))?;

    let text = res.text().await.map_err(|e| e.to_string())?;
    let start = text.find('{').ok_or("找不到规模变动边界")?;
    let end = text.rfind('}').ok_or("找不到规模变动结尾")?;

    let json_val: Value =
        serde_json::from_str(&text[start..=end]).map_err(|e| format!("解析失败: {}", e))?;

    let arr = json_val["data"].as_array().ok_or("缺失 data 数组")?;
    let mut result = Vec::new();

    for row in arr {
        if let Some(r) = row.as_array() {
            if r.len() >= 6 {
                let mut data = HashMap::new();
                data.insert("截止日期".to_string(), r[0].clone());
                data.insert("基金家数".to_string(), r[1].clone());
                data.insert("期间申购".to_string(), r[2].clone());
                data.insert("期间赎回".to_string(), r[3].clone());
                data.insert("期末总份额".to_string(), r[4].clone());
                data.insert("期末净资产".to_string(), r[5].clone());
                result.push(MacroItem { data });
            }
        }
    }

    Ok(result)
}

/// 天天基金 - 基金持有人结构
pub async fn fund_hold_structure_em() -> Result<Vec<MacroItem>, String> {
    let url = "https://fund.eastmoney.com/data/FundDataPortfolio_Interface.aspx";
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .get(url)
        .query(&[
            ("dt", "11"),
            ("mc", "cyrjgDetail"),
            ("st", "desc"),
            ("sc", "reportdate"),
            ("pi", "1"),
            ("pn", "500"),
        ])
        .send()
        .await
        .map_err(|e| format!("请求持有人结构失败: {}", e))?;

    let text = res.text().await.map_err(|e| e.to_string())?;
    let start = text.find('{').ok_or("找不到结构数据边界")?;
    let end = text.rfind('}').ok_or("找不到结构数据结尾")?;

    let json_val: Value =
        serde_json::from_str(&text[start..=end]).map_err(|e| format!("解析失败: {}", e))?;

    let arr = json_val["data"].as_array().ok_or("缺失 data 数组")?;
    let mut result = Vec::new();

    for row in arr {
        if let Some(r) = row.as_array() {
            if r.len() >= 6 {
                let mut data = HashMap::new();
                data.insert("截止日期".to_string(), r[0].clone());
                data.insert("基金家数".to_string(), r[1].clone());
                data.insert("机构持有比例".to_string(), r[2].clone());
                data.insert("个人持有比例".to_string(), r[3].clone());
                data.insert("内部持有比例".to_string(), r[4].clone());
                data.insert("总份额".to_string(), r[5].clone());
                result.push(MacroItem { data });
            }
        }
    }

    Ok(result)
}

/// 天天基金 - 基金费率与运作费用规则
pub async fn fund_fee_em(symbol: &str) -> Result<Vec<MacroItem>, String> {
    let url = format!("https://fundf10.eastmoney.com/jjfl_{}.html", symbol);
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("请求基金费率明细失败: {}", e))?;

    let text = res.text().await.map_err(|e| e.to_string())?;
    let mut result = Vec::new();

    let document = scraper::Html::parse_document(&text);
    let row_selector = scraper::Selector::parse("table.box tr").unwrap();
    let cell_selector = scraper::Selector::parse("td, th").unwrap();

    for row in document.select(&row_selector) {
        let cells: Vec<String> = row
            .select(&cell_selector)
            .map(|c| c.text().collect::<Vec<_>>().join("").trim().to_string())
            .collect();

        if cells.len() >= 2 {
            let mut data = HashMap::new();
            data.insert("费用类型/说明".to_string(), Value::String(cells[0].clone()));
            data.insert("费率或条件".to_string(), Value::String(cells[1].clone()));
            result.push(MacroItem { data });
        }
    }

    Ok(result)
}

/// 天天基金 - 开放式基金每日净值全量
pub async fn fund_open_fund_daily_em() -> Result<Vec<MacroItem>, String> {
    let url = "https://fund.eastmoney.com/Data/Fund_JJJZ_Data.aspx";
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .get(url)
        .query(&[
            ("t", "1"),
            ("page", "1,50000"),
            ("js", "reData"),
            ("sort", "fcode,asc"),
        ])
        .send()
        .await
        .map_err(|e| format!("请求开放式基金每日净值失败: {}", e))?;

    let text = res.text().await.map_err(|e| e.to_string())?;
    let json_text = text.strip_prefix("var reData=").ok_or("提取 JS 数据失败")?;

    let json_val: Value =
        serde_json::from_str(json_text).map_err(|e| format!("JSON 解析失败: {}", e))?;

    let arr = json_val["datas"].as_array().ok_or("未获得 datas 数组")?;
    let mut result = Vec::new();

    for row in arr {
        if let Some(r) = row.as_array() {
            if r.len() >= 5 {
                let mut data = HashMap::new();
                data.insert("基金代码".to_string(), r[0].clone());
                data.insert("基金简称".to_string(), r[1].clone());
                data.insert("单位净值".to_string(), r[3].clone());
                data.insert("累计净值".to_string(), r[4].clone());
                result.push(MacroItem { data });
            }
        }
    }

    Ok(result)
}

/// 天天基金 - 货币型基金每日收益
pub async fn fund_money_fund_daily_em() -> Result<Vec<MacroItem>, String> {
    let url = "https://fund.eastmoney.com/HBJJ_data.aspx";
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .get(url)
        .query(&[("v", "0.12345678"), ("page", "1,5000")])
        .send()
        .await
        .map_err(|e| format!("请求货币型基金收益失败: {}", e))?;

    let text = res.text().await.map_err(|e| e.to_string())?;
    let start = text.find('{').ok_or("找不到包含 JSON 的边界")?;
    let end = text.rfind('}').ok_or("找不到 JSON 结束位置")?;

    let json_val: Value =
        serde_json::from_str(&text[start..=end]).map_err(|e| format!("JSON 解析失败: {}", e))?;

    let arr = json_val["datas"].as_array().ok_or("未获得 datas 数组")?;
    let mut result = Vec::new();

    for row in arr {
        if let Some(r) = row.as_array() {
            if r.len() >= 6 {
                let mut data = HashMap::new();
                data.insert("基金代码".to_string(), r[0].clone());
                data.insert("基金简称".to_string(), r[1].clone());
                data.insert("万份收益".to_string(), r[2].clone());
                data.insert("7日年化收益率".to_string(), r[3].clone());
                result.push(MacroItem { data });
            }
        }
    }

    Ok(result)
}

/// 东方财富 - 场内交易型基金(ETF/LOF)业绩排行
pub async fn fund_exchange_rank_em() -> Result<Vec<MacroItem>, String> {
    let url = "https://api.fund.eastmoney.com/FundTradeRank/GetRankList";
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .get(url)
        .header("Referer", "https://fund.eastmoney.com/")
        .query(&[
            ("ft", "cn"),
            ("sc", "1z"),
            ("st", "desc"),
            ("pi", "1"),
            ("pn", "10000"),
            ("isab", "1"),
        ])
        .send()
        .await
        .map_err(|e| format!("请求场内基金排行失败: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("接口状态码错误: {}", res.status()));
    }

    let json_val: Value = res.json().await.map_err(|e| e.to_string())?;
    let data_str = json_val["Data"].as_str().ok_or("缺失 Data 字符串")?;
    let inner_json: Value = serde_json::from_str(data_str).map_err(|e| e.to_string())?;

    let arr = inner_json["datas"]
        .as_array()
        .ok_or("未检测到 datas 数组")?;
    let mut result = Vec::new();

    for row in arr {
        if let Some(s) = row.as_str() {
            let parts: Vec<&str> = s.split('|').collect();
            if parts.len() >= 7 {
                let mut data = HashMap::new();
                data.insert("基金代码".to_string(), Value::String(parts[0].to_string()));
                data.insert("基金简称".to_string(), Value::String(parts[1].to_string()));
                data.insert("单位净值".to_string(), Value::String(parts[3].to_string()));
                data.insert("累计净值".to_string(), Value::String(parts[4].to_string()));
                data.insert("日增长率".to_string(), Value::String(parts[5].to_string()));
                result.push(MacroItem { data });
            }
        }
    }

    Ok(result)
}

/// 东方财富 - 基金概况档案
pub async fn fund_overview_em(symbol: &str) -> Result<Vec<MacroItem>, String> {
    let url = format!("https://fundf10.eastmoney.com/jbgk_{}.html", symbol);
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("请求基金概况失败: {}", e))?;

    let text = res.text().await.map_err(|e| e.to_string())?;
    let mut result = Vec::new();

    let document = scraper::Html::parse_document(&text);
    let row_selector = scraper::Selector::parse("table.info tr").unwrap();
    let cell_selector = scraper::Selector::parse("td, th").unwrap();

    for row in document.select(&row_selector) {
        let cells: Vec<String> = row
            .select(&cell_selector)
            .map(|c| c.text().collect::<Vec<_>>().join("").trim().to_string())
            .collect();

        if cells.len() >= 2 {
            let mut data = HashMap::new();
            data.insert("项目".to_string(), Value::String(cells[0].clone()));
            data.insert("内容".to_string(), Value::String(cells[1].clone()));
            result.push(MacroItem { data });
        }
    }

    Ok(result)
}

/// 雪球(蛋卷)基金 - 历史历史业绩表现
pub async fn fund_individual_achievement_xq(symbol: &str) -> Result<Vec<MacroItem>, String> {
    let url = format!("https://danjuanfunds.com/djapi/fund/achievement/{}", symbol);
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("请求雪球基金业绩失败: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("雪球接口报错: {}", res.status()));
    }

    let json_val: Value = res.json().await.map_err(|e| e.to_string())?;
    let arr = json_val["data"]["achievement_list"]
        .as_array()
        .ok_or("缺失 achievement_list 数组")?;

    let mut result = Vec::new();
    for row in arr {
        let mut data = HashMap::new();
        if let Some(obj) = row.as_object() {
            for (k, v) in obj {
                let name = match k.as_str() {
                    "period" => "周期",
                    "achievement" => "业绩率",
                    "rank" => "同类排名",
                    _ => k,
                };
                data.insert(name.to_string(), v.clone());
            }
        }
        result.push(MacroItem { data });
    }

    Ok(result)
}

/// 同花顺 - 基金基本信息
pub async fn fund_info_ths(symbol: &str) -> Result<Vec<MacroItem>, String> {
    let url = format!("https://fund.10jqka.com.cn/{}/", symbol);
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("请求同花顺基金信息失败: {}", e))?;

    let text = res.text().await.map_err(|e| e.to_string())?;
    let mut result = Vec::new();

    let document = scraper::Html::parse_document(&text);
    let row_selector = scraper::Selector::parse("table tr").unwrap();
    let cell_selector = scraper::Selector::parse("td, th").unwrap();

    for row in document.select(&row_selector) {
        let cells: Vec<String> = row
            .select(&cell_selector)
            .map(|c| c.text().collect::<Vec<_>>().join("").trim().to_string())
            .collect();

        if cells.len() >= 2 {
            let mut data = HashMap::new();
            data.insert("项目".to_string(), Value::String(cells[0].clone()));
            data.insert("值".to_string(), Value::String(cells[1].clone()));
            result.push(MacroItem { data });
        }
    }

    Ok(result)
}

/// 雪球(蛋卷)基金 - 盈利概率预测分析
pub async fn fund_individual_profit_probability_xq(symbol: &str) -> Result<Vec<MacroItem>, String> {
    let url = format!(
        "https://danjuanfunds.com/djapi/fund/profit_ratio/{}",
        symbol
    );
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("请求雪球盈利概率失败: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("雪球接口返回错误: {}", res.status()));
    }

    let json_val: Value = res.json().await.map_err(|e| e.to_string())?;
    let arr = json_val["data"]["profit_ratio_list"]
        .as_array()
        .ok_or("缺失 profit_ratio_list 数组")?;

    let mut result = Vec::new();
    for row in arr {
        let mut data = HashMap::new();
        if let Some(obj) = row.as_object() {
            for (k, v) in obj {
                let name = match k.as_str() {
                    "holding_days" => "持有天数",
                    "profit_ratio" => "盈利概率",
                    "avg_yield" => "平均收益率",
                    _ => k,
                };
                data.insert(name.to_string(), v.clone());
            }
        }
        result.push(MacroItem { data });
    }

    Ok(result)
}

/// 雪球(蛋卷)基金 - 详细持仓资产占比结构
pub async fn fund_individual_detail_hold_xq(symbol: &str) -> Result<Vec<MacroItem>, String> {
    let url = format!(
        "https://danjuanfunds.com/djapi/fund/detail/asset_allocation/{}",
        symbol
    );
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("请求雪球持仓占比失败: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("雪球接口返回错误: {}", res.status()));
    }

    let json_val: Value = res.json().await.map_err(|e| e.to_string())?;
    let arr = json_val["data"]["asset_allocation_list"]
        .as_array()
        .ok_or("缺失 asset_allocation_list 数组")?;

    let mut result = Vec::new();
    for row in arr {
        let mut data = HashMap::new();
        if let Some(obj) = row.as_object() {
            for (k, v) in obj {
                let name = match k.as_str() {
                    "type_name" => "资产类型",
                    "percent" => "持仓占比",
                    _ => k,
                };
                data.insert(name.to_string(), v.clone());
            }
        }
        result.push(MacroItem { data });
    }

    Ok(result)
}

/// 巨潮资讯 - 基金定期报告股票持仓明细
pub async fn fund_report_stock_cninfo() -> Result<Vec<MacroItem>, String> {
    let url = "http://webapi.cninfo.com.cn/api/sysapi/p_sysapi1081";
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .post(url)
        .header("mktcode", "000001")
        .send()
        .await
        .map_err(|e| format!("请求巨潮股票持仓失败: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("巨潮接口代码: {}", res.status()));
    }

    let json_val: Value = res.json().await.map_err(|e| e.to_string())?;
    let arr = json_val["records"].as_array().ok_or("缺失 records 数组")?;

    let mut result = Vec::new();
    for row in arr {
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

/// 零壹智库 - 股票型基金仓位测算
pub async fn fund_stock_position_lg() -> Result<Vec<MacroItem>, String> {
    let url = "https://www.100ppi.com/sf/day-577.html";
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .get(url)
        .send()
        .await
        .map_err(|e| format!("请求基金仓位测算失败: {}", e))?;

    let text = res.text().await.map_err(|e| e.to_string())?;
    let mut result = Vec::new();

    let document = scraper::Html::parse_document(&text);
    let row_selector = scraper::Selector::parse("table.list-table tr").unwrap();
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
            data.insert("日期".to_string(), Value::String(cells[0].clone()));
            data.insert("仓位测算".to_string(), Value::String(cells[1].clone()));
            data.insert("变动".to_string(), Value::String(cells[2].clone()));
            result.push(MacroItem { data });
        }
    }

    Ok(result)
}

/// 零壹智库 - 混合型基金仓位测算
pub async fn fund_balance_position_lg() -> Result<Vec<MacroItem>, String> {
    let url = "https://www.100ppi.com/sf/day-578.html";
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .get(url)
        .send()
        .await
        .map_err(|e| format!("请求混合型基金仓位测算失败: {}", e))?;

    let text = res.text().await.map_err(|e| e.to_string())?;
    let mut result = Vec::new();

    let document = scraper::Html::parse_document(&text);
    let row_selector = scraper::Selector::parse("table.list-table tr").unwrap();
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
            data.insert("日期".to_string(), Value::String(cells[0].clone()));
            data.insert("仓位测算".to_string(), Value::String(cells[1].clone()));
            data.insert("变动".to_string(), Value::String(cells[2].clone()));
            result.push(MacroItem { data });
        }
    }

    Ok(result)
}

/// 零壹智库 - 灵活配置型基金仓位测算
pub async fn fund_linghuo_position_lg() -> Result<Vec<MacroItem>, String> {
    let url = "https://www.100ppi.com/sf/day-579.html";
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .get(url)
        .send()
        .await
        .map_err(|e| format!("请求灵活配置型基金仓位测算失败: {}", e))?;

    let text = res.text().await.map_err(|e| e.to_string())?;
    let mut result = Vec::new();

    let document = scraper::Html::parse_document(&text);
    let row_selector = scraper::Selector::parse("table.list-table tr").unwrap();
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
            data.insert("日期".to_string(), Value::String(cells[0].clone()));
            data.insert("仓位测算".to_string(), Value::String(cells[1].clone()));
            data.insert("变动".to_string(), Value::String(cells[2].clone()));
            result.push(MacroItem { data });
        }
    }

    Ok(result)
}

/// 天天基金 - 上海证券基金评级
pub async fn fund_rating_sh() -> Result<Vec<MacroItem>, String> {
    let url = "https://fund.eastmoney.com/data/fundrating_3.html";
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .get(url)
        .send()
        .await
        .map_err(|e| format!("请求上海证券评级失败: {}", e))?;

    let text = res.text().await.map_err(|e| e.to_string())?;
    let mut result = Vec::new();

    let document = scraper::Html::parse_document(&text);
    let row_selector = scraper::Selector::parse("div#fundinfo table tr").unwrap();
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

        if cells.len() >= 5 {
            let mut data = HashMap::new();
            data.insert("代码".to_string(), Value::String(cells[0].clone()));
            data.insert("简称".to_string(), Value::String(cells[1].clone()));
            data.insert(
                "上海证券3年评级".to_string(),
                Value::String(cells[3].clone()),
            );
            data.insert(
                "上海证券5年评级".to_string(),
                Value::String(cells[4].clone()),
            );
            result.push(MacroItem { data });
        }
    }

    Ok(result)
}

/// 东方财富 - 基金公司历年管理规模排行
pub async fn fund_aum_hist_em(year: &str) -> Result<Vec<MacroItem>, String> {
    let url = format!(
        "https://fund.eastmoney.com/Company/home/HistoryScaleTable?year={}",
        year
    );
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("请求历年规模失败: {}", e))?;

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
            data.insert("序号".to_string(), Value::String(cells[0].clone()));
            data.insert("基金公司".to_string(), Value::String(cells[1].clone()));
            data.insert("总规模".to_string(), Value::String(cells[2].clone()));
            result.push(MacroItem { data });
        }
    }

    Ok(result)
}

/// 天天基金 - 招商证券基金评级
pub async fn fund_rating_zs() -> Result<Vec<MacroItem>, String> {
    let url = "https://fund.eastmoney.com/data/fundrating_2.html";
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .get(url)
        .send()
        .await
        .map_err(|e| format!("请求招商证券评级失败: {}", e))?;

    let text = res.text().await.map_err(|e| e.to_string())?;
    let mut result = Vec::new();

    let document = scraper::Html::parse_document(&text);
    let row_selector = scraper::Selector::parse("div#fundinfo table tr").unwrap();
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

        if cells.len() >= 5 {
            let mut data = HashMap::new();
            data.insert("代码".to_string(), Value::String(cells[0].clone()));
            data.insert("简称".to_string(), Value::String(cells[1].clone()));
            data.insert(
                "招商证券3年评级".to_string(),
                Value::String(cells[3].clone()),
            );
            data.insert(
                "招商证券5年评级".to_string(),
                Value::String(cells[4].clone()),
            );
            result.push(MacroItem { data });
        }
    }

    Ok(result)
}

/// 天天基金 - 济安金信基金评级
pub async fn fund_rating_ja() -> Result<Vec<MacroItem>, String> {
    let url = "https://fund.eastmoney.com/data/fundrating_4.html";
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .get(url)
        .send()
        .await
        .map_err(|e| format!("请求济安金信评级失败: {}", e))?;

    let text = res.text().await.map_err(|e| e.to_string())?;
    let mut result = Vec::new();

    let document = scraper::Html::parse_document(&text);
    let row_selector = scraper::Selector::parse("div#fundinfo table tr").unwrap();
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

        if cells.len() >= 5 {
            let mut data = HashMap::new();
            data.insert("代码".to_string(), Value::String(cells[0].clone()));
            data.insert("简称".to_string(), Value::String(cells[1].clone()));
            data.insert(
                "济安金信3年评级".to_string(),
                Value::String(cells[3].clone()),
            );
            result.push(MacroItem { data });
        }
    }

    Ok(result)
}

/// 东方财富 - 货币型基金业绩排行
pub async fn fund_money_rank_em() -> Result<Vec<MacroItem>, String> {
    let url = "https://api.fund.eastmoney.com/FundTradeRank/GetRankList";
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .get(url)
        .header("Referer", "https://fund.eastmoney.com/")
        .query(&[
            ("ft", "hb"),
            ("sc", "1z"),
            ("st", "desc"),
            ("pi", "1"),
            ("pn", "10000"),
            ("isab", "1"),
        ])
        .send()
        .await
        .map_err(|e| format!("请求货币型基金排行失败: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("接口状态码: {}", res.status()));
    }

    let json_val: Value = res.json().await.map_err(|e| e.to_string())?;
    let data_str = json_val["Data"].as_str().ok_or("缺失 Data 字符串")?;
    let inner_json: Value = serde_json::from_str(data_str).map_err(|e| e.to_string())?;

    let arr = inner_json["datas"]
        .as_array()
        .ok_or("未检测到 datas 数组")?;
    let mut result = Vec::new();

    for row in arr {
        if let Some(s) = row.as_str() {
            let parts: Vec<&str> = s.split('|').collect();
            if parts.len() >= 6 {
                let mut data = HashMap::new();
                data.insert("基金代码".to_string(), Value::String(parts[0].to_string()));
                data.insert("基金简称".to_string(), Value::String(parts[1].to_string()));
                data.insert("万份收益".to_string(), Value::String(parts[3].to_string()));
                data.insert(
                    "7日年化收益率".to_string(),
                    Value::String(parts[4].to_string()),
                );
                result.push(MacroItem { data });
            }
        }
    }

    Ok(result)
}

/// 东方财富 - 理财型基金业绩排行
pub async fn fund_lcx_rank_em() -> Result<Vec<MacroItem>, String> {
    let url = "https://api.fund.eastmoney.com/FundTradeRank/GetRankList";
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .get(url)
        .header("Referer", "https://fund.eastmoney.com/")
        .query(&[
            ("ft", "lc"),
            ("sc", "1z"),
            ("st", "desc"),
            ("pi", "1"),
            ("pn", "10000"),
            ("isab", "1"),
        ])
        .send()
        .await
        .map_err(|e| format!("请求理财型基金排行失败: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("接口状态码: {}", res.status()));
    }

    let json_val: Value = res.json().await.map_err(|e| e.to_string())?;
    let data_str = json_val["Data"].as_str().ok_or("缺失 Data 字符串")?;
    let inner_json: Value = serde_json::from_str(data_str).map_err(|e| e.to_string())?;

    let arr = inner_json["datas"]
        .as_array()
        .ok_or("未检测到 datas 数组")?;
    let mut result = Vec::new();

    for row in arr {
        if let Some(s) = row.as_str() {
            let parts: Vec<&str> = s.split('|').collect();
            if parts.len() >= 6 {
                let mut data = HashMap::new();
                data.insert("基金代码".to_string(), Value::String(parts[0].to_string()));
                data.insert("基金简称".to_string(), Value::String(parts[1].to_string()));
                data.insert("万份收益".to_string(), Value::String(parts[3].to_string()));
                data.insert(
                    "7日年化收益率".to_string(),
                    Value::String(parts[4].to_string()),
                );
                result.push(MacroItem { data });
            }
        }
    }

    Ok(result)
}

/// 东方财富 - 香港基金业绩排行
pub async fn fund_hk_rank_em() -> Result<Vec<MacroItem>, String> {
    let url = "https://api.fund.eastmoney.com/FundTradeRank/GetRankList";
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .get(url)
        .header("Referer", "https://fund.eastmoney.com/")
        .query(&[
            ("ft", "hk"),
            ("sc", "1z"),
            ("st", "desc"),
            ("pi", "1"),
            ("pn", "10000"),
            ("isab", "1"),
        ])
        .send()
        .await
        .map_err(|e| format!("请求香港基金排行失败: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("接口状态码: {}", res.status()));
    }

    let json_val: Value = res.json().await.map_err(|e| e.to_string())?;
    let data_str = json_val["Data"].as_str().ok_or("缺失 Data 字符串")?;
    let inner_json: Value = serde_json::from_str(data_str).map_err(|e| e.to_string())?;

    let arr = inner_json["datas"]
        .as_array()
        .ok_or("未检测到 datas 数组")?;
    let mut result = Vec::new();

    for row in arr {
        if let Some(s) = row.as_str() {
            let parts: Vec<&str> = s.split('|').collect();
            if parts.len() >= 7 {
                let mut data = HashMap::new();
                data.insert("基金代码".to_string(), Value::String(parts[0].to_string()));
                data.insert("基金简称".to_string(), Value::String(parts[1].to_string()));
                data.insert("单位净值".to_string(), Value::String(parts[3].to_string()));
                data.insert("日增长率".to_string(), Value::String(parts[5].to_string()));
                result.push(MacroItem { data });
            }
        }
    }

    Ok(result)
}

/// 中国证券投资基金业协会 - 会员名录信息
pub async fn amac_member_info() -> Result<Vec<MacroItem>, String> {
    let url = "https://gs.amac.org.cn/amac-infodisc/res/member/member/list";
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .post(url)
        .header("Content-Type", "application/json")
        .body(r#"{"page":"0","size":"200"}"#)
        .send()
        .await
        .map_err(|e| format!("请求中基协会员信息失败: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("中基协接口代码: {}", res.status()));
    }

    let json_val: Value = res.json().await.map_err(|e| e.to_string())?;
    let arr = json_val["content"].as_array().ok_or("缺失 content 数组")?;

    let mut result = Vec::new();
    for row in arr {
        let mut data = HashMap::new();
        if let Some(obj) = row.as_object() {
            for (k, v) in obj {
                let name = match k.as_str() {
                    "membroName" => "机构名称",
                    "membroType" => "会员类型",
                    "membroCode" => "会员编码",
                    "joinTime" => "入会时间",
                    _ => k,
                };
                data.insert(name.to_string(), v.clone());
            }
        }
        result.push(MacroItem { data });
    }

    Ok(result)
}

/// 中国证券投资基金业协会 - 私募基金管理人登记公示
pub async fn amac_manager_info() -> Result<Vec<MacroItem>, String> {
    let url = "https://gs.amac.org.cn/amac-infodisc/res/pof/manager/list";
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .post(url)
        .header("Content-Type", "application/json")
        .body(r#"{"page":"0","size":"200"}"#)
        .send()
        .await
        .map_err(|e| format!("请求中基协管理人信息失败: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("中基协接口代码: {}", res.status()));
    }

    let json_val: Value = res.json().await.map_err(|e| e.to_string())?;
    let arr = json_val["content"].as_array().ok_or("缺失 content 数组")?;

    let mut result = Vec::new();
    for row in arr {
        let mut data = HashMap::new();
        if let Some(obj) = row.as_object() {
            for (k, v) in obj {
                let name = match k.as_str() {
                    "managerName" => "私募基金管理人名称",
                    "primaryInvestType" => "机构类型",
                    "regNo" => "登记编号",
                    "establishDate" => "成立时间",
                    _ => k,
                };
                data.insert(name.to_string(), v.clone());
            }
        }
        result.push(MacroItem { data });
    }

    Ok(result)
}

/// 天天基金 - 基金分红公告
pub async fn fund_announcement_dividend_em() -> Result<Vec<MacroItem>, String> {
    let url = "https://datacenter-web.eastmoney.com/api/data/v1/get?reportName=RPT_FUND_DIVIDEND_DETAIL&columns=ALL&sortColumns=NOTICE_DATE&sortTypes=-1&pageNumber=1&pageSize=500";
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .get(url)
        .send()
        .await
        .map_err(|e| format!("请求分红公告失败: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("接口代码: {}", res.status()));
    }

    let json_val: Value = res.json().await.map_err(|e| e.to_string())?;
    let arr = json_val["result"]["data"]
        .as_array()
        .ok_or("缺失 result.data 数组")?;

    let mut result = Vec::new();
    for row in arr {
        let mut data = HashMap::new();
        if let Some(obj) = row.as_object() {
            for (k, v) in obj {
                let name = match k.as_str() {
                    "SECURITY_CODE" => "基金代码",
                    "SECURITY_NAME" => "基金简称",
                    "NOTICE_DATE" => "公告日期",
                    "RECORD_DATE" => "权益登记日",
                    "EX_DIVIDEND_DATE" => "除息日",
                    "DIVIDEND_PAYMENT_DATE" => "派息日",
                    "DIVIDEND_PER_SHARE" => "每份分红",
                    _ => k,
                };
                data.insert(name.to_string(), v.clone());
            }
        }
        result.push(MacroItem { data });
    }

    Ok(result)
}

/// 天天基金 - 基金定期报告公告
pub async fn fund_announcement_report_em() -> Result<Vec<MacroItem>, String> {
    let url = "https://datacenter-web.eastmoney.com/api/data/v1/get?reportName=RPT_FUND_PERIODIC_REPORT&columns=ALL&sortColumns=NOTICE_DATE&sortTypes=-1&pageNumber=1&pageSize=500";
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .get(url)
        .send()
        .await
        .map_err(|e| format!("请求定期报告公告失败: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("接口代码: {}", res.status()));
    }

    let json_val: Value = res.json().await.map_err(|e| e.to_string())?;
    let arr = json_val["result"]["data"]
        .as_array()
        .ok_or("缺失 result.data 数组")?;

    let mut result = Vec::new();
    for row in arr {
        let mut data = HashMap::new();
        if let Some(obj) = row.as_object() {
            for (k, v) in obj {
                let name = match k.as_str() {
                    "SECURITY_CODE" => "基金代码",
                    "SECURITY_NAME" => "基金简称",
                    "NOTICE_DATE" => "公告日期",
                    "REPORT_TITLE" => "报告标题",
                    "REPORT_TYPE" => "报告类型",
                    _ => k,
                };
                data.insert(name.to_string(), v.clone());
            }
        }
        result.push(MacroItem { data });
    }

    Ok(result)
}

/// 天天基金 - 基金人员变动公告
pub async fn fund_announcement_personnel_em() -> Result<Vec<MacroItem>, String> {
    let url = "https://datacenter-web.eastmoney.com/api/data/v1/get?reportName=RPT_FUND_MANAGER_CHANGE&columns=ALL&sortColumns=NOTICE_DATE&sortTypes=-1&pageNumber=1&pageSize=500";
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .get(url)
        .send()
        .await
        .map_err(|e| format!("请求人员变动公告失败: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("接口代码: {}", res.status()));
    }

    let json_val: Value = res.json().await.map_err(|e| e.to_string())?;
    let arr = json_val["result"]["data"]
        .as_array()
        .ok_or("缺失 result.data 数组")?;

    let mut result = Vec::new();
    for row in arr {
        let mut data = HashMap::new();
        if let Some(obj) = row.as_object() {
            for (k, v) in obj {
                let name = match k.as_str() {
                    "SECURITY_CODE" => "基金代码",
                    "SECURITY_NAME" => "基金简称",
                    "NOTICE_DATE" => "公告日期",
                    "CHANGE_TYPE" => "变动类型",
                    "PERSON_NAME" => "人员姓名",
                    _ => k,
                };
                data.insert(name.to_string(), v.clone());
            }
        }
        result.push(MacroItem { data });
    }

    Ok(result)
}

/// 巨潮资讯 - 基金定期报告行业配置明细
pub async fn fund_report_industry_allocation_cninfo() -> Result<Vec<MacroItem>, String> {
    let url = "http://webapi.cninfo.com.cn/api/sysapi/p_sysapi1082";
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .post(url)
        .header("mktcode", "000001")
        .send()
        .await
        .map_err(|e| format!("请求巨潮行业配置失败: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("巨潮接口代码: {}", res.status()));
    }

    let json_val: Value = res.json().await.map_err(|e| e.to_string())?;
    let arr = json_val["records"].as_array().ok_or("缺失 records 数组")?;

    let mut result = Vec::new();
    for row in arr {
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

/// 天天基金 - 基金持仓重大变动明细
pub async fn fund_portfolio_change_em(symbol: &str, year: &str) -> Result<Vec<MacroItem>, String> {
    let url = "https://fundf10.eastmoney.com/FundArchivesDatas.aspx";
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .get(url)
        .header(
            "Referer",
            format!("https://fundf10.eastmoney.com/zgbd_{}.html", symbol),
        )
        .query(&[("type", "zgbd"), ("code", symbol), ("year", year)])
        .send()
        .await
        .map_err(|e| format!("请求持仓重大变动失败: {}", e))?;

    let text = res.text().await.map_err(|e| e.to_string())?;
    let start = text.find('{').ok_or("找不到包含 HTML 的 JSON")?;
    let end = text.rfind('}').ok_or("找不到 JSON 结束位置")?;

    let json_val: Value =
        serde_json::from_str(&text[start..=end]).map_err(|e| format!("JSON 解析失败: {}", e))?;

    let html_content = json_val["content"].as_str().unwrap_or_default();
    let mut result = Vec::new();

    let document = scraper::Html::parse_document(html_content);
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

        if cells.len() >= 5 {
            let mut data = HashMap::new();
            data.insert("股票代码".to_string(), Value::String(cells[1].clone()));
            data.insert("股票名称".to_string(), Value::String(cells[2].clone()));
            data.insert(
                "本期累计买入金额".to_string(),
                Value::String(cells[3].clone()),
            );
            data.insert(
                "占期初基金资产净值比例".to_string(),
                Value::String(cells[4].clone()),
            );
            result.push(MacroItem { data });
        }
    }

    Ok(result)
}

/// 新浪财经 - ETF 基金分类与行情
pub async fn fund_etf_category_sina(symbol: &str) -> Result<Vec<MacroItem>, String> {
    let node = match symbol {
        "股票型" => "etf_hq_fund",
        "QDII" => "qdii_etf_hq_fund",
        _ => "etf_hq_fund",
    };

    let url = "https://vip.stock.finance.sina.com.cn/quotes_service/api/json_v2.php/Market_Center.getHQNodeData";
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .get(url)
        .query(&[
            ("page", "1"),
            ("num", "500"),
            ("sort", "symbol"),
            ("asc", "1"),
            ("node", node),
        ])
        .send()
        .await
        .map_err(|e| format!("请求新浪 ETF 行情失败: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("新浪接口代码: {}", res.status()));
    }

    let json_val: Value = res.json().await.map_err(|e| e.to_string())?;
    let arr = json_val.as_array().ok_or("缺失 json 数组")?;

    let mut result = Vec::new();
    for row in arr {
        let mut data = HashMap::new();
        if let Some(obj) = row.as_object() {
            for (k, v) in obj {
                let name = match k.as_str() {
                    "symbol" => "代码",
                    "name" => "名称",
                    "trade" => "最新价",
                    "changepercent" => "涨跌幅",
                    _ => k,
                };
                data.insert(name.to_string(), v.clone());
            }
        }
        result.push(MacroItem { data });
    }

    Ok(result)
}

/// 上海证券交易所 - ETF 规模分布
pub async fn fund_etf_scale_sse() -> Result<Vec<MacroItem>, String> {
    let url = "https://query.sse.com.cn/common41/getBaseEtfInfo.do?sqlId=COMMON_SSE_ZQPX_ETF_ZJS_L";
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .get(url)
        .header("Referer", "https://www.sse.com.cn/")
        .send()
        .await
        .map_err(|e| format!("请求上交所 ETF 规模失败: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("上交所接口代码: {}", res.status()));
    }

    let json_val: Value = res.json().await.map_err(|e| e.to_string())?;
    let arr = json_val["result"].as_array().ok_or("缺失 result 数组")?;

    let mut result = Vec::new();
    for row in arr {
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

/// 深交所 - ETF 规模与日报分布
pub async fn fund_etf_scale_szse() -> Result<Vec<MacroItem>, String> {
    let url =
        "https://www.szse.cn/api/report/ShowReport/data?SHOWBD_PROCODE=SHOWBD_1002&CATALOGID=1945";
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .get(url)
        .send()
        .await
        .map_err(|e| format!("请求深交所 ETF 规模失败: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("深交所接口代码: {}", res.status()));
    }

    let json_val: Value = res.json().await.map_err(|e| e.to_string())?;
    let arr = json_val[0]["data"]
        .as_array()
        .ok_or("缺失深交所 data 数组")?;

    let mut result = Vec::new();
    for row in arr {
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

/// 新浪财经 - 开放式基金规模
pub async fn fund_scale_open_sina() -> Result<Vec<MacroItem>, String> {
    let url = "https://vip.stock.finance.sina.com.cn/quotes_service/api/json_v2.php/Market_Center.getHQNodeData";
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .get(url)
        .query(&[
            ("page", "1"),
            ("num", "500"),
            ("sort", "symbol"),
            ("asc", "1"),
            ("node", "open_fund"),
        ])
        .send()
        .await
        .map_err(|e| format!("请求新浪开放式基金规模失败: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("新浪接口代码: {}", res.status()));
    }

    let json_val: Value = res.json().await.map_err(|e| e.to_string())?;
    let arr = json_val.as_array().ok_or("缺失 json 数组")?;

    let mut result = Vec::new();
    for row in arr {
        let mut data = HashMap::new();
        if let Some(obj) = row.as_object() {
            for (k, v) in obj {
                let name = match k.as_str() {
                    "symbol" => "基金代码",
                    "name" => "基金简称",
                    "trade" => "单位净值",
                    _ => k,
                };
                data.insert(name.to_string(), v.clone());
            }
        }
        result.push(MacroItem { data });
    }

    Ok(result)
}

/// 巨潮资讯 - 基金报告资产配置明细
pub async fn fund_report_asset_allocation_cninfo() -> Result<Vec<MacroItem>, String> {
    let url = "http://webapi.cninfo.com.cn/api/sysapi/p_sysapi1080";
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .post(url)
        .header("mktcode", "000001")
        .send()
        .await
        .map_err(|e| format!("请求巨潮资产配置失败: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("巨潮接口代码: {}", res.status()));
    }

    let json_val: Value = res.json().await.map_err(|e| e.to_string())?;
    let arr = json_val["records"].as_array().ok_or("缺失 records 数组")?;

    let mut result = Vec::new();
    for row in arr {
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

/// 同花顺 - 新发基金行情列表
pub async fn fund_new_found_ths() -> Result<Vec<MacroItem>, String> {
    let url = "https://fund.10jqka.com.cn/data/client/myfund/xfjjList";
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .get(url)
        .send()
        .await
        .map_err(|e| format!("请求同花顺新发基金失败: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("接口代码: {}", res.status()));
    }

    let json_val: Value = res.json().await.map_err(|e| e.to_string())?;
    let arr = json_val["data"].as_array().ok_or("缺失 data 数组")?;

    let mut result = Vec::new();
    for row in arr {
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

/// 雪球(蛋卷)基金 - 基金诊断分析与风格评估
pub async fn fund_individual_analysis_xq(symbol: &str) -> Result<Vec<MacroItem>, String> {
    let url = format!("https://danjuanfunds.com/djapi/fund/derived/{}", symbol);
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("请求雪球基金分析失败: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("雪球接口返回错误: {}", res.status()));
    }

    let json_val: Value = res.json().await.map_err(|e| e.to_string())?;
    let data_obj = &json_val["data"];

    let mut result = Vec::new();
    if let Some(obj) = data_obj.as_object() {
        for (k, v) in obj {
            let mut data = HashMap::new();
            data.insert("项目".to_string(), Value::String(k.clone()));
            data.insert("分析值".to_string(), v.clone());
            result.push(MacroItem { data });
        }
    }

    Ok(result)
}

/// 同花顺理财 - 基金数据-每日净值与分类实时行情
pub async fn fund_etf_category_ths(symbol: &str) -> Result<Vec<MacroItem>, String> {
    let inner_symbol = match symbol {
        "股票型" => "gpx",
        "债券型" => "zqx",
        "混合型" => "hhx",
        "LOF" => "LOF",
        "QDII" => "QDII",
        "保本型" => "bbx",
        "指数型" => "zsx",
        "全部" => "all",
        _ => "ETF",
    };

    let url = format!(
        "https://fund.10jqka.com.cn/data/Net/info/{}_rate_desc_0_0_1_9999_0_0_0_jsonp_g.html",
        inner_symbol
    );
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("请求同花顺分类净值失败: {}", e))?;

    let text = res.text().await.map_err(|e| e.to_string())?;
    let start = text.find('{').ok_or("找不到包含 JSON 的边界")?;
    let end = text.rfind('}').ok_or("找不到 JSON 结束位置")?;

    let json_val: Value =
        serde_json::from_str(&text[start..=end]).map_err(|e| format!("JSON 解析失败: {}", e))?;

    let data_map = json_val["data"]["data"]
        .as_object()
        .ok_or("缺失 data.data 对象")?;

    let mut result = Vec::new();
    for (_k, v) in data_map {
        let mut data = HashMap::new();
        if let Some(obj) = v.as_object() {
            for (col_k, col_v) in obj {
                let name = match col_k.as_str() {
                    "code" => "基金代码",
                    "name" => "基金名称",
                    "typename" => "基金类型",
                    "net" => "单位净值",
                    "totalnet" => "累计净值",
                    "rate" => "增长率",
                    _ => col_k,
                };
                data.insert(name.to_string(), col_v.clone());
            }
        }
        result.push(MacroItem { data });
    }

    Ok(result)
}

/// 东方财富 - ETF 历史日线/分钟行情
pub async fn fund_etf_hist_em(symbol: &str, period: &str) -> Result<Vec<MacroItem>, String> {
    let klt = match period {
        "1" => "1",
        "5" => "5",
        "15" => "15",
        "30" => "30",
        "60" => "60",
        _ => "101", // 默认日线
    };

    // 简单判断市场前缀（上交所 1.，深交所 0.）
    let secid = if symbol.starts_with("51") || symbol.starts_with("56") || symbol.starts_with("58")
    {
        format!("1.{}", symbol)
    } else {
        format!("0.{}", symbol)
    };

    let url = "https://push2his.eastmoney.com/api/qt/stock/kline/get";
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .get(url)
        .query(&[
            ("secid", secid.as_str()),
            ("klt", klt),
            ("fqt", "1"),
            ("lmt", "50000"),
            ("end", "20500000"),
            ("iscca", "1"),
            ("fields1", "f1,f2,f3,f4,f5,f6,f7,f8"),
            (
                "fields2",
                "f51,f52,f53,f54,f55,f56,f57,f58,f59,f60,f61,f62,f63,f64",
            ),
            ("ut", "f057cbcbce2a86e2866ab8877db1d059"),
            ("forcect", "1"),
        ])
        .send()
        .await
        .map_err(|e| format!("请求 ETF K线失败: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("ETF K线接口错误: {}", res.status()));
    }

    let json_val: Value = res.json().await.map_err(|e| e.to_string())?;
    let arr = json_val["data"]["klines"]
        .as_array()
        .ok_or("未获得 klines 数据")?;

    let mut result = Vec::new();
    for row in arr {
        if let Some(s) = row.as_str() {
            let parts: Vec<&str> = s.split(',').collect();
            if parts.len() >= 6 {
                let mut data = HashMap::new();
                data.insert("日期/时间".to_string(), Value::String(parts[0].to_string()));
                data.insert("开盘".to_string(), Value::String(parts[1].to_string()));
                data.insert("收盘".to_string(), Value::String(parts[2].to_string()));
                data.insert("最高".to_string(), Value::String(parts[3].to_string()));
                data.insert("最低".to_string(), Value::String(parts[4].to_string()));
                data.insert("成交量".to_string(), Value::String(parts[5].to_string()));
                result.push(MacroItem { data });
            }
        }
    }

    Ok(result)
}

/// 东方财富 - LOF 历史日线行情
pub async fn fund_lof_hist_em(symbol: &str) -> Result<Vec<MacroItem>, String> {
    fund_etf_hist_em(symbol, "daily").await
}

/// 天天基金 - 基金持仓债券明细
pub async fn fund_portfolio_bond_hold_em(
    symbol: &str,
    year: &str,
) -> Result<Vec<MacroItem>, String> {
    let url = "https://fundf10.eastmoney.com/FundArchivesDatas.aspx";
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .get(url)
        .header(
            "Referer",
            format!("https://fundf10.eastmoney.com/zqcc_{}.html", symbol),
        )
        .query(&[("type", "zqcc"), ("code", symbol), ("year", year)])
        .send()
        .await
        .map_err(|e| format!("请求持仓债券失败: {}", e))?;

    let text = res.text().await.map_err(|e| e.to_string())?;
    let start = text.find('{').ok_or("找不到包含 HTML 的 JSON")?;
    let end = text.rfind('}').ok_or("找不到 JSON 结束位置")?;

    let json_val: Value =
        serde_json::from_str(&text[start..=end]).map_err(|e| format!("JSON 解析失败: {}", e))?;

    let html_content = json_val["content"].as_str().unwrap_or_default();
    let mut result = Vec::new();

    let document = scraper::Html::parse_document(html_content);
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

        if cells.len() >= 5 {
            let mut data = HashMap::new();
            data.insert("债券代码".to_string(), Value::String(cells[1].clone()));
            data.insert("债券名称".to_string(), Value::String(cells[2].clone()));
            data.insert("占净值比例".to_string(), Value::String(cells[3].clone()));
            data.insert("持仓市值".to_string(), Value::String(cells[4].clone()));
            result.push(MacroItem { data });
        }
    }

    Ok(result)
}

/// 天天基金 - 基金行业配置明细
pub async fn fund_portfolio_industry_allocation_em(symbol: &str) -> Result<Vec<MacroItem>, String> {
    let url = "https://fundf10.eastmoney.com/FundArchivesDatas.aspx";
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .get(url)
        .header(
            "Referer",
            format!("https://fundf10.eastmoney.com/hypz_{}.html", symbol),
        )
        .query(&[("type", "hypz"), ("code", symbol)])
        .send()
        .await
        .map_err(|e| format!("请求行业配置失败: {}", e))?;

    let text = res.text().await.map_err(|e| e.to_string())?;
    let start = text.find('{').ok_or("找不到包含 HTML 的 JSON")?;
    let end = text.rfind('}').ok_or("找不到 JSON 结束位置")?;

    let json_val: Value =
        serde_json::from_str(&text[start..=end]).map_err(|e| format!("JSON 解析失败: {}", e))?;

    let html_content = json_val["content"].as_str().unwrap_or_default();
    let mut result = Vec::new();

    let document = scraper::Html::parse_document(html_content);
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
            data.insert("行业类别".to_string(), Value::String(cells[1].clone()));
            data.insert("占净值比例".to_string(), Value::String(cells[2].clone()));
            data.insert("市值".to_string(), Value::String(cells[3].clone()));
            result.push(MacroItem { data });
        }
    }

    Ok(result)
}

/// 雪球(蛋卷)基金 - 基金个人基本信息
pub async fn fund_individual_basic_info_xq(symbol: &str) -> Result<Vec<MacroItem>, String> {
    let url = format!("https://danjuanfunds.com/djapi/fund/{}", symbol);
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("请求雪球基金信息失败: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("雪球接口返回错误: {}", res.status()));
    }

    let json_val: Value = res.json().await.map_err(|e| e.to_string())?;
    let data_obj = &json_val["data"];

    let mut result = Vec::new();
    if let Some(obj) = data_obj.as_object() {
        for (k, v) in obj {
            let mapped_name = match k.as_str() {
                "fd_code" => "基金代码",
                "fd_name" => "基金名称",
                "fd_full_name" => "基金全称",
                "found_date" => "成立时间",
                "totshare" => "最新规模",
                "keeper_name" => "基金公司",
                "manager_name" => "基金经理",
                "trup_name" => "托管银行",
                "type_desc" => "基金类型",
                _ => k,
            };
            let mut data = HashMap::new();
            data.insert("item".to_string(), Value::String(mapped_name.to_string()));
            data.insert("value".to_string(), v.clone());
            result.push(MacroItem { data });
        }
    }

    Ok(result)
}

/// 东方财富 - 指数型基金信息
pub async fn fund_info_index_em(symbol: &str, indicator: &str) -> Result<Vec<MacroItem>, String> {
    let symbol_map = match symbol {
        "沪深指数" => "053",
        "行业主题" => "054",
        "大盘指数" => "01",
        "中盘指数" => "02",
        "小盘指数" => "03",
        "股票指数" => "050|001",
        "债券指数" => "050|003",
        _ => "",
    };

    let indicator_map = match indicator {
        "被动指数型" => "051",
        "增强指数型" => "052",
        _ => "",
    };

    let (fr, ftype) = if symbol == "股票指数" || symbol == "债券指数" {
        let parts: Vec<&str> = symbol_map.split('|').collect();
        (parts[0], parts[1])
    } else {
        (symbol_map, "")
    };

    let url = "https://api.fund.eastmoney.com/FundTradeRank/GetRankList";
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .get(url)
        .header("Referer", "https://fund.eastmoney.com/")
        .query(&[
            ("ft", "zs"),
            ("sc", "1n"),
            ("st", "desc"),
            ("pi", "1"),
            ("pn", "10000"),
            ("fr", fr),
            ("ftype", ftype),
            ("fr1", indicator_map),
            ("isab", "1"),
        ])
        .send()
        .await
        .map_err(|e| format!("请求指数基金列表失败: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("指数基金接口代码: {}", res.status()));
    }

    let json_val: Value = res.json().await.map_err(|e| e.to_string())?;
    let data_str = json_val["Data"].as_str().ok_or("缺失 Data 字符串")?;
    let inner_json: Value = serde_json::from_str(data_str).map_err(|e| e.to_string())?;

    let arr = inner_json["datas"]
        .as_array()
        .ok_or("未检测到 datas 数组")?;
    let mut result = Vec::new();

    for row in arr {
        if let Some(s) = row.as_str() {
            let parts: Vec<&str> = s.split('|').collect();
            if parts.len() >= 5 {
                let mut data = HashMap::new();
                data.insert("基金代码".to_string(), Value::String(parts[0].to_string()));
                data.insert("基金名称".to_string(), Value::String(parts[1].to_string()));
                data.insert("单位净值".to_string(), Value::String(parts[3].to_string()));
                data.insert("累计净值".to_string(), Value::String(parts[4].to_string()));
                result.push(MacroItem { data });
            }
        }
    }

    Ok(result)
}
