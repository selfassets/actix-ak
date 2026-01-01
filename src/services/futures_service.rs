use anyhow::{Result, anyhow};
use chrono::Utc;
use chrono_tz::Asia::Shanghai;
use reqwest::Client;
use crate::models::{FuturesInfo, FuturesHistoryData, FuturesQuery, FuturesExchange};

// 获取北京时间
fn get_beijing_time() -> chrono::DateTime<Utc> {
    Utc::now().with_timezone(&Shanghai).with_timezone(&Utc)
}

const SINA_FUTURES_REALTIME_API: &str = "https://hq.sinajs.cn";
const SINA_FUTURES_LIST_API: &str = "https://vip.stock.finance.sina.com.cn/quotes_service/api/json_v2.php/Market_Center.getHQFuturesData";

pub struct FuturesService {
    client: Client,
}

impl FuturesService {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    // 获取期货实时数据
    pub async fn get_futures_info(&self, symbol: &str) -> Result<FuturesInfo> {
        let formatted_symbol = self.format_symbol_for_realtime(symbol);
        let rn_code = self.generate_random_code();
        // 注意：URL格式是 /rn= 而不是 ?rn=，这是新浪API的特殊格式
        let url = format!("{}/rn={}&list={}", SINA_FUTURES_REALTIME_API, rn_code, formatted_symbol);
        
        let response = self.client
            .get(&url)
            .header("Accept", "*/*")
            .header("Accept-Encoding", "gzip, deflate")
            .header("Accept-Language", "zh-CN,zh;q=0.9,en;q=0.8")
            .header("Cache-Control", "no-cache")
            .header("Host", "hq.sinajs.cn")
            .header("Pragma", "no-cache")
            .header("Proxy-Connection", "keep-alive")
            .header("Referer", "https://vip.stock.finance.sina.com.cn/")
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/97.0.4692.71 Safari/537.36")
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(anyhow!("Failed to fetch data: {}", response.status()));
        }

        let text = response.text().await?;
        self.parse_sina_realtime_data(&text, symbol)
    }

    // 获取多个期货合约数据
    pub async fn get_multiple_futures(&self, symbols: &[String]) -> Result<Vec<FuturesInfo>> {
        let formatted_symbols: Vec<String> = symbols.iter()
            .map(|s| self.format_symbol_for_realtime(s))
            .collect();
        
        let symbols_str = formatted_symbols.join(",");
        let rn_code = self.generate_random_code();
        // 注意：URL格式是 /rn= 而不是 ?rn=，这是新浪API的特殊格式
        let url = format!("{}/rn={}&list={}", SINA_FUTURES_REALTIME_API, rn_code, symbols_str);
        
        let response = self.client
            .get(&url)
            .header("Accept", "*/*")
            .header("Accept-Encoding", "gzip, deflate")
            .header("Accept-Language", "zh-CN,zh;q=0.9,en;q=0.8")
            .header("Cache-Control", "no-cache")
            .header("Host", "hq.sinajs.cn")
            .header("Pragma", "no-cache")
            .header("Proxy-Connection", "keep-alive")
            .header("Referer", "https://vip.stock.finance.sina.com.cn/")
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/97.0.4692.71 Safari/537.36")
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(anyhow!("Failed to fetch data: {}", response.status()));
        }

        let text = response.text().await?;
        self.parse_multiple_realtime_data(&text, symbols)
    }

    // 获取期货列表（通过新浪API获取品种数据）
    pub async fn list_main_futures(&self, query: &FuturesQuery) -> Result<Vec<FuturesInfo>> {
        match query.exchange.as_deref() {
            Some(exchange) => {
                let node = self.get_exchange_node(exchange);
                self.get_futures_by_node(&node, query.limit).await
            }
            None => {
                // 获取所有交易所的主力合约
                let mut all_futures = Vec::new();
                let exchanges = vec!["DCE", "CZCE", "SHFE", "CFFEX"];
                
                for exchange in exchanges {
                    let node = self.get_exchange_node(exchange);
                    match self.get_futures_by_node(&node, Some(5)).await {
                        Ok(mut futures) => all_futures.append(&mut futures),
                        Err(e) => log::warn!("Failed to get futures for {}: {}", exchange, e),
                    }
                }
                
                let limit = query.limit.unwrap_or(all_futures.len());
                all_futures.truncate(limit);
                Ok(all_futures)
            }
        }
    }

    // 通过新浪API获取指定品种的期货数据
    async fn get_futures_by_node(&self, node: &str, limit: Option<usize>) -> Result<Vec<FuturesInfo>> {
        let response = self.client
            .get(SINA_FUTURES_LIST_API)
            .query(&[
                ("page", "1"),
                ("sort", "position"),
                ("asc", "0"),
                ("node", node),
                ("base", "futures"),
            ])
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(anyhow!("Failed to fetch futures list: {}", response.status()));
        }

        let json_data: serde_json::Value = response.json().await?;
        let mut futures_list = Vec::new();

        if let Some(data_array) = json_data.as_array() {
            let limit = limit.unwrap_or(data_array.len());
            for item in data_array.iter().take(limit) {
                if let Ok(futures_info) = self.parse_sina_list_data(item) {
                    futures_list.push(futures_info);
                }
            }
        }

        Ok(futures_list)
    }

    // 获取支持的交易所列表
    pub fn get_exchanges(&self) -> Vec<FuturesExchange> {
        vec![
            FuturesExchange {
                code: "DCE".to_string(),
                name: "大连商品交易所".to_string(),
                description: "Dalian Commodity Exchange".to_string(),
            },
            FuturesExchange {
                code: "CZCE".to_string(),
                name: "郑州商品交易所".to_string(),
                description: "Zhengzhou Commodity Exchange".to_string(),
            },
            FuturesExchange {
                code: "SHFE".to_string(),
                name: "上海期货交易所".to_string(),
                description: "Shanghai Futures Exchange".to_string(),
            },
            FuturesExchange {
                code: "INE".to_string(),
                name: "上海国际能源交易中心".to_string(),
                description: "Shanghai International Energy Exchange".to_string(),
            },
            FuturesExchange {
                code: "CFFEX".to_string(),
                name: "中国金融期货交易所".to_string(),
                description: "China Financial Futures Exchange".to_string(),
            },
        ]
    }

    // 生成随机数（模拟新浪的rn参数）
    fn generate_random_code(&self) -> String {
        use std::time::{SystemTime, UNIX_EPOCH};
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        format!("{:x}", timestamp % 0x7FFFFFFF)
    }

    // 格式化期货合约代码为新浪实时数据格式
    // akshare使用小写nf_前缀，金融期货使用CFF_前缀
    fn format_symbol_for_realtime(&self, symbol: &str) -> String {
        let symbol_upper = symbol.to_uppercase();
        
        // 如果已经是新浪格式，直接返回（转为小写）
        if symbol_upper.starts_with("NF_") {
            return format!("nf_{}", &symbol_upper[3..]);
        }
        if symbol_upper.starts_with("CFF_") {
            return format!("CFF_{}", &symbol_upper[4..]);
        }
        
        // 根据合约代码判断交易所并添加前缀
        // 金融期货使用CFF_前缀，商品期货使用nf_前缀（小写）
        if self.is_cffex_symbol(&symbol_upper) {
            format!("CFF_{}", symbol_upper)
        } else {
            format!("nf_{}", symbol_upper)
        }
    }

    // 获取交易所对应的node参数
    fn get_exchange_node(&self, exchange: &str) -> String {
        match exchange.to_uppercase().as_str() {
            "DCE" => "dce_qh".to_string(),
            "CZCE" => "czce_qh".to_string(), 
            "SHFE" => "shfe_qh".to_string(),
            "CFFEX" => "cffex_qh".to_string(),
            "INE" => "ine_qh".to_string(),
            _ => "dce_qh".to_string(), // 默认大商所
        }
    }

    // 判断是否为中金所合约
    fn is_cffex_symbol(&self, symbol: &str) -> bool {
        let cffex_products = ["IF", "IC", "IH", "T", "TF", "TS"];
        cffex_products.iter().any(|&product| symbol.starts_with(product))
    }

    // 解析新浪期货实时数据
    // 根据akshare的实现，商品期货(CF)数据格式:
    // var hq_str_nf_V2309="PVC2309,09:00:00,6500,6520,6480,6490,6495,6500,6498,6499,6490,100,200,50000,100000,...";
    // 字段顺序: [0]名称,[1]时间,[2]开盘,[3]最高,[4]最低,[5]昨收,[6]买价,[7]卖价,[8]最新价,[9]均价,[10]昨结算,[11]买量,[12]卖量,[13]持仓,[14]成交量
    fn parse_sina_realtime_data(&self, data: &str, original_symbol: &str) -> Result<FuturesInfo> {
        // 检查数据是否为空
        if data.trim().is_empty() || data.contains(r#"="";") || data.contains(r#"="";"#) {
            return Err(anyhow!("Empty data returned from API"));
        }

        // 解析数据：var hq_str_nf_XXX="data1,data2,...";
        // 按分号分割多条数据，取第一条
        for item in data.split(';') {
            let item = item.trim();
            if item.is_empty() {
                continue;
            }
            
            // 分割等号，获取数据部分
            let parts: Vec<&str> = item.split('=').collect();
            if parts.len() < 2 {
                continue;
            }
            
            let data_part = parts[1].trim_matches('"').trim_matches('\'');
            if data_part.is_empty() {
                continue;
            }
            
            let fields: Vec<&str> = data_part.split(',').collect();
            
            if fields.len() < 15 {
                return Err(anyhow!("Insufficient data fields: got {}, expected at least 15. Data: {}", fields.len(), data_part));
            }

            // 按照akshare的字段顺序解析
            let name = fields[0].to_string();
            let open = fields[2].parse::<f64>().unwrap_or(0.0);
            let high = fields[3].parse::<f64>().unwrap_or(0.0);
            let low = fields[4].parse::<f64>().unwrap_or(0.0);
            let current_price = fields[8].parse::<f64>().unwrap_or(0.0);
            let prev_settlement = fields[10].parse::<f64>().unwrap_or(0.0);
            let open_interest = fields[13].parse::<u64>().ok();
            let volume = fields[14].parse::<u64>().unwrap_or(0);

            let change = current_price - prev_settlement;
            let change_percent = if prev_settlement != 0.0 {
                (change / prev_settlement) * 100.0
            } else {
                0.0
            };

            let beijing_time = get_beijing_time();

            return Ok(FuturesInfo {
                symbol: original_symbol.to_string(),
                name,
                current_price,
                change,
                change_percent,
                volume,
                open,
                high,
                low,
                settlement: None,
                prev_settlement: Some(prev_settlement),
                open_interest,
                updated_at: beijing_time.to_rfc3339(),
            });
        }
        
        Err(anyhow!("No valid data found in response: {}", data))
    }

    // 解析多个期货合约实时数据
    // akshare的解析方式：按分号分割，然后按等号分割获取数据
    fn parse_multiple_realtime_data(&self, data: &str, original_symbols: &[String]) -> Result<Vec<FuturesInfo>> {
        let mut results = Vec::new();
        
        // 按分号分割多条数据
        let items: Vec<&str> = data.split(';')
            .filter(|s| !s.trim().is_empty())
            .collect();
        
        for (i, item) in items.iter().enumerate() {
            if i < original_symbols.len() {
                match self.parse_sina_realtime_data(item, &original_symbols[i]) {
                    Ok(futures_info) => results.push(futures_info),
                    Err(e) => {
                        log::warn!("Failed to parse data for {}: {}", original_symbols[i], e);
                        continue;
                    }
                }
            }
        }
        
        Ok(results)
    }

    // 解析新浪期货列表数据
    fn parse_sina_list_data(&self, item: &serde_json::Value) -> Result<FuturesInfo> {
        let symbol = item["symbol"].as_str().unwrap_or("").to_string();
        let name = item["name"].as_str().unwrap_or("").to_string();
        let current_price = item["trade"].as_str().unwrap_or("0").parse::<f64>().unwrap_or(0.0);
        let prev_settlement = item["presettlement"].as_str().unwrap_or("0").parse::<f64>().unwrap_or(0.0);
        let open = item["open"].as_str().unwrap_or("0").parse::<f64>().unwrap_or(0.0);
        let high = item["high"].as_str().unwrap_or("0").parse::<f64>().unwrap_or(0.0);
        let low = item["low"].as_str().unwrap_or("0").parse::<f64>().unwrap_or(0.0);
        let volume = item["volume"].as_str().unwrap_or("0").parse::<u64>().unwrap_or(0);
        let open_interest = item["position"].as_str().unwrap_or("0").parse::<u64>().ok();
        let settlement = item["settlement"].as_str().unwrap_or("0").parse::<f64>().ok();

        let change = current_price - prev_settlement;
        let change_percent = if prev_settlement != 0.0 {
            (change / prev_settlement) * 100.0
        } else {
            0.0
        };

        let beijing_time = get_beijing_time();

        Ok(FuturesInfo {
            symbol,
            name,
            current_price,
            change,
            change_percent,
            volume,
            open,
            high,
            low,
            settlement,
            prev_settlement: Some(prev_settlement),
            open_interest,
            updated_at: beijing_time.to_rfc3339(),
        })
    }
}

// 获取期货历史数据（通过新浪API）
// 新浪提供分钟和日线数据接口
pub async fn get_futures_history(symbol: &str, query: &FuturesQuery) -> Result<Vec<FuturesHistoryData>> {
    let client = Client::new();
    let limit = query.limit.unwrap_or(30);
    
    // 新浪期货日线数据API
    let url = "https://stock2.finance.sina.com.cn/futures/api/jsonp.php/var%20_temp=/InnerFuturesNewService.getDailyKLine";
    
    let response = client
        .get(url)
        .query(&[("symbol", symbol)])
        .header("Referer", "https://finance.sina.com.cn/")
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(anyhow!("Failed to fetch history data: {}", response.status()));
    }

    let text = response.text().await?;
    parse_sina_history_data(&text, symbol, limit)
}

// 获取期货分钟数据
pub async fn get_futures_minute_data(symbol: &str, period: &str) -> Result<Vec<FuturesHistoryData>> {
    let client = Client::new();
    
    // period: "1", "5", "15", "30", "60" 分钟
    let url = "https://stock2.finance.sina.com.cn/futures/api/jsonp.php/=/InnerFuturesNewService.getFewMinLine";
    
    let response = client
        .get(url)
        .query(&[("symbol", symbol), ("type", period)])
        .header("Referer", "https://finance.sina.com.cn/")
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(anyhow!("Failed to fetch minute data: {}", response.status()));
    }

    let text = response.text().await?;
    parse_sina_minute_data(&text, symbol)
}

// 解析新浪期货日线历史数据
fn parse_sina_history_data(data: &str, symbol: &str, limit: usize) -> Result<Vec<FuturesHistoryData>> {
    // 数据格式: var _temp=([["2024-01-02","75000","75500","74800","75100","100000","50000","75050"],...]);
    let mut history = Vec::new();
    
    // 提取JSON数组部分
    let start = data.find("([");
    let end = data.rfind("])");
    
    if start.is_none() || end.is_none() {
        return Err(anyhow!("Invalid history data format"));
    }
    
    let json_str = &data[start.unwrap() + 1..end.unwrap() + 1];
    let json_data: serde_json::Value = serde_json::from_str(json_str)
        .map_err(|e| anyhow!("Failed to parse JSON: {}", e))?;
    
    if let Some(arr) = json_data.as_array() {
        for (i, item) in arr.iter().rev().take(limit).enumerate() {
            if let Some(fields) = item.as_array() {
                if fields.len() >= 8 {
                    history.push(FuturesHistoryData {
                        symbol: symbol.to_string(),
                        date: fields[0].as_str().unwrap_or("").to_string(),
                        open: fields[1].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                        high: fields[2].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                        low: fields[3].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                        close: fields[4].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                        volume: fields[5].as_str().unwrap_or("0").parse().unwrap_or(0),
                        open_interest: fields[6].as_str().unwrap_or("0").parse().ok(),
                        settlement: fields[7].as_str().unwrap_or("0").parse().ok(),
                    });
                }
            }
        }
    }
    
    // 按日期正序排列
    history.reverse();
    Ok(history)
}

// 解析新浪期货分钟数据
fn parse_sina_minute_data(data: &str, symbol: &str) -> Result<Vec<FuturesHistoryData>> {
    // 数据格式: =([["2024-01-02 09:00","75000","75500","74800","75100","100000","50000"],...]);
    let mut history = Vec::new();
    
    let start = data.find("([");
    let end = data.rfind("])");
    
    if start.is_none() || end.is_none() {
        return Err(anyhow!("Invalid minute data format"));
    }
    
    let json_str = &data[start.unwrap() + 1..end.unwrap() + 1];
    let json_data: serde_json::Value = serde_json::from_str(json_str)
        .map_err(|e| anyhow!("Failed to parse JSON: {}", e))?;
    
    if let Some(arr) = json_data.as_array() {
        for item in arr.iter() {
            if let Some(fields) = item.as_array() {
                if fields.len() >= 7 {
                    history.push(FuturesHistoryData {
                        symbol: symbol.to_string(),
                        date: fields[0].as_str().unwrap_or("").to_string(),
                        open: fields[1].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                        high: fields[2].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                        low: fields[3].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                        close: fields[4].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                        volume: fields[5].as_str().unwrap_or("0").parse().unwrap_or(0),
                        open_interest: fields[6].as_str().unwrap_or("0").parse().ok(),
                        settlement: None,
                    });
                }
            }
        }
    }
    
    Ok(history)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_symbol_for_realtime_commodity() {
        let service = FuturesService::new();
        
        // 测试商品期货合约代码格式化
        assert_eq!(service.format_symbol_for_realtime("CU2405"), "nf_CU2405");
        assert_eq!(service.format_symbol_for_realtime("AL2405"), "nf_AL2405");
        assert_eq!(service.format_symbol_for_realtime("RB2405"), "nf_RB2405");
        assert_eq!(service.format_symbol_for_realtime("V2309"), "nf_V2309");
    }

    #[test]
    fn test_format_symbol_for_realtime_financial() {
        let service = FuturesService::new();
        
        // 测试金融期货合约代码格式化
        assert_eq!(service.format_symbol_for_realtime("IF2401"), "CFF_IF2401");
        assert_eq!(service.format_symbol_for_realtime("IC2401"), "CFF_IC2401");
        assert_eq!(service.format_symbol_for_realtime("IH2401"), "CFF_IH2401");
        assert_eq!(service.format_symbol_for_realtime("T2406"), "CFF_T2406");
        assert_eq!(service.format_symbol_for_realtime("TF2406"), "CFF_TF2406");
    }

    #[test]
    fn test_format_symbol_already_formatted() {
        let service = FuturesService::new();
        
        // 测试已经格式化的合约代码
        assert_eq!(service.format_symbol_for_realtime("nf_CU2405"), "nf_CU2405");
        assert_eq!(service.format_symbol_for_realtime("NF_CU2405"), "nf_CU2405");
        assert_eq!(service.format_symbol_for_realtime("CFF_IF2401"), "CFF_IF2401");
    }

    #[test]
    fn test_is_cffex_symbol() {
        let service = FuturesService::new();
        
        // 测试金融期货品种判断
        assert!(service.is_cffex_symbol("IF2401"));
        assert!(service.is_cffex_symbol("IC2401"));
        assert!(service.is_cffex_symbol("IH2401"));
        assert!(service.is_cffex_symbol("T2406"));
        assert!(service.is_cffex_symbol("TF2406"));
        assert!(service.is_cffex_symbol("TS2406"));
        
        // 测试商品期货品种判断
        assert!(!service.is_cffex_symbol("CU2405"));
        assert!(!service.is_cffex_symbol("AL2405"));
        assert!(!service.is_cffex_symbol("RB2405"));
    }

    #[test]
    fn test_get_exchange_node() {
        let service = FuturesService::new();
        
        assert_eq!(service.get_exchange_node("DCE"), "dce_qh");
        assert_eq!(service.get_exchange_node("CZCE"), "czce_qh");
        assert_eq!(service.get_exchange_node("SHFE"), "shfe_qh");
        assert_eq!(service.get_exchange_node("CFFEX"), "cffex_qh");
        assert_eq!(service.get_exchange_node("INE"), "ine_qh");
        assert_eq!(service.get_exchange_node("dce"), "dce_qh"); // 测试小写
        assert_eq!(service.get_exchange_node("unknown"), "dce_qh"); // 测试未知交易所
    }

    #[test]
    fn test_generate_random_code() {
        let service = FuturesService::new();
        
        let code1 = service.generate_random_code();
        let code2 = service.generate_random_code();
        
        // 验证生成的是十六进制字符串
        assert!(code1.chars().all(|c| c.is_ascii_hexdigit()));
        assert!(code2.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_get_exchanges() {
        let service = FuturesService::new();
        let exchanges = service.get_exchanges();
        
        assert_eq!(exchanges.len(), 5);
        
        let codes: Vec<&str> = exchanges.iter().map(|e| e.code.as_str()).collect();
        assert!(codes.contains(&"DCE"));
        assert!(codes.contains(&"CZCE"));
        assert!(codes.contains(&"SHFE"));
        assert!(codes.contains(&"INE"));
        assert!(codes.contains(&"CFFEX"));
    }

    #[test]
    fn test_parse_sina_realtime_data_valid() {
        let service = FuturesService::new();
        
        // 模拟新浪API返回的数据格式
        let mock_data = r#"var hq_str_nf_CU2405="铜2405,09:00:00,75000,75500,74800,74900,75100,75200,75150,75100,74950,100,200,50000,100000,0,0,0,0,0,0,0,0,0,0,0,0,0";"#;
        
        let result = service.parse_sina_realtime_data(mock_data, "CU2405");
        assert!(result.is_ok());
        
        let info = result.unwrap();
        assert_eq!(info.symbol, "CU2405");
        assert_eq!(info.name, "铜2405");
        assert_eq!(info.open, 75000.0);
        assert_eq!(info.high, 75500.0);
        assert_eq!(info.low, 74800.0);
        assert_eq!(info.current_price, 75150.0);
        assert_eq!(info.prev_settlement, Some(74950.0));
        assert_eq!(info.volume, 100000);
        assert_eq!(info.open_interest, Some(50000));
    }

    #[test]
    fn test_parse_sina_realtime_data_empty() {
        let service = FuturesService::new();
        
        // 测试空数据
        let empty_data = r#"var hq_str_nf_CU2405="";"#;
        let result = service.parse_sina_realtime_data(empty_data, "CU2405");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_sina_realtime_data_insufficient_fields() {
        let service = FuturesService::new();
        
        // 测试字段不足的数据
        let insufficient_data = r#"var hq_str_nf_CU2405="铜2405,09:00:00,75000";"#;
        let result = service.parse_sina_realtime_data(insufficient_data, "CU2405");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_multiple_realtime_data() {
        let service = FuturesService::new();
        
        // 模拟多个合约的数据
        let mock_data = r#"var hq_str_nf_CU2405="铜2405,09:00:00,75000,75500,74800,74900,75100,75200,75150,75100,74950,100,200,50000,100000,0,0,0,0,0,0,0,0,0,0,0,0,0";var hq_str_nf_AL2405="铝2405,09:00:00,19000,19200,18900,18950,19050,19100,19080,19050,18980,50,100,30000,80000,0,0,0,0,0,0,0,0,0,0,0,0,0";"#;
        
        let symbols = vec!["CU2405".to_string(), "AL2405".to_string()];
        let result = service.parse_multiple_realtime_data(mock_data, &symbols);
        assert!(result.is_ok());
        
        let infos = result.unwrap();
        assert_eq!(infos.len(), 2);
        assert_eq!(infos[0].symbol, "CU2405");
        assert_eq!(infos[1].symbol, "AL2405");
    }

    #[test]
    fn test_parse_sina_list_data() {
        let service = FuturesService::new();
        
        // 模拟新浪期货列表API返回的JSON数据
        let mock_json = serde_json::json!({
            "symbol": "CU2405",
            "name": "铜2405",
            "trade": "75150",
            "presettlement": "74950",
            "open": "75000",
            "high": "75500",
            "low": "74800",
            "volume": "100000",
            "position": "50000",
            "settlement": "75100"
        });
        
        let result = service.parse_sina_list_data(&mock_json);
        assert!(result.is_ok());
        
        let info = result.unwrap();
        assert_eq!(info.symbol, "CU2405");
        assert_eq!(info.name, "铜2405");
        assert_eq!(info.current_price, 75150.0);
        assert_eq!(info.prev_settlement, Some(74950.0));
        assert_eq!(info.open, 75000.0);
        assert_eq!(info.high, 75500.0);
        assert_eq!(info.low, 74800.0);
        assert_eq!(info.volume, 100000);
        assert_eq!(info.open_interest, Some(50000));
        assert_eq!(info.settlement, Some(75100.0));
    }

    #[test]
    fn test_parse_sina_history_data() {
        // 模拟新浪历史数据API返回格式
        let mock_data = r#"var _temp=([["2024-01-02","75000","75500","74800","75100","100000","50000","75050"],["2024-01-03","75100","75600","74900","75200","110000","51000","75150"]]);"#;
        
        let result = parse_sina_history_data(mock_data, "CU2405", 10);
        assert!(result.is_ok());
        
        let history = result.unwrap();
        assert_eq!(history.len(), 2);
        assert_eq!(history[0].date, "2024-01-02");
        assert_eq!(history[0].open, 75000.0);
        assert_eq!(history[0].high, 75500.0);
        assert_eq!(history[0].low, 74800.0);
        assert_eq!(history[0].close, 75100.0);
        assert_eq!(history[0].volume, 100000);
    }

    #[test]
    fn test_parse_sina_minute_data() {
        // 模拟新浪分钟数据API返回格式
        let mock_data = r#"=([["2024-01-02 09:00","75000","75100","74950","75050","10000","50000"],["2024-01-02 09:01","75050","75150","75000","75100","8000","50100"]]);"#;
        
        let result = parse_sina_minute_data(mock_data, "CU2405");
        assert!(result.is_ok());
        
        let history = result.unwrap();
        assert_eq!(history.len(), 2);
        assert_eq!(history[0].date, "2024-01-02 09:00");
        assert_eq!(history[0].open, 75000.0);
    }

    #[test]
    fn test_get_beijing_time() {
        let beijing_time = get_beijing_time();
        // 验证返回的是有效的时间
        assert!(beijing_time.timestamp() > 0);
    }
}