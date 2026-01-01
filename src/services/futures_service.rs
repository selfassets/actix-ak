use anyhow::{Result, anyhow};
use chrono::Utc;
use reqwest::Client;
use crate::models::{FuturesInfo, FuturesHistoryData, FuturesQuery, FuturesExchange};

const SINA_FUTURES_API: &str = "https://hq.sinajs.cn/list=";

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
        let formatted_symbol = self.format_symbol(symbol);
        let url = format!("{}{}", SINA_FUTURES_API, formatted_symbol);
        
        let response = self.client
            .get(&url)
            .header("Referer", "https://finance.sina.com.cn/")
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(anyhow!("Failed to fetch data: {}", response.status()));
        }

        let text = response.text().await?;
        self.parse_sina_futures_data(&text, symbol)
    }

    // 获取多个期货合约数据
    pub async fn get_multiple_futures(&self, symbols: &[String]) -> Result<Vec<FuturesInfo>> {
        let formatted_symbols: Vec<String> = symbols.iter()
            .map(|s| self.format_symbol(s))
            .collect();
        
        let symbols_str = formatted_symbols.join(",");
        let url = format!("{}{}", SINA_FUTURES_API, symbols_str);
        
        let response = self.client
            .get(&url)
            .header("Referer", "https://finance.sina.com.cn/")
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(anyhow!("Failed to fetch data: {}", response.status()));
        }

        let text = response.text().await?;
        self.parse_multiple_sina_futures_data(&text, symbols)
    }

    // 获取期货列表（主要合约）
    pub async fn list_main_futures(&self, query: &FuturesQuery) -> Result<Vec<FuturesInfo>> {
        let main_contracts = self.get_main_contracts(query.exchange.as_deref());
        let limit = query.limit.unwrap_or(main_contracts.len());
        let symbols: Vec<String> = main_contracts.into_iter().take(limit).collect();
        
        self.get_multiple_futures(&symbols).await
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

    // 格式化期货合约代码为新浪格式
    fn format_symbol(&self, symbol: &str) -> String {
        let symbol_upper = symbol.to_uppercase();
        
        // 如果已经是新浪格式，直接返回
        if symbol_upper.starts_with("NF_") || symbol_upper.starts_with("CFF_") {
            return symbol_upper;
        }
        
        // 根据合约代码判断交易所并添加前缀
        if self.is_dce_symbol(&symbol_upper) {
            format!("NF_{}", symbol_upper)
        } else if self.is_czce_symbol(&symbol_upper) {
            format!("NF_{}", symbol_upper)
        } else if self.is_shfe_symbol(&symbol_upper) {
            format!("NF_{}", symbol_upper)
        } else if self.is_ine_symbol(&symbol_upper) {
            format!("NF_{}", symbol_upper)
        } else if self.is_cffex_symbol(&symbol_upper) {
            format!("CFF_{}", symbol_upper)
        } else {
            // 默认使用NF前缀
            format!("NF_{}", symbol_upper)
        }
    }

    // 判断是否为大商所合约
    fn is_dce_symbol(&self, symbol: &str) -> bool {
        let dce_products = ["A", "B", "C", "CS", "M", "Y", "P", "FB", "BB", "JD", "L", "V", "PP", "J", "JM", "I", "EG", "EB", "PG", "LH", "RR"];
        dce_products.iter().any(|&product| symbol.starts_with(product))
    }

    // 判断是否为郑商所合约
    fn is_czce_symbol(&self, symbol: &str) -> bool {
        let czce_products = ["WH", "PM", "CF", "SR", "TA", "OI", "RI", "RS", "RM", "JR", "LR", "AP", "CJ", "UR", "SA", "FG", "MA", "ZC", "SF", "SM", "PF"];
        czce_products.iter().any(|&product| symbol.starts_with(product))
    }

    // 判断是否为上期所合约
    fn is_shfe_symbol(&self, symbol: &str) -> bool {
        let shfe_products = ["CU", "AL", "ZN", "PB", "NI", "SN", "AU", "AG", "RB", "WR", "HC", "FU", "BU", "RU", "SP", "SS"];
        shfe_products.iter().any(|&product| symbol.starts_with(product))
    }

    // 判断是否为能源中心合约
    fn is_ine_symbol(&self, symbol: &str) -> bool {
        let ine_products = ["SC", "NR", "LU", "BC"];
        ine_products.iter().any(|&product| symbol.starts_with(product))
    }

    // 判断是否为中金所合约
    fn is_cffex_symbol(&self, symbol: &str) -> bool {
        let cffex_products = ["IF", "IC", "IH", "T", "TF", "TS"];
        cffex_products.iter().any(|&product| symbol.starts_with(product))
    }

    // 获取主力合约列表
    fn get_main_contracts(&self, exchange: Option<&str>) -> Vec<String> {
        let mut contracts = Vec::new();
        
        match exchange {
            Some("DCE") => {
                contracts.extend(vec!["A2405", "M2405", "Y2405", "C2405", "CS2405", "P2405", "L2405", "V2405", "PP2405", "J2405", "JM2405", "I2405"]);
            },
            Some("CZCE") => {
                contracts.extend(vec!["WH405", "CF405", "SR405", "TA405", "OI405", "RM405", "MA405", "ZC405", "FG405", "AP405"]);
            },
            Some("SHFE") => {
                contracts.extend(vec!["CU2405", "AL2405", "ZN2405", "PB2405", "NI2405", "AU2406", "AG2406", "RB2405", "HC2405", "RU2405"]);
            },
            Some("INE") => {
                contracts.extend(vec!["SC2405", "NR2405", "LU2405"]);
            },
            Some("CFFEX") => {
                contracts.extend(vec!["IF2404", "IC2404", "IH2404", "T2406", "TF2406"]);
            },
            _ => {
                // 返回所有主力合约
                contracts.extend(vec![
                    "A2405", "M2405", "Y2405", "C2405", "L2405", "V2405", "PP2405", "J2405", "JM2405", "I2405",
                    "WH405", "CF405", "SR405", "TA405", "OI405", "RM405", "MA405", "ZC405", "FG405",
                    "CU2405", "AL2405", "ZN2405", "PB2405", "NI2405", "AU2406", "AG2406", "RB2405", "HC2405", "RU2405",
                    "SC2405", "NR2405", "LU2405",
                    "IF2404", "IC2404", "IH2404", "T2406", "TF2406"
                ]);
            }
        }
        
        contracts.into_iter().map(|s| s.to_string()).collect()
    }

    // 解析新浪期货数据
    fn parse_sina_futures_data(&self, data: &str, original_symbol: &str) -> Result<FuturesInfo> {
        // 新浪期货数据格式: var hq_str_NF_CU2405="铜2405,62970,62830,63200,63200,62830,62970,62980,446224,28089671840,62970,1,62960,2,62950,1,62980,1,62990,1,63000,1,63010,1,2024-03-15,15:00:00,00";
        
        if !data.contains("=") {
            return Err(anyhow!("Invalid data format"));
        }

        let parts: Vec<&str> = data.split('=').collect();
        if parts.len() < 2 {
            return Err(anyhow!("Invalid data format"));
        }

        let data_part = parts[1].trim_matches('"').trim_matches(';');
        let fields: Vec<&str> = data_part.split(',').collect();
        
        if fields.len() < 20 {
            return Err(anyhow!("Insufficient data fields"));
        }

        let current_price = fields[1].parse::<f64>().unwrap_or(0.0);
        let prev_settlement = fields[2].parse::<f64>().unwrap_or(0.0);
        let open = fields[3].parse::<f64>().unwrap_or(0.0);
        let high = fields[4].parse::<f64>().unwrap_or(0.0);
        let low = fields[5].parse::<f64>().unwrap_or(0.0);
        let settlement = fields[6].parse::<f64>().ok();
        let volume = fields[7].parse::<u64>().unwrap_or(0);
        let open_interest = fields[8].parse::<u64>().ok();

        let change = current_price - prev_settlement;
        let change_percent = if prev_settlement != 0.0 {
            (change / prev_settlement) * 100.0
        } else {
            0.0
        };

        Ok(FuturesInfo {
            symbol: original_symbol.to_string(),
            name: fields[0].to_string(),
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
            updated_at: Utc::now(),
        })
    }

    // 解析多个期货合约数据
    fn parse_multiple_sina_futures_data(&self, data: &str, original_symbols: &[String]) -> Result<Vec<FuturesInfo>> {
        let mut results = Vec::new();
        let lines: Vec<&str> = data.lines().collect();
        
        for (i, line) in lines.iter().enumerate() {
            if i < original_symbols.len() {
                match self.parse_sina_futures_data(line, &original_symbols[i]) {
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