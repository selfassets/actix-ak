use anyhow::{Result, anyhow};
use chrono::Utc;
use chrono_tz::Asia::Shanghai;
use regex::Regex;
use reqwest::Client;
use std::collections::HashMap;
use crate::models::{
    FuturesInfo, FuturesHistoryData, FuturesQuery, FuturesExchange,
    FuturesSymbolMark, FuturesContractDetail, ForeignFuturesSymbol,
    FuturesMainContract, FuturesMainDailyData, FuturesHoldPosition,
    ForeignFuturesHistData, ForeignFuturesDetail, ForeignFuturesDetailItem,
    FuturesFeesInfo
};

// 获取北京时间字符串（带+08:00时区）
fn get_beijing_time() -> String {
    Utc::now().with_timezone(&Shanghai).to_rfc3339()
}

// 新浪期货API常量
const SINA_FUTURES_REALTIME_API: &str = "https://hq.sinajs.cn";
const SINA_FUTURES_LIST_API: &str = "https://vip.stock.finance.sina.com.cn/quotes_service/api/json_v2.php/Market_Center.getHQFuturesData";
const SINA_FUTURES_SYMBOL_URL: &str = "https://vip.stock.finance.sina.com.cn/quotes_service/view/js/qihuohangqing.js";
const SINA_FUTURES_DAILY_API: &str = "https://stock2.finance.sina.com.cn/futures/api/jsonp.php/var%20_temp=/InnerFuturesNewService.getDailyKLine";
const SINA_FUTURES_MINUTE_API: &str = "https://stock2.finance.sina.com.cn/futures/api/jsonp.php/=/InnerFuturesNewService.getFewMinLine";
const SINA_CONTRACT_DETAIL_URL: &str = "https://finance.sina.com.cn/futures/quotes";

/// 期货数据服务
/// 参考 akshare/futures/futures_zh_sina.py 实现
pub struct FuturesService {
    client: Client,
    // 缓存品种映射数据
    symbol_mark_cache: Option<Vec<FuturesSymbolMark>>,
}

impl FuturesService {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            symbol_mark_cache: None,
        }
    }

    // ==================== 品种映射相关 ====================

    /// 获取期货品种和代码映射表
    /// 对应 akshare 的 futures_symbol_mark() 函数
    /// 从新浪JS文件动态解析品种信息
    pub async fn get_symbol_mark(&mut self) -> Result<Vec<FuturesSymbolMark>> {
        // 如果有缓存，直接返回
        if let Some(ref cache) = self.symbol_mark_cache {
            return Ok(cache.clone());
        }

        println!("📡 请求品种映射数据 URL: {}", SINA_FUTURES_SYMBOL_URL);
        
        let response = self.client
            .get(SINA_FUTURES_SYMBOL_URL)
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(anyhow!("获取品种映射失败: {}", response.status()));
        }

        // 使用 GBK 编码读取（兼容 GB2312）
        let bytes = response.bytes().await?;
        let text = encoding_rs::GBK.decode(&bytes).0.to_string();
        
        // 解析JS中的品种数据
        let symbols = self.parse_symbol_mark_js(&text)?;
        
        // 缓存结果
        self.symbol_mark_cache = Some(symbols.clone());
        
        Ok(symbols)
    }

    /// 解析新浪JS文件中的品种映射数据
    /// JS格式: ARRFUTURESNODES = { czce: ['郑州商品交易所', ['PTA', 'pta_qh', '16'], ...], ... }
    fn parse_symbol_mark_js(&self, js_text: &str) -> Result<Vec<FuturesSymbolMark>> {
        let mut symbols = Vec::new();
        
        // 查找 ARRFUTURESNODES 对象
        let start = js_text.find("ARRFUTURESNODES = {");
        let end = js_text.find("};");
        
        if start.is_none() || end.is_none() {
            return Err(anyhow!("无法解析品种映射JS数据"));
        }
        
        let content = &js_text[start.unwrap()..end.unwrap() + 2];
        
        // 解析各交易所数据
        let exchanges = vec![
            ("czce", "郑州商品交易所"),
            ("dce", "大连商品交易所"),
            ("shfe", "上海期货交易所"),
            ("cffex", "中国金融期货交易所"),
            ("gfex", "广州期货交易所"),
        ];
        
        for (exchange_code, exchange_name) in exchanges {
            // 查找交易所数据块
            let pattern = format!(r"{}\s*:\s*\[", exchange_code);
            let re = Regex::new(&pattern).unwrap();
            
            if let Some(m) = re.find(content) {
                let start_pos = m.end();
                // 找到对应的结束位置
                let remaining = &content[start_pos..];
                
                // 解析品种数组 ['品种名', 'node', '数字']
                let item_re = Regex::new(r"\['([^']+)',\s*'([^']+)',\s*'[^']*'").unwrap();
                
                for cap in item_re.captures_iter(remaining) {
                    let symbol_name = cap.get(1).map(|m| m.as_str()).unwrap_or("");
                    let mark = cap.get(2).map(|m| m.as_str()).unwrap_or("");
                    
                    if !symbol_name.is_empty() && !mark.is_empty() && mark.ends_with("_qh") {
                        symbols.push(FuturesSymbolMark {
                            exchange: exchange_name.to_string(),
                            symbol: symbol_name.to_string(),
                            mark: mark.to_string(),
                        });
                    }
                }
            }
        }
        
        println!("📊 解析到 {} 个品种映射", symbols.len());
        Ok(symbols)
    }

    /// 根据品种名称获取对应的node参数
    pub async fn get_symbol_node(&mut self, symbol: &str) -> Result<String> {
        let symbols = self.get_symbol_mark().await?;
        
        for s in &symbols {
            if s.symbol == symbol {
                return Ok(s.mark.clone());
            }
        }
        
        Err(anyhow!("未找到品种 {} 的映射", symbol))
    }

    /// 获取指定交易所的所有品种
    pub async fn get_exchange_symbols(&mut self, exchange: &str) -> Result<Vec<FuturesSymbolMark>> {
        let symbols = self.get_symbol_mark().await?;
        
        let exchange_name = match exchange.to_uppercase().as_str() {
            "CZCE" => "郑州商品交易所",
            "DCE" => "大连商品交易所",
            "SHFE" => "上海期货交易所",
            "CFFEX" => "中国金融期货交易所",
            "GFEX" => "广州期货交易所",
            "INE" => "上海期货交易所", // INE归属上期所
            _ => return Err(anyhow!("未知交易所: {}", exchange)),
        };
        
        Ok(symbols.into_iter()
            .filter(|s| s.exchange == exchange_name)
            .collect())
    }


    // ==================== 实时行情相关 ====================

    /// 获取单个期货合约实时数据
    /// 对应 akshare 的 futures_zh_spot() 函数
    pub async fn get_futures_info(&self, symbol: &str) -> Result<FuturesInfo> {
        let formatted_symbol = self.format_symbol_for_realtime(symbol);
        let rn_code = self.generate_random_code();
        let url = format!("{}/rn={}&list={}", SINA_FUTURES_REALTIME_API, rn_code, formatted_symbol);
        
        println!("📡 请求实时行情 URL: {}", url);
        
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
            return Err(anyhow!("获取数据失败: {}", response.status()));
        }

        let text = response.text().await?;
        self.parse_sina_realtime_data(&text, symbol)
    }

    /// 获取多个期货合约实时数据
    /// 对应 akshare 的 futures_zh_spot() 支持多合约
    pub async fn get_multiple_futures(&self, symbols: &[String]) -> Result<Vec<FuturesInfo>> {
        let formatted_symbols: Vec<String> = symbols.iter()
            .map(|s| self.format_symbol_for_realtime(s))
            .collect();
        
        let symbols_str = formatted_symbols.join(",");
        let rn_code = self.generate_random_code();
        let url = format!("{}/rn={}&list={}", SINA_FUTURES_REALTIME_API, rn_code, symbols_str);
        
        println!("📡 请求批量实时行情 URL: {}", url);
        
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
            return Err(anyhow!("获取数据失败: {}", response.status()));
        }

        let text = response.text().await?;
        self.parse_multiple_realtime_data(&text, symbols)
    }

    /// 获取品种所有合约实时数据
    /// 对应 akshare 的 futures_zh_realtime() 函数
    pub async fn get_futures_realtime_by_symbol(&mut self, symbol: &str) -> Result<Vec<FuturesInfo>> {
        let node = self.get_symbol_node(symbol).await?;
        self.get_futures_by_node(&node, None).await
    }

    /// 获取期货列表（按交易所或品种）
    pub async fn list_main_futures(&mut self, query: &FuturesQuery) -> Result<Vec<FuturesInfo>> {
        match query.exchange.as_deref() {
            Some(exchange) => {
                // 获取该交易所的所有品种
                let exchange_symbols = self.get_exchange_symbols(exchange).await?;
                let mut all_futures = Vec::new();
                let limit = query.limit.unwrap_or(20);
                
                // 遍历品种获取数据
                for symbol_mark in exchange_symbols.iter().take(5) {
                    match self.get_futures_by_node(&symbol_mark.mark, Some(1)).await {
                        Ok(mut futures) => all_futures.append(&mut futures),
                        Err(e) => log::warn!("获取品种 {} 数据失败: {}", symbol_mark.symbol, e),
                    }
                    if all_futures.len() >= limit {
                        break;
                    }
                }
                
                // 按持仓量排序
                all_futures.sort_by(|a, b| b.open_interest.cmp(&a.open_interest));
                all_futures.truncate(limit);
                Ok(all_futures)
            }
            None => {
                // 获取所有交易所的主力合约
                let mut all_futures = Vec::new();
                let exchanges = vec!["SHFE", "DCE", "CZCE", "CFFEX"];
                
                for exchange in exchanges {
                    if let Ok(symbols) = self.get_exchange_symbols(exchange).await {
                        for symbol_mark in symbols.iter().take(2) {
                            if let Ok(mut futures) = self.get_futures_by_node(&symbol_mark.mark, Some(1)).await {
                                all_futures.append(&mut futures);
                            }
                        }
                    }
                }
                
                let limit = query.limit.unwrap_or(all_futures.len());
                all_futures.truncate(limit);
                Ok(all_futures)
            }
        }
    }

    /// 通过node参数获取期货数据
    /// 对应 akshare 的 futures_zh_realtime_v1() 函数
    pub async fn get_futures_by_node(&self, node: &str, limit: Option<usize>) -> Result<Vec<FuturesInfo>> {
        let full_url = format!("{}?page=1&sort=position&asc=0&node={}&base=futures", 
            SINA_FUTURES_LIST_API, node);
        println!("📡 请求期货列表 URL: {}", full_url);
        
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
            return Err(anyhow!("获取期货列表失败: {}", response.status()));
        }

        let text = response.text().await?;
        println!("📥 原始响应数据: {}", &text[..std::cmp::min(300, text.len())]);
        
        let json_data: serde_json::Value = serde_json::from_str(&text)
            .map_err(|e| anyhow!("解析JSON失败: {}", e))?;
        
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


    // ==================== 主力合约相关 ====================

    /// 获取交易所主力合约列表
    /// 对应 akshare 的 match_main_contract() 函数
    pub async fn get_main_contracts(&mut self, exchange: &str) -> Result<Vec<String>> {
        let exchange_symbols = self.get_exchange_symbols(exchange).await?;
        let mut main_contracts = Vec::new();
        
        for symbol_mark in &exchange_symbols {
            // 获取该品种的所有合约
            match self.get_futures_by_node(&symbol_mark.mark, Some(5)).await {
                Ok(futures) => {
                    if futures.len() > 0 {
                        // 找出持仓量最大的合约作为主力合约
                        if let Some(main) = futures.iter()
                            .max_by_key(|f| f.open_interest.unwrap_or(0)) {
                            main_contracts.push(main.symbol.clone());
                            println!("  {} 主力合约: {}", symbol_mark.symbol, main.symbol);
                        }
                    }
                }
                Err(e) => {
                    log::warn!("获取 {} 合约失败: {}", symbol_mark.symbol, e);
                }
            }
        }
        
        Ok(main_contracts)
    }

    // ==================== K线数据相关 ====================

    /// 获取期货合约详情
    /// 对应 akshare 的 futures_contract_detail() 函数
    pub async fn get_contract_detail(&self, symbol: &str) -> Result<FuturesContractDetail> {
        let url = format!("{}/{}.shtml", SINA_CONTRACT_DETAIL_URL, symbol);
        println!("📡 请求合约详情 URL: {}", url);
        
        let response = self.client
            .get(&url)
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(anyhow!("获取合约详情失败: {}", response.status()));
        }

        // 使用 GBK 编码读取（兼容 GB2312）
        let bytes = response.bytes().await?;
        let text = encoding_rs::GBK.decode(&bytes).0.to_string();
        
        self.parse_contract_detail(&text, symbol)
    }

    /// 解析合约详情HTML
    fn parse_contract_detail(&self, html: &str, symbol: &str) -> Result<FuturesContractDetail> {
        // 简化解析，提取关键信息
        let extract_value = |pattern: &str| -> String {
            let re = Regex::new(pattern).ok();
            re.and_then(|r| r.captures(html))
                .and_then(|c| c.get(1))
                .map(|m| m.as_str().trim().to_string())
                .unwrap_or_default()
        };

        Ok(FuturesContractDetail {
            symbol: symbol.to_string(),
            name: extract_value(r"<title>([^<]+)</title>"),
            exchange: extract_value(r"上市交易所[：:]\s*([^<\n]+)"),
            trading_unit: extract_value(r"交易单位[：:]\s*([^<\n]+)"),
            quote_unit: extract_value(r"报价单位[：:]\s*([^<\n]+)"),
            min_price_change: extract_value(r"最小变动价位[：:]\s*([^<\n]+)"),
            price_limit: extract_value(r"涨跌停板幅度[：:]\s*([^<\n]+)"),
            contract_months: extract_value(r"合约交割月份[：:]\s*([^<\n]+)"),
            trading_hours: extract_value(r"交易时间[：:]\s*([^<\n]+)"),
            last_trading_day: extract_value(r"最后交易日[：:]\s*([^<\n]+)"),
            last_delivery_day: extract_value(r"最后交割日[：:]\s*([^<\n]+)"),
            delivery_grade: extract_value(r"交割品级[：:]\s*([^<\n]+)"),
            margin: extract_value(r"最低交易保证金[：:]\s*([^<\n]+)"),
            delivery_method: extract_value(r"交割方式[：:]\s*([^<\n]+)"),
        })
    }

    /// 获取支持的交易所列表
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
            FuturesExchange {
                code: "GFEX".to_string(),
                name: "广州期货交易所".to_string(),
                description: "Guangzhou Futures Exchange".to_string(),
            },
        ]
    }


    // ==================== 辅助函数 ====================

    /// 生成随机数（模拟新浪的rn参数）
    fn generate_random_code(&self) -> String {
        use std::time::{SystemTime, UNIX_EPOCH};
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        format!("{:x}", timestamp % 0x7FFFFFFF)
    }

    /// 格式化期货合约代码为新浪实时数据格式
    /// 商品期货使用小写 nf_ 前缀，金融期货使用 CFF_ 前缀
    fn format_symbol_for_realtime(&self, symbol: &str) -> String {
        let symbol_upper = symbol.to_uppercase();
        
        // 如果已经是新浪格式，直接返回
        if symbol_upper.starts_with("NF_") {
            return format!("nf_{}", &symbol_upper[3..]);
        }
        if symbol_upper.starts_with("CFF_") {
            return format!("CFF_{}", &symbol_upper[4..]);
        }
        
        // 根据合约代码判断交易所并添加前缀
        if self.is_cffex_symbol(&symbol_upper) {
            format!("CFF_{}", symbol_upper)
        } else {
            format!("nf_{}", symbol_upper)
        }
    }

    /// 判断是否为中金所合约
    fn is_cffex_symbol(&self, symbol: &str) -> bool {
        let cffex_products = ["IF", "IC", "IH", "IM", "T", "TF", "TS", "TL"];
        cffex_products.iter().any(|&product| symbol.starts_with(product))
    }

    /// 解析新浪期货实时数据
    fn parse_sina_realtime_data(&self, data: &str, original_symbol: &str) -> Result<FuturesInfo> {
        if data.trim().is_empty() || data.contains(r#"="";") || data.contains(r#"="";"#) {
            return Err(anyhow!("API返回空数据"));
        }

        for item in data.split(';') {
            let item = item.trim();
            if item.is_empty() {
                continue;
            }
            
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
                return Err(anyhow!("数据字段不足: 期望至少15个，实际{}个", fields.len()));
            }

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
                updated_at: get_beijing_time(),
            });
        }
        
        Err(anyhow!("无法解析响应数据: {}", data))
    }

    /// 解析多个期货合约实时数据
    fn parse_multiple_realtime_data(&self, data: &str, original_symbols: &[String]) -> Result<Vec<FuturesInfo>> {
        let mut results = Vec::new();
        
        let items: Vec<&str> = data.split(';')
            .filter(|s| !s.trim().is_empty())
            .collect();
        
        for (i, item) in items.iter().enumerate() {
            if i < original_symbols.len() {
                match self.parse_sina_realtime_data(item, &original_symbols[i]) {
                    Ok(futures_info) => results.push(futures_info),
                    Err(e) => {
                        log::warn!("解析 {} 数据失败: {}", original_symbols[i], e);
                        continue;
                    }
                }
            }
        }
        
        Ok(results)
    }

    /// 解析新浪期货列表数据
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
            updated_at: get_beijing_time(),
        })
    }
}


// ==================== 独立函数（K线数据） ====================

/// 获取期货日K线历史数据
/// 对应 akshare 的 futures_zh_daily_sina() 函数
pub async fn get_futures_history(symbol: &str, query: &FuturesQuery) -> Result<Vec<FuturesHistoryData>> {
    let client = Client::new();
    let limit = query.limit.unwrap_or(30);
    
    let full_url = format!("{}?symbol={}", SINA_FUTURES_DAILY_API, symbol);
    println!("📡 请求日K线数据 URL: {}", full_url);
    
    let response = client
        .get(SINA_FUTURES_DAILY_API)
        .query(&[("symbol", symbol)])
        .header("Referer", "https://finance.sina.com.cn/")
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(anyhow!("获取历史数据失败: {}", response.status()));
    }

    let text = response.text().await?;
    println!("📥 原始响应数据: {}", &text[..std::cmp::min(300, text.len())]);
    parse_sina_history_data(&text, symbol, limit)
}

/// 获取期货分钟K线数据
/// 对应 akshare 的 futures_zh_minute_sina() 函数
/// period: "1", "5", "15", "30", "60" 分钟
#[allow(dead_code)]
pub async fn get_futures_minute_data(symbol: &str, period: &str) -> Result<Vec<FuturesHistoryData>> {
    let client = Client::new();
    
    let full_url = format!("{}?symbol={}&type={}", SINA_FUTURES_MINUTE_API, symbol, period);
    println!("📡 请求分钟K线数据 URL: {}", full_url);
    
    let response = client
        .get(SINA_FUTURES_MINUTE_API)
        .query(&[("symbol", symbol), ("type", period)])
        .header("Referer", "https://finance.sina.com.cn/")
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(anyhow!("获取分钟数据失败: {}", response.status()));
    }

    let text = response.text().await?;
    println!("📥 原始响应数据: {}", &text[..std::cmp::min(300, text.len())]);
    parse_sina_minute_data(&text, symbol)
}

/// 解析新浪期货日K线历史数据
fn parse_sina_history_data(data: &str, symbol: &str, limit: usize) -> Result<Vec<FuturesHistoryData>> {
    let mut history = Vec::new();
    
    let start = data.find("([");
    let end = data.rfind("])");
    
    if start.is_none() || end.is_none() {
        println!("❌ 未找到有效的JSON数据边界");
        return Err(anyhow!("无效的历史数据格式"));
    }
    
    let json_str = &data[start.unwrap() + 1..end.unwrap() + 1];
    println!("📊 解析JSON数据，长度: {} 字节", json_str.len());
    
    let json_data: serde_json::Value = serde_json::from_str(json_str)
        .map_err(|e| anyhow!("解析JSON失败: {}", e))?;
    
    if let Some(arr) = json_data.as_array() {
        println!("📈 解析到 {} 条K线数据", arr.len());
        
        let start_idx = if arr.len() > limit { arr.len() - limit } else { 0 };
        
        for item in arr.iter().skip(start_idx) {
            // JSON对象格式
            if item.is_object() {
                let date = item["d"].as_str().unwrap_or("").to_string();
                let open = item["o"].as_str().unwrap_or("0").parse().unwrap_or(0.0);
                let high = item["h"].as_str().unwrap_or("0").parse().unwrap_or(0.0);
                let low = item["l"].as_str().unwrap_or("0").parse().unwrap_or(0.0);
                let close = item["c"].as_str().unwrap_or("0").parse().unwrap_or(0.0);
                let volume = item["v"].as_str().unwrap_or("0").parse().unwrap_or(0);
                let open_interest = item["p"].as_str().unwrap_or("0").parse().ok();
                let settlement = item["s"].as_str().unwrap_or("0").parse().ok();
                
                history.push(FuturesHistoryData {
                    symbol: symbol.to_string(),
                    date,
                    open,
                    high,
                    low,
                    close,
                    volume,
                    open_interest,
                    settlement,
                });
            }
            // 数组格式（兼容）
            else if let Some(fields) = item.as_array() {
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
    
    Ok(history)
}

/// 解析新浪期货分钟K线数据
fn parse_sina_minute_data(data: &str, symbol: &str) -> Result<Vec<FuturesHistoryData>> {
    let mut history = Vec::new();
    
    let start = data.find("([");
    let end = data.rfind("])");
    
    if start.is_none() || end.is_none() {
        println!("❌ 未找到有效的JSON数据边界");
        return Err(anyhow!("无效的分钟数据格式"));
    }
    
    let json_str = &data[start.unwrap() + 1..end.unwrap() + 1];
    println!("📊 解析JSON数据，长度: {} 字节", json_str.len());
    
    let json_data: serde_json::Value = serde_json::from_str(json_str)
        .map_err(|e| anyhow!("解析JSON失败: {}", e))?;
    
    if let Some(arr) = json_data.as_array() {
        println!("📈 解析到 {} 条K线数据", arr.len());
        
        for item in arr.iter() {
            // JSON对象格式
            if item.is_object() {
                history.push(FuturesHistoryData {
                    symbol: symbol.to_string(),
                    date: item["d"].as_str().unwrap_or("").to_string(),
                    open: item["o"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                    high: item["h"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                    low: item["l"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                    close: item["c"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                    volume: item["v"].as_str().unwrap_or("0").parse().unwrap_or(0),
                    open_interest: item["p"].as_str().unwrap_or("0").parse().ok(),
                    settlement: None,
                });
            }
            // 数组格式（兼容）
            else if let Some(fields) = item.as_array() {
                if fields.len() >= 6 {
                    history.push(FuturesHistoryData {
                        symbol: symbol.to_string(),
                        date: fields[0].as_str().unwrap_or("").to_string(),
                        open: fields[1].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                        high: fields[2].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                        low: fields[3].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                        close: fields[4].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                        volume: fields[5].as_str().unwrap_or("0").parse().unwrap_or(0),
                        open_interest: fields.get(6).and_then(|v| v.as_str()).and_then(|s| s.parse().ok()),
                        settlement: None,
                    });
                }
            }
        }
    }
    
    Ok(history)
}


// ==================== 外盘期货相关 ====================

/// 获取外盘期货品种列表
/// 对应 akshare 的 futures_hq_subscribe_exchange_symbol() 函数
pub fn get_foreign_futures_symbols() -> Vec<ForeignFuturesSymbol> {
    vec![
        ForeignFuturesSymbol { symbol: "新加坡铁矿石".to_string(), code: "FEF".to_string() },
        ForeignFuturesSymbol { symbol: "马棕油".to_string(), code: "FCPO".to_string() },
        ForeignFuturesSymbol { symbol: "日橡胶".to_string(), code: "RSS3".to_string() },
        ForeignFuturesSymbol { symbol: "美国原糖".to_string(), code: "RS".to_string() },
        ForeignFuturesSymbol { symbol: "CME比特币期货".to_string(), code: "BTC".to_string() },
        ForeignFuturesSymbol { symbol: "NYBOT-棉花".to_string(), code: "CT".to_string() },
        ForeignFuturesSymbol { symbol: "LME镍3个月".to_string(), code: "NID".to_string() },
        ForeignFuturesSymbol { symbol: "LME铅3个月".to_string(), code: "PBD".to_string() },
        ForeignFuturesSymbol { symbol: "LME锡3个月".to_string(), code: "SND".to_string() },
        ForeignFuturesSymbol { symbol: "LME锌3个月".to_string(), code: "ZSD".to_string() },
        ForeignFuturesSymbol { symbol: "LME铝3个月".to_string(), code: "AHD".to_string() },
        ForeignFuturesSymbol { symbol: "LME铜3个月".to_string(), code: "CAD".to_string() },
        ForeignFuturesSymbol { symbol: "CBOT-黄豆".to_string(), code: "S".to_string() },
        ForeignFuturesSymbol { symbol: "CBOT-小麦".to_string(), code: "W".to_string() },
        ForeignFuturesSymbol { symbol: "CBOT-玉米".to_string(), code: "C".to_string() },
        ForeignFuturesSymbol { symbol: "CBOT-黄豆油".to_string(), code: "BO".to_string() },
        ForeignFuturesSymbol { symbol: "CBOT-黄豆粉".to_string(), code: "SM".to_string() },
        ForeignFuturesSymbol { symbol: "COMEX铜".to_string(), code: "HG".to_string() },
        ForeignFuturesSymbol { symbol: "NYMEX天然气".to_string(), code: "NG".to_string() },
        ForeignFuturesSymbol { symbol: "NYMEX原油".to_string(), code: "CL".to_string() },
        ForeignFuturesSymbol { symbol: "COMEX白银".to_string(), code: "SI".to_string() },
        ForeignFuturesSymbol { symbol: "COMEX黄金".to_string(), code: "GC".to_string() },
        ForeignFuturesSymbol { symbol: "布伦特原油".to_string(), code: "OIL".to_string() },
        ForeignFuturesSymbol { symbol: "伦敦金".to_string(), code: "XAU".to_string() },
        ForeignFuturesSymbol { symbol: "伦敦银".to_string(), code: "XAG".to_string() },
        ForeignFuturesSymbol { symbol: "伦敦铂金".to_string(), code: "XPT".to_string() },
        ForeignFuturesSymbol { symbol: "伦敦钯金".to_string(), code: "XPD".to_string() },
        ForeignFuturesSymbol { symbol: "欧洲碳排放".to_string(), code: "EUA".to_string() },
    ]
}

/// 获取外盘期货实时行情
/// 对应 akshare 的 futures_foreign_commodity_realtime() 函数
pub async fn get_foreign_futures_realtime(codes: &[String]) -> Result<Vec<FuturesInfo>> {
    let client = Client::new();
    
    let symbols_str = codes.iter()
        .map(|c| format!("hf_{}", c))
        .collect::<Vec<_>>()
        .join(",");
    
    let url = format!("{}?list={}", SINA_FUTURES_REALTIME_API, symbols_str);
    println!("📡 请求外盘期货行情 URL: {}", url);
    
    let response = client
        .get(&url)
        .header("Accept", "*/*")
        .header("Accept-Encoding", "gzip, deflate, br")
        .header("Accept-Language", "zh-CN,zh;q=0.9,en;q=0.8")
        .header("Cache-Control", "no-cache")
        .header("Host", "hq.sinajs.cn")
        .header("Pragma", "no-cache")
        .header("Referer", "https://finance.sina.com.cn/")
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(anyhow!("获取外盘期货数据失败: {}", response.status()));
    }

    let text = response.text().await?;
    println!("📥 原始响应数据: {}", &text[..std::cmp::min(500, text.len())]);
    
    parse_foreign_futures_data(&text, codes)
}

/// 解析外盘期货数据
fn parse_foreign_futures_data(data: &str, codes: &[String]) -> Result<Vec<FuturesInfo>> {
    let mut results = Vec::new();
    let symbol_map = get_foreign_futures_symbols();
    let code_to_name: HashMap<String, String> = symbol_map.iter()
        .map(|s| (s.code.clone(), s.symbol.clone()))
        .collect();
    
    for (i, item) in data.split(';').filter(|s| !s.trim().is_empty()).enumerate() {
        if i >= codes.len() {
            break;
        }
        
        let parts: Vec<&str> = item.split('=').collect();
        if parts.len() < 2 {
            continue;
        }
        
        let data_part = parts[1].trim_matches('"').trim_matches('\'');
        if data_part.is_empty() {
            continue;
        }
        
        let fields: Vec<&str> = data_part.split(',').collect();
        if fields.len() < 13 {
            continue;
        }
        
        let code = &codes[i];
        let name = code_to_name.get(code).cloned().unwrap_or(code.clone());
        
        let current_price = fields[0].parse::<f64>().unwrap_or(0.0);
        let _bid = fields[2].parse::<f64>().unwrap_or(0.0);
        let _ask = fields[3].parse::<f64>().unwrap_or(0.0);
        let high = fields[4].parse::<f64>().unwrap_or(0.0);
        let low = fields[5].parse::<f64>().unwrap_or(0.0);
        let prev_settlement = fields[7].parse::<f64>().unwrap_or(0.0);
        let open = fields[8].parse::<f64>().unwrap_or(0.0);
        let open_interest = fields[9].parse::<u64>().ok();
        
        let change = current_price - prev_settlement;
        let change_percent = if prev_settlement != 0.0 {
            (change / prev_settlement) * 100.0
        } else {
            0.0
        };
        
        results.push(FuturesInfo {
            symbol: code.clone(),
            name,
            current_price,
            change,
            change_percent,
            volume: 0, // 外盘数据格式不同
            open,
            high,
            low,
            settlement: None,
            prev_settlement: Some(prev_settlement),
            open_interest,
            updated_at: get_beijing_time(),
        });
    }
    
    Ok(results)
}

/// 外盘期货日K线API
const SINA_FOREIGN_DAILY_API: &str = "https://stock2.finance.sina.com.cn/futures/api/jsonp.php";

/// 获取外盘期货历史数据（日K线）
/// 对应 akshare 的 futures_foreign_hist() 函数
/// symbol: 外盘期货代码，如 "ZSD"(LME锌), "GC"(COMEX黄金)
pub async fn get_futures_foreign_hist(symbol: &str) -> Result<Vec<ForeignFuturesHistData>> {
    let client = Client::new();
    
    // 构建日期参数
    let now = Utc::now().with_timezone(&Shanghai);
    let today = format!("{}_{}_{}",
        now.format("%Y"),
        now.format("%-m"),
        now.format("%-d")
    );
    
    let url = format!(
        "{}/var%20_S{}=/GlobalFuturesService.getGlobalFuturesDailyKLine",
        SINA_FOREIGN_DAILY_API, today
    );
    
    println!("📡 请求外盘期货历史数据 URL: {}", url);
    
    let response = client
        .get(&url)
        .query(&[
            ("symbol", symbol),
            ("_", &today),
            ("source", "web"),
        ])
        .header("Referer", "https://finance.sina.com.cn/")
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(anyhow!("获取外盘期货历史数据失败: {}", response.status()));
    }

    let text = response.text().await?;
    println!("📥 原始响应数据长度: {} 字节", text.len());
    
    parse_foreign_hist_data(&text)
}

/// 解析外盘期货历史数据
fn parse_foreign_hist_data(data: &str) -> Result<Vec<ForeignFuturesHistData>> {
    let mut history = Vec::new();
    
    // 找到JSON数组的位置
    let start = data.find('[');
    let end = data.rfind(']');
    
    if start.is_none() || end.is_none() {
        return Err(anyhow!("无效的外盘期货历史数据格式"));
    }
    
    let json_str = &data[start.unwrap()..end.unwrap() + 1];
    
    let json_data: serde_json::Value = serde_json::from_str(json_str)
        .map_err(|e| anyhow!("解析JSON失败: {}", e))?;
    
    if let Some(arr) = json_data.as_array() {
        println!("📈 解析到 {} 条外盘期货历史数据", arr.len());
        
        for item in arr {
            if item.is_object() {
                // 新浪返回的字段: date, open, high, low, close, volume
                history.push(ForeignFuturesHistData {
                    date: item["date"].as_str().unwrap_or("").to_string(),
                    open: item["open"].as_str()
                        .or_else(|| item["open"].as_f64().map(|_| ""))
                        .and_then(|s| if s.is_empty() { item["open"].as_f64() } else { s.parse().ok() })
                        .unwrap_or(0.0),
                    high: item["high"].as_str()
                        .or_else(|| item["high"].as_f64().map(|_| ""))
                        .and_then(|s| if s.is_empty() { item["high"].as_f64() } else { s.parse().ok() })
                        .unwrap_or(0.0),
                    low: item["low"].as_str()
                        .or_else(|| item["low"].as_f64().map(|_| ""))
                        .and_then(|s| if s.is_empty() { item["low"].as_f64() } else { s.parse().ok() })
                        .unwrap_or(0.0),
                    close: item["close"].as_str()
                        .or_else(|| item["close"].as_f64().map(|_| ""))
                        .and_then(|s| if s.is_empty() { item["close"].as_f64() } else { s.parse().ok() })
                        .unwrap_or(0.0),
                    volume: item["volume"].as_str()
                        .and_then(|s| s.parse().ok())
                        .or_else(|| item["volume"].as_u64())
                        .unwrap_or(0),
                });
            }
        }
    }
    
    Ok(history)
}

/// 获取外盘期货合约详情
/// 对应 akshare 的 futures_foreign_detail() 函数
/// symbol: 外盘期货代码，如 "ZSD"(LME锌), "GC"(COMEX黄金)
pub async fn get_futures_foreign_detail(symbol: &str) -> Result<ForeignFuturesDetail> {
    let client = Client::new();
    
    let url = format!("https://finance.sina.com.cn/futures/quotes/{}.shtml", symbol);
    println!("📡 请求外盘期货合约详情 URL: {}", url);
    
    let response = client
        .get(&url)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(anyhow!("获取外盘期货合约详情失败: {}", response.status()));
    }

    // 使用 GBK 编码读取
    let bytes = response.bytes().await?;
    let text = encoding_rs::GBK.decode(&bytes).0.to_string();
    
    parse_foreign_detail_html(&text)
}

/// 解析外盘期货合约详情HTML
fn parse_foreign_detail_html(html: &str) -> Result<ForeignFuturesDetail> {
    let mut items = Vec::new();
    
    // 查找第7个表格（索引6），这是合约详情表格
    let table_re = Regex::new(r"<table[^>]*>([\s\S]*?)</table>").unwrap();
    let tables: Vec<_> = table_re.captures_iter(html).collect();
    
    // 尝试找到合约详情表格（通常是第7个表格）
    let target_table_index = if tables.len() > 6 { 6 } else { tables.len().saturating_sub(1) };
    
    if tables.is_empty() {
        return Err(anyhow!("未找到合约详情表格"));
    }
    
    let table_content = tables.get(target_table_index)
        .and_then(|cap| cap.get(1))
        .map(|m| m.as_str())
        .unwrap_or("");
    
    // 解析表格行
    let row_re = Regex::new(r"<tr[^>]*>([\s\S]*?)</tr>").unwrap();
    let cell_re = Regex::new(r"<t[dh][^>]*>([\s\S]*?)</t[dh]>").unwrap();
    
    // 清理HTML标签的辅助函数
    let clean_html = |s: &str| -> String {
        let tag_re = Regex::new(r"<[^>]+>").unwrap();
        tag_re.replace_all(s, "").trim().to_string()
    };
    
    for row_cap in row_re.captures_iter(table_content) {
        let row_content = row_cap.get(1).map(|m| m.as_str()).unwrap_or("");
        let cells: Vec<_> = cell_re.captures_iter(row_content)
            .filter_map(|c| c.get(1).map(|m| clean_html(m.as_str())))
            .collect();
        
        // 处理两列的行（名称-值对）
        if cells.len() >= 2 {
            let name = cells[0].clone();
            let value = cells[1].clone();
            
            if !name.is_empty() && !value.is_empty() {
                items.push(ForeignFuturesDetailItem { name, value });
            }
            
            // 如果有4列，处理第二对
            if cells.len() >= 4 {
                let name2 = cells[2].clone();
                let value2 = cells[3].clone();
                
                if !name2.is_empty() && !value2.is_empty() {
                    items.push(ForeignFuturesDetailItem { name: name2, value: value2 });
                }
            }
        }
    }
    
    println!("📊 解析到 {} 条合约详情项", items.len());
    Ok(ForeignFuturesDetail { items })
}


// ==================== 期货交易费用相关 ====================

/// OpenCTP期货交易费用API
const OPENCTP_FEES_URL: &str = "http://openctp.cn/fees.html";

/// 获取期货交易费用参照表
/// 对应 akshare 的 futures_fees_info() 函数
/// 数据来源: http://openctp.cn/fees.html
pub async fn get_futures_fees_info() -> Result<Vec<FuturesFeesInfo>> {
    let client = Client::new();
    
    println!("📡 请求期货交易费用数据 URL: {}", OPENCTP_FEES_URL);
    
    let response = client
        .get(OPENCTP_FEES_URL)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(anyhow!("获取期货交易费用数据失败: {}", response.status()));
    }

    let text = response.text().await?;
    parse_fees_html(&text)
}

/// 解析期货交易费用HTML
fn parse_fees_html(html: &str) -> Result<Vec<FuturesFeesInfo>> {
    let mut fees_list = Vec::new();
    
    // 提取更新时间
    let time_re = Regex::new(r"Generated at ([^.]+)\.").unwrap();
    let updated_at = time_re.captures(html)
        .and_then(|c| c.get(1))
        .map(|m| m.as_str().trim().to_string())
        .unwrap_or_else(|| "未知".to_string());
    
    println!("📅 数据更新时间: {}", updated_at);
    
    // 查找表格
    let table_re = Regex::new(r"<table[^>]*>([\s\S]*?)</table>").unwrap();
    let table_match = table_re.captures(html);
    
    if table_match.is_none() {
        return Err(anyhow!("未找到费用数据表格"));
    }
    
    let table_content = table_match.unwrap().get(1).map(|m| m.as_str()).unwrap_or("");
    
    // 解析表格行
    let row_re = Regex::new(r"<tr[^>]*>([\s\S]*?)</tr>").unwrap();
    let cell_re = Regex::new(r"<t[dh][^>]*>([\s\S]*?)</t[dh]>").unwrap();
    
    // 清理HTML标签
    let clean_html = |s: &str| -> String {
        let tag_re = Regex::new(r"<[^>]+>").unwrap();
        tag_re.replace_all(s, "").trim().to_string()
    };
    
    let mut is_header = true;
    for row_cap in row_re.captures_iter(table_content) {
        let row_content = row_cap.get(1).map(|m| m.as_str()).unwrap_or("");
        let cells: Vec<_> = cell_re.captures_iter(row_content)
            .filter_map(|c| c.get(1).map(|m| clean_html(m.as_str())))
            .collect();
        
        // 跳过表头
        if is_header {
            is_header = false;
            continue;
        }
        
        // 期望的列: 交易所, 品种, 合约, 合约乘数, 最小变动价位, 保证金率, 开仓手续费, 平仓手续费, 平今手续费
        if cells.len() >= 9 {
            fees_list.push(FuturesFeesInfo {
                exchange: cells[0].clone(),
                product: cells[1].clone(),
                contract: cells[2].clone(),
                contract_size: cells[3].clone(),
                price_tick: cells[4].clone(),
                margin_rate: cells[5].clone(),
                open_fee: cells[6].clone(),
                close_fee: cells[7].clone(),
                close_today_fee: cells[8].clone(),
                updated_at: updated_at.clone(),
            });
        }
    }
    
    println!("📊 解析到 {} 条期货费用数据", fees_list.len());
    Ok(fees_list)
}


// ==================== 主力连续合约相关 ====================

/// 新浪主力连续合约日K线API
const SINA_MAIN_DAILY_API: &str = "https://stock2.finance.sina.com.cn/futures/api/jsonp.php";

/// 新浪持仓排名API
const SINA_HOLD_POS_API: &str = "https://vip.stock.finance.sina.com.cn/q/view/vFutures_Positions_cjcc.php";

/// 获取主力连续合约一览表
/// 对应 akshare 的 futures_display_main_sina() 函数
/// 返回所有交易所的主力连续合约列表
pub async fn get_futures_display_main_sina() -> Result<Vec<FuturesMainContract>> {
    let mut all_contracts = Vec::new();
    
    for exchange in &["dce", "czce", "shfe", "cffex", "gfex"] {
        match get_main_contracts_by_exchange(exchange).await {
            Ok(mut contracts) => all_contracts.append(&mut contracts),
            Err(e) => {
                log::warn!("获取 {} 主力连续合约失败: {}", exchange, e);
            }
        }
    }
    
    Ok(all_contracts)
}

/// 获取指定交易所的主力连续合约
/// 对应 akshare 的 match_main_contract() 函数（返回连续合约版本）
async fn get_main_contracts_by_exchange(exchange: &str) -> Result<Vec<FuturesMainContract>> {
    let client = Client::new();
    let mut contracts = Vec::new();
    
    // 获取交易所品种列表
    let symbol_url = "https://vip.stock.finance.sina.com.cn/quotes_service/view/js/qihuohangqing.js";
    let response = client
        .get(symbol_url)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .send()
        .await?;
    
    let bytes = response.bytes().await?;
    let text = encoding_rs::GBK.decode(&bytes).0.to_string();
    
    // 解析交易所品种的node列表
    let nodes = parse_exchange_nodes(&text, exchange)?;
    
    // 遍历每个品种，获取主力连续合约
    for node in nodes {
        let list_url = "https://vip.stock.finance.sina.com.cn/quotes_service/api/json_v2.php/Market_Center.getHQFuturesData";
        
        let response = client
            .get(list_url)
            .query(&[
                ("page", "1"),
                ("sort", "position"),
                ("asc", "0"),
                ("node", &node),
                ("base", "futures"),
            ])
            .send()
            .await;
        
        if let Ok(resp) = response {
            if let Ok(text) = resp.text().await {
                if let Ok(json_data) = serde_json::from_str::<serde_json::Value>(&text) {
                    if let Some(arr) = json_data.as_array() {
                        // 查找主力连续合约（名称包含"连续"且代码以0结尾）
                        for item in arr {
                            let name = item["name"].as_str().unwrap_or("");
                            let symbol = item["symbol"].as_str().unwrap_or("");
                            
                            if name.contains("连续") && symbol.ends_with("0") {
                                contracts.push(FuturesMainContract {
                                    symbol: symbol.to_string(),
                                    name: name.to_string(),
                                    exchange: exchange.to_uppercase(),
                                });
                                break; // 每个品种只取一个连续合约
                            }
                        }
                    }
                }
            }
        }
    }
    
    Ok(contracts)
}

/// 解析交易所的品种node列表
fn parse_exchange_nodes(js_text: &str, exchange: &str) -> Result<Vec<String>> {
    let mut nodes = Vec::new();
    
    let start = js_text.find("ARRFUTURESNODES = {");
    let end = js_text.find("};");
    
    if start.is_none() || end.is_none() {
        return Err(anyhow!("无法解析品种映射JS数据"));
    }
    
    let content = &js_text[start.unwrap()..end.unwrap() + 2];
    
    // 查找交易所数据块
    let pattern = format!(r"{}\s*:\s*\[", exchange);
    let re = Regex::new(&pattern).unwrap();
    
    if let Some(m) = re.find(content) {
        let start_pos = m.end();
        let remaining = &content[start_pos..];
        
        // 解析品种数组 ['品种名', 'node', '数字']
        let item_re = Regex::new(r"\['[^']+',\s*'([^']+)',\s*'[^']*'").unwrap();
        
        for cap in item_re.captures_iter(remaining) {
            if let Some(node) = cap.get(1) {
                let node_str = node.as_str();
                if node_str.ends_with("_qh") {
                    nodes.push(node_str.to_string());
                }
            }
        }
    }
    
    Ok(nodes)
}

/// 获取主力连续合约日K线数据
/// 对应 akshare 的 futures_main_sina() 函数
/// symbol: 主力连续合约代码，如 "V0", "RB0", "IF0"
/// start_date/end_date: 日期范围，格式 YYYYMMDD
pub async fn get_futures_main_sina(
    symbol: &str,
    start_date: Option<&str>,
    end_date: Option<&str>,
) -> Result<Vec<FuturesMainDailyData>> {
    let client = Client::new();
    
    // 构建URL（新浪API格式）
    let trade_date = "20210817";
    let trade_date_fmt = format!("{}_{}_{}",
        &trade_date[..4], &trade_date[4..6], &trade_date[6..]);
    
    let url = format!(
        "{}/var%20_{}{}=/InnerFuturesNewService.getDailyKLine?symbol={}&_={}",
        SINA_MAIN_DAILY_API, symbol, trade_date_fmt, symbol, trade_date_fmt
    );
    
    println!("📡 请求主力连续日K线 URL: {}", url);
    
    let response = client
        .get(&url)
        .header("Referer", "https://finance.sina.com.cn/")
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(anyhow!("获取主力连续数据失败: {}", response.status()));
    }

    let text = response.text().await?;
    println!("📥 原始响应数据长度: {} 字节", text.len());
    
    // 解析数据
    let mut data = parse_main_daily_data(&text)?;
    
    // 按日期范围过滤
    if let Some(start) = start_date {
        data.retain(|d| d.date.replace("-", "") >= start.to_string());
    }
    if let Some(end) = end_date {
        data.retain(|d| d.date.replace("-", "") <= end.to_string());
    }
    
    Ok(data)
}

/// 解析主力连续日K线数据
fn parse_main_daily_data(data: &str) -> Result<Vec<FuturesMainDailyData>> {
    let mut history = Vec::new();
    
    let start = data.find("([");
    let end = data.rfind("])");
    
    if start.is_none() || end.is_none() {
        return Err(anyhow!("无效的主力连续数据格式"));
    }
    
    let json_str = &data[start.unwrap() + 1..end.unwrap() + 1];
    
    let json_data: serde_json::Value = serde_json::from_str(json_str)
        .map_err(|e| anyhow!("解析JSON失败: {}", e))?;
    
    if let Some(arr) = json_data.as_array() {
        for item in arr {
            if item.is_object() {
                history.push(FuturesMainDailyData {
                    date: item["d"].as_str().unwrap_or("").to_string(),
                    open: item["o"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                    high: item["h"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                    low: item["l"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                    close: item["c"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                    volume: item["v"].as_str().unwrap_or("0").parse().unwrap_or(0),
                    hold: item["p"].as_str().unwrap_or("0").parse().unwrap_or(0),
                    settle: item["s"].as_str().and_then(|s| s.parse().ok()),
                });
            }
        }
    }
    
    Ok(history)
}

/// 获取期货持仓排名数据
/// 对应 akshare 的 futures_hold_pos_sina() 函数
/// pos_type: "volume"(成交量), "long"(多单持仓), "short"(空单持仓)
/// contract: 合约代码，如 "OI2501", "IC2403"
/// date: 查询日期，格式 YYYYMMDD
pub async fn get_futures_hold_pos_sina(
    pos_type: &str,
    contract: &str,
    date: &str,
) -> Result<Vec<FuturesHoldPosition>> {
    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()?;
    
    // 格式化日期为 YYYY-MM-DD
    let formatted_date = if date.len() == 8 {
        format!("{}-{}-{}", &date[..4], &date[4..6], &date[6..])
    } else {
        date.to_string()
    };
    
    let url = format!("{}?t_breed={}&t_date={}", SINA_HOLD_POS_API, contract, formatted_date);
    println!("📡 请求持仓排名 URL: {}", url);
    
    let response = client
        .get(&url)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
        .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8")
        .header("Accept-Language", "zh-CN,zh;q=0.9,en;q=0.8")
        .header("Accept-Encoding", "gzip, deflate")
        .header("Connection", "keep-alive")
        .header("Referer", "https://vip.stock.finance.sina.com.cn/")
        .header("Host", "vip.stock.finance.sina.com.cn")
        .send()
        .await?;

    let status = response.status();
    if !status.is_success() {
        // 检查是否是IP被封禁
        if status.as_u16() == 456 || status.as_u16() == 403 {
            return Err(anyhow!("IP被新浪封禁，请稍后重试（5-60分钟后自动解封）"));
        }
        return Err(anyhow!("获取持仓排名失败: {}", status));
    }

    // 使用 GBK 编码读取
    let bytes = response.bytes().await?;
    let text = encoding_rs::GBK.decode(&bytes).0.to_string();
    
    // 检查是否返回了拒绝访问页面
    if text.contains("拒绝访问") || text.contains("IP 存在异常访问") {
        return Err(anyhow!("IP被新浪封禁，请稍后重试（5-60分钟后自动解封）"));
    }
    
    // 根据类型选择解析的表格索引
    let table_index = match pos_type {
        "volume" => 2,
        "long" => 3,
        "short" => 4,
        _ => return Err(anyhow!("无效的持仓类型: {}, 应为 volume/long/short", pos_type)),
    };
    
    parse_hold_pos_html(&text, table_index, pos_type)
}

/// 解析持仓排名HTML数据
fn parse_hold_pos_html(html: &str, table_index: usize, pos_type: &str) -> Result<Vec<FuturesHoldPosition>> {
    let mut positions = Vec::new();
    
    // 简单的HTML表格解析
    // 查找所有表格
    let table_re = Regex::new(r"<table[^>]*>([\s\S]*?)</table>").unwrap();
    let tables: Vec<_> = table_re.captures_iter(html).collect();
    
    if tables.len() <= table_index {
        return Err(anyhow!("未找到持仓排名数据表格"));
    }
    
    let table_content = tables[table_index].get(1).map(|m| m.as_str()).unwrap_or("");
    
    // 解析表格行
    let row_re = Regex::new(r"<tr[^>]*>([\s\S]*?)</tr>").unwrap();
    let cell_re = Regex::new(r"<td[^>]*>([\s\S]*?)</td>").unwrap();
    
    let value_col_name = match pos_type {
        "volume" => "成交量",
        "long" => "多单持仓",
        "short" => "空单持仓",
        _ => "数值",
    };
    
    for (i, row_cap) in row_re.captures_iter(table_content).enumerate() {
        // 跳过表头和合计行
        if i == 0 {
            continue;
        }
        
        let row_content = row_cap.get(1).map(|m| m.as_str()).unwrap_or("");
        let cells: Vec<_> = cell_re.captures_iter(row_content)
            .filter_map(|c| c.get(1).map(|m| m.as_str().trim()))
            .collect();
        
        if cells.len() >= 3 {
            // 清理HTML标签
            let clean_text = |s: &str| -> String {
                let tag_re = Regex::new(r"<[^>]+>").unwrap();
                tag_re.replace_all(s, "").trim().to_string()
            };
            
            let rank_str = clean_text(cells[0]);
            let company = clean_text(cells[1]);
            let value_str = clean_text(cells[2]);
            
            // 跳过合计行
            if rank_str.contains("合计") || company.contains("合计") {
                continue;
            }
            
            let rank = rank_str.parse::<u32>().unwrap_or(0);
            let value = value_str.replace(",", "").parse::<i64>().unwrap_or(0);
            
            // 解析增减值（如果有第4列）
            let change = if cells.len() >= 4 {
                clean_text(cells[3]).replace(",", "").parse::<i64>().unwrap_or(0)
            } else {
                0
            };
            
            if rank > 0 {
                positions.push(FuturesHoldPosition {
                    rank,
                    company,
                    value,
                    change,
                });
            }
        }
    }
    
    println!("📊 解析到 {} 条{}排名数据", positions.len(), value_col_name);
    Ok(positions)
}


// ==================== 测试模块 ====================

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== 单元测试 ====================

    /// 测试合约代码格式化（商品期货）
    #[test]
    fn test_format_symbol_commodity() {
        println!("\n========== 测试商品期货合约代码格式化 ==========");
        let service = FuturesService::new();
        
        let test_cases = vec![
            ("CU2405", "nf_CU2405"),
            ("AL2405", "nf_AL2405"),
            ("RB2405", "nf_RB2405"),
        ];
        
        for (input, expected) in &test_cases {
            let result = service.format_symbol_for_realtime(input);
            println!("  {} -> {} (期望: {})", input, result, expected);
            assert_eq!(result, *expected);
        }
        println!("✅ 商品期货格式化测试通过！");
    }

    /// 测试合约代码格式化（金融期货）
    #[test]
    fn test_format_symbol_financial() {
        println!("\n========== 测试金融期货合约代码格式化 ==========");
        let service = FuturesService::new();
        
        let test_cases = vec![
            ("IF2401", "CFF_IF2401"),
            ("IC2401", "CFF_IC2401"),
            ("T2406", "CFF_T2406"),
        ];
        
        for (input, expected) in &test_cases {
            let result = service.format_symbol_for_realtime(input);
            println!("  {} -> {} (期望: {})", input, result, expected);
            assert_eq!(result, *expected);
        }
        println!("✅ 金融期货格式化测试通过！");
    }

    /// 测试中金所合约判断
    #[test]
    fn test_is_cffex_symbol() {
        println!("\n========== 测试中金所合约判断 ==========");
        let service = FuturesService::new();
        
        let cffex_symbols = vec!["IF2401", "IC2401", "IH2401", "T2406", "TF2406", "TS2406", "IM2401", "TL2406"];
        for symbol in &cffex_symbols {
            assert!(service.is_cffex_symbol(symbol), "{} 应该是中金所合约", symbol);
        }
        
        let non_cffex = vec!["CU2405", "AL2405", "RB2405"];
        for symbol in &non_cffex {
            assert!(!service.is_cffex_symbol(symbol), "{} 不应该是中金所合约", symbol);
        }
        println!("✅ 中金所合约判断测试通过！");
    }

    /// 测试随机码生成
    #[test]
    fn test_generate_random_code() {
        println!("\n========== 测试随机码生成 ==========");
        let service = FuturesService::new();
        
        let code = service.generate_random_code();
        println!("  生成的随机码: {}", code);
        assert!(code.chars().all(|c| c.is_ascii_hexdigit()));
        println!("✅ 随机码生成测试通过！");
    }

    /// 测试交易所列表
    #[test]
    fn test_get_exchanges() {
        println!("\n========== 测试获取交易所列表 ==========");
        let service = FuturesService::new();
        let exchanges = service.get_exchanges();
        
        println!("  交易所数量: {}", exchanges.len());
        for ex in &exchanges {
            println!("    【{}】{}", ex.code, ex.name);
        }
        
        assert!(exchanges.len() >= 5);
        println!("✅ 交易所列表测试通过！");
    }

    /// 测试外盘期货品种列表
    #[test]
    fn test_get_foreign_futures_symbols() {
        println!("\n========== 测试外盘期货品种列表 ==========");
        let symbols = get_foreign_futures_symbols();
        
        println!("  外盘品种数量: {}", symbols.len());
        for s in symbols.iter().take(5) {
            println!("    {} -> {}", s.symbol, s.code);
        }
        
        assert!(symbols.len() > 20);
        println!("✅ 外盘期货品种列表测试通过！");
    }

    /// 测试北京时间
    #[test]
    fn test_get_beijing_time() {
        println!("\n========== 测试北京时间获取 ==========");
        let time = get_beijing_time();
        println!("  当前北京时间: {}", time);
        assert!(time.contains("+08:00"));
        println!("✅ 北京时间测试通过！");
    }

    /// 测试解析实时数据
    #[test]
    fn test_parse_realtime_data() {
        println!("\n========== 测试解析实时数据 ==========");
        let service = FuturesService::new();
        
        let mock_data = r#"var hq_str_nf_CU2405="铜2405,09:00:00,75000,75500,74800,74900,75100,75200,75150,75100,74950,100,200,50000,100000,0,0,0,0,0,0,0,0,0,0,0,0,0";"#;
        
        let result = service.parse_sina_realtime_data(mock_data, "CU2405");
        assert!(result.is_ok());
        
        let info = result.unwrap();
        println!("  合约: {} - {}", info.symbol, info.name);
        println!("  最新价: {}", info.current_price);
        
        assert_eq!(info.symbol, "CU2405");
        assert_eq!(info.name, "铜2405");
        println!("✅ 解析实时数据测试通过！");
    }

    /// 测试解析列表数据
    #[test]
    fn test_parse_list_data() {
        println!("\n========== 测试解析列表数据 ==========");
        let service = FuturesService::new();
        
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
        println!("  合约: {} - {}", info.symbol, info.name);
        assert_eq!(info.symbol, "CU2405");
        println!("✅ 解析列表数据测试通过！");
    }

    // ==================== 异步集成测试 ====================

    /// 测试动态获取品种映射
    #[tokio::test]
    async fn test_get_symbol_mark() {
        println!("\n========== 测试动态获取品种映射 ==========");
        let mut service = FuturesService::new();
        
        match service.get_symbol_mark().await {
            Ok(symbols) => {
                println!("✅ 获取成功！共 {} 个品种", symbols.len());
                println!("  前10个品种:");
                for s in symbols.iter().take(10) {
                    println!("    【{}】{} -> {}", s.exchange, s.symbol, s.mark);
                }
            }
            Err(e) => {
                println!("❌ 获取失败: {}", e);
            }
        }
    }

    /// 测试获取交易所品种
    #[tokio::test]
    async fn test_get_exchange_symbols() {
        println!("\n========== 测试获取交易所品种 ==========");
        let mut service = FuturesService::new();
        
        for exchange in &["SHFE", "DCE", "CZCE", "CFFEX"] {
            match service.get_exchange_symbols(exchange).await {
                Ok(symbols) => {
                    println!("  {} 品种数量: {}", exchange, symbols.len());
                    for s in symbols.iter().take(3) {
                        println!("    {} -> {}", s.symbol, s.mark);
                    }
                }
                Err(e) => {
                    println!("  {} 获取失败: {}", exchange, e);
                }
            }
        }
    }

    /// 测试获取单个期货实时数据
    #[tokio::test]
    async fn test_fetch_single_futures() {
        println!("\n========== 测试获取单个期货实时数据 ==========");
        let service = FuturesService::new();
        
        match service.get_futures_info("CU2602").await {
            Ok(info) => {
                println!("✅ 获取成功！");
                println!("  合约: {} - {}", info.symbol, info.name);
                println!("  最新价: {:.2}", info.current_price);
                println!("  涨跌幅: {:.2}%", info.change_percent);
            }
            Err(e) => {
                println!("❌ 获取失败: {}", e);
            }
        }
    }

    /// 测试获取期货列表
    #[tokio::test]
    async fn test_fetch_futures_list() {
        println!("\n========== 测试获取期货列表 ==========");
        let mut service = FuturesService::new();
        
        let query = FuturesQuery {
            symbol: None,
            exchange: Some("SHFE".to_string()),
            category: None,
            start_date: None,
            end_date: None,
            limit: Some(5),
        };
        
        match service.list_main_futures(&query).await {
            Ok(futures) => {
                println!("✅ 获取成功！共 {} 条", futures.len());
                for f in &futures {
                    println!("  【{}】{} - {:.2}", f.symbol, f.name, f.current_price);
                }
            }
            Err(e) => {
                println!("❌ 获取失败: {}", e);
            }
        }
    }

    /// 测试获取日K线数据
    #[tokio::test]
    async fn test_fetch_daily_kline() {
        println!("\n========== 测试获取日K线数据 ==========");
        
        let query = FuturesQuery {
            symbol: None,
            exchange: None,
            category: None,
            start_date: None,
            end_date: None,
            limit: Some(10),
        };
        
        match get_futures_history("CU2602", &query).await {
            Ok(history) => {
                println!("✅ 获取成功！共 {} 条", history.len());
                println!("{:<12} {:>10} {:>10} {:>10} {:>10}", "日期", "开盘", "最高", "最低", "收盘");
                for h in history.iter().take(5) {
                    println!("{:<12} {:>10.2} {:>10.2} {:>10.2} {:>10.2}", 
                        h.date, h.open, h.high, h.low, h.close);
                }
            }
            Err(e) => {
                println!("❌ 获取失败: {}", e);
            }
        }
    }

    /// 测试获取分钟K线数据
    #[tokio::test]
    async fn test_fetch_minute_kline() {
        println!("\n========== 测试获取分钟K线数据 ==========");
        
        match get_futures_minute_data("CU2602", "5").await {
            Ok(history) => {
                println!("✅ 获取成功！共 {} 条", history.len());
                println!("  最近5条:");
                for h in history.iter().rev().take(5) {
                    println!("    {} - O:{:.2} H:{:.2} L:{:.2} C:{:.2}", 
                        h.date, h.open, h.high, h.low, h.close);
                }
            }
            Err(e) => {
                println!("❌ 获取失败: {}", e);
            }
        }
    }

    /// 测试获取主力合约
    #[tokio::test]
    async fn test_get_main_contracts() {
        println!("\n========== 测试获取主力合约 ==========");
        let mut service = FuturesService::new();
        
        match service.get_main_contracts("SHFE").await {
            Ok(contracts) => {
                println!("✅ 获取成功！上期所主力合约:");
                for c in contracts.iter().take(5) {
                    println!("  {}", c);
                }
            }
            Err(e) => {
                println!("❌ 获取失败: {}", e);
            }
        }
    }

    /// 测试获取外盘期货行情
    #[tokio::test]
    async fn test_fetch_foreign_futures() {
        println!("\n========== 测试获取外盘期货行情 ==========");
        
        let codes = vec!["GC".to_string(), "SI".to_string(), "CL".to_string()];
        
        match get_foreign_futures_realtime(&codes).await {
            Ok(futures) => {
                println!("✅ 获取成功！共 {} 条", futures.len());
                for f in &futures {
                    println!("  【{}】{} - {:.2}", f.symbol, f.name, f.current_price);
                }
            }
            Err(e) => {
                println!("❌ 获取失败: {}", e);
            }
        }
    }

    // ==================== 新增API测试 ====================

    /// 测试获取主力连续合约一览表
    #[tokio::test]
    async fn test_futures_display_main_sina() {
        println!("\n========== 测试获取主力连续合约一览表 ==========");
        
        match get_futures_display_main_sina().await {
            Ok(contracts) => {
                println!("✅ 获取成功！共 {} 个主力连续合约", contracts.len());
                println!("\n  前20个合约:");
                for c in contracts.iter().take(20) {
                    println!("    【{}】{} - {}", c.exchange, c.symbol, c.name);
                }
            }
            Err(e) => {
                println!("❌ 获取失败: {}", e);
            }
        }
    }

    /// 测试获取主力连续日K线数据
    #[tokio::test]
    async fn test_futures_main_sina() {
        println!("\n========== 测试获取主力连续日K线数据 ==========");
        
        // 测试获取PVC连续合约数据
        match get_futures_main_sina("V0", None, None).await {
            Ok(data) => {
                println!("✅ 获取V0成功！共 {} 条数据", data.len());
                println!("\n  最近10条:");
                println!("  {:<12} {:>10} {:>10} {:>10} {:>10} {:>12} {:>12}", 
                    "日期", "开盘", "最高", "最低", "收盘", "成交量", "持仓量");
                for d in data.iter().rev().take(10) {
                    println!("  {:<12} {:>10.2} {:>10.2} {:>10.2} {:>10.2} {:>12} {:>12}", 
                        d.date, d.open, d.high, d.low, d.close, d.volume, d.hold);
                }
            }
            Err(e) => {
                println!("❌ 获取V0失败: {}", e);
            }
        }
        
        // 测试带日期范围
        println!("\n  测试日期范围过滤 (20240101-20240301):");
        match get_futures_main_sina("RB0", Some("20240101"), Some("20240301")).await {
            Ok(data) => {
                println!("  ✅ 获取RB0成功！范围内 {} 条数据", data.len());
                for d in data.iter().take(5) {
                    println!("    {} - O:{:.2} H:{:.2} L:{:.2} C:{:.2}", 
                        d.date, d.open, d.high, d.low, d.close);
                }
            }
            Err(e) => {
                println!("  ❌ 获取RB0失败: {}", e);
            }
        }
    }

    /// 测试获取期货持仓排名数据
    #[tokio::test]
    async fn test_futures_hold_pos_sina() {
        println!("\n========== 测试获取期货持仓排名数据 ==========");
        
        // 测试成交量排名
        println!("\n  1. 测试成交量排名:");
        match get_futures_hold_pos_sina("volume", "RB2510", "20250107").await {
            Ok(positions) => {
                println!("  ✅ 获取成功！共 {} 条", positions.len());
                println!("  {:<6} {:<20} {:>12} {:>12}", "名次", "期货公司", "成交量", "增减");
                for p in positions.iter().take(10) {
                    println!("  {:<6} {:<20} {:>12} {:>12}", p.rank, p.company, p.value, p.change);
                }
            }
            Err(e) => {
                println!("  ❌ 获取失败: {}", e);
            }
        }
        
        // 测试多单持仓排名
        println!("\n  2. 测试多单持仓排名:");
        match get_futures_hold_pos_sina("long", "RB2510", "20250107").await {
            Ok(positions) => {
                println!("  ✅ 获取成功！共 {} 条", positions.len());
                for p in positions.iter().take(5) {
                    println!("    {} - {} 多单:{} 增减:{}", p.rank, p.company, p.value, p.change);
                }
            }
            Err(e) => {
                println!("  ❌ 获取失败: {}", e);
            }
        }
        
        // 测试空单持仓排名
        println!("\n  3. 测试空单持仓排名:");
        match get_futures_hold_pos_sina("short", "RB2510", "20250107").await {
            Ok(positions) => {
                println!("  ✅ 获取成功！共 {} 条", positions.len());
                for p in positions.iter().take(5) {
                    println!("    {} - {} 空单:{} 增减:{}", p.rank, p.company, p.value, p.change);
                }
            }
            Err(e) => {
                println!("  ❌ 获取失败: {}", e);
            }
        }
    }

    /// 测试解析交易所品种nodes
    #[test]
    fn test_parse_exchange_nodes() {
        println!("\n========== 测试解析交易所品种nodes ==========");
        
        // 模拟JS数据
        let mock_js = r#"
        ARRFUTURESNODES = {
            czce: ['郑州商品交易所', ['PTA', 'pta_qh', '16'], ['白糖', 'sr_qh', '17']],
            dce: ['大连商品交易所', ['豆粕', 'm_qh', '1'], ['玉米', 'c_qh', '2']],
            shfe: ['上海期货交易所', ['铜', 'tong_qh', '3'], ['铝', 'lv_qh', '4']]
        };
        "#;
        
        for exchange in &["czce", "dce", "shfe"] {
            match parse_exchange_nodes(mock_js, exchange) {
                Ok(nodes) => {
                    println!("  {} 品种nodes: {:?}", exchange, nodes);
                }
                Err(e) => {
                    println!("  {} 解析失败: {}", exchange, e);
                }
            }
        }
        println!("✅ 解析测试完成！");
    }

    // ==================== 外盘期货历史数据测试 ====================

    /// 测试获取外盘期货历史数据
    #[tokio::test]
    async fn test_futures_foreign_hist() {
        println!("\n========== 测试获取外盘期货历史数据 ==========");
        
        // 测试LME锌
        println!("\n  1. 测试LME锌3个月(ZSD):");
        match get_futures_foreign_hist("ZSD").await {
            Ok(data) => {
                println!("  ✅ 获取成功！共 {} 条数据", data.len());
                println!("  {:<12} {:>10} {:>10} {:>10} {:>10} {:>12}", 
                    "日期", "开盘", "最高", "最低", "收盘", "成交量");
                for d in data.iter().rev().take(10) {
                    println!("  {:<12} {:>10.2} {:>10.2} {:>10.2} {:>10.2} {:>12}", 
                        d.date, d.open, d.high, d.low, d.close, d.volume);
                }
            }
            Err(e) => {
                println!("  ❌ 获取失败: {}", e);
            }
        }
        
        // 测试COMEX黄金
        println!("\n  2. 测试COMEX黄金(GC):");
        match get_futures_foreign_hist("GC").await {
            Ok(data) => {
                println!("  ✅ 获取成功！共 {} 条数据", data.len());
                for d in data.iter().rev().take(5) {
                    println!("    {} - O:{:.2} H:{:.2} L:{:.2} C:{:.2}", 
                        d.date, d.open, d.high, d.low, d.close);
                }
            }
            Err(e) => {
                println!("  ❌ 获取失败: {}", e);
            }
        }
        
        // 测试NYMEX原油
        println!("\n  3. 测试NYMEX原油(CL):");
        match get_futures_foreign_hist("CL").await {
            Ok(data) => {
                println!("  ✅ 获取成功！共 {} 条数据", data.len());
                for d in data.iter().rev().take(5) {
                    println!("    {} - O:{:.2} H:{:.2} L:{:.2} C:{:.2}", 
                        d.date, d.open, d.high, d.low, d.close);
                }
            }
            Err(e) => {
                println!("  ❌ 获取失败: {}", e);
            }
        }
    }

    /// 测试获取外盘期货合约详情
    #[tokio::test]
    async fn test_futures_foreign_detail() {
        println!("\n========== 测试获取外盘期货合约详情 ==========");
        
        // 测试LME锌
        println!("\n  1. 测试LME锌3个月(ZSD):");
        match get_futures_foreign_detail("ZSD").await {
            Ok(detail) => {
                println!("  ✅ 获取成功！共 {} 条详情项", detail.items.len());
                for item in &detail.items {
                    println!("    {}: {}", item.name, item.value);
                }
            }
            Err(e) => {
                println!("  ❌ 获取失败: {}", e);
            }
        }
        
        // 测试COMEX黄金
        println!("\n  2. 测试COMEX黄金(GC):");
        match get_futures_foreign_detail("GC").await {
            Ok(detail) => {
                println!("  ✅ 获取成功！共 {} 条详情项", detail.items.len());
                for item in detail.items.iter().take(10) {
                    println!("    {}: {}", item.name, item.value);
                }
            }
            Err(e) => {
                println!("  ❌ 获取失败: {}", e);
            }
        }
    }
}
