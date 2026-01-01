//! 期货数据服务
//! 
//! 提供期货数据的获取和处理逻辑，参考 akshare 实现
//! 
//! ## 数据来源
//! - 新浪财经：实时行情、K线数据、持仓排名
//! - 100ppi：现货价格及基差数据
//! - 99期货网：库存数据
//! - OpenCTP：交易费用数据
//! - 国泰君安：交易规则数据
//! 
//! ## 主要功能
//! - 期货实时行情获取
//! - 日K线/分钟K线数据
//! - 品种映射和交易所信息
//! - 主力连续合约数据
//! - 持仓排名数据
//! - 外盘期货数据
//! - 现货价格及基差
//! - 交易费用和规则

#![allow(dead_code)]

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
    FuturesFeesInfo, FuturesCommInfo, FuturesRule,
    Futures99Symbol, FuturesInventory99, FuturesSpotPrice, FuturesSpotPricePrevious,
    PositionRankData, RankTableResponse, RankSum,
    CzceWarehouseReceipt, CzceWarehouseReceiptResponse,
    DceWarehouseReceipt,
    ShfeWarehouseReceipt, ShfeWarehouseReceiptResponse,
    GfexWarehouseReceipt, GfexWarehouseReceiptResponse,
    SinaHoldPosition
};

/// 获取北京时间字符串（ISO 8601 格式，带+08:00时区）
fn get_beijing_time() -> String {
    Utc::now().with_timezone(&Shanghai).to_rfc3339()
}

// ==================== 新浪期货 API 常量 ====================

/// 新浪期货实时行情 API
const SINA_FUTURES_REALTIME_API: &str = "https://hq.sinajs.cn";
/// 新浪期货列表 API
const SINA_FUTURES_LIST_API: &str = "https://vip.stock.finance.sina.com.cn/quotes_service/api/json_v2.php/Market_Center.getHQFuturesData";
/// 新浪期货品种映射 JS 文件
const SINA_FUTURES_SYMBOL_URL: &str = "https://vip.stock.finance.sina.com.cn/quotes_service/view/js/qihuohangqing.js";
/// 新浪期货日K线 API
const SINA_FUTURES_DAILY_API: &str = "https://stock2.finance.sina.com.cn/futures/api/jsonp.php/var%20_temp=/InnerFuturesNewService.getDailyKLine";
/// 新浪期货分钟K线 API
const SINA_FUTURES_MINUTE_API: &str = "https://stock2.finance.sina.com.cn/futures/api/jsonp.php/=/InnerFuturesNewService.getFewMinLine";
/// 新浪期货合约详情页面
const SINA_CONTRACT_DETAIL_URL: &str = "https://finance.sina.com.cn/futures/quotes";

/// 期货数据服务
/// 
/// 封装期货数据的获取逻辑，参考 akshare/futures/futures_zh_sina.py 实现
/// 
/// ## 功能
/// - 品种映射：获取期货品种和代码的映射关系
/// - 实时行情：获取单个或多个合约的实时数据
/// - K线数据：获取日K线和分钟K线
/// - 合约详情：获取合约的交易规则
pub struct FuturesService {
    /// HTTP 客户端
    client: Client,
    /// 品种映射缓存
    symbol_mark_cache: Option<Vec<FuturesSymbolMark>>,
}

impl FuturesService {
    /// 创建新的期货服务实例
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            symbol_mark_cache: None,
        }
    }

    // ==================== 品种映射相关 ====================

    /// 获取期货品种和代码映射表
    /// 
    /// 对应 akshare 的 futures_symbol_mark() 函数
    /// 从新浪 JS 文件动态解析品种信息
    /// 
    /// # 返回
    /// 品种映射列表，包含交易所、品种名称和 node 参数
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
        
        // 解析 JS 中的品种数据
        let symbols = self.parse_symbol_mark_js(&text)?;
        
        // 缓存结果
        self.symbol_mark_cache = Some(symbols.clone());
        
        Ok(symbols)
    }

    /// 解析新浪 JS 文件中的品种映射数据
    /// 
    /// JS 格式: ARRFUTURESNODES = { czce: ['郑州商品交易所', ['PTA', 'pta_qh', '16'], ...], ... }
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
    /// 支持精确匹配和模糊匹配（包含关键字）
    pub async fn get_symbol_node(&mut self, symbol: &str) -> Result<String> {
        let symbols = self.get_symbol_mark().await?;
        
        // 先尝试精确匹配
        for s in &symbols {
            if s.symbol == symbol {
                return Ok(s.mark.clone());
            }
        }
        
        // 再尝试模糊匹配（品种名包含输入关键字）
        for s in &symbols {
            if s.symbol.contains(symbol) {
                return Ok(s.mark.clone());
            }
        }
        
        Err(anyhow!("未找到品种 {} 的映射，请使用 /futures/symbols 查看可用品种", symbol))
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
    
    // 查找tbody内容
    let tbody_start = html.find("<tbody>");
    let tbody_end = html.find("</tbody>");
    
    if tbody_start.is_none() || tbody_end.is_none() {
        return Err(anyhow!("未找到费用数据表格"));
    }
    
    let tbody_content = &html[tbody_start.unwrap()..tbody_end.unwrap()];
    
    // 按行分割
    for row in tbody_content.split("<tr>").skip(1) {
        // 提取所有td内容
        let cells: Vec<String> = row.split("<td")
            .skip(1)
            .filter_map(|cell| {
                // 找到>和</td>之间的内容
                let start = cell.find('>')?;
                let end = cell.find("</td>")?;
                let content = &cell[start + 1..end];
                // 移除style属性等HTML标签
                let clean = content
                    .replace("style=\"background-color:yellow;\"", "")
                    .replace("style=\"background-color:red;\"", "")
                    .trim()
                    .to_string();
                Some(clean)
            })
            .collect();
        
        // 表格列: 交易所(0), 合约代码(1), 合约名称(2), 品种代码(3), 品种名称(4), 
        // 合约乘数(5), 最小跳动(6), 开仓费率(7), 开仓费用/手(8), 平仓费率(9), 
        // 平仓费用/手(10), 平今费率(11), 平今费用/手(12), 做多保证金率(13), 
        // 做多保证金/手(14), 做空保证金率(15), ...
        if cells.len() >= 16 {
            fees_list.push(FuturesFeesInfo {
                exchange: cells[0].clone(),
                contract_code: cells[1].clone(),
                contract_name: cells[2].clone(),
                product_code: cells[3].clone(),
                product_name: cells[4].clone(),
                contract_size: cells[5].clone(),
                price_tick: cells[6].clone(),
                open_fee_rate: cells[7].clone(),
                open_fee: cells[8].clone(),
                close_fee_rate: cells[9].clone(),
                close_fee: cells[10].clone(),
                close_today_fee_rate: cells[11].clone(),
                close_today_fee: cells[12].clone(),
                long_margin_rate: cells[13].clone(),
                short_margin_rate: cells[15].clone(),
                updated_at: updated_at.clone(),
            });
        }
    }
    
    println!("📊 解析到 {} 条期货费用数据", fees_list.len());
    Ok(fees_list)
}

/// 获取期货手续费信息
/// 对应 akshare 的 futures_comm_info() 函数
/// 数据来源: https://www.9qihuo.com/qihuoshouxufei
/// 注意: 九期网数据源目前不可用，建议使用 futures_fees_info (OpenCTP) 替代
/// exchange: 交易所名称，可选值：所有/上海期货交易所/大连商品交易所/郑州商品交易所/上海国际能源交易中心/中国金融期货交易所/广州期货交易所
pub async fn get_futures_comm_info(_exchange: Option<&str>) -> Result<Vec<FuturesCommInfo>> {
    // 九期网数据源目前不可用，直接返回错误
    // 建议使用 get_futures_fees_info() (OpenCTP数据源) 替代
    Err(anyhow!(
        "九期网数据源(9qihuo.com)目前不可用，请使用 /futures/fees 接口(OpenCTP数据源)获取期货手续费信息"
    ))
}

// ==================== 期货交易规则相关 ====================

/// 国泰君安期货交易日历API
const GTJA_CALENDAR_URL: &str = "https://www.gtjaqh.com/pc/calendar";

/// 获取期货交易规则
/// 对应 akshare 的 futures_rule() 函数
/// 数据来源: https://www.gtjaqh.com/pc/calendar.html
/// date: 交易日期，格式 YYYYMMDD，需要指定为交易日且是近期的日期
pub async fn get_futures_rule(date: Option<&str>) -> Result<Vec<FuturesRule>> {
    let client = Client::builder()
        .danger_accept_invalid_certs(true)  // 忽略SSL证书验证
        .build()?;
    
    // 默认使用当前日期
    let query_date = date.unwrap_or_else(|| {
        let now = Utc::now().with_timezone(&Shanghai);
        Box::leak(now.format("%Y%m%d").to_string().into_boxed_str())
    });
    
    let url = format!("{}?date={}", GTJA_CALENDAR_URL, query_date);
    println!("📡 请求期货交易规则数据 URL: {}", url);
    
    let response = client
        .get(&url)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8")
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(anyhow!("获取期货交易规则数据失败: {}", response.status()));
    }

    let text = response.text().await?;
    parse_futures_rule_html(&text)
}

/// 解析期货交易规则HTML
fn parse_futures_rule_html(html: &str) -> Result<Vec<FuturesRule>> {
    use scraper::{Html, Selector};
    
    let mut rules = Vec::new();
    
    // 检查是否包含交易规则数据
    if !html.contains("交易保证金比例") && !html.contains("涨跌停板幅度") {
        return Err(anyhow!("未找到交易规则数据表格"));
    }
    
    // 使用scraper解析HTML
    let document = Html::parse_document(html);
    
    // 选择所有表格行
    let tr_selector = Selector::parse("tr").unwrap();
    let td_selector = Selector::parse("td").unwrap();
    let th_selector = Selector::parse("th").unwrap();
    
    for row in document.select(&tr_selector) {
        // 提取所有单元格（td和th）
        let mut cells: Vec<String> = Vec::new();
        
        // 先尝试td
        for cell in row.select(&td_selector) {
            let text = cell.text().collect::<Vec<_>>().join("").trim().to_string();
            cells.push(text);
        }
        
        // 如果没有td，尝试th（表头行）
        if cells.is_empty() {
            for cell in row.select(&th_selector) {
                let text = cell.text().collect::<Vec<_>>().join("").trim().to_string();
                cells.push(text);
            }
        }
        
        // 跳过只有一个单元格的行（日期行）
        if cells.len() <= 1 {
            continue;
        }
        
        // 只检查前4列来判断是否为表头行（避免误判数据行中的特殊说明列）
        let header_cells: Vec<&String> = cells.iter().take(4).collect();
        let is_header = header_cells.iter().any(|c| {
            c.contains("交易所") || c.contains("交易保证金比例") || 
            *c == "品种" || c.contains("保证金收取标准")
        });
        
        if is_header {
            continue;
        }
        
        // 数据行至少需要6列
        if cells.len() >= 6 {
            let exchange = cells.get(0).cloned().unwrap_or_default();
            let product = cells.get(1).cloned().unwrap_or_default();
            let code = cells.get(2).cloned().unwrap_or_default();
            
            // 跳过空行或表头行
            if exchange.is_empty() && product.is_empty() {
                continue;
            }
            if exchange == "交易所" || product == "品种" {
                continue;
            }
            
            let margin_rate = cells.get(3)
                .and_then(|s| {
                    let s = s.trim_end_matches('%').trim();
                    if s == "--" || s.is_empty() { None } else { s.parse::<f64>().ok() }
                });
            
            let price_limit = cells.get(4)
                .and_then(|s| {
                    let s = s.trim_end_matches('%').trim();
                    if s == "--" || s.is_empty() { None } else { s.parse::<f64>().ok() }
                });
            
            let contract_size = cells.get(5).and_then(|s| s.parse::<f64>().ok());
            let price_tick = cells.get(6).and_then(|s| s.parse::<f64>().ok());
            let max_order_size = cells.get(7).and_then(|s| s.parse::<u64>().ok());
            let special_note = cells.get(8).cloned().filter(|s| !s.is_empty());
            let remark = cells.get(9).cloned().filter(|s| !s.is_empty());
            
            rules.push(FuturesRule {
                exchange,
                product,
                code,
                margin_rate,
                price_limit,
                contract_size,
                price_tick,
                max_order_size,
                special_note,
                remark,
            });
        }
    }
    
    println!("📊 解析到 {} 条期货交易规则数据", rules.len());
    Ok(rules)
}

// ==================== 99期货网库存数据 ====================

const QH99_STOCK_URL: &str = "https://www.99qh.com/data/stockIn";

/// 获取99期货网品种映射表
/// 对应 akshare 的 __get_99_symbol_map() 函数
pub async fn get_99_symbol_map() -> Result<Vec<Futures99Symbol>> {
    let client = Client::builder()
        .danger_accept_invalid_certs(true)
        .build()?;
    
    println!("📡 请求99期货网品种映射 URL: {}", QH99_STOCK_URL);
    
    let response = client
        .get(QH99_STOCK_URL)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(anyhow!("获取99期货网品种映射失败: {}", response.status()));
    }

    let text = response.text().await?;
    
    // 使用scraper解析HTML，提取__NEXT_DATA__中的JSON
    use scraper::{Html, Selector};
    let document = Html::parse_document(&text);
    let script_selector = Selector::parse("script#__NEXT_DATA__").unwrap();
    
    let script = document.select(&script_selector).next()
        .ok_or_else(|| anyhow!("未找到__NEXT_DATA__脚本标签"))?;
    
    let json_text = script.text().collect::<String>();
    let json_data: serde_json::Value = serde_json::from_str(&json_text)
        .map_err(|e| anyhow!("解析JSON失败: {}", e))?;
    
    let mut symbols = Vec::new();
    
    // 解析品种列表
    if let Some(variety_list) = json_data["props"]["pageProps"]["data"]["varietyListData"].as_array() {
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
    // 获取品种映射
    let symbols = get_99_symbol_map().await?;
    
    // 查找品种ID
    let product_id = symbols.iter()
        .find(|s| s.name == symbol || s.code.eq_ignore_ascii_case(symbol))
        .map(|s| s.product_id)
        .ok_or_else(|| anyhow!("未找到品种 {} 对应的编号", symbol))?;
    
    println!("📡 品种 {} 对应的ID: {}", symbol, product_id);
    
    let client = Client::builder()
        .danger_accept_invalid_certs(true)
        .build()?;
    
    // 直接从页面获取数据（包含图表数据）
    let url = format!("{}?productId={}", QH99_STOCK_URL, product_id);
    println!("📡 请求99期货网库存数据 URL: {}", url);
    
    let response = client
        .get(&url)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(anyhow!("获取99期货网库存数据失败: {}", response.status()));
    }

    let text = response.text().await?;
    
    // 使用scraper解析HTML，提取__NEXT_DATA__中的JSON
    use scraper::{Html, Selector};
    let document = Html::parse_document(&text);
    let script_selector = Selector::parse("script#__NEXT_DATA__").unwrap();
    
    let script = document.select(&script_selector).next()
        .ok_or_else(|| anyhow!("未找到__NEXT_DATA__脚本标签"))?;
    
    let json_text = script.text().collect::<String>();
    let json_data: serde_json::Value = serde_json::from_str(&json_text)
        .map_err(|e| anyhow!("解析JSON失败: {}", e))?;
    
    let mut inventory_list = Vec::new();
    
    // 从positionTrendChartListData.list获取数据
    // 格式: [日期, 收盘价, 库存]
    if let Some(list) = json_data["props"]["pageProps"]["data"]["positionTrendChartListData"]["list"].as_array() {
        for item in list {
            if let Some(arr) = item.as_array() {
                let date = arr.get(0)
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                
                let close_price = arr.get(1)
                    .and_then(|v| {
                        if v.is_null() { None }
                        else if let Some(s) = v.as_str() { s.parse::<f64>().ok() }
                        else { v.as_f64() }
                    });
                
                let inventory = arr.get(2)
                    .and_then(|v| {
                        if v.is_null() { None }
                        else if let Some(n) = v.as_i64() { Some(n as f64) }
                        else if let Some(n) = v.as_f64() { Some(n) }
                        else { None }
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
    
    // 按日期排序
    inventory_list.sort_by(|a, b| a.date.cmp(&b.date));
    
    println!("📊 解析到 {} 条库存数据", inventory_list.len());
    Ok(inventory_list)
}

// ==================== 现货价格及基差数据 ====================

const SPOT_PRICE_URL: &str = "https://www.100ppi.com/sf";

/// 中文品种名称到英文代码的映射
fn chinese_to_english(name: &str) -> Option<&'static str> {
    // 先尝试精确匹配
    let result = match name {
        // 上海期货交易所
        "铜" => Some("CU"),
        "螺纹钢" => Some("RB"),
        "锌" => Some("ZN"),
        "铝" => Some("AL"),
        "黄金" => Some("AU"),
        "线材" => Some("WR"),
        "天然橡胶" => Some("RU"),
        "铅" => Some("PB"),
        "白银" => Some("AG"),
        "沥青" | "石油沥青" => Some("BU"),
        "热轧卷板" => Some("HC"),
        "镍" => Some("NI"),
        "锡" => Some("SN"),
        "燃料油" => Some("FU"),
        "不锈钢" => Some("SS"),
        "纸浆" => Some("SP"),
        "氧化铝" => Some("AO"),
        "丁二烯橡胶" => Some("BR"),
        // 大连商品交易所
        "豆一" => Some("A"),
        "豆二" => Some("B"),
        "豆粕" => Some("M"),
        "豆油" => Some("Y"),
        "玉米" => Some("C"),
        "玉米淀粉" => Some("CS"),
        "棕榈油" => Some("P"),
        "鸡蛋" => Some("JD"),
        "聚乙烯" | "LLDPE" => Some("L"),
        "聚氯乙烯" | "PVC" => Some("V"),
        "聚丙烯" | "PP" => Some("PP"),
        "焦炭" => Some("J"),
        "焦煤" => Some("JM"),
        "铁矿石" => Some("I"),
        "乙二醇" => Some("EG"),
        "苯乙烯" => Some("EB"),
        "液化石油气" | "LPG" => Some("PG"),
        "生猪" => Some("LH"),
        // 郑州商品交易所
        "白糖" => Some("SR"),
        "棉花" => Some("CF"),
        "PTA" => Some("TA"),
        "菜籽油" | "菜油" | "菜籽油OI" => Some("OI"),
        "菜籽粕" | "菜粕" => Some("RM"),
        "甲醇" | "甲醇MA" => Some("MA"),
        "玻璃" => Some("FG"),
        "动力煤" => Some("ZC"),
        "硅铁" => Some("SF"),
        "锰硅" => Some("SM"),
        "苹果" => Some("AP"),
        "红枣" => Some("CJ"),
        "尿素" => Some("UR"),
        "纯碱" => Some("SA"),
        "短纤" | "涤纶短纤" => Some("PF"),
        "花生" => Some("PK"),
        "菜籽" => Some("RS"),
        "棉纱" => Some("CY"),
        "粳稻" => Some("JR"),
        "晚籼稻" => Some("LR"),
        "早籼稻" => Some("RI"),
        "强麦" | "强麦WH" => Some("WH"),
        "普麦" => Some("PM"),
        "烧碱" => Some("SH"),
        // 上海国际能源交易中心
        "原油" => Some("SC"),
        "20号胶" => Some("NR"),
        "低硫燃料油" => Some("LU"),
        "国际铜" => Some("BC"),
        // 广州期货交易所
        "工业硅" => Some("SI"),
        "碳酸锂" => Some("LC"),
        // 中国金融期货交易所
        "沪深300" => Some("IF"),
        "上证50" => Some("IH"),
        "中证500" => Some("IC"),
        "中证1000" => Some("IM"),
        "2年期国债" => Some("TS"),
        "5年期国债" => Some("TF"),
        "10年期国债" => Some("T"),
        "30年期国债" => Some("TL"),
        // 其他别名
        "PX" => Some("PX"),
        _ => None,
    };
    
    if result.is_some() {
        return result;
    }
    
    // 如果精确匹配失败，尝试模糊匹配（处理带代码后缀的情况）
    if name.contains("菜籽油") { return Some("OI"); }
    if name.contains("甲醇") { return Some("MA"); }
    if name.contains("强麦") { return Some("WH"); }
    if name.contains("棉纱") { return Some("CY"); }
    
    None
}

/// 获取期货现货价格及基差数据
/// 对应 akshare 的 futures_spot_price() 函数
/// 数据来源: https://www.100ppi.com/sf/
/// date: 交易日期，格式 YYYYMMDD
/// symbols: 品种代码列表，为空时返回所有品种
pub async fn get_futures_spot_price(date: &str, symbols: Option<Vec<&str>>) -> Result<Vec<FuturesSpotPrice>> {
    use scraper::{Html, Selector};
    
    // 格式化日期
    let formatted_date = if date.len() == 8 {
        format!("{}-{}-{}", &date[0..4], &date[4..6], &date[6..8])
    } else {
        date.to_string()
    };
    
    let url = format!("{}/day-{}.html", SPOT_PRICE_URL, formatted_date);
    println!("📡 请求现货价格数据 URL: {}", url);
    
    let client = Client::new();
    let response = client
        .get(&url)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8")
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(anyhow!("获取现货价格数据失败: {}", response.status()));
    }

    // 网页是UTF-8编码
    let text = response.text().await?;
    
    // 解析HTML
    let document = Html::parse_document(&text);
    
    // 查找ID为fdata的表格
    let table_selector = Selector::parse("table#fdata").unwrap();
    let tr_selector = Selector::parse("tr").unwrap();
    let td_selector = Selector::parse("td").unwrap();
    
    let mut spot_prices = Vec::new();
    
    let main_table = document.select(&table_selector).next();
    if main_table.is_none() {
        return Err(anyhow!("未找到数据表格(#fdata)"));
    }
    
    let main_table = main_table.unwrap();
    let rows: Vec<_> = main_table.select(&tr_selector).collect();
    
    for row in rows {
        let cells: Vec<String> = row.select(&td_selector)
            .map(|cell| cell.text().collect::<Vec<_>>().join("").trim().to_string())
            .collect();
        
        // 数据行有12个单元格：品种、现货价、近月代码、近月价、近月基差合并、近月基差、近月基差率、主力代码、主力价、主力基差合并、主力基差、主力基差率
        // 跳过非数据行（少于10个单元格）
        if cells.len() < 10 {
            continue;
        }
        
        let first_cell = cells[0].replace('\u{a0}', "").trim().to_string();
        
        // 跳过表头行和交易所分隔行
        if first_cell.contains("交易所") || first_cell == "商品" || first_cell.is_empty() {
            continue;
        }
        
        // 尝试解析品种名称
        let chinese_name = first_cell.trim();
        let symbol = match chinese_to_english(chinese_name) {
            Some(s) => s.to_string(),
            None => {
                // 如果是英文代码（如PTA），直接使用
                if chinese_name.chars().all(|c| c.is_ascii_alphabetic()) {
                    chinese_name.to_uppercase()
                } else {
                    continue;
                }
            }
        };
        
        // 如果指定了品种列表，检查是否在列表中
        if let Some(ref filter_symbols) = symbols {
            if !filter_symbols.iter().any(|s| s.eq_ignore_ascii_case(&symbol)) {
                continue;
            }
        }
        
        // 解析数值
        // 第1列(index 0): 品种名
        // 第2列(index 1): 现货价格
        // 第3列(index 2): 近月合约代码
        // 第4列(index 3): 近月价格
        // 第5-7列: 近月基差相关
        // 第8列(index 7): 主力合约代码
        // 第9列(index 8): 主力价格
        // 第10-12列: 主力基差相关
        
        let spot_price = cells.get(1)
            .map(|s| s.replace('\u{a0}', "").replace(",", ""))
            .and_then(|s| s.trim().parse::<f64>().ok())
            .unwrap_or(0.0);
        
        if spot_price == 0.0 {
            continue;
        }
        
        let near_contract_raw = cells.get(2)
            .map(|s| s.replace('\u{a0}', ""))
            .unwrap_or_default();
        let near_contract_price = cells.get(3)
            .map(|s| s.replace('\u{a0}', "").replace(",", ""))
            .and_then(|s| s.trim().parse::<f64>().ok())
            .unwrap_or(0.0);
        
        let dominant_contract_raw = cells.get(7)
            .map(|s| s.replace('\u{a0}', ""))
            .unwrap_or_default();
        let dominant_contract_price = cells.get(8)
            .map(|s| s.replace('\u{a0}', "").replace(",", ""))
            .and_then(|s| s.trim().parse::<f64>().ok())
            .unwrap_or(0.0);
        
        // 提取合约月份并构建合约代码
        let near_month = extract_contract_month(&near_contract_raw);
        let dominant_month = extract_contract_month(&dominant_contract_raw);
        
        let near_contract = format!("{}{}", symbol.to_lowercase(), near_month);
        let dominant_contract = format!("{}{}", symbol.to_lowercase(), dominant_month);
        
        // 计算基差
        // 基差 = 期货价格 - 现货价格
        let near_basis = near_contract_price - spot_price;
        let dom_basis = dominant_contract_price - spot_price;
        
        // 计算基差率
        let near_basis_rate = if spot_price != 0.0 {
            near_contract_price / spot_price - 1.0
        } else {
            0.0
        };
        
        let dom_basis_rate = if spot_price != 0.0 {
            dominant_contract_price / spot_price - 1.0
        } else {
            0.0
        };
        
        spot_prices.push(FuturesSpotPrice {
            date: date.replace("-", ""),
            symbol,
            spot_price,
            near_contract,
            near_contract_price,
            dominant_contract,
            dominant_contract_price,
            near_basis,
            dom_basis,
            near_basis_rate,
            dom_basis_rate,
        });
    }
    
    println!("📊 解析到 {} 条现货价格数据", spot_prices.len());
    Ok(spot_prices)
}

/// 从合约代码中提取月份
fn extract_contract_month(contract: &str) -> String {
    // 提取数字部分
    let digits: String = contract.chars().filter(|c| c.is_ascii_digit()).collect();
    if digits.len() >= 4 {
        digits[digits.len()-4..].to_string()
    } else {
        digits
    }
}

// ==================== 现货价格历史数据（sf2） ====================

const SPOT_PRICE_PREVIOUS_URL: &str = "https://www.100ppi.com/sf2";

/// 获取期货现货价格及基差历史数据（包含180日统计）
/// 对应 akshare 的 futures_spot_price_previous() 函数
/// 数据来源: https://www.100ppi.com/sf2/
/// date: 交易日期，格式 YYYYMMDD
pub async fn get_futures_spot_price_previous(date: &str) -> Result<Vec<FuturesSpotPricePrevious>> {
    use scraper::{Html, Selector};
    
    // 格式化日期
    let formatted_date = if date.len() == 8 {
        format!("{}-{}-{}", &date[0..4], &date[4..6], &date[6..8])
    } else {
        date.to_string()
    };
    
    let url = format!("{}/day-{}.html", SPOT_PRICE_PREVIOUS_URL, formatted_date);
    println!("📡 请求现货价格历史数据 URL: {}", url);
    
    let client = Client::new();
    let response = client
        .get(&url)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8")
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(anyhow!("获取现货价格历史数据失败: {}", response.status()));
    }

    let text = response.text().await?;
    
    // 解析HTML
    let document = Html::parse_document(&text);
    
    // 查找ID为fdata的表格
    let table_selector = Selector::parse("table#fdata").unwrap();
    let tr_selector = Selector::parse("tr").unwrap();
    let td_selector = Selector::parse("td").unwrap();
    
    let mut spot_prices = Vec::new();
    
    let main_table = document.select(&table_selector).next();
    if main_table.is_none() {
        return Err(anyhow!("未找到数据表格(#fdata)"));
    }
    
    let main_table = main_table.unwrap();
    let rows: Vec<_> = main_table.select(&tr_selector).collect();
    
    for row in rows {
        let cells: Vec<String> = row.select(&td_selector)
            .map(|cell| cell.text().collect::<Vec<_>>().join("").trim().to_string())
            .collect();
        
        // sf2页面的数据行有8列或更多
        // 商品、现货价格、主力合约代码、主力合约价格、主力合约基差、180日最高、180日最低、180日平均
        if cells.len() < 8 {
            continue;
        }
        
        let first_cell = cells[0].replace('\u{a0}', "").trim().to_string();
        
        // 跳过表头行和交易所分隔行
        if first_cell.contains("交易所") || first_cell == "商品" || first_cell.is_empty() {
            continue;
        }
        
        // 解析现货价格
        let spot_price = cells.get(1)
            .map(|s| s.replace('\u{a0}', "").replace(",", ""))
            .and_then(|s| s.trim().parse::<f64>().ok())
            .unwrap_or(0.0);
        
        if spot_price == 0.0 {
            continue;
        }
        
        // 主力合约代码
        let dominant_contract = cells.get(2)
            .map(|s| s.replace('\u{a0}', "").trim().to_string())
            .unwrap_or_default();
        
        // 主力合约价格
        let dominant_price = cells.get(3)
            .map(|s| s.replace('\u{a0}', "").replace(",", ""))
            .and_then(|s| s.trim().parse::<f64>().ok())
            .unwrap_or(0.0);
        
        // 主力合约基差（格式如 "-176-0.22%" 或 "80.03%"）
        let basis_str = cells.get(4)
            .map(|s| s.replace('\u{a0}', ""))
            .unwrap_or_default();
        
        let (basis, basis_rate) = parse_basis_string(&basis_str);
        
        // 180日统计数据
        let basis_180d_high = cells.get(5)
            .map(|s| s.replace('\u{a0}', "").replace(",", ""))
            .and_then(|s| s.trim().parse::<f64>().ok());
        
        let basis_180d_low = cells.get(6)
            .map(|s| s.replace('\u{a0}', "").replace(",", ""))
            .and_then(|s| s.trim().parse::<f64>().ok());
        
        let basis_180d_avg = cells.get(7)
            .map(|s| s.replace('\u{a0}', "").replace(",", ""))
            .and_then(|s| s.trim().parse::<f64>().ok());
        
        spot_prices.push(FuturesSpotPricePrevious {
            commodity: first_cell,
            spot_price,
            dominant_contract,
            dominant_price,
            basis,
            basis_rate,
            basis_180d_high,
            basis_180d_low,
            basis_180d_avg,
        });
    }
    
    println!("📊 解析到 {} 条现货价格历史数据", spot_prices.len());
    Ok(spot_prices)
}

/// 解析基差字符串，如 "-176-0.22%" 或 "80.03%"
fn parse_basis_string(s: &str) -> (f64, f64) {
    let s = s.trim();
    
    if s.is_empty() {
        return (0.0, 0.0);
    }
    
    // 查找百分号位置
    if let Some(pct_pos) = s.rfind('%') {
        let before_pct = &s[..pct_pos];
        
        // 尝试找到基差率的起始位置
        // 格式可能是: "-176-0.22" 或 "80.03" 或 "2309.00"
        // 从后往前找，找到基差率部分
        
        // 先尝试解析整个字符串为数字（只有基差率的情况）
        if let Ok(rate) = before_pct.parse::<f64>() {
            return (0.0, rate);
        }
        
        // 否则尝试分离基差和基差率
        // 查找最后一个负号或正号（不在开头的）
        let chars: Vec<char> = before_pct.chars().collect();
        let mut split_pos = None;
        
        for i in (1..chars.len()).rev() {
            if chars[i] == '-' || chars[i] == '+' {
                // 检查前一个字符是否是数字（确保这是分隔符而不是负号）
                if i > 0 && chars[i-1].is_ascii_digit() {
                    split_pos = Some(i);
                    break;
                }
            }
        }
        
        if let Some(pos) = split_pos {
            let basis_str: String = chars[..pos].iter().collect();
            let rate_str: String = chars[pos..].iter().collect();
            
            let basis = basis_str.parse::<f64>().unwrap_or(0.0);
            let rate = rate_str.parse::<f64>().unwrap_or(0.0);
            
            return (basis, rate);
        }
        
        // 如果没有找到分隔符，整个都是基差率
        let rate = before_pct.parse::<f64>().unwrap_or(0.0);
        return (0.0, rate);
    }
    
    // 没有百分号，尝试直接解析为基差
    let basis = s.parse::<f64>().unwrap_or(0.0);
    (basis, 0.0)
}

/// 获取期货现货价格日线数据（日期范围）
/// 对应 akshare 的 futures_spot_price_daily() 函数
/// 数据来源: https://www.100ppi.com/sf/
/// start_date: 开始日期，格式 YYYYMMDD
/// end_date: 结束日期，格式 YYYYMMDD
/// symbols: 品种代码列表，为空时返回所有品种
pub async fn get_futures_spot_price_daily(
    start_date: &str, 
    end_date: &str, 
    symbols: Option<Vec<&str>>
) -> Result<Vec<FuturesSpotPrice>> {
    use chrono::NaiveDate;
    
    // 解析日期
    let start = NaiveDate::parse_from_str(start_date, "%Y%m%d")
        .map_err(|e| anyhow!("无效的开始日期格式: {}", e))?;
    let end = NaiveDate::parse_from_str(end_date, "%Y%m%d")
        .map_err(|e| anyhow!("无效的结束日期格式: {}", e))?;
    
    if start > end {
        return Err(anyhow!("开始日期不能大于结束日期"));
    }
    
    println!("📡 获取现货价格日线数据: {} 至 {}", start_date, end_date);
    
    let mut all_data = Vec::new();
    let mut current = start;
    
    while current <= end {
        let date_str = current.format("%Y%m%d").to_string();
        
        // 获取当天数据
        match get_futures_spot_price(&date_str, symbols.clone()).await {
            Ok(data) => {
                if !data.is_empty() {
                    all_data.extend(data);
                }
            }
            Err(e) => {
                // 非交易日或数据缺失，跳过
                println!("  ⚠️ {} 数据获取失败（可能是非交易日）: {}", date_str, e);
            }
        }
        
        // 下一天
        current = current.succ_opt().unwrap_or(current);
    }
    
    println!("📊 共获取 {} 条现货价格日线数据", all_data.len());
    Ok(all_data)
}

/// 解析期货手续费HTML
#[allow(dead_code)]
fn parse_comm_info_html(html: &str, exchange_filter: Option<&str>) -> Result<Vec<FuturesCommInfo>> {
    let mut all_data = Vec::new();
    
    // 查找表格
    let table_re = Regex::new(r"<table[^>]*>([\s\S]*?)</table>").unwrap();
    let tables: Vec<_> = table_re.captures_iter(html).collect();
    
    if tables.is_empty() {
        return Err(anyhow!("未找到手续费数据表格"));
    }
    
    // 获取第一个表格（主数据表格）
    let table_content = tables[0].get(1).map(|m| m.as_str()).unwrap_or("");
    
    // 解析表格行
    let row_re = Regex::new(r"<tr[^>]*>([\s\S]*?)</tr>").unwrap();
    let cell_re = Regex::new(r"<td[^>]*>([\s\S]*?)</td>").unwrap();
    
    // 清理HTML标签
    let clean_html = |s: &str| -> String {
        let tag_re = Regex::new(r"<[^>]+>").unwrap();
        tag_re.replace_all(s, "").trim().to_string()
    };
    
    // 交易所分隔标记
    let exchange_markers = vec![
        "上海期货交易所",
        "大连商品交易所", 
        "郑州商品交易所",
        "上海国际能源交易中心",
        "广州期货交易所",
        "中国金融期货交易所",
    ];
    
    let mut current_exchange = String::new();
    let mut skip_header_rows = 0;
    
    for row_cap in row_re.captures_iter(table_content) {
        let row_content = row_cap.get(1).map(|m| m.as_str()).unwrap_or("");
        let cells: Vec<String> = cell_re.captures_iter(row_content)
            .filter_map(|c| c.get(1).map(|m| clean_html(m.as_str())))
            .collect();
        
        if cells.is_empty() {
            continue;
        }
        
        // 检查是否是交易所标题行
        let first_cell = &cells[0];
        let mut is_exchange_header = false;
        for marker in &exchange_markers {
            if first_cell.contains(marker) {
                current_exchange = marker.to_string();
                skip_header_rows = 2; // 跳过接下来的2行表头
                is_exchange_header = true;
                break;
            }
        }
        
        if is_exchange_header {
            continue;
        }
        
        // 跳过表头行
        if skip_header_rows > 0 {
            skip_header_rows -= 1;
            continue;
        }
        
        // 跳过空行或无效行
        if current_exchange.is_empty() || cells.len() < 12 {
            continue;
        }
        
        // 根据交易所过滤
        if let Some(filter) = exchange_filter {
            if filter != "所有" && current_exchange != filter {
                continue;
            }
        }
        
        // 解析数据行
        // 列: 合约品种(0), 现价(1), 涨/跌停板(2), 保证金-买开(3), 保证金-卖开(4), 
        // 保证金/每手(5), 手续费标准-开仓(6), 手续费标准-平昨(7), 手续费标准-平今(8),
        // 每跳毛利(9), 手续费(开+平)(10), 每跳净利(11), 备注(12)
        
        // 解析合约品种 "品种名(代码)"
        let contract_str = &cells[0];
        let (contract_name, contract_code) = if let Some(idx) = contract_str.find('(') {
            let name = contract_str[..idx].trim().to_string();
            let code = contract_str[idx+1..].trim_end_matches(')').to_string();
            (name, code)
        } else {
            (contract_str.clone(), String::new())
        };
        
        // 解析涨跌停板 "涨停/跌停"
        let limit_str = cells.get(2).map(|s| s.as_str()).unwrap_or("");
        let (limit_up, limit_down) = if let Some(idx) = limit_str.find('/') {
            let up = limit_str[..idx].trim().parse::<f64>().ok();
            let down = limit_str[idx+1..].trim().parse::<f64>().ok();
            (up, down)
        } else {
            (None, None)
        };
        
        // 解析手续费标准（可能是"万分之X"或"X元"）
        let parse_fee = |s: &str| -> (Option<f64>, Option<f64>) {
            if s.contains("万分之") {
                let ratio = s.replace("万分之", "")
                    .split('/')
                    .next()
                    .and_then(|v| v.trim().parse::<f64>().ok())
                    .map(|v| v / 10000.0);
                (ratio, None)
            } else if s.contains("元") {
                let yuan = s.replace("元", "").trim().parse::<f64>().ok();
                (None, yuan)
            } else {
                (None, None)
            }
        };
        
        let (fee_open_ratio, fee_open_yuan) = parse_fee(cells.get(6).map(|s| s.as_str()).unwrap_or(""));
        let (fee_close_yesterday_ratio, fee_close_yesterday_yuan) = parse_fee(cells.get(7).map(|s| s.as_str()).unwrap_or(""));
        let (fee_close_today_ratio, fee_close_today_yuan) = parse_fee(cells.get(8).map(|s| s.as_str()).unwrap_or(""));
        
        all_data.push(FuturesCommInfo {
            exchange: current_exchange.clone(),
            contract_name,
            contract_code,
            current_price: cells.get(1).and_then(|s| s.parse::<f64>().ok()),
            limit_up,
            limit_down,
            margin_buy: cells.get(3).and_then(|s| s.trim_end_matches('%').parse::<f64>().ok()),
            margin_sell: cells.get(4).and_then(|s| s.trim_end_matches('%').parse::<f64>().ok()),
            margin_per_lot: cells.get(5).and_then(|s| s.trim_end_matches('元').parse::<f64>().ok()),
            fee_open_ratio,
            fee_open_yuan,
            fee_close_yesterday_ratio,
            fee_close_yesterday_yuan,
            fee_close_today_ratio,
            fee_close_today_yuan,
            profit_per_tick: cells.get(9).and_then(|s| s.parse::<f64>().ok()),
            fee_total: cells.get(10).and_then(|s| s.trim_end_matches('元').parse::<f64>().ok()),
            net_profit_per_tick: cells.get(11).and_then(|s| s.parse::<f64>().ok()),
            remark: cells.get(12).cloned(),
        });
    }
    
    println!("📊 解析到 {} 条期货手续费数据", all_data.len());
    Ok(all_data)
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


// ==================== 期货持仓排名表（交易所数据） ====================

#[allow(dead_code)]
/// 上海期货交易所会员成交及持仓排名表API
const SHFE_VOL_RANK_URL: &str = "https://www.shfe.com.cn/data/tradedata/future/dailydata/pm";

#[allow(dead_code)]
/// 中国金融期货交易所持仓排名API
const CFFEX_VOL_RANK_URL: &str = "http://www.cffex.com.cn/sj/ccpm";

#[allow(dead_code)]
/// 郑州商品交易所持仓排名API
const CZCE_VOL_RANK_URL: &str = "http://www.czce.com.cn/cn/DFSStaticFiles/Future";

#[allow(dead_code)]
/// 大连商品交易所持仓排名API
const DCE_VOL_RANK_URL: &str = "http://www.dce.com.cn/dcereport/publicweb/dailystat/memberDealPosi/batchDownload";

#[allow(dead_code)]
/// 从合约代码中提取品种代码
fn extract_variety(symbol: &str) -> String {
    let re = Regex::new(r"^([A-Za-z]+)").unwrap();
    re.captures(symbol)
        .and_then(|c| c.get(1))
        .map(|m| m.as_str().to_uppercase())
        .unwrap_or_default()
}

#[allow(dead_code)]
/// 获取上海期货交易所会员成交及持仓排名表
/// 对应 akshare 的 get_shfe_rank_table() 函数
/// 数据来源: https://www.shfe.com.cn/
/// date: 交易日期，格式 YYYYMMDD，数据从 20020107 开始
/// vars_list: 品种代码列表，如 ["CU", "AL"]，为空时返回所有品种
pub async fn get_shfe_rank_table(date: &str, vars_list: Option<Vec<&str>>) -> Result<Vec<RankTableResponse>> {
    let client = Client::new();
    
    let url = format!("{}{}.dat", SHFE_VOL_RANK_URL, date);
    println!("📡 请求上期所持仓排名数据 URL: {}", url);
    
    let response = client
        .get(&url)
        .header("User-Agent", "Mozilla/4.0 (compatible; MSIE 5.5; Windows NT)")
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(anyhow!("获取上期所持仓排名数据失败: {}", response.status()));
    }

    let text = response.text().await?;
    
    // 解析JSON数据
    let json_data: serde_json::Value = serde_json::from_str(&text)
        .map_err(|e| anyhow!("解析JSON失败: {}", e))?;
    
    let cursor = json_data["o_cursor"].as_array()
        .ok_or_else(|| anyhow!("未找到o_cursor数据"))?;
    
    // 按合约分组
    let mut symbol_data: HashMap<String, Vec<PositionRankData>> = HashMap::new();
    
    for item in cursor {
        let rank = item["RANK"].as_i64().unwrap_or(0) as i32;
        if rank <= 0 {
            continue;
        }
        
        let symbol = item["INSTRUMENTID"].as_str().unwrap_or("").trim().to_uppercase();
        if symbol.is_empty() {
            continue;
        }
        
        let variety = extract_variety(&symbol);
        
        // 如果指定了品种列表，检查是否在列表中
        if let Some(ref vars) = vars_list {
            if !vars.iter().any(|v| v.eq_ignore_ascii_case(&variety)) {
                continue;
            }
        }
        
        let data = PositionRankData {
            rank,
            vol_party_name: item["PARTICIPANTABBR1"].as_str().unwrap_or("").trim().to_string(),
            vol: item["CJ1"].as_i64().unwrap_or(0),
            vol_chg: item["CJ1_CHG"].as_i64().unwrap_or(0),
            long_party_name: item["PARTICIPANTABBR2"].as_str().unwrap_or("").trim().to_string(),
            long_open_interest: item["CJ2"].as_i64().unwrap_or(0),
            long_open_interest_chg: item["CJ2_CHG"].as_i64().unwrap_or(0),
            short_party_name: item["PARTICIPANTABBR3"].as_str().unwrap_or("").trim().to_string(),
            short_open_interest: item["CJ3"].as_i64().unwrap_or(0),
            short_open_interest_chg: item["CJ3_CHG"].as_i64().unwrap_or(0),
            symbol: symbol.clone(),
            variety,
        };
        
        symbol_data.entry(symbol).or_insert_with(Vec::new).push(data);
    }
    
    // 转换为响应格式
    let mut result: Vec<RankTableResponse> = symbol_data.into_iter()
        .map(|(symbol, data)| RankTableResponse { symbol, data })
        .collect();
    
    // 按合约代码排序
    result.sort_by(|a, b| a.symbol.cmp(&b.symbol));
    
    println!("📊 解析到 {} 个合约的持仓排名数据", result.len());
    Ok(result)
}

/// 获取中国金融期货交易所前20会员持仓排名数据
/// 对应 akshare 的 get_cffex_rank_table() 函数
/// 数据来源: http://www.cffex.com.cn/ccpm/
/// date: 交易日期，格式 YYYYMMDD，数据从 20100416 开始
/// vars_list: 品种代码列表，如 ["IF", "IC"]，为空时返回所有品种
pub async fn get_cffex_rank_table(date: &str, vars_list: Option<Vec<&str>>) -> Result<Vec<RankTableResponse>> {
    let client = Client::new();
    
    // 中金所品种列表
    let cffex_vars = vec!["IF", "IC", "IM", "IH", "T", "TF", "TS", "TL"];
    
    // 过滤品种
    let target_vars: Vec<&str> = match vars_list {
        Some(vars) => vars.into_iter()
            .filter(|v| cffex_vars.iter().any(|cv| cv.eq_ignore_ascii_case(v)))
            .collect(),
        None => cffex_vars.clone(),
    };
    
    let mut all_results: Vec<RankTableResponse> = Vec::new();
    
    // 格式化日期
    let year_month = &date[..6];
    let day = &date[6..8];
    
    for var in target_vars {
        let url = format!("{}/{}/{}/{}_1.csv", CFFEX_VOL_RANK_URL, year_month, day, var);
        println!("📡 请求中金所 {} 持仓排名数据 URL: {}", var, url);
        
        let response = client
            .get(&url)
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
            .send()
            .await;
        
        let response = match response {
            Ok(r) => r,
            Err(e) => {
                log::warn!("获取 {} 数据失败: {}", var, e);
                continue;
            }
        };
        
        if !response.status().is_success() {
            log::warn!("获取 {} 数据失败: {}", var, response.status());
            continue;
        }
        
        // 使用GBK编码读取
        let bytes = response.bytes().await?;
        let text = encoding_rs::GBK.decode(&bytes).0.to_string();
        
        // 解析CSV数据
        // CSV格式: 日期,合约,名次,成交量会员,成交量,增减,多单会员,多单,增减,空单会员,空单,增减
        let mut symbol_data: HashMap<String, Vec<PositionRankData>> = HashMap::new();
        
        let lines: Vec<&str> = text.lines().collect();
        
        for line in lines {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            
            // 跳过表头行
            if line.contains("交易日") || line.contains("合约") || line.contains("名次") {
                continue;
            }
            
            let fields: Vec<&str> = line.split(',').collect();
            if fields.len() < 12 {
                continue;
            }
            
            // 字段: 0=日期, 1=合约, 2=名次, 3=成交量会员, 4=成交量, 5=增减, 
            //       6=多单会员, 7=多单, 8=增减, 9=空单会员, 10=空单, 11=增减
            let symbol = fields[1].trim().to_string();
            if symbol.is_empty() {
                continue;
            }
            
            let rank = fields[2].trim().parse::<i32>().unwrap_or(0);
            if rank <= 0 {
                continue;
            }
            
            let variety = extract_variety(&symbol);
            
            let data = PositionRankData {
                rank,
                vol_party_name: fields[3].trim().to_string(),
                vol: fields[4].trim().replace(",", "").parse().unwrap_or(0),
                vol_chg: fields[5].trim().replace(",", "").parse().unwrap_or(0),
                long_party_name: fields[6].trim().to_string(),
                long_open_interest: fields[7].trim().replace(",", "").parse().unwrap_or(0),
                long_open_interest_chg: fields[8].trim().replace(",", "").parse().unwrap_or(0),
                short_party_name: fields[9].trim().to_string(),
                short_open_interest: fields[10].trim().replace(",", "").parse().unwrap_or(0),
                short_open_interest_chg: fields[11].trim().replace(",", "").parse().unwrap_or(0),
                symbol: symbol.clone(),
                variety,
            };
            
            symbol_data.entry(symbol).or_insert_with(Vec::new).push(data);
        }
        
        // 转换为响应格式
        for (symbol, data) in symbol_data {
            all_results.push(RankTableResponse { symbol, data });
        }
    }
    
    // 按合约代码排序
    all_results.sort_by(|a, b| a.symbol.cmp(&b.symbol));
    
    println!("📊 解析到 {} 个合约的持仓排名数据", all_results.len());
    Ok(all_results)
}

/// 获取郑州商品交易所前20会员持仓排名数据
/// 对应 akshare 的 get_rank_table_czce() 函数
/// 数据来源: https://www.czce.com.cn/cn/jysj/ccpm/H077003004index_1.htm
/// date: 交易日期，格式 YYYYMMDD，数据从 20151008 开始
pub async fn get_rank_table_czce(date: &str) -> Result<Vec<RankTableResponse>> {
    use calamine::{Reader, open_workbook_auto_from_rs};
    
    let client = Client::new();
    
    // 根据日期选择文件格式
    let year = &date[..4];
    let url = if date >= "20251102" {
        format!("https://www.czce.com.cn/cn/DFSStaticFiles/Future/{}/{}/FutureDataHolding.xlsx", year, date)
    } else {
        format!("https://www.czce.com.cn/cn/DFSStaticFiles/Future/{}/{}/FutureDataHolding.xls", year, date)
    };
    
    println!("📡 请求郑商所持仓排名数据 URL: {}", url);
    
    let response = client
        .get(&url)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(anyhow!("获取郑商所持仓排名数据失败: {}", response.status()));
    }

    let bytes = response.bytes().await?;
    
    // 使用calamine自动检测格式解析Excel文件
    use std::io::Cursor;
    let cursor = Cursor::new(bytes.as_ref());
    
    let mut workbook = open_workbook_auto_from_rs(cursor)
        .map_err(|e| anyhow!("打开Excel文件失败: {}", e))?;
    
    // 获取第一个工作表
    let sheet_names = workbook.sheet_names();
    if sheet_names.is_empty() {
        return Err(anyhow!("Excel文件没有工作表"));
    }
    let first_sheet = sheet_names[0].clone();
    
    let range = workbook.worksheet_range(&first_sheet)
        .map_err(|e| anyhow!("读取工作表失败: {}", e))?;
    
    let mut symbol_data: HashMap<String, Vec<PositionRankData>> = HashMap::new();
    let mut current_symbol = String::new();
    
    for row in range.rows() {
        if row.len() == 0 {
            continue;
        }
        
        let first_cell = row[0].to_string();
        
        // 检查是否是合约标题行（包含品种代码）
        if first_cell.contains("品种") || first_cell.contains("合约") {
            // 提取合约代码
            let re = Regex::new(r"([A-Za-z]+\d+)").unwrap();
            if let Some(cap) = re.captures(&first_cell) {
                current_symbol = cap.get(1).map(|m| m.as_str().to_uppercase()).unwrap_or_default();
            }
            continue;
        }
        
        // 跳过表头行和合计行
        if first_cell.contains("名次") || first_cell.contains("合计") || first_cell.is_empty() {
            continue;
        }
        
        // 解析数据行
        if row.len() >= 10 && !current_symbol.is_empty() {
            let rank = row[0].to_string().parse::<i32>().unwrap_or(0);
            if rank <= 0 {
                continue;
            }
            
            let variety = extract_variety(&current_symbol);
            
            let parse_num = |s: &str| -> i64 {
                s.replace(",", "").replace("-", "0").trim().parse().unwrap_or(0)
            };
            
            let data = PositionRankData {
                rank,
                vol_party_name: row[1].to_string(),
                vol: parse_num(&row[2].to_string()),
                vol_chg: parse_num(&row[3].to_string()),
                long_party_name: row[4].to_string(),
                long_open_interest: parse_num(&row[5].to_string()),
                long_open_interest_chg: parse_num(&row[6].to_string()),
                short_party_name: row[7].to_string(),
                short_open_interest: parse_num(&row[8].to_string()),
                short_open_interest_chg: parse_num(&row[9].to_string()),
                symbol: current_symbol.clone(),
                variety,
            };
            
            symbol_data.entry(current_symbol.clone()).or_insert_with(Vec::new).push(data);
        }
    }
    
    // 转换为响应格式
    let mut result: Vec<RankTableResponse> = symbol_data.into_iter()
        .map(|(symbol, data)| RankTableResponse { symbol, data })
        .collect();
    
    // 按合约代码排序
    result.sort_by(|a, b| a.symbol.cmp(&b.symbol));
    
    println!("📊 解析到 {} 个合约的持仓排名数据", result.len());
    Ok(result)
}

/// 获取大连商品交易所前20会员持仓排名数据
/// 对应 akshare 的 get_dce_rank_table() 函数
/// 数据来源: http://www.dce.com.cn/dalianshangpin/xqsj/tjsj26/rtj/rcjccpm/index.html
/// date: 交易日期，格式 YYYYMMDD，数据从 20060104 开始
/// vars_list: 品种代码列表，如 ["M", "Y"]，为空时返回所有品种
/// 注意: 大商所API有反爬虫机制，可能需要特殊的cookie或认证
pub async fn get_dce_rank_table(date: &str, vars_list: Option<Vec<&str>>) -> Result<Vec<RankTableResponse>> {
    let client = Client::builder()
        .cookie_store(true)
        .build()?;
    
    // 先访问主页获取cookie
    let _home_resp = client
        .get("http://www.dce.com.cn/dalianshangpin/xqsj/tjsj26/rtj/rcjccpm/index.html")
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
        .send()
        .await;
    
    let payload = serde_json::json!({
        "tradeDate": date,
        "varietyId": "a",
        "contractId": "a2601",
        "tradeType": "1",
        "lang": "zh"
    });
    
    println!("📡 请求大商所持仓排名数据 URL: {}", DCE_VOL_RANK_URL);
    
    let response = client
        .post(DCE_VOL_RANK_URL)
        .json(&payload)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
        .header("Accept", "application/json, text/plain, */*")
        .header("Accept-Language", "zh-CN,zh;q=0.9,en;q=0.8")
        .header("Accept-Encoding", "gzip, deflate")
        .header("Origin", "http://www.dce.com.cn")
        .header("Referer", "http://www.dce.com.cn/dalianshangpin/xqsj/tjsj26/rtj/rcjccpm/index.html")
        .header("Connection", "keep-alive")
        .send()
        .await?;

    if !response.status().is_success() {
        // 大商所API有反爬虫机制，返回更友好的错误信息
        if response.status().as_u16() == 412 {
            return Err(anyhow!(
                "大商所API访问被拒绝(412)，该交易所有反爬虫机制。\n\
                建议: 1) 稍后重试 2) 使用浏览器手动下载数据 3) 使用akshare的futures_dce_position_rank()接口"
            ));
        }
        return Err(anyhow!("获取大商所持仓排名数据失败: {}", response.status()));
    }

    let bytes = response.bytes().await?;
    
    // 解析ZIP文件
    use std::io::{Cursor, Read};
    let cursor = Cursor::new(bytes.as_ref());
    let mut archive = zip::ZipArchive::new(cursor)
        .map_err(|e| anyhow!("打开ZIP文件失败: {}", e))?;
    
    let mut symbol_data: HashMap<String, Vec<PositionRankData>> = HashMap::new();
    
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)
            .map_err(|e| anyhow!("读取ZIP文件失败: {}", e))?;
        
        let file_name = file.name().to_string();
        
        // 只处理以日期开头的文件
        if !file_name.starts_with(date) {
            continue;
        }
        
        // 提取合约代码（文件名格式: 20230706_m2309_成交量_买持仓_卖持仓排名.txt）
        let parts: Vec<&str> = file_name.split('_').collect();
        if parts.len() < 2 {
            continue;
        }
        let symbol = parts[1].to_uppercase();
        let variety = extract_variety(&symbol);
        
        // 如果指定了品种列表，检查是否在列表中
        if let Some(ref vars) = vars_list {
            if !vars.iter().any(|v| v.eq_ignore_ascii_case(&variety)) {
                continue;
            }
        }
        
        // 读取文件内容
        let mut content = Vec::new();
        file.read_to_end(&mut content)?;
        
        // 尝试不同编码
        let text = match String::from_utf8(content.clone()) {
            Ok(s) => s,
            Err(_) => encoding_rs::GBK.decode(&content).0.to_string(),
        };
        
        // 解析文件内容
        let lines: Vec<&str> = text.lines().collect();
        
        // 找到三个表格的起始位置（成交量、买持仓、卖持仓）
        let mut vol_start = None;
        let mut long_start = None;
        let mut short_start = None;
        
        for (i, line) in lines.iter().enumerate() {
            if line.contains("名次") {
                if vol_start.is_none() {
                    vol_start = Some(i);
                } else if long_start.is_none() {
                    long_start = Some(i);
                } else if short_start.is_none() {
                    short_start = Some(i);
                }
            }
        }
        
        if vol_start.is_none() || long_start.is_none() || short_start.is_none() {
            continue;
        }
        
        // 解析三个表格的数据
        let vol_data = parse_dce_table_section(&lines, vol_start.unwrap(), long_start.unwrap());
        let long_data = parse_dce_table_section(&lines, long_start.unwrap(), short_start.unwrap());
        let short_data = parse_dce_table_section(&lines, short_start.unwrap(), lines.len());
        
        // 合并数据
        let max_len = vol_data.len().max(long_data.len()).max(short_data.len());
        let mut rank_data = Vec::new();
        
        for i in 0..max_len {
            let (vol_name, vol, vol_chg) = vol_data.get(i).cloned().unwrap_or_default();
            let (long_name, long_oi, long_chg) = long_data.get(i).cloned().unwrap_or_default();
            let (short_name, short_oi, short_chg) = short_data.get(i).cloned().unwrap_or_default();
            
            rank_data.push(PositionRankData {
                rank: (i + 1) as i32,
                vol_party_name: vol_name,
                vol,
                vol_chg,
                long_party_name: long_name,
                long_open_interest: long_oi,
                long_open_interest_chg: long_chg,
                short_party_name: short_name,
                short_open_interest: short_oi,
                short_open_interest_chg: short_chg,
                symbol: symbol.clone(),
                variety: variety.clone(),
            });
        }
        
        if !rank_data.is_empty() {
            symbol_data.insert(symbol, rank_data);
        }
    }
    
    // 转换为响应格式
    let mut result: Vec<RankTableResponse> = symbol_data.into_iter()
        .map(|(symbol, data)| RankTableResponse { symbol, data })
        .collect();
    
    // 按合约代码排序
    result.sort_by(|a, b| a.symbol.cmp(&b.symbol));
    
    println!("📊 解析到 {} 个合约的持仓排名数据", result.len());
    Ok(result)
}

/// 解析大商所表格数据段
fn parse_dce_table_section(lines: &[&str], start: usize, end: usize) -> Vec<(String, i64, i64)> {
    let mut result = Vec::new();
    
    for i in (start + 1)..end {
        let line = lines[i].trim();
        if line.is_empty() || line.contains("总计") || line.contains("合计") {
            continue;
        }
        
        // 分割字段（可能是制表符或空格分隔）
        let fields: Vec<&str> = line.split(|c| c == '\t' || c == ' ')
            .filter(|s| !s.is_empty())
            .collect();
        
        if fields.len() >= 4 {
            let name = fields[1].trim().to_string();
            let value: i64 = fields[2].trim().replace(",", "").parse().unwrap_or(0);
            let change: i64 = fields[3].trim().replace(",", "").parse().unwrap_or(0);
            
            result.push((name, value, change));
        }
    }
    
    result
}


// ==================== 大商所持仓排名（备用接口） ====================

/// 大连商品交易所-每日持仓排名-具体合约
/// 对应 akshare 的 futures_dce_position_rank() 函数
/// 数据来源: http://www.dce.com.cn/dalianshangpin/xqsj/tjsj26/rtj/rcjccpm/index.html
/// 
/// 该接口通过下载ZIP文件获取持仓排名数据，比 get_dce_rank_table 更稳定
/// 
/// date: 交易日期，格式 YYYYMMDD
/// vars_list: 品种代码列表，如 ["M", "Y"]，为空时返回所有品种
pub async fn futures_dce_position_rank(date: &str, vars_list: Option<Vec<&str>>) -> Result<Vec<RankTableResponse>> {
    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()?;
    
    let url = "http://www.dce.com.cn/dcereport/publicweb/dailystat/memberDealPosi/batchDownload";
    
    let payload = serde_json::json!({
        "tradeDate": date,
        "varietyId": "a",
        "contractId": "a2601",
        "tradeType": "1",
        "lang": "zh"
    });
    
    println!("📡 请求大商所持仓排名数据(ZIP) URL: {}", url);
    
    let response = client
        .post(url)
        .json(&payload)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
        .header("Accept", "*/*")
        .header("Accept-Language", "zh-CN,zh;q=0.9,en;q=0.8")
        .header("Accept-Encoding", "gzip, deflate")
        .header("Origin", "http://www.dce.com.cn")
        .header("Referer", "http://www.dce.com.cn/dalianshangpin/xqsj/tjsj26/rtj/rcjccpm/index.html")
        .send()
        .await?;

    if !response.status().is_success() {
        // 大商所API有反爬虫机制，返回更友好的错误信息
        if response.status().as_u16() == 412 {
            return Err(anyhow!(
                "大商所API访问被拒绝(412)，该交易所有反爬虫机制。\n\
                建议: 1) 稍后重试 2) 使用浏览器手动下载数据 3) 尝试 futures_dce_position_rank_other() 接口"
            ));
        }
        return Err(anyhow!("获取大商所持仓排名数据失败: {}", response.status()));
    }

    let bytes = response.bytes().await?;
    
    // 解析ZIP文件
    use std::io::{Cursor, Read};
    let cursor = Cursor::new(bytes.as_ref());
    let mut archive = match zip::ZipArchive::new(cursor) {
        Ok(a) => a,
        Err(e) => return Err(anyhow!("打开ZIP文件失败: {}，可能是非交易日或数据不存在", e)),
    };
    
    let mut symbol_data: HashMap<String, Vec<PositionRankData>> = HashMap::new();
    
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)
            .map_err(|e| anyhow!("读取ZIP文件失败: {}", e))?;
        
        let file_name = file.name().to_string();
        
        // 只处理以日期开头的文件
        if !file_name.starts_with(date) {
            continue;
        }
        
        // 提取合约代码（文件名格式: 20230706_m2309_成交量_买持仓_卖持仓排名.txt）
        let parts: Vec<&str> = file_name.split('_').collect();
        if parts.len() < 2 {
            continue;
        }
        let symbol = parts[1].to_uppercase();
        let variety = extract_variety(&symbol);
        
        // 如果指定了品种列表，检查是否在列表中
        if let Some(ref vars) = vars_list {
            if !vars.iter().any(|v| v.eq_ignore_ascii_case(&variety)) {
                continue;
            }
        }
        
        // 读取文件内容
        let mut content = Vec::new();
        file.read_to_end(&mut content)?;
        
        // 尝试不同编码
        let text = match String::from_utf8(content.clone()) {
            Ok(s) => s,
            Err(_) => encoding_rs::GBK.decode(&content).0.to_string(),
        };
        
        // 解析文件内容
        match parse_dce_position_file(&text, &symbol, &variety) {
            Ok(data) => {
                if !data.is_empty() {
                    symbol_data.insert(symbol, data);
                }
            }
            Err(e) => {
                log::warn!("解析 {} 数据失败: {}", symbol, e);
            }
        }
    }
    
    // 转换为响应格式
    let mut result: Vec<RankTableResponse> = symbol_data.into_iter()
        .map(|(symbol, data)| RankTableResponse { symbol, data })
        .collect();
    
    // 按合约代码排序
    result.sort_by(|a, b| a.symbol.cmp(&b.symbol));
    
    println!("📊 解析到 {} 个合约的持仓排名数据", result.len());
    Ok(result)
}

/// 解析大商所持仓排名文件内容
fn parse_dce_position_file(text: &str, symbol: &str, variety: &str) -> Result<Vec<PositionRankData>> {
    let lines: Vec<&str> = text.lines().collect();
    
    // 检查是否有会员类别行（需要跳过末尾6行）
    let has_member_type = lines.iter().any(|l| l.contains("会员类别"));
    let effective_lines: Vec<&str> = if has_member_type {
        lines[..lines.len().saturating_sub(6)].to_vec()
    } else {
        lines.clone()
    };
    
    // 找到三个表格的起始位置（名次行）
    let mut start_indices: Vec<usize> = Vec::new();
    for (i, line) in effective_lines.iter().enumerate() {
        if line.starts_with("名次") || line.contains("\t名次") {
            start_indices.push(i);
        }
    }
    
    if start_indices.len() < 3 {
        return Err(anyhow!("未找到完整的三个表格"));
    }
    
    // 检查是否有有效数据（成交量表格至少要有5行数据）
    if start_indices.len() >= 2 && start_indices[1] - start_indices[0] < 5 {
        return Ok(Vec::new()); // 无有效数据
    }
    
    // 找到总计/合计行
    let mut end_indices: Vec<usize> = Vec::new();
    for (i, line) in effective_lines.iter().enumerate() {
        if line.contains("总计") || line.contains("合计") {
            end_indices.push(i);
        }
    }
    
    if end_indices.len() < 3 {
        return Err(anyhow!("未找到完整的三个表格结束标记"));
    }
    
    // 解析三个表格
    let vol_data = parse_dce_rank_section(&effective_lines, start_indices[0] + 1, end_indices[0]);
    let long_data = parse_dce_rank_section(&effective_lines, start_indices[1] + 1, end_indices[1]);
    let short_data = parse_dce_rank_section(&effective_lines, start_indices[2] + 1, end_indices[2]);
    
    // 合并数据
    let max_len = vol_data.len().max(long_data.len()).max(short_data.len());
    let mut result = Vec::new();
    
    for i in 0..max_len {
        let (vol_name, vol, vol_chg) = vol_data.get(i).cloned().unwrap_or_default();
        let (long_name, long_oi, long_chg) = long_data.get(i).cloned().unwrap_or_default();
        let (short_name, short_oi, short_chg) = short_data.get(i).cloned().unwrap_or_default();
        
        result.push(PositionRankData {
            rank: (i + 1) as i32,
            vol_party_name: vol_name,
            vol,
            vol_chg,
            long_party_name: long_name,
            long_open_interest: long_oi,
            long_open_interest_chg: long_chg,
            short_party_name: short_name,
            short_open_interest: short_oi,
            short_open_interest_chg: short_chg,
            symbol: symbol.to_string(),
            variety: variety.to_string(),
        });
    }
    
    Ok(result)
}

/// 解析大商所排名表格段落
fn parse_dce_rank_section(lines: &[&str], start: usize, end: usize) -> Vec<(String, i64, i64)> {
    let mut result = Vec::new();
    
    for i in start..end {
        if i >= lines.len() {
            break;
        }
        let line = lines[i].trim();
        if line.is_empty() {
            continue;
        }
        
        // 分割字段（制表符或多空格分隔）
        let fields: Vec<&str> = line.split(|c| c == '\t')
            .filter(|s| !s.is_empty())
            .collect();
        
        // 如果制表符分割不够，尝试空格分割
        let fields = if fields.len() < 4 {
            line.split_whitespace().collect::<Vec<&str>>()
        } else {
            fields
        };
        
        if fields.len() >= 4 {
            // 字段顺序: 名次, 会员简称, 成交量/持仓量, 增减
            let name = fields[1].trim().replace(",", "").replace("-", "");
            let value: i64 = fields[2].trim().replace(",", "").replace("-", "0").parse().unwrap_or(0);
            let change: i64 = fields[3].trim().replace(",", "").replace("-", "0").parse().unwrap_or(0);
            
            if !name.is_empty() {
                result.push((name, value, change));
            }
        }
    }
    
    result
}


/// 大连商品交易所-每日持仓排名-具体合约-补充接口
/// 对应 akshare 的 futures_dce_position_rank_other() 函数
/// 数据来源: http://www.dce.com.cn/publicweb/quotesdata/memberDealPosiQuotes.html
/// 
/// 该接口通过HTML表单POST获取数据，当主接口不可用时可作为备选
/// 
/// date: 交易日期，格式 YYYYMMDD
pub async fn futures_dce_position_rank_other(date: &str) -> Result<Vec<RankTableResponse>> {
    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()?;
    
    let url = "http://www.dce.com.cn/publicweb/quotesdata/memberDealPosiQuotes.html";
    
    // 解析日期
    let year: i32 = date[0..4].parse().map_err(|_| anyhow!("无效的日期格式"))?;
    let month: i32 = date[4..6].parse().map_err(|_| anyhow!("无效的日期格式"))?;
    let day: i32 = date[6..8].parse().map_err(|_| anyhow!("无效的日期格式"))?;
    
    println!("📡 请求大商所持仓排名数据(HTML) URL: {}", url);
    
    // 第一步：获取品种列表
    let payload = [
        ("memberDealPosiQuotes.variety", "c"),
        ("memberDealPosiQuotes.trade_type", "0"),
        ("year", &year.to_string()),
        ("month", &(month - 1).to_string()),  // 月份从0开始
        ("day", &day.to_string()),
        ("contract.contract_id", "all"),
        ("contract.variety_id", "c"),
        ("contract", ""),
    ];
    
    let response = client
        .post(url)
        .form(&payload)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
        .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8")
        .header("Accept-Language", "zh-CN,zh;q=0.9,en;q=0.8")
        .header("Origin", "http://www.dce.com.cn")
        .header("Referer", "http://www.dce.com.cn/publicweb/quotesdata/memberDealPosiQuotes.html")
        .send()
        .await?;

    if !response.status().is_success() {
        // 大商所API有反爬虫机制，返回更友好的错误信息
        if response.status().as_u16() == 412 {
            return Err(anyhow!(
                "大商所API访问被拒绝(412)，该交易所有反爬虫机制。\n\
                建议: 1) 稍后重试 2) 使用浏览器手动下载数据"
            ));
        }
        return Err(anyhow!("获取大商所品种列表失败: {}", response.status()));
    }

    let html = response.text().await?;
    
    // 解析品种列表
    let symbol_list = parse_dce_symbol_list(&html)?;
    
    if symbol_list.is_empty() {
        return Err(anyhow!("未找到品种列表，可能是非交易日"));
    }
    
    println!("📊 找到 {} 个品种", symbol_list.len());
    
    let mut all_results: Vec<RankTableResponse> = Vec::new();
    
    // 遍历每个品种获取合约列表和数据
    for symbol in &symbol_list {
        // 获取该品种的合约列表
        let payload = [
            ("memberDealPosiQuotes.variety", symbol.as_str()),
            ("memberDealPosiQuotes.trade_type", "0"),
            ("year", &year.to_string()),
            ("month", &(month - 1).to_string()),
            ("day", &day.to_string()),
            ("contract.contract_id", "all"),
            ("contract.variety_id", symbol.as_str()),
            ("contract", ""),
        ];
        
        let response = match client
            .post(url)
            .form(&payload)
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
            .send()
            .await {
                Ok(r) => r,
                Err(e) => {
                    log::warn!("获取 {} 合约列表失败: {}", symbol, e);
                    continue;
                }
            };
        
        if !response.status().is_success() {
            continue;
        }
        
        let html = response.text().await?;
        let contract_list = parse_dce_contract_list(&html, symbol);
        
        // 获取每个合约的持仓排名数据
        for contract in &contract_list {
            let payload = [
                ("memberDealPosiQuotes.variety", symbol.as_str()),
                ("memberDealPosiQuotes.trade_type", "0"),
                ("year", &year.to_string()),
                ("month", &(month - 1).to_string()),
                ("day", &format!("{:02}", day)),
                ("contract.contract_id", contract.as_str()),
                ("contract.variety_id", symbol.as_str()),
                ("contract", ""),
            ];
            
            let response = match client
                .post(url)
                .form(&payload)
                .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
                .send()
                .await {
                    Ok(r) => r,
                    Err(e) => {
                        log::warn!("获取 {} 数据失败: {}", contract, e);
                        continue;
                    }
                };
            
            if !response.status().is_success() {
                continue;
            }
            
            let html = response.text().await?;
            
            // 解析HTML表格数据
            match parse_dce_html_table(&html, contract, symbol) {
                Ok(data) => {
                    if !data.is_empty() {
                        all_results.push(RankTableResponse {
                            symbol: contract.to_uppercase(),
                            data,
                        });
                    }
                }
                Err(e) => {
                    log::warn!("解析 {} 数据失败: {}", contract, e);
                }
            }
        }
    }
    
    // 按合约代码排序
    all_results.sort_by(|a, b| a.symbol.cmp(&b.symbol));
    
    println!("📊 解析到 {} 个合约的持仓排名数据", all_results.len());
    Ok(all_results)
}

/// 解析大商所品种列表
fn parse_dce_symbol_list(html: &str) -> Result<Vec<String>> {
    let document = scraper::Html::parse_document(html);
    let selector = scraper::Selector::parse("input.selBox").unwrap();
    
    let mut symbols = Vec::new();
    
    for element in document.select(&selector) {
        if let Some(onclick) = element.value().attr("onclick") {
            // 格式: javascript:setVariety('a');
            if let Some(start) = onclick.find("setVariety('") {
                let rest = &onclick[start + 12..];
                if let Some(end) = rest.find("'") {
                    let symbol = &rest[..end];
                    if !symbol.is_empty() {
                        symbols.push(symbol.to_string());
                    }
                }
            }
        }
    }
    
    // 如果上面的方法没找到，尝试另一种选择器
    if symbols.is_empty() {
        let selector = scraper::Selector::parse(".selBox input").unwrap();
        for element in document.select(&selector) {
            if let Some(onclick) = element.value().attr("onclick") {
                if let Some(start) = onclick.find("setVariety(") {
                    let rest = &onclick[start + 11..];
                    if let Some(end) = rest.find(")") {
                        let symbol = rest[..end].trim_matches(|c| c == '\'' || c == '"');
                        if !symbol.is_empty() {
                            symbols.push(symbol.to_string());
                        }
                    }
                }
            }
        }
    }
    
    Ok(symbols)
}

/// 解析大商所合约列表
fn parse_dce_contract_list(html: &str, symbol: &str) -> Vec<String> {
    let document = scraper::Html::parse_document(html);
    let selector = scraper::Selector::parse("input[name='contract']").unwrap();
    
    let mut contracts = Vec::new();
    
    for element in document.select(&selector) {
        if let Some(onclick) = element.value().attr("onclick") {
            // 格式: javascript:setContract_id('2401');
            if let Some(start) = onclick.find("setContract_id('") {
                let rest = &onclick[start + 16..];
                if let Some(end) = rest.find("'") {
                    let contract_suffix = &rest[..end];
                    // 如果合约后缀是4位数字，需要加上品种前缀
                    let contract = if contract_suffix.len() == 4 && contract_suffix.chars().all(|c| c.is_ascii_digit()) {
                        format!("{}{}", symbol, contract_suffix)
                    } else {
                        contract_suffix.to_string()
                    };
                    if !contract.is_empty() {
                        contracts.push(contract);
                    }
                }
            }
        }
    }
    
    contracts
}

/// 解析大商所HTML表格数据
fn parse_dce_html_table(html: &str, contract: &str, variety: &str) -> Result<Vec<PositionRankData>> {
    let document = scraper::Html::parse_document(html);
    
    // 查找数据表格（通常是第二个表格）
    let table_selector = scraper::Selector::parse("table").unwrap();
    let tables: Vec<_> = document.select(&table_selector).collect();
    
    if tables.len() < 2 {
        return Err(anyhow!("未找到数据表格"));
    }
    
    let data_table = tables[1];
    let row_selector = scraper::Selector::parse("tr").unwrap();
    let cell_selector = scraper::Selector::parse("td").unwrap();
    
    let mut result = Vec::new();
    
    for row in data_table.select(&row_selector) {
        let cells: Vec<_> = row.select(&cell_selector).collect();
        
        // 跳过表头和合计行
        if cells.len() < 12 {
            continue;
        }
        
        let first_cell = cells[0].text().collect::<String>().trim().to_string();
        if first_cell.is_empty() || first_cell.contains("名次") || first_cell.contains("合计") || first_cell.contains("总计") {
            continue;
        }
        
        // 解析排名
        let rank: i32 = first_cell.parse().unwrap_or(0);
        if rank == 0 {
            continue;
        }
        
        // 解析各列数据
        // 列顺序: 名次, 会员简称, 成交量, 增减, _, 会员简称, 持买单量, 增减, _, 会员简称, 持卖单量, 增减
        let get_text = |idx: usize| -> String {
            cells.get(idx)
                .map(|c| c.text().collect::<String>().trim().replace(",", "").replace("-", "0"))
                .unwrap_or_default()
        };
        
        let get_num = |idx: usize| -> i64 {
            get_text(idx).parse().unwrap_or(0)
        };
        
        result.push(PositionRankData {
            rank,
            vol_party_name: get_text(1),
            vol: get_num(2),
            vol_chg: get_num(3),
            long_party_name: get_text(5),
            long_open_interest: get_num(6),
            long_open_interest_chg: get_num(7),
            short_party_name: get_text(9),
            short_open_interest: get_num(10),
            short_open_interest_chg: get_num(11),
            symbol: contract.to_uppercase(),
            variety: variety.to_uppercase(),
        });
    }
    
    Ok(result)
}


// ==================== 仓单日报相关 ====================

/// 郑州商品交易所-交易数据-仓单日报
/// 对应 akshare 的 futures_warehouse_receipt_czce() 函数
/// 数据来源: http://www.czce.com.cn/cn/jysj/cdrb/H770310index_1.htm
/// 
/// date: 交易日期，格式 YYYYMMDD
pub async fn futures_warehouse_receipt_czce(date: &str) -> Result<Vec<CzceWarehouseReceiptResponse>> {
    let client = Client::builder()
        .danger_accept_invalid_certs(true)  // 忽略SSL证书验证
        .timeout(std::time::Duration::from_secs(30))
        .build()?;
    
    // 根据日期选择文件格式（20251101之后使用xlsx格式）
    let date_num: i32 = date.parse().unwrap_or(0);
    let url = if date_num > 20251101 {
        format!(
            "http://www.czce.com.cn/cn/DFSStaticFiles/Future/{}/{}/FutureDataWhsheet.xlsx",
            &date[0..4], date
        )
    } else {
        format!(
            "http://www.czce.com.cn/cn/DFSStaticFiles/Future/{}/{}/FutureDataWhsheet.xls",
            &date[0..4], date
        )
    };
    
    println!("📡 请求郑商所仓单日报数据 URL: {}", url);
    
    let response = client
        .get(&url)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(anyhow!("获取郑商所仓单日报数据失败: {}，可能是非交易日", response.status()));
    }

    let bytes = response.bytes().await?;
    
    // 使用calamine解析Excel文件
    use std::io::Cursor;
    use calamine::{Reader, open_workbook_auto_from_rs};
    
    let cursor = Cursor::new(bytes.as_ref());
    let mut workbook = open_workbook_auto_from_rs(cursor)
        .map_err(|e| anyhow!("打开Excel文件失败: {}", e))?;
    
    // 获取第一个工作表
    let sheet_names = workbook.sheet_names().to_vec();
    if sheet_names.is_empty() {
        return Err(anyhow!("Excel文件没有工作表"));
    }
    
    let range = workbook.worksheet_range(&sheet_names[0])
        .map_err(|e| anyhow!("读取工作表失败: {}", e))?;
    
    // 将数据转换为二维数组
    let mut rows: Vec<Vec<String>> = Vec::new();
    for row in range.rows() {
        let row_data: Vec<String> = row.iter()
            .map(|cell| {
                match cell {
                    calamine::Data::String(s) => s.clone(),
                    calamine::Data::Float(f) => format!("{}", f),
                    calamine::Data::Int(i) => format!("{}", i),
                    calamine::Data::Bool(b) => format!("{}", b),
                    calamine::Data::DateTime(dt) => format!("{}", dt),
                    calamine::Data::Error(e) => format!("{:?}", e),
                    calamine::Data::Empty => String::new(),
                    _ => String::new(),
                }
            })
            .collect();
        rows.push(row_data);
    }
    
    // 找到所有"品种"开头的行索引（每个品种的起始位置）
    let mut index_list: Vec<usize> = Vec::new();
    for (i, row) in rows.iter().enumerate() {
        if !row.is_empty() && row[0].starts_with("品种") {
            index_list.push(i);
        }
    }
    index_list.push(rows.len());
    
    let mut result: Vec<CzceWarehouseReceiptResponse> = Vec::new();
    
    // 解析每个品种的数据
    for i in 0..index_list.len() - 1 {
        let start_idx = index_list[i];
        let end_idx = index_list[i + 1];
        
        if start_idx >= rows.len() {
            continue;
        }
        
        // 提取品种代码（从"品种：XX"中提取字母部分）
        let first_cell = &rows[start_idx][0];
        let symbol = extract_letters(first_cell);
        
        if symbol.is_empty() {
            continue;
        }
        
        // 找到表头行（通常是品种行的下一行或下两行）
        let mut header_idx = start_idx + 1;
        while header_idx < end_idx {
            if !rows[header_idx].is_empty() && 
               (rows[header_idx][0].contains("仓库") || rows[header_idx][0].contains("简称")) {
                break;
            }
            header_idx += 1;
        }
        
        if header_idx >= end_idx {
            continue;
        }
        
        // 解析数据行
        let mut data: Vec<CzceWarehouseReceipt> = Vec::new();
        for row_idx in (header_idx + 1)..end_idx {
            let row = &rows[row_idx];
            
            // 跳过空行和合计行
            if row.is_empty() || row[0].is_empty() || row[0].contains("合计") || row[0].contains("小计") {
                continue;
            }
            
            // 解析仓库名称和数据
            let warehouse = row.get(0).cloned().unwrap_or_default().trim().to_string();
            if warehouse.is_empty() {
                continue;
            }
            
            // 解析数值字段（仓单数量、有效预报、增减）
            let parse_num = |s: &str| -> Option<i64> {
                let cleaned = s.trim().replace(",", "").replace("-", "");
                if cleaned.is_empty() {
                    None
                } else {
                    cleaned.parse().ok()
                }
            };
            
            let warehouse_receipt = row.get(1).map(|s| parse_num(s)).flatten();
            let valid_forecast = row.get(2).map(|s| parse_num(s)).flatten();
            let change = row.get(3).map(|s| parse_num(s)).flatten();
            
            data.push(CzceWarehouseReceipt {
                warehouse,
                warehouse_receipt,
                valid_forecast,
                change,
            });
        }
        
        if !data.is_empty() {
            result.push(CzceWarehouseReceiptResponse {
                symbol,
                data,
            });
        }
    }
    
    // 按品种代码排序
    result.sort_by(|a, b| a.symbol.cmp(&b.symbol));
    
    println!("📊 解析到 {} 个品种的仓单日报数据", result.len());
    Ok(result)
}

/// 从字符串中提取字母部分
fn extract_letters(s: &str) -> String {
    s.chars().filter(|c| c.is_ascii_alphabetic()).collect::<String>().to_uppercase()
}


/// 大连商品交易所-行情数据-统计数据-日统计-仓单日报
/// 对应 akshare 的 futures_warehouse_receipt_dce() 函数
/// 数据来源: http://www.dce.com.cn/dalianshangpin/xqsj/tjsj26/rtj/cdrb/index.html
/// 
/// date: 交易日期，格式 YYYYMMDD
pub async fn futures_warehouse_receipt_dce(date: &str) -> Result<Vec<DceWarehouseReceipt>> {
    let client = Client::builder()
        .cookie_store(true)
        .timeout(std::time::Duration::from_secs(30))
        .build()?;
    
    // 先访问主页获取cookie
    let _home_resp = client
        .get("http://www.dce.com.cn/dalianshangpin/xqsj/tjsj26/rtj/cdrb/index.html")
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
        .send()
        .await;
    
    let url = "http://www.dce.com.cn/dcereport/publicweb/dailystat/wbillWeeklyQuotes";
    
    let payload = serde_json::json!({
        "tradeDate": date,
        "varietyId": "all"
    });
    
    println!("📡 请求大商所仓单日报数据 URL: {}", url);
    
    let response = client
        .post(url)
        .json(&payload)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
        .header("Accept", "application/json, text/plain, */*")
        .header("Accept-Language", "zh-CN,zh;q=0.9,en;q=0.8")
        .header("Origin", "http://www.dce.com.cn")
        .header("Referer", "http://www.dce.com.cn/dalianshangpin/xqsj/tjsj26/rtj/cdrb/index.html")
        .send()
        .await?;

    if !response.status().is_success() {
        // 大商所API有反爬虫机制，返回更友好的错误信息
        if response.status().as_u16() == 412 {
            return Err(anyhow!(
                "大商所API访问被拒绝(412)，该交易所有反爬虫机制。\n\
                建议: 1) 稍后重试 2) 使用浏览器手动查看数据"
            ));
        }
        return Err(anyhow!("获取大商所仓单日报数据失败: {}，可能是非交易日", response.status()));
    }

    let json_data: serde_json::Value = response.json().await?;
    
    // 解析数据
    let entity_list = json_data["data"]["entityList"].as_array()
        .ok_or_else(|| anyhow!("未找到entityList数据"))?;
    
    let mut result: Vec<DceWarehouseReceipt> = Vec::new();
    
    for item in entity_list {
        let variety_code = item["varietyOrder"].as_str().unwrap_or("").to_uppercase();
        let variety_name = item["variety"].as_str().unwrap_or("").to_string();
        let warehouse = item["whAbbr"].as_str().unwrap_or("").to_string();
        let delivery_location = item["deliveryAbbr"].as_str()
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string());
        
        // 解析数值字段
        let last_receipt = item["lastWbillQty"].as_i64()
            .or_else(|| item["lastWbillQty"].as_str().and_then(|s| s.parse().ok()))
            .unwrap_or(0);
        let today_receipt = item["wbillQty"].as_i64()
            .or_else(|| item["wbillQty"].as_str().and_then(|s| s.parse().ok()))
            .unwrap_or(0);
        let change = item["diff"].as_i64()
            .or_else(|| item["diff"].as_str().and_then(|s| s.parse().ok()))
            .unwrap_or(0);
        
        result.push(DceWarehouseReceipt {
            variety_code,
            variety_name,
            warehouse,
            delivery_location,
            last_receipt,
            today_receipt,
            change,
        });
    }
    
    println!("📊 解析到 {} 条仓单日报数据", result.len());
    Ok(result)
}


/// 上海期货交易所-指定交割仓库期货仓单日报
/// 对应 akshare 的 futures_shfe_warehouse_receipt() 函数
/// 数据来源: https://www.shfe.com.cn/data/tradedata/future/dailydata/{date}dailystock.dat
/// 
/// date: 交易日期，格式 YYYYMMDD（数据从 20140519 开始）
pub async fn futures_shfe_warehouse_receipt(date: &str) -> Result<Vec<ShfeWarehouseReceiptResponse>> {
    let client = Client::new();
    
    let url = format!(
        "https://www.shfe.com.cn/data/tradedata/future/dailydata/{}dailystock.dat",
        date
    );
    
    println!("📡 请求上期所仓单日报 URL: {}", url);
    
    let response = client
        .get(&url)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
        .header("Referer", "https://www.shfe.com.cn/")
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(anyhow!("获取上期所仓单日报数据失败: {}，可能是非交易日或日期格式错误", response.status()));
    }

    let json_data: serde_json::Value = response.json().await?;
    
    // 解析 o_cursor 数组
    let o_cursor = json_data["o_cursor"].as_array()
        .ok_or_else(|| anyhow!("未找到o_cursor数据"))?;
    
    // 按品种分组
    let mut grouped: std::collections::HashMap<String, Vec<ShfeWarehouseReceipt>> = std::collections::HashMap::new();
    
    for item in o_cursor {
        // 品种名称，去除 $ 后面的内容
        let var_name = item["VARNAME"].as_str().unwrap_or("")
            .split('$').next().unwrap_or("").trim().to_string();
        
        if var_name.is_empty() {
            continue;
        }
        
        // 地区名称，去除 $ 后面的内容
        let reg_name = item["REGNAME"].as_str().unwrap_or("")
            .split('$').next().unwrap_or("").trim().to_string();
        
        // 仓库简称，去除 $ 后面的内容
        let wh_name = item["WHABBRNAME"].as_str().unwrap_or("")
            .split('$').next().unwrap_or("").trim().to_string();
        
        // 解析数值字段
        let last_receipt = item["WRTWGHTS"].as_i64()
            .or_else(|| item["WRTWGHTS"].as_str().and_then(|s| s.parse().ok()))
            .unwrap_or(0);
        let today_receipt = item["WRTQTY"].as_i64()
            .or_else(|| item["WRTQTY"].as_str().and_then(|s| s.parse().ok()))
            .unwrap_or(0);
        let change = item["WRTCHANGE"].as_i64()
            .or_else(|| item["WRTCHANGE"].as_str().and_then(|s| s.parse().ok()))
            .unwrap_or(0);
        
        // 单位
        let unit = item["UNIT"].as_str().unwrap_or("").to_string();
        
        let receipt = ShfeWarehouseReceipt {
            variety: var_name.clone(),
            region: reg_name,
            warehouse: wh_name,
            last_receipt,
            today_receipt,
            change,
            unit,
        };
        
        grouped.entry(var_name).or_insert_with(Vec::new).push(receipt);
    }
    
    // 转换为响应格式
    let mut result: Vec<ShfeWarehouseReceiptResponse> = grouped.into_iter()
        .map(|(symbol, data)| ShfeWarehouseReceiptResponse { symbol, data })
        .collect();
    
    // 按品种名称排序
    result.sort_by(|a, b| a.symbol.cmp(&b.symbol));
    
    println!("📊 解析到 {} 个品种的仓单日报数据", result.len());
    Ok(result)
}


/// 广州期货交易所-行情数据-仓单日报
/// 对应 akshare 的 futures_gfex_warehouse_receipt() 函数
/// 数据来源: http://www.gfex.com.cn/gfex/cdrb/hqsj_tjsj.shtml
/// 
/// date: 交易日期，格式 YYYYMMDD
pub async fn futures_gfex_warehouse_receipt(date: &str) -> Result<Vec<GfexWarehouseReceiptResponse>> {
    let client = Client::new();
    
    let url = "http://www.gfex.com.cn/u/interfacesWebTdWbillWeeklyQuotes/loadList";
    
    let payload = [("gen_date", date)];
    
    println!("📡 请求广期所仓单日报数据 URL: {}", url);
    
    let response = client
        .post(url)
        .form(&payload)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(anyhow!("获取广期所仓单日报数据失败: {}，可能是非交易日", response.status()));
    }

    let json_data: serde_json::Value = response.json().await?;
    
    // 解析数据
    let data_array = json_data["data"].as_array()
        .ok_or_else(|| anyhow!("未找到data数组"))?;
    
    // 收集所有品种代码
    let mut symbol_set: std::collections::HashSet<String> = std::collections::HashSet::new();
    for item in data_array {
        if let Some(symbol) = item["varietyOrder"].as_str() {
            if !symbol.is_empty() {
                symbol_set.insert(symbol.to_uppercase());
            }
        }
    }
    
    // 按品种分组数据
    let mut result: Vec<GfexWarehouseReceiptResponse> = Vec::new();
    
    for symbol in symbol_set {
        let mut data: Vec<GfexWarehouseReceipt> = Vec::new();
        
        for item in data_array {
            let item_symbol = item["varietyOrder"].as_str().unwrap_or("").to_uppercase();
            if item_symbol != symbol {
                continue;
            }
            
            // 检查whType是否有效（过滤无效数据）
            let wh_type = item["whType"].as_str()
                .or_else(|| item["whType"].as_i64().map(|_| ""))
                .unwrap_or("");
            if wh_type.is_empty() && item["whType"].is_null() {
                continue;
            }
            
            let variety = item["variety"].as_str().unwrap_or("").to_string();
            let warehouse = item["whAbbr"].as_str().unwrap_or("").to_string();
            
            // 解析数值字段
            let last_receipt = item["lastWbillQty"].as_i64()
                .or_else(|| item["lastWbillQty"].as_str().and_then(|s| s.parse().ok()))
                .unwrap_or(0);
            let today_receipt = item["wbillQty"].as_i64()
                .or_else(|| item["wbillQty"].as_str().and_then(|s| s.parse().ok()))
                .unwrap_or(0);
            let change = item["regWbillQty"].as_i64()
                .or_else(|| item["regWbillQty"].as_str().and_then(|s| s.parse().ok()))
                .unwrap_or(0);
            
            data.push(GfexWarehouseReceipt {
                variety,
                warehouse,
                last_receipt,
                today_receipt,
                change,
            });
        }
        
        if !data.is_empty() {
            result.push(GfexWarehouseReceiptResponse {
                symbol,
                data,
            });
        }
    }
    
    // 按品种代码排序
    result.sort_by(|a, b| a.symbol.cmp(&b.symbol));
    
    println!("📊 解析到 {} 个品种的仓单日报数据", result.len());
    Ok(result)
}


// ==================== 新浪期货持仓排名 ====================

/// 新浪财经-期货-成交持仓排名
/// 对应 akshare 的 futures_hold_pos_sina() 函数
/// 数据来源: https://vip.stock.finance.sina.com.cn/q/view/vFutures_Positions_cjcc.php
/// 
/// symbol: 数据类型，可选 "成交量"/"多单持仓"/"空单持仓" 或 "volume"/"long"/"short"
/// contract: 期货合约代码，如 "OI2501", "IC2403"
/// date: 查询日期，格式 YYYYMMDD
pub async fn futures_hold_pos_sina(symbol: &str, contract: &str, date: &str) -> Result<Vec<SinaHoldPosition>> {
    use crate::models::SinaHoldPosType;
    
    let pos_type = SinaHoldPosType::from_str(symbol)
        .ok_or_else(|| anyhow!("无效的symbol参数: {}，可选: 成交量/多单持仓/空单持仓", symbol))?;
    
    let client = Client::new();
    
    // 格式化日期为 YYYY-MM-DD
    let formatted_date = format!("{}-{}-{}", &date[0..4], &date[4..6], &date[6..8]);
    
    let url = "https://vip.stock.finance.sina.com.cn/q/view/vFutures_Positions_cjcc.php";
    
    println!("📡 请求新浪期货持仓数据 URL: {}?t_breed={}&t_date={}", url, contract, formatted_date);
    
    let response = client
        .get(url)
        .query(&[("t_breed", contract), ("t_date", &formatted_date)])
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
        .header("Referer", "https://vip.stock.finance.sina.com.cn/")
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(anyhow!("获取新浪期货持仓数据失败: {}", response.status()));
    }

    // 使用GBK编码读取
    let bytes = response.bytes().await?;
    let html = encoding_rs::GBK.decode(&bytes).0.to_string();
    
    // 解析HTML表格
    let document = scraper::Html::parse_document(&html);
    let table_selector = scraper::Selector::parse("table").unwrap();
    let tables: Vec<_> = document.select(&table_selector).collect();
    
    let table_index = pos_type.table_index();
    if tables.len() <= table_index {
        return Err(anyhow!("未找到数据表格，可能是非交易日或合约不存在"));
    }
    
    let target_table = tables[table_index];
    let row_selector = scraper::Selector::parse("tr").unwrap();
    let cell_selector = scraper::Selector::parse("td").unwrap();
    
    let mut result: Vec<SinaHoldPosition> = Vec::new();
    
    for row in target_table.select(&row_selector) {
        let cells: Vec<_> = row.select(&cell_selector).collect();
        
        if cells.len() < 3 {
            continue;
        }
        
        // 获取单元格文本
        let rank_text = cells[0].text().collect::<String>().trim().to_string();
        let company_text = cells[1].text().collect::<String>().trim().to_string();
        let value_text = cells[2].text().collect::<String>().trim().replace(",", "");
        let change_text = if cells.len() > 3 {
            cells[3].text().collect::<String>().trim().replace(",", "")
        } else {
            "0".to_string()
        };
        
        // 跳过表头和合计行
        let rank: i32 = match rank_text.parse() {
            Ok(r) => r,
            Err(_) => continue,
        };
        
        // 跳过合计行（通常名次为0或负数）
        if rank <= 0 {
            continue;
        }
        
        let value: i64 = value_text.parse().unwrap_or(0);
        let change: i64 = change_text.parse().unwrap_or(0);
        
        result.push(SinaHoldPosition {
            rank,
            company: company_text,
            value,
            change,
        });
    }
    
    println!("📊 解析到 {} 条持仓排名数据", result.len());
    Ok(result)
}


// ==================== 持仓排名汇总相关 ====================

/// 获取广州期货交易所前20会员持仓排名数据
/// 对应 akshare 的 futures_gfex_position_rank() 函数
/// 数据来源: http://www.gfex.com.cn/gfex/rcjccpm/hqsj_tjsj.shtml
/// date: 交易日期，格式 YYYYMMDD，数据从 20231110 开始
/// vars_list: 品种代码列表，如 ["SI", "LC"]，为空时返回所有品种
pub async fn get_gfex_rank_table(date: &str, vars_list: Option<Vec<&str>>) -> Result<Vec<RankTableResponse>> {
    let client = Client::new();
    
    // 广期所品种列表
    let gfex_vars = vec!["SI", "LC", "PS"];
    
    // 过滤品种
    let target_vars: Vec<String> = match vars_list {
        Some(vars) => vars.into_iter()
            .filter(|v| gfex_vars.iter().any(|gv| gv.eq_ignore_ascii_case(v)))
            .map(|v| v.to_lowercase())
            .collect(),
        None => gfex_vars.iter().map(|v| v.to_lowercase()).collect(),
    };
    
    let mut all_results: Vec<RankTableResponse> = Vec::new();
    
    for var in target_vars {
        // 获取该品种的合约列表
        let contract_list = match get_gfex_contract_list(&client, &var, date).await {
            Ok(list) => list,
            Err(e) => {
                log::warn!("获取广期所 {} 合约列表失败: {}", var, e);
                continue;
            }
        };
        
        // 获取每个合约的持仓排名数据
        for contract in contract_list {
            match get_gfex_contract_data(&client, &var, &contract, date).await {
                Ok(data) => {
                    if !data.is_empty() {
                        all_results.push(RankTableResponse {
                            symbol: contract.to_uppercase(),
                            data,
                        });
                    }
                }
                Err(e) => {
                    log::warn!("获取广期所 {} 合约数据失败: {}", contract, e);
                }
            }
        }
    }
    
    // 按合约代码排序
    all_results.sort_by(|a, b| a.symbol.cmp(&b.symbol));
    
    println!("📊 解析到 {} 个合约的持仓排名数据", all_results.len());
    Ok(all_results)
}

/// 获取广期所合约列表
async fn get_gfex_contract_list(client: &Client, symbol: &str, date: &str) -> Result<Vec<String>> {
    let url = "http://www.gfex.com.cn/u/interfacesWebTiMemberDealPosiQuotes/loadListContract_id";
    
    let payload = [
        ("variety", symbol),
        ("trade_date", date),
    ];
    
    let response = client
        .post(url)
        .form(&payload)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/119.0.0.0 Safari/537.36")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .send()
        .await?;
    
    if !response.status().is_success() {
        return Err(anyhow!("获取广期所合约列表失败: {}", response.status()));
    }
    
    let json_data: serde_json::Value = response.json().await?;
    
    // 尝试多种解析方式
    let contracts: Vec<String> = if let Some(data) = json_data["data"].as_array() {
        data.iter()
            .filter_map(|item| {
                // 尝试作为数组解析 [[contract_id, ...], ...]
                if let Some(arr) = item.as_array() {
                    arr.first().and_then(|v| v.as_str()).map(|s| s.to_string())
                }
                // 尝试作为对象解析 [{contract_id: "xxx"}, ...]
                else if let Some(obj) = item.as_object() {
                    obj.values().next().and_then(|v| v.as_str()).map(|s| s.to_string())
                }
                // 尝试作为字符串解析 ["contract1", "contract2", ...]
                else {
                    item.as_str().map(|s| s.to_string())
                }
            })
            .collect()
    } else {
        Vec::new()
    };
    
    Ok(contracts)
}

/// 获取广期所合约持仓排名数据
async fn get_gfex_contract_data(client: &Client, symbol: &str, contract_id: &str, date: &str) -> Result<Vec<PositionRankData>> {
    let url = "http://www.gfex.com.cn/u/interfacesWebTiMemberDealPosiQuotes/loadList";
    
    let mut vol_data: Vec<(String, i64, i64)> = Vec::new();
    let mut long_data: Vec<(String, i64, i64)> = Vec::new();
    let mut short_data: Vec<(String, i64, i64)> = Vec::new();
    
    // 获取三种类型的数据: 1=成交量, 2=买持仓, 3=卖持仓
    for data_type in 1..=3 {
        let payload = [
            ("trade_date", date),
            ("trade_type", "0"),
            ("variety", symbol),
            ("contract_id", contract_id),
            ("data_type", &data_type.to_string()),
        ];
        
        let response = client
            .post(url)
            .form(&payload)
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
            .send()
            .await?;
        
        if !response.status().is_success() {
            continue;
        }
        
        let json_data: serde_json::Value = response.json().await?;
        
        if let Some(data) = json_data["data"].as_array() {
            let parsed: Vec<(String, i64, i64)> = data.iter()
                .filter_map(|item| {
                    let name = item["abbr"].as_str().unwrap_or("").to_string();
                    let qty = item["todayQty"].as_str()
                        .or_else(|| item["todayQty"].as_i64().map(|_| ""))
                        .unwrap_or("0")
                        .parse::<i64>()
                        .or_else(|_| item["todayQty"].as_i64().ok_or(()))
                        .unwrap_or(0);
                    let chg = item["qtySub"].as_str()
                        .or_else(|| item["todayQtyChg"].as_str())
                        .unwrap_or("0")
                        .parse::<i64>()
                        .or_else(|_| item["qtySub"].as_i64().or_else(|| item["todayQtyChg"].as_i64()).ok_or(()))
                        .unwrap_or(0);
                    
                    if name.is_empty() || name == "合计" {
                        None
                    } else {
                        Some((name, qty, chg))
                    }
                })
                .collect();
            
            match data_type {
                1 => vol_data = parsed,
                2 => long_data = parsed,
                3 => short_data = parsed,
                _ => {}
            }
        }
    }
    
    // 合并数据
    let max_len = vol_data.len().max(long_data.len()).max(short_data.len());
    let mut result = Vec::new();
    
    for i in 0..max_len {
        let (vol_name, vol, vol_chg) = vol_data.get(i).cloned().unwrap_or_default();
        let (long_name, long_oi, long_chg) = long_data.get(i).cloned().unwrap_or_default();
        let (short_name, short_oi, short_chg) = short_data.get(i).cloned().unwrap_or_default();
        
        result.push(PositionRankData {
            rank: (i + 1) as i32,
            vol_party_name: vol_name,
            vol,
            vol_chg,
            long_party_name: long_name,
            long_open_interest: long_oi,
            long_open_interest_chg: long_chg,
            short_party_name: short_name,
            short_open_interest: short_oi,
            short_open_interest_chg: short_chg,
            symbol: contract_id.to_uppercase(),
            variety: symbol.to_uppercase(),
        });
    }
    
    Ok(result)
}


// ==================== 广期所持仓排名（公开接口） ====================

/// 获取广州期货交易所品种列表
/// 对应 akshare 的 __futures_gfex_vars_list() 函数
/// 数据来源: http://www.gfex.com.cn/gfex/rcjccpm/hqsj_tjsj.shtml
pub async fn get_gfex_vars_list() -> Result<Vec<String>> {
    let client = Client::new();
    let url = "http://www.gfex.com.cn/u/interfacesWebVariety/loadList";
    
    println!("📡 请求广期所品种列表 URL: {}", url);
    
    let response = client
        .post(url)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/119.0.0.0 Safari/537.36")
        .header("Content-Length", "0")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .send()
        .await?;
    
    if !response.status().is_success() {
        return Err(anyhow!("获取广期所品种列表失败: {}", response.status()));
    }
    
    let json_data: serde_json::Value = response.json().await?;
    
    let data = json_data["data"].as_array()
        .ok_or_else(|| anyhow!("未找到data数组"))?;
    
    let vars: Vec<String> = data.iter()
        .filter_map(|item| item["varietyId"].as_str())
        .map(|s| s.to_string())
        .collect();
    
    println!("📊 获取到 {} 个品种", vars.len());
    Ok(vars)
}

/// 广州期货交易所-日成交持仓排名
/// 对应 akshare 的 futures_gfex_position_rank() 函数
/// 数据来源: http://www.gfex.com.cn/gfex/rcjccpm/hqsj_tjsj.shtml
/// 
/// date: 交易日期，格式 YYYYMMDD，数据从 20231110 开始
/// vars_list: 品种代码列表，如 ["SI", "LC"]，为空时返回所有品种
/// 
/// 返回: 按合约分组的持仓排名数据
pub async fn futures_gfex_position_rank(date: &str, vars_list: Option<Vec<&str>>) -> Result<Vec<RankTableResponse>> {
    let client = Client::new();
    
    // 获取品种列表
    let target_vars: Vec<String> = match vars_list {
        Some(vars) => vars.into_iter().map(|v| v.to_lowercase()).collect(),
        None => {
            // 如果未指定品种，获取所有品种
            match get_gfex_vars_list().await {
                Ok(vars) => vars,
                Err(e) => {
                    log::warn!("获取广期所品种列表失败: {}，使用默认品种列表", e);
                    vec!["si".to_string(), "lc".to_string(), "ps".to_string()]
                }
            }
        }
    };
    
    println!("📡 请求广期所持仓排名数据，品种: {:?}", target_vars);
    
    let mut all_results: Vec<RankTableResponse> = Vec::new();
    
    for var in target_vars {
        // 获取该品种的合约列表
        let contract_list = match get_gfex_contract_list(&client, &var, date).await {
            Ok(list) => list,
            Err(e) => {
                log::warn!("获取广期所 {} 合约列表失败: {}", var, e);
                continue;
            }
        };
        
        if contract_list.is_empty() {
            log::warn!("广期所 {} 在 {} 无合约数据", var, date);
            continue;
        }
        
        println!("  品种 {} 有 {} 个合约", var.to_uppercase(), contract_list.len());
        
        // 获取每个合约的持仓排名数据
        for contract in contract_list {
            match get_gfex_contract_data(&client, &var, &contract, date).await {
                Ok(data) => {
                    if !data.is_empty() {
                        all_results.push(RankTableResponse {
                            symbol: contract.to_uppercase(),
                            data,
                        });
                    }
                }
                Err(e) => {
                    log::warn!("获取广期所 {} 合约数据失败: {}", contract, e);
                }
            }
        }
    }
    
    // 按合约代码排序
    all_results.sort_by(|a, b| a.symbol.cmp(&b.symbol));
    
    println!("📊 解析到 {} 个合约的持仓排名数据", all_results.len());
    Ok(all_results)
}


/// 获取单日期货持仓排名汇总数据
/// 对应 akshare 的 get_rank_sum() 函数
/// 采集五个期货交易所前5、前10、前15、前20会员持仓排名数据
/// date: 交易日期，格式 YYYYMMDD
/// vars_list: 品种代码列表，如 ["RB", "CU"]，为空时返回所有品种
pub async fn get_rank_sum(date: &str, vars_list: Option<Vec<String>>) -> Result<Vec<RankSum>> {
    // 各交易所品种列表
    let dce_vars: Vec<&str> = vec!["C", "CS", "A", "B", "M", "Y", "P", "FB", "BB", "JD", "L", "V", "PP", "J", "JM", "I", "EG", "RR", "EB", "PG", "LH", "LG", "BZ"];
    let shfe_vars: Vec<&str> = vec!["CU", "AL", "ZN", "PB", "NI", "SN", "AU", "AG", "RB", "WR", "HC", "FU", "BU", "RU", "SC", "NR", "SP", "SS", "LU", "BC", "AO", "BR", "EC", "AD"];
    let czce_vars: Vec<&str> = vec!["WH", "PM", "CF", "SR", "TA", "OI", "RI", "MA", "ME", "FG", "RS", "RM", "ZC", "JR", "LR", "SF", "SM", "WT", "TC", "GN", "RO", "ER", "SRX", "SRY", "WSX", "WSY", "CY", "AP", "UR", "CJ", "SA", "PK", "PF", "PX", "SH", "PR"];
    let cffex_vars: Vec<&str> = vec!["IF", "IC", "IM", "IH", "T", "TF", "TS", "TL"];
    let gfex_vars: Vec<&str> = vec!["SI", "LC", "PS"];
    
    // 过滤品种
    let filter_vars = |exchange_vars: &[&str], target: &Option<Vec<String>>| -> Vec<String> {
        match target {
            Some(vars) => exchange_vars.iter()
                .filter(|v| vars.iter().any(|tv| tv.eq_ignore_ascii_case(v)))
                .map(|v| v.to_string())
                .collect(),
            None => exchange_vars.iter().map(|v| v.to_string()).collect(),
        }
    };
    
    let dce_target = filter_vars(&dce_vars, &vars_list);
    let shfe_target = filter_vars(&shfe_vars, &vars_list);
    let czce_target = filter_vars(&czce_vars, &vars_list);
    let cffex_target = filter_vars(&cffex_vars, &vars_list);
    let gfex_target = filter_vars(&gfex_vars, &vars_list);
    
    // 收集所有交易所的排名数据
    let mut all_rank_data: HashMap<String, Vec<PositionRankData>> = HashMap::new();
    
    // 获取大商所数据
    if !dce_target.is_empty() {
        let dce_refs: Vec<&str> = dce_target.iter().map(|s| s.as_str()).collect();
        match get_dce_rank_table(date, Some(dce_refs)).await {
            Ok(data) => {
                for item in data {
                    all_rank_data.insert(item.symbol.clone(), item.data);
                }
            }
            Err(e) => log::warn!("获取大商所数据失败: {}", e),
        }
    }
    
    // 获取上期所数据
    if !shfe_target.is_empty() {
        let shfe_refs: Vec<&str> = shfe_target.iter().map(|s| s.as_str()).collect();
        match get_shfe_rank_table(date, Some(shfe_refs)).await {
            Ok(data) => {
                for item in data {
                    all_rank_data.insert(item.symbol.clone(), item.data);
                }
            }
            Err(e) => log::warn!("获取上期所数据失败: {}", e),
        }
    }
    
    // 获取郑商所数据
    if !czce_target.is_empty() {
        match get_rank_table_czce(date).await {
            Ok(data) => {
                for item in data {
                    // 过滤品种
                    let variety = extract_variety(&item.symbol);
                    if czce_target.iter().any(|v| v.eq_ignore_ascii_case(&variety)) {
                        all_rank_data.insert(item.symbol.clone(), item.data);
                    }
                }
            }
            Err(e) => log::warn!("获取郑商所数据失败: {}", e),
        }
    }
    
    // 获取中金所数据
    if !cffex_target.is_empty() {
        let cffex_refs: Vec<&str> = cffex_target.iter().map(|s| s.as_str()).collect();
        match get_cffex_rank_table(date, Some(cffex_refs)).await {
            Ok(data) => {
                for item in data {
                    all_rank_data.insert(item.symbol.clone(), item.data);
                }
            }
            Err(e) => log::warn!("获取中金所数据失败: {}", e),
        }
    }
    
    // 获取广期所数据
    if !gfex_target.is_empty() {
        let gfex_refs: Vec<&str> = gfex_target.iter().map(|s| s.as_str()).collect();
        match get_gfex_rank_table(date, Some(gfex_refs)).await {
            Ok(data) => {
                for item in data {
                    all_rank_data.insert(item.symbol.clone(), item.data);
                }
            }
            Err(e) => log::warn!("获取广期所数据失败: {}", e),
        }
    }
    
    // 计算汇总数据
    let mut results: Vec<RankSum> = Vec::new();
    
    for (symbol, data) in &all_rank_data {
        let variety = extract_variety(symbol);
        
        // 按排名过滤
        let top5: Vec<&PositionRankData> = data.iter().filter(|d| d.rank <= 5).collect();
        let top10: Vec<&PositionRankData> = data.iter().filter(|d| d.rank <= 10).collect();
        let top15: Vec<&PositionRankData> = data.iter().filter(|d| d.rank <= 15).collect();
        let top20: Vec<&PositionRankData> = data.iter().filter(|d| d.rank <= 20).collect();
        
        let rank_sum = RankSum {
            symbol: symbol.clone(),
            variety: variety.clone(),
            vol_top5: top5.iter().map(|d| d.vol).sum(),
            vol_chg_top5: top5.iter().map(|d| d.vol_chg).sum(),
            long_open_interest_top5: top5.iter().map(|d| d.long_open_interest).sum(),
            long_open_interest_chg_top5: top5.iter().map(|d| d.long_open_interest_chg).sum(),
            short_open_interest_top5: top5.iter().map(|d| d.short_open_interest).sum(),
            short_open_interest_chg_top5: top5.iter().map(|d| d.short_open_interest_chg).sum(),
            vol_top10: top10.iter().map(|d| d.vol).sum(),
            vol_chg_top10: top10.iter().map(|d| d.vol_chg).sum(),
            long_open_interest_top10: top10.iter().map(|d| d.long_open_interest).sum(),
            long_open_interest_chg_top10: top10.iter().map(|d| d.long_open_interest_chg).sum(),
            short_open_interest_top10: top10.iter().map(|d| d.short_open_interest).sum(),
            short_open_interest_chg_top10: top10.iter().map(|d| d.short_open_interest_chg).sum(),
            vol_top15: top15.iter().map(|d| d.vol).sum(),
            vol_chg_top15: top15.iter().map(|d| d.vol_chg).sum(),
            long_open_interest_top15: top15.iter().map(|d| d.long_open_interest).sum(),
            long_open_interest_chg_top15: top15.iter().map(|d| d.long_open_interest_chg).sum(),
            short_open_interest_top15: top15.iter().map(|d| d.short_open_interest).sum(),
            short_open_interest_chg_top15: top15.iter().map(|d| d.short_open_interest_chg).sum(),
            vol_top20: top20.iter().map(|d| d.vol).sum(),
            vol_chg_top20: top20.iter().map(|d| d.vol_chg).sum(),
            long_open_interest_top20: top20.iter().map(|d| d.long_open_interest).sum(),
            long_open_interest_chg_top20: top20.iter().map(|d| d.long_open_interest_chg).sum(),
            short_open_interest_top20: top20.iter().map(|d| d.short_open_interest).sum(),
            short_open_interest_chg_top20: top20.iter().map(|d| d.short_open_interest_chg).sum(),
            date: date.to_string(),
        };
        
        results.push(rank_sum);
    }
    
    // 添加品种汇总（将同一品种的所有合约数据汇总）
    let mut variety_sums: HashMap<String, RankSum> = HashMap::new();
    
    for result in &results {
        let variety = &result.variety;
        
        // 只对上期所、大商所、中金所的品种进行汇总
        let should_sum = shfe_vars.iter().any(|v| v.eq_ignore_ascii_case(variety))
            || dce_vars.iter().any(|v| v.eq_ignore_ascii_case(variety))
            || cffex_vars.iter().any(|v| v.eq_ignore_ascii_case(variety));
        
        if should_sum {
            variety_sums.entry(variety.clone())
                .and_modify(|sum| {
                    sum.vol_top5 += result.vol_top5;
                    sum.vol_chg_top5 += result.vol_chg_top5;
                    sum.long_open_interest_top5 += result.long_open_interest_top5;
                    sum.long_open_interest_chg_top5 += result.long_open_interest_chg_top5;
                    sum.short_open_interest_top5 += result.short_open_interest_top5;
                    sum.short_open_interest_chg_top5 += result.short_open_interest_chg_top5;
                    sum.vol_top10 += result.vol_top10;
                    sum.vol_chg_top10 += result.vol_chg_top10;
                    sum.long_open_interest_top10 += result.long_open_interest_top10;
                    sum.long_open_interest_chg_top10 += result.long_open_interest_chg_top10;
                    sum.short_open_interest_top10 += result.short_open_interest_top10;
                    sum.short_open_interest_chg_top10 += result.short_open_interest_chg_top10;
                    sum.vol_top15 += result.vol_top15;
                    sum.vol_chg_top15 += result.vol_chg_top15;
                    sum.long_open_interest_top15 += result.long_open_interest_top15;
                    sum.long_open_interest_chg_top15 += result.long_open_interest_chg_top15;
                    sum.short_open_interest_top15 += result.short_open_interest_top15;
                    sum.short_open_interest_chg_top15 += result.short_open_interest_chg_top15;
                    sum.vol_top20 += result.vol_top20;
                    sum.vol_chg_top20 += result.vol_chg_top20;
                    sum.long_open_interest_top20 += result.long_open_interest_top20;
                    sum.long_open_interest_chg_top20 += result.long_open_interest_chg_top20;
                    sum.short_open_interest_top20 += result.short_open_interest_top20;
                    sum.short_open_interest_chg_top20 += result.short_open_interest_chg_top20;
                })
                .or_insert_with(|| RankSum {
                    symbol: variety.clone(),
                    variety: variety.clone(),
                    date: date.to_string(),
                    ..*result
                });
        }
    }
    
    // 将品种汇总添加到结果中
    for (_, sum) in variety_sums {
        results.push(sum);
    }
    
    // 按合约代码排序
    results.sort_by(|a, b| a.symbol.cmp(&b.symbol));
    
    println!("📊 计算得到 {} 条持仓排名汇总数据", results.len());
    Ok(results)
}

/// 获取日期范围内的期货持仓排名汇总数据
/// 对应 akshare 的 get_rank_sum_daily() 函数
/// 采集五个期货交易所前5、前10、前15、前20会员持仓排名数据
/// start_day: 开始日期，格式 YYYYMMDD
/// end_day: 结束日期，格式 YYYYMMDD
/// vars_list: 品种代码列表，如 ["RB", "CU"]，为空时返回所有品种
pub async fn get_rank_sum_daily(
    start_day: &str,
    end_day: &str,
    vars_list: Option<Vec<String>>,
) -> Result<Vec<RankSum>> {
    use chrono::NaiveDate;
    
    // 解析日期
    let start = NaiveDate::parse_from_str(start_day, "%Y%m%d")
        .map_err(|e| anyhow!("解析开始日期失败: {}", e))?;
    let end = NaiveDate::parse_from_str(end_day, "%Y%m%d")
        .map_err(|e| anyhow!("解析结束日期失败: {}", e))?;
    
    if start > end {
        return Err(anyhow!("开始日期不能大于结束日期"));
    }
    
    let mut all_results: Vec<RankSum> = Vec::new();
    let mut current = start;
    
    while current <= end {
        let date_str = current.format("%Y%m%d").to_string();
        println!("📅 正在获取 {} 的持仓排名数据...", date_str);
        
        // 克隆 vars_list 用于每次调用
        let vars_clone: Option<Vec<String>> = vars_list.clone();
        
        match get_rank_sum(&date_str, vars_clone).await {
            Ok(mut data) => {
                if !data.is_empty() {
                    println!("  ✅ 获取到 {} 条数据", data.len());
                    all_results.append(&mut data);
                } else {
                    println!("  ⚠️ {} 无数据（可能是非交易日）", date_str);
                }
            }
            Err(e) => {
                println!("  ❌ {} 获取失败: {}", date_str, e);
            }
        }
        
        // 下一天
        current = current.succ_opt().unwrap_or(current);
    }
    
    println!("📊 共获取 {} 条持仓排名汇总数据", all_results.len());
    Ok(all_results)
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

    /// 测试获取期货交易费用参照表
    #[tokio::test]
    async fn test_futures_fees_info() {
        println!("\n========== 测试获取期货交易费用参照表 ==========");
        
        match get_futures_fees_info().await {
            Ok(fees) => {
                println!("✅ 获取成功！共 {} 条费用数据", fees.len());
                println!("\n  前20条数据:");
                println!("  {:<6} {:<10} {:<8} {:<8} {:>8} {:>8} {:>10} {:>10} {:>10}", 
                    "交易所", "合约代码", "品种", "乘数", "开仓费", "平仓费", "平今费", "多保证金", "空保证金");
                for f in fees.iter().take(20) {
                    println!("  {:<6} {:<10} {:<8} {:<8} {:>8} {:>8} {:>10} {:>10} {:>10}", 
                        f.exchange, f.contract_code, f.product_name, f.contract_size, 
                        f.open_fee, f.close_fee, f.close_today_fee, f.long_margin_rate, f.short_margin_rate);
                }
                
                // 显示更新时间
                if let Some(first) = fees.first() {
                    println!("\n  📅 数据更新时间: {}", first.updated_at);
                }
            }
            Err(e) => {
                println!("❌ 获取失败: {}", e);
            }
        }
    }

    /// 测试获取期货手续费信息（九期网）
    #[tokio::test]
    async fn test_futures_comm_info() {
        println!("\n========== 测试获取期货手续费信息（九期网） ==========");
        
        // 测试获取所有交易所
        println!("\n  1. 测试获取所有交易所数据:");
        match get_futures_comm_info(Some("所有")).await {
            Ok(data) => {
                println!("  ✅ 获取成功！共 {} 条数据", data.len());
                println!("\n  前10条数据:");
                println!("  {:<12} {:<10} {:<8} {:>8} {:>8} {:>10} {:>10}", 
                    "交易所", "合约名称", "代码", "现价", "保证金%", "开仓费", "平今费");
                for d in data.iter().take(10) {
                    let fee_open = d.fee_open_yuan.map(|v| format!("{}元", v))
                        .or_else(|| d.fee_open_ratio.map(|v| format!("{:.4}‱", v * 10000.0)))
                        .unwrap_or("-".to_string());
                    let fee_today = d.fee_close_today_yuan.map(|v| format!("{}元", v))
                        .or_else(|| d.fee_close_today_ratio.map(|v| format!("{:.4}‱", v * 10000.0)))
                        .unwrap_or("-".to_string());
                    println!("  {:<12} {:<10} {:<8} {:>8.0} {:>8.1} {:>10} {:>10}", 
                        d.exchange, d.contract_name, d.contract_code, 
                        d.current_price.unwrap_or(0.0),
                        d.margin_buy.unwrap_or(0.0),
                        fee_open, fee_today);
                }
            }
            Err(e) => {
                println!("  ❌ 获取失败: {}", e);
            }
        }
        
        // 测试获取上海期货交易所
        println!("\n  2. 测试获取上海期货交易所数据:");
        match get_futures_comm_info(Some("上海期货交易所")).await {
            Ok(data) => {
                println!("  ✅ 获取成功！共 {} 条数据", data.len());
                for d in data.iter().take(5) {
                    println!("    {} ({}) - 现价:{:.0} 保证金:{:.1}%", 
                        d.contract_name, d.contract_code, 
                        d.current_price.unwrap_or(0.0),
                        d.margin_buy.unwrap_or(0.0));
                }
            }
            Err(e) => {
                println!("  ❌ 获取失败: {}", e);
            }
        }
        
        // 测试获取中国金融期货交易所
        println!("\n  3. 测试获取中国金融期货交易所数据:");
        match get_futures_comm_info(Some("中国金融期货交易所")).await {
            Ok(data) => {
                println!("  ✅ 获取成功！共 {} 条数据", data.len());
                for d in data.iter().take(5) {
                    println!("    {} ({}) - 现价:{:.0} 保证金:{:.1}%", 
                        d.contract_name, d.contract_code, 
                        d.current_price.unwrap_or(0.0),
                        d.margin_buy.unwrap_or(0.0));
                }
            }
            Err(e) => {
                println!("  ❌ 获取失败: {}", e);
            }
        }
    }

    /// 测试获取期货交易规则
    #[tokio::test]
    async fn test_futures_rule() {
        println!("\n========== 测试获取期货交易规则 ==========");
        
        // 测试获取交易规则（使用指定日期，因为默认日期可能是非交易日）
        println!("\n  1. 测试获取交易规则（指定日期 20250328）:");
        match get_futures_rule(Some("20250328")).await {
            Ok(rules) => {
                println!("  ✅ 获取成功！共 {} 条规则数据", rules.len());
                println!("\n  前20条数据:");
                println!("  {:<12} {:<10} {:<8} {:>10} {:>10} {:>10} {:>10} {:>10}", 
                    "交易所", "品种", "代码", "保证金%", "涨跌停%", "合约乘数", "最小变动", "最大手数");
                for r in rules.iter().take(20) {
                    let margin = r.margin_rate.map(|v| format!("{:.1}", v)).unwrap_or("--".to_string());
                    let limit = r.price_limit.map(|v| format!("{:.1}", v)).unwrap_or("--".to_string());
                    let size = r.contract_size.map(|v| format!("{:.0}", v)).unwrap_or("--".to_string());
                    let tick = r.price_tick.map(|v| format!("{:.2}", v)).unwrap_or("--".to_string());
                    let max_order = r.max_order_size.map(|v| format!("{}", v)).unwrap_or("--".to_string());
                    println!("  {:<12} {:<10} {:<8} {:>10} {:>10} {:>10} {:>10} {:>10}", 
                        r.exchange, r.product, r.code, margin, limit, size, tick, max_order);
                }
                
                // 验证数据
                assert!(rules.len() > 50, "应该有超过50条规则数据");
            }
            Err(e) => {
                println!("  ❌ 获取失败: {}", e);
            }
        }
        
        // 测试默认日期（可能是非交易日，允许失败）
        println!("\n  2. 测试获取交易规则（默认日期）:");
        match get_futures_rule(None).await {
            Ok(rules) => {
                println!("  ✅ 获取成功！共 {} 条规则数据", rules.len());
            }
            Err(e) => {
                println!("  ⚠️ 获取失败（可能是非交易日）: {}", e);
            }
        }
    }

    /// 测试获取99期货网库存数据
    #[tokio::test]
    async fn test_futures_inventory_99() {
        println!("\n========== 测试获取99期货网库存数据 ==========");
        
        // 测试获取品种映射
        println!("\n  1. 测试获取品种映射:");
        match get_99_symbol_map().await {
            Ok(symbols) => {
                println!("  ✅ 获取成功！共 {} 个品种", symbols.len());
                println!("\n  前10个品种:");
                for s in symbols.iter().take(10) {
                    println!("    {} ({}) - ID: {}", s.name, s.code, s.product_id);
                }
            }
            Err(e) => {
                println!("  ❌ 获取失败: {}", e);
            }
        }
        
        // 测试获取库存数据（使用中文名称）
        println!("\n  2. 测试获取库存数据（豆一）:");
        match get_futures_inventory_99("豆一").await {
            Ok(data) => {
                println!("  ✅ 获取成功！共 {} 条数据", data.len());
                println!("\n  最近10条:");
                for d in data.iter().rev().take(10) {
                    println!("    {} - 收盘价: {:>10.2} - 库存: {:>10.0}", 
                        d.date, 
                        d.close_price.unwrap_or(0.0),
                        d.inventory.unwrap_or(0.0));
                }
            }
            Err(e) => {
                println!("  ❌ 获取失败: {}", e);
            }
        }
        
        // 测试获取库存数据（使用代码）
        println!("\n  3. 测试获取库存数据（cu）:");
        match get_futures_inventory_99("cu").await {
            Ok(data) => {
                println!("  ✅ 获取成功！共 {} 条数据", data.len());
                println!("\n  最近5条:");
                for d in data.iter().rev().take(5) {
                    println!("    {} - 收盘价: {:>10.2} - 库存: {:>10.0}", 
                        d.date, 
                        d.close_price.unwrap_or(0.0),
                        d.inventory.unwrap_or(0.0));
                }
            }
            Err(e) => {
                println!("  ❌ 获取失败: {}", e);
            }
        }
    }

    /// 测试获取现货价格及基差数据
    #[tokio::test]
    async fn test_futures_spot_price() {
        println!("\n========== 测试获取现货价格及基差数据 ==========");
        
        // 测试获取所有品种
        println!("\n  1. 测试获取所有品种（20240430）:");
        match get_futures_spot_price("20240430", None).await {
            Ok(data) => {
                println!("  ✅ 获取成功！共 {} 条数据", data.len());
                println!("\n  前15条:");
                println!("  {:<8} {:>10} {:>12} {:>10} {:>12} {:>10} {:>10}", 
                    "品种", "现货价", "近月合约", "近月价", "主力合约", "主力价", "主力基差");
                for d in data.iter().take(15) {
                    println!("  {:<8} {:>10.2} {:>12} {:>10.2} {:>12} {:>10.2} {:>10.2}", 
                        d.symbol, d.spot_price, d.near_contract, d.near_contract_price,
                        d.dominant_contract, d.dominant_contract_price, d.dom_basis);
                }
            }
            Err(e) => {
                println!("  ❌ 获取失败: {}", e);
            }
        }
        
        // 测试获取指定品种
        println!("\n  2. 测试获取指定品种（RB,CU,AU）:");
        match get_futures_spot_price("20240430", Some(vec!["RB", "CU", "AU"])).await {
            Ok(data) => {
                println!("  ✅ 获取成功！共 {} 条数据", data.len());
                for d in &data {
                    println!("    【{}】现货:{:.2} 主力:{} 价格:{:.2} 基差:{:.2} 基差率:{:.2}%", 
                        d.symbol, d.spot_price, d.dominant_contract, 
                        d.dominant_contract_price, d.dom_basis, d.dom_basis_rate * 100.0);
                }
            }
            Err(e) => {
                println!("  ❌ 获取失败: {}", e);
            }
        }
        
        // 测试最近日期
        println!("\n  3. 测试获取最近日期（20250106）:");
        match get_futures_spot_price("20250106", Some(vec!["RB", "CU"])).await {
            Ok(data) => {
                println!("  ✅ 获取成功！共 {} 条数据", data.len());
                for d in &data {
                    println!("    【{}】现货:{:.2} 主力:{} 价格:{:.2} 基差:{:.2}", 
                        d.symbol, d.spot_price, d.dominant_contract, 
                        d.dominant_contract_price, d.dom_basis);
                }
            }
            Err(e) => {
                println!("  ⚠️ 获取失败（可能是非交易日）: {}", e);
            }
        }
    }

    /// 测试获取现货价格历史数据（包含180日统计）
    #[tokio::test]
    async fn test_futures_spot_price_previous() {
        println!("\n========== 测试获取现货价格历史数据 ==========");
        
        // 测试获取历史数据
        println!("\n  1. 测试获取历史数据（20240430）:");
        match get_futures_spot_price_previous("20240430").await {
            Ok(data) => {
                println!("  ✅ 获取成功！共 {} 条数据", data.len());
                println!("\n  前15条:");
                println!("  {:<10} {:>10} {:>8} {:>10} {:>10} {:>8} {:>10} {:>10} {:>10}", 
                    "商品", "现货价", "主力", "主力价", "基差", "基差率%", "180高", "180低", "180均");
                for d in data.iter().take(15) {
                    println!("  {:<10} {:>10.2} {:>8} {:>10.2} {:>10.2} {:>8.2} {:>10.2} {:>10.2} {:>10.2}", 
                        d.commodity, d.spot_price, d.dominant_contract, d.dominant_price,
                        d.basis, d.basis_rate,
                        d.basis_180d_high.unwrap_or(0.0),
                        d.basis_180d_low.unwrap_or(0.0),
                        d.basis_180d_avg.unwrap_or(0.0));
                }
            }
            Err(e) => {
                println!("  ❌ 获取失败: {}", e);
            }
        }
        
        // 测试较早日期
        println!("\n  2. 测试获取较早日期（20230601）:");
        match get_futures_spot_price_previous("20230601").await {
            Ok(data) => {
                println!("  ✅ 获取成功！共 {} 条数据", data.len());
                for d in data.iter().take(5) {
                    println!("    【{}】现货:{:.2} 主力:{} 基差:{:.2} 180日均值:{:.2}", 
                        d.commodity, d.spot_price, d.dominant_contract, 
                        d.basis, d.basis_180d_avg.unwrap_or(0.0));
                }
            }
            Err(e) => {
                println!("  ⚠️ 获取失败: {}", e);
            }
        }
    }

    /// 测试获取现货价格日线数据（日期范围）
    #[tokio::test]
    async fn test_futures_spot_price_daily() {
        println!("\n========== 测试获取现货价格日线数据 ==========");
        
        // 测试获取日期范围数据
        println!("\n  1. 测试获取日期范围数据（20240429-20240430，RB,CU）:");
        match get_futures_spot_price_daily("20240429", "20240430", Some(vec!["RB", "CU"])).await {
            Ok(data) => {
                println!("  ✅ 获取成功！共 {} 条数据", data.len());
                println!("\n  数据详情:");
                for d in &data {
                    println!("    {} 【{}】现货:{:.2} 主力:{} 价格:{:.2} 基差:{:.2}", 
                        d.date, d.symbol, d.spot_price, d.dominant_contract, 
                        d.dominant_contract_price, d.dom_basis);
                }
            }
            Err(e) => {
                println!("  ❌ 获取失败: {}", e);
            }
        }
        
        // 测试获取单日所有品种
        println!("\n  2. 测试获取单日所有品种（20240430）:");
        match get_futures_spot_price_daily("20240430", "20240430", None).await {
            Ok(data) => {
                println!("  ✅ 获取成功！共 {} 条数据", data.len());
                println!("\n  前10条:");
                for d in data.iter().take(10) {
                    println!("    {} 【{}】现货:{:.2} 基差:{:.2}", 
                        d.date, d.symbol, d.spot_price, d.dom_basis);
                }
            }
            Err(e) => {
                println!("  ❌ 获取失败: {}", e);
            }
        }
    }

    // ==================== 持仓排名表测试 ====================

    /// 测试获取上期所持仓排名表
    #[tokio::test]
    async fn test_get_shfe_rank_table() {
        println!("\n========== 测试获取上期所持仓排名表 ==========");
        
        match get_shfe_rank_table("20250107", Some(vec!["CU", "AL"])).await {
            Ok(data) => {
                println!("✅ 获取成功！共 {} 个合约", data.len());
                for item in data.iter().take(3) {
                    println!("\n  合约: {}", item.symbol);
                    println!("  {:<6} {:<12} {:>10} {:>10} {:<12} {:>10} {:>10}", 
                        "名次", "成交量会员", "成交量", "增减", "多单会员", "多单", "增减");
                    for row in item.data.iter().take(5) {
                        println!("  {:<6} {:<12} {:>10} {:>10} {:<12} {:>10} {:>10}", 
                            row.rank, row.vol_party_name, row.vol, row.vol_chg,
                            row.long_party_name, row.long_open_interest, row.long_open_interest_chg);
                    }
                }
            }
            Err(e) => {
                println!("❌ 获取失败: {}", e);
            }
        }
    }

    /// 测试获取中金所持仓排名表
    #[tokio::test]
    async fn test_get_cffex_rank_table() {
        println!("\n========== 测试获取中金所持仓排名表 ==========");
        
        match get_cffex_rank_table("20250107", Some(vec!["IF", "IC"])).await {
            Ok(data) => {
                println!("✅ 获取成功！共 {} 个合约", data.len());
                for item in data.iter().take(3) {
                    println!("\n  合约: {}", item.symbol);
                    for row in item.data.iter().take(5) {
                        println!("    {} - {} 成交:{} 多单:{} 空单:{}", 
                            row.rank, row.vol_party_name, row.vol,
                            row.long_open_interest, row.short_open_interest);
                    }
                }
            }
            Err(e) => {
                println!("❌ 获取失败: {}", e);
            }
        }
    }

    /// 测试获取郑商所持仓排名表
    #[tokio::test]
    async fn test_get_rank_table_czce() {
        println!("\n========== 测试获取郑商所持仓排名表 ==========");
        
        match get_rank_table_czce("20250107").await {
            Ok(data) => {
                println!("✅ 获取成功！共 {} 个合约", data.len());
                for item in data.iter().take(3) {
                    println!("\n  合约: {}", item.symbol);
                    for row in item.data.iter().take(5) {
                        println!("    {} - {} 成交:{} 多单:{} 空单:{}", 
                            row.rank, row.vol_party_name, row.vol,
                            row.long_open_interest, row.short_open_interest);
                    }
                }
            }
            Err(e) => {
                println!("❌ 获取失败: {}", e);
            }
        }
    }

    /// 测试获取大商所持仓排名表
    #[tokio::test]
    async fn test_get_dce_rank_table() {
        println!("\n========== 测试获取大商所持仓排名表 ==========");
        
        match get_dce_rank_table("20250107", Some(vec!["M", "Y"])).await {
            Ok(data) => {
                println!("✅ 获取成功！共 {} 个合约", data.len());
                for item in data.iter().take(3) {
                    println!("\n  合约: {}", item.symbol);
                    for row in item.data.iter().take(5) {
                        println!("    {} - {} 成交:{} 多单:{} 空单:{}", 
                            row.rank, row.vol_party_name, row.vol,
                            row.long_open_interest, row.short_open_interest);
                    }
                }
            }
            Err(e) => {
                println!("❌ 获取失败: {}", e);
            }
        }
    }

    /// 测试获取广期所持仓排名表
    #[tokio::test]
    async fn test_get_gfex_rank_table() {
        println!("\n========== 测试获取广期所持仓排名表 ==========");
        
        match get_gfex_rank_table("20250107", Some(vec!["SI", "LC"])).await {
            Ok(data) => {
                println!("✅ 获取成功！共 {} 个合约", data.len());
                for item in data.iter().take(3) {
                    println!("\n  合约: {}", item.symbol);
                    for row in item.data.iter().take(5) {
                        println!("    {} - {} 成交:{} 多单:{} 空单:{}", 
                            row.rank, row.vol_party_name, row.vol,
                            row.long_open_interest, row.short_open_interest);
                    }
                }
            }
            Err(e) => {
                println!("❌ 获取失败: {}", e);
            }
        }
    }

    /// 测试获取广期所品种列表
    #[tokio::test]
    async fn test_get_gfex_vars_list() {
        println!("\n========== 测试获取广期所品种列表 ==========");
        
        match get_gfex_vars_list().await {
            Ok(vars) => {
                println!("✅ 获取成功！共 {} 个品种", vars.len());
                for var in &vars {
                    println!("  - {}", var);
                }
            }
            Err(e) => {
                println!("❌ 获取失败: {}", e);
            }
        }
    }

    /// 测试获取广期所持仓排名数据（公开接口）
    #[tokio::test]
    async fn test_futures_gfex_position_rank() {
        println!("\n========== 测试获取广期所持仓排名数据 ==========");
        
        // 测试获取指定品种（使用较近的交易日）
        println!("\n  1. 测试获取指定品种（SI, LC）:");
        match futures_gfex_position_rank("20251226", Some(vec!["SI", "LC"])).await {
            Ok(data) => {
                println!("  ✅ 获取成功！共 {} 个合约", data.len());
                for item in data.iter().take(3) {
                    println!("\n    合约: {}", item.symbol);
                    for row in item.data.iter().take(5) {
                        println!("      {} - {} 成交:{} 多单:{} 空单:{}", 
                            row.rank, row.vol_party_name, row.vol,
                            row.long_open_interest, row.short_open_interest);
                    }
                }
            }
            Err(e) => {
                println!("  ❌ 获取失败: {}", e);
            }
        }
        
        // 测试获取所有品种
        println!("\n  2. 测试获取所有品种:");
        match futures_gfex_position_rank("20251226", None).await {
            Ok(data) => {
                println!("  ✅ 获取成功！共 {} 个合约", data.len());
                // 只显示前5个合约
                for item in data.iter().take(5) {
                    println!("    合约: {} ({})", item.symbol, 
                        item.data.first().map(|d| d.variety.as_str()).unwrap_or(""));
                }
            }
            Err(e) => {
                println!("  ❌ 获取失败: {}", e);
            }
        }
    }

    /// 测试获取单日持仓排名汇总数据
    #[tokio::test]
    async fn test_get_rank_sum() {
        println!("\n========== 测试获取单日持仓排名汇总数据 ==========");
        
        // 测试获取指定品种
        println!("\n  1. 测试获取指定品种（RB, CU）:");
        match get_rank_sum("20250107", Some(vec!["RB".to_string(), "CU".to_string()])).await {
            Ok(data) => {
                println!("  ✅ 获取成功！共 {} 条数据", data.len());
                println!("\n  {:<12} {:<8} {:>12} {:>12} {:>12} {:>12}", 
                    "合约", "品种", "成交量Top5", "多单Top5", "空单Top5", "日期");
                for d in data.iter().take(10) {
                    println!("  {:<12} {:<8} {:>12} {:>12} {:>12} {:>12}", 
                        d.symbol, d.variety, d.vol_top5, 
                        d.long_open_interest_top5, d.short_open_interest_top5, d.date);
                }
            }
            Err(e) => {
                println!("  ❌ 获取失败: {}", e);
            }
        }
    }

    /// 测试获取日期范围内的持仓排名汇总数据
    #[tokio::test]
    async fn test_get_rank_sum_daily() {
        println!("\n========== 测试获取日期范围内的持仓排名汇总数据 ==========");
        
        // 测试获取单日数据
        println!("\n  1. 测试获取单日数据（20250107）:");
        match get_rank_sum_daily("20250107", "20250107", Some(vec!["RB".to_string(), "CU".to_string()])).await {
            Ok(data) => {
                println!("  ✅ 获取成功！共 {} 条数据", data.len());
                println!("\n  {:<12} {:<8} {:>12} {:>12} {:>12}", 
                    "合约", "品种", "成交量Top10", "多单Top10", "空单Top10");
                for d in data.iter().take(10) {
                    println!("  {:<12} {:<8} {:>12} {:>12} {:>12}", 
                        d.symbol, d.variety, d.vol_top10, 
                        d.long_open_interest_top10, d.short_open_interest_top10);
                }
            }
            Err(e) => {
                println!("  ❌ 获取失败: {}", e);
            }
        }
        
        // 测试获取多日数据
        println!("\n  2. 测试获取多日数据（20250106-20250107）:");
        match get_rank_sum_daily("20250106", "20250107", Some(vec!["IF".to_string()])).await {
            Ok(data) => {
                println!("  ✅ 获取成功！共 {} 条数据", data.len());
                for d in data.iter().take(10) {
                    println!("    {} {} - 成交量Top20:{} 多单Top20:{} 空单Top20:{}", 
                        d.date, d.symbol, d.vol_top20, 
                        d.long_open_interest_top20, d.short_open_interest_top20);
                }
            }
            Err(e) => {
                println!("  ❌ 获取失败: {}", e);
            }
        }
    }

    /// 测试获取大商所持仓排名数据（ZIP接口）
    #[tokio::test]
    async fn test_futures_dce_position_rank() {
        println!("\n========== 测试获取大商所持仓排名数据（ZIP接口） ==========");
        
        // 测试获取指定品种
        println!("\n  1. 测试获取指定品种（M, Y）:");
        match futures_dce_position_rank("20250107", Some(vec!["M", "Y"])).await {
            Ok(data) => {
                println!("  ✅ 获取成功！共 {} 个合约", data.len());
                for item in data.iter().take(3) {
                    println!("\n    合约: {}", item.symbol);
                    for row in item.data.iter().take(5) {
                        println!("      {} - {} 成交:{} 多单:{} 空单:{}", 
                            row.rank, row.vol_party_name, row.vol,
                            row.long_open_interest, row.short_open_interest);
                    }
                }
            }
            Err(e) => {
                println!("  ❌ 获取失败: {}", e);
            }
        }
        
        // 测试获取所有品种
        println!("\n  2. 测试获取所有品种:");
        match futures_dce_position_rank("20250107", None).await {
            Ok(data) => {
                println!("  ✅ 获取成功！共 {} 个合约", data.len());
                // 只显示前5个合约
                for item in data.iter().take(5) {
                    println!("    合约: {} ({})", item.symbol, 
                        item.data.first().map(|d| d.variety.as_str()).unwrap_or(""));
                }
            }
            Err(e) => {
                println!("  ❌ 获取失败: {}", e);
            }
        }
    }

    /// 测试获取大商所持仓排名数据（HTML接口）
    #[tokio::test]
    async fn test_futures_dce_position_rank_other() {
        println!("\n========== 测试获取大商所持仓排名数据（HTML接口） ==========");
        
        // 注意：这个接口比较慢，因为需要多次HTTP请求
        match futures_dce_position_rank_other("20250107").await {
            Ok(data) => {
                println!("✅ 获取成功！共 {} 个合约", data.len());
                for item in data.iter().take(3) {
                    println!("\n  合约: {}", item.symbol);
                    for row in item.data.iter().take(5) {
                        println!("    {} - {} 成交:{} 多单:{} 空单:{}", 
                            row.rank, row.vol_party_name, row.vol,
                            row.long_open_interest, row.short_open_interest);
                    }
                }
            }
            Err(e) => {
                println!("❌ 获取失败: {}", e);
            }
        }
    }

    /// 测试获取郑商所仓单日报数据
    #[tokio::test]
    async fn test_futures_warehouse_receipt_czce() {
        println!("\n========== 测试获取郑商所仓单日报数据 ==========");
        
        // 测试获取仓单日报（使用较新日期）
        println!("\n  1. 测试获取仓单日报（20251014）:");
        match futures_warehouse_receipt_czce("20251014").await {
            Ok(data) => {
                println!("  ✅ 获取成功！共 {} 个品种", data.len());
                for item in data.iter().take(5) {
                    println!("\n    品种: {}", item.symbol);
                    for row in item.data.iter().take(3) {
                        println!("      {} - 仓单:{:?} 有效预报:{:?} 增减:{:?}", 
                            row.warehouse, row.warehouse_receipt, 
                            row.valid_forecast, row.change);
                    }
                }
            }
            Err(e) => {
                println!("  ❌ 获取失败: {}", e);
            }
        }
        
        // 测试获取较早日期
        println!("\n  2. 测试获取较早日期（20240701）:");
        match futures_warehouse_receipt_czce("20240701").await {
            Ok(data) => {
                println!("  ✅ 获取成功！共 {} 个品种", data.len());
                // 只显示品种列表
                let symbols: Vec<&str> = data.iter().map(|d| d.symbol.as_str()).collect();
                println!("    品种列表: {:?}", symbols);
            }
            Err(e) => {
                println!("  ⚠️ 获取失败（可能是非交易日）: {}", e);
            }
        }
    }

    /// 测试获取大商所仓单日报数据
    #[tokio::test]
    async fn test_futures_warehouse_receipt_dce() {
        println!("\n========== 测试获取大商所仓单日报数据 ==========");
        
        // 测试获取仓单日报
        println!("\n  1. 测试获取仓单日报（20251226）:");
        match futures_warehouse_receipt_dce("20251226").await {
            Ok(data) => {
                println!("  ✅ 获取成功！共 {} 条数据", data.len());
                println!("\n  {:<8} {:<10} {:<20} {:>10} {:>10} {:>8}", 
                    "品种代码", "品种名称", "仓库/分库", "昨日仓单", "今日仓单", "增减");
                for row in data.iter().take(15) {
                    println!("  {:<8} {:<10} {:<20} {:>10} {:>10} {:>8}", 
                        row.variety_code, row.variety_name, row.warehouse,
                        row.last_receipt, row.today_receipt, row.change);
                }
            }
            Err(e) => {
                println!("  ❌ 获取失败: {}", e);
            }
        }
        
        // 测试获取较早日期
        println!("\n  2. 测试获取较早日期（20240701）:");
        match futures_warehouse_receipt_dce("20240701").await {
            Ok(data) => {
                println!("  ✅ 获取成功！共 {} 条数据", data.len());
                // 统计品种数量
                let mut varieties: std::collections::HashSet<&str> = std::collections::HashSet::new();
                for row in &data {
                    varieties.insert(&row.variety_code);
                }
                println!("    涉及 {} 个品种", varieties.len());
            }
            Err(e) => {
                println!("  ⚠️ 获取失败（可能是非交易日）: {}", e);
            }
        }
    }

    /// 测试获取上期所仓单日报数据
    #[tokio::test]
    async fn test_futures_shfe_warehouse_receipt() {
        println!("\n========== 测试获取上期所仓单日报数据 ==========");
        
        // 测试获取仓单日报
        println!("\n  1. 测试获取仓单日报（20251226）:");
        match futures_shfe_warehouse_receipt("20251226").await {
            Ok(data) => {
                println!("  ✅ 获取成功！共 {} 个品种", data.len());
                for item in data.iter().take(3) {
                    println!("\n    品种: {}", item.symbol);
                    for row in item.data.iter().take(3) {
                        println!("      {} - {} 昨日:{} 今日:{} 增减:{} {}", 
                            row.region, row.warehouse,
                            row.last_receipt, row.today_receipt, row.change, row.unit);
                    }
                }
            }
            Err(e) => {
                println!("  ❌ 获取失败: {}", e);
            }
        }
        
        // 测试获取较早日期
        println!("\n  2. 测试获取较早日期（20240701）:");
        match futures_shfe_warehouse_receipt("20240701").await {
            Ok(data) => {
                println!("  ✅ 获取成功！共 {} 个品种", data.len());
                // 只显示品种列表
                let symbols: Vec<&str> = data.iter().map(|d| d.symbol.as_str()).collect();
                println!("    品种列表: {:?}", symbols);
            }
            Err(e) => {
                println!("  ⚠️ 获取失败（可能是非交易日）: {}", e);
            }
        }
    }

    /// 测试获取广期所仓单日报数据
    #[tokio::test]
    async fn test_futures_gfex_warehouse_receipt() {
        println!("\n========== 测试获取广期所仓单日报数据 ==========");
        
        // 测试获取仓单日报
        println!("\n  1. 测试获取仓单日报（20251226）:");
        match futures_gfex_warehouse_receipt("20251226").await {
            Ok(data) => {
                println!("  ✅ 获取成功！共 {} 个品种", data.len());
                for item in data.iter().take(5) {
                    println!("\n    品种: {}", item.symbol);
                    for row in item.data.iter().take(3) {
                        println!("      {} - {} 昨日:{} 今日:{} 增减:{}", 
                            row.variety, row.warehouse,
                            row.last_receipt, row.today_receipt, row.change);
                    }
                }
            }
            Err(e) => {
                println!("  ❌ 获取失败: {}", e);
            }
        }
        
        // 测试获取较早日期
        println!("\n  2. 测试获取较早日期（20240701）:");
        match futures_gfex_warehouse_receipt("20240701").await {
            Ok(data) => {
                println!("  ✅ 获取成功！共 {} 个品种", data.len());
                // 只显示品种列表
                let symbols: Vec<&str> = data.iter().map(|d| d.symbol.as_str()).collect();
                println!("    品种列表: {:?}", symbols);
            }
            Err(e) => {
                println!("  ⚠️ 获取失败（可能是非交易日）: {}", e);
            }
        }
    }

    /// 测试获取新浪期货持仓排名数据（新版）
    #[tokio::test]
    async fn test_futures_hold_pos_sina_v2() {
        println!("\n========== 测试获取新浪期货持仓排名数据（新版） ==========");
        
        // 测试获取成交量排名
        println!("\n  1. 测试获取成交量排名（OI2501）:");
        match futures_hold_pos_sina("成交量", "OI2501", "20241016").await {
            Ok(data) => {
                println!("  ✅ 获取成功！共 {} 条数据", data.len());
                println!("\n  {:<6} {:<15} {:>12} {:>12}", "名次", "期货公司", "成交量", "增减");
                for row in data.iter().take(10) {
                    println!("  {:<6} {:<15} {:>12} {:>12}", 
                        row.rank, row.company, row.value, row.change);
                }
            }
            Err(e) => {
                println!("  ❌ 获取失败: {}", e);
            }
        }
        
        // 测试获取多单持仓排名
        println!("\n  2. 测试获取多单持仓排名（OI2501）:");
        match futures_hold_pos_sina("多单持仓", "OI2501", "20241016").await {
            Ok(data) => {
                println!("  ✅ 获取成功！共 {} 条数据", data.len());
                for row in data.iter().take(5) {
                    println!("    {} - {} 多单:{} 增减:{}", 
                        row.rank, row.company, row.value, row.change);
                }
            }
            Err(e) => {
                println!("  ❌ 获取失败: {}", e);
            }
        }
        
        // 测试获取空单持仓排名
        println!("\n  3. 测试获取空单持仓排名（OI2501）:");
        match futures_hold_pos_sina("空单持仓", "OI2501", "20241016").await {
            Ok(data) => {
                println!("  ✅ 获取成功！共 {} 条数据", data.len());
                for row in data.iter().take(5) {
                    println!("    {} - {} 空单:{} 增减:{}", 
                        row.rank, row.company, row.value, row.change);
                }
            }
            Err(e) => {
                println!("  ❌ 获取失败: {}", e);
            }
        }
    }
}
