use anyhow::{Result, anyhow};
use chrono::Utc;
use chrono_tz::Asia::Shanghai;
use reqwest::Client;
use regex::Regex;
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
        let url = format!("{}?rn={}&list={}", SINA_FUTURES_REALTIME_API, rn_code, formatted_symbol);
        
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
        let url = format!("{}?rn={}&list={}", SINA_FUTURES_REALTIME_API, rn_code, symbols_str);
        
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
    fn format_symbol_for_realtime(&self, symbol: &str) -> String {
        let symbol_upper = symbol.to_uppercase();
        
        // 如果已经是新浪格式，直接返回
        if symbol_upper.starts_with("NF_") || symbol_upper.starts_with("CFF_") {
            return symbol_upper;
        }
        
        // 根据合约代码判断交易所并添加前缀
        if self.is_cffex_symbol(&symbol_upper) {
            format!("CFF_{}", symbol_upper)
        } else {
            format!("NF_{}", symbol_upper)
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
    // 数据格式: var hq_str_nf_CU2405="铜2405,62970,62830,63200,63200,62830,62970,62980,446224,28089671840,62970,1,62960,2,62950,1,62980,1,62990,1,63000,1,63010,1,2024-03-15,15:00:00,00";
    // 字段说明: [0]名称,[1]昨结算,[2]开盘,[3]最高,[4]最低,[5]昨收,[6]买价,[7]卖价,[8]最新价,[9]结算价,[10]昨结算,[11]买量,[12]卖量,[13]持仓,[14]成交量
    fn parse_sina_realtime_data(&self, data: &str, original_symbol: &str) -> Result<FuturesInfo> {
        // 检查数据是否为空
        if data.trim().is_empty() || data.contains("\"\"") {
            return Err(anyhow!("Empty data returned from API"));
        }

        let re = Regex::new(r#"var hq_str_[^=]+=["']([^"']+)["']"#).unwrap();
        let caps = re.captures(data)
            .ok_or_else(|| anyhow!("Invalid data format: no match found in: {}", data))?;
        
        let data_part = caps.get(1)
            .ok_or_else(|| anyhow!("Invalid data format: no data part"))?
            .as_str();
        
        let fields: Vec<&str> = data_part.split(',').collect();
        
        if fields.len() < 15 {
            return Err(anyhow!("Insufficient data fields: got {}, expected at least 15. Data: {}", fields.len(), data_part));
        }

        let name = fields[0].to_string();
        let open = fields[2].parse::<f64>().unwrap_or(0.0);
        let high = fields[3].parse::<f64>().unwrap_or(0.0);
        let low = fields[4].parse::<f64>().unwrap_or(0.0);
        let current_price = fields[8].parse::<f64>().unwrap_or(0.0);
        let settlement = fields[9].parse::<f64>().ok();
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

        Ok(FuturesInfo {
            symbol: original_symbol.to_string(),
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

    // 解析多个期货合约实时数据
    fn parse_multiple_realtime_data(&self, data: &str, original_symbols: &[String]) -> Result<Vec<FuturesInfo>> {
        let mut results = Vec::new();
        let lines: Vec<&str> = data.lines().collect();
        
        for (i, line) in lines.iter().enumerate() {
            if i < original_symbols.len() && !line.trim().is_empty() {
                match self.parse_sina_realtime_data(line, &original_symbols[i]) {
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

// 模拟历史数据获取（新浪不直接提供历史数据API）
pub async fn get_futures_history(symbol: &str, query: &FuturesQuery) -> Result<Vec<FuturesHistoryData>> {
    // 这里可以集成其他提供历史数据的API，如Wind、同花顺等
    // 目前返回模拟数据
    let mut history = Vec::new();
    let limit = query.limit.unwrap_or(30);
    
    for i in 0..limit {
        let base_price = 60000.0;
        let variation = (i as f64 * 50.0) - 1500.0;
        
        history.push(FuturesHistoryData {
            symbol: symbol.to_string(),
            date: format!("2024-03-{:02}", i + 1),
            open: base_price + variation,
            high: base_price + variation + 200.0,
            low: base_price + variation - 150.0,
            close: base_price + variation + 50.0,
            volume: 100_000 + (i as u64 * 5_000),
            settlement: Some(base_price + variation + 25.0),
            open_interest: Some(500_000 + (i as u64 * 1_000)),
        });
    }
    
    Ok(history)
}