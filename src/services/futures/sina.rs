//! 新浪期货服务
//!
//! 封装期货数据的获取逻辑，参考 akshare/futures/futures_zh_sina.py 实现

use crate::models::{
    FuturesContractDetail, FuturesExchange, FuturesInfo, FuturesQuery, FuturesSymbolMark,
};
use anyhow::{anyhow, Result};
use regex::Regex;
use reqwest::Client;

use super::common::{
    get_beijing_time, SINA_CONTRACT_DETAIL_URL, SINA_FUTURES_LIST_API, SINA_FUTURES_REALTIME_API,
    SINA_FUTURES_SYMBOL_URL,
};

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
    pub async fn get_symbol_mark(&mut self) -> Result<Vec<FuturesSymbolMark>> {
        if let Some(ref cache) = self.symbol_mark_cache {
            return Ok(cache.clone());
        }

        println!("📡 请求品种映射数据 URL: {}", SINA_FUTURES_SYMBOL_URL);

        let response = self
            .client
            .get(SINA_FUTURES_SYMBOL_URL)
            .header(
                "User-Agent",
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36",
            )
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(anyhow!("获取品种映射失败: {}", response.status()));
        }

        let bytes = response.bytes().await?;
        let text = encoding_rs::GBK.decode(&bytes).0.to_string();

        let symbols = self.parse_symbol_mark_js(&text)?;
        self.symbol_mark_cache = Some(symbols.clone());

        Ok(symbols)
    }

    /// 解析新浪 JS 文件中的品种映射数据
    fn parse_symbol_mark_js(&self, js_text: &str) -> Result<Vec<FuturesSymbolMark>> {
        let mut symbols = Vec::new();

        let start = js_text.find("ARRFUTURESNODES = {");
        let end = js_text.find("};");

        if start.is_none() || end.is_none() {
            return Err(anyhow!("无法解析品种映射JS数据"));
        }

        let content = &js_text[start.unwrap()..end.unwrap() + 2];

        let exchanges = vec![
            ("czce", "郑州商品交易所"),
            ("dce", "大连商品交易所"),
            ("shfe", "上海期货交易所"),
            ("cffex", "中国金融期货交易所"),
            ("gfex", "广州期货交易所"),
        ];

        let item_re = Regex::new(r"\['([^']+)',\s*'([^']+)',\s*'[^']*'").unwrap();

        for (exchange_code, exchange_name) in exchanges {
            let pattern = format!(r"{}\s*:\s*\[", exchange_code);
            let re = Regex::new(&pattern).unwrap();

            if let Some(m) = re.find(content) {
                let start_pos = m.end();
                let remaining = &content[start_pos..];

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

        for s in &symbols {
            if s.symbol.contains(symbol) {
                return Ok(s.mark.clone());
            }
        }

        Err(anyhow!(
            "未找到品种 {} 的映射，请使用 /futures/symbols 查看可用品种",
            symbol
        ))
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
            "INE" => "上海期货交易所",
            _ => return Err(anyhow!("未知交易所: {}", exchange)),
        };

        Ok(symbols
            .into_iter()
            .filter(|s| s.exchange == exchange_name)
            .collect())
    }

    // ==================== 实时行情相关 ====================

    /// 获取单个期货合约实时数据
    pub async fn get_futures_info(&self, symbol: &str) -> Result<FuturesInfo> {
        let formatted_symbol = self.format_symbol_for_realtime(symbol);
        let rn_code = self.generate_random_code();
        let url = format!(
            "{}/rn={}&list={}",
            SINA_FUTURES_REALTIME_API, rn_code, formatted_symbol
        );

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
    pub async fn get_multiple_futures(&self, symbols: &[String]) -> Result<Vec<FuturesInfo>> {
        let formatted_symbols: Vec<String> = symbols
            .iter()
            .map(|s| self.format_symbol_for_realtime(s))
            .collect();

        let symbols_str = formatted_symbols.join(",");
        let rn_code = self.generate_random_code();
        let url = format!(
            "{}/rn={}&list={}",
            SINA_FUTURES_REALTIME_API, rn_code, symbols_str
        );

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
    pub async fn get_futures_realtime_by_symbol(
        &mut self,
        symbol: &str,
    ) -> Result<Vec<FuturesInfo>> {
        let node = self.get_symbol_node(symbol).await?;
        self.get_futures_by_node(&node, None).await
    }

    /// 获取期货列表（按交易所或品种）
    pub async fn list_main_futures(&mut self, query: &FuturesQuery) -> Result<Vec<FuturesInfo>> {
        match query.exchange.as_deref() {
            Some(exchange) => {
                let exchange_symbols = self.get_exchange_symbols(exchange).await?;
                let mut all_futures = Vec::new();
                let limit = query.limit.unwrap_or(20);

                for symbol_mark in exchange_symbols.iter().take(5) {
                    match self.get_futures_by_node(&symbol_mark.mark, Some(1)).await {
                        Ok(mut futures) => all_futures.append(&mut futures),
                        Err(e) => log::warn!("获取品种 {} 数据失败: {}", symbol_mark.symbol, e),
                    }
                    if all_futures.len() >= limit {
                        break;
                    }
                }

                all_futures.sort_by(|a, b| b.open_interest.cmp(&a.open_interest));
                all_futures.truncate(limit);
                Ok(all_futures)
            }
            None => {
                let mut all_futures = Vec::new();
                let exchanges = vec!["SHFE", "DCE", "CZCE", "CFFEX"];

                for exchange in exchanges {
                    if let Ok(symbols) = self.get_exchange_symbols(exchange).await {
                        for symbol_mark in symbols.iter().take(2) {
                            if let Ok(mut futures) =
                                self.get_futures_by_node(&symbol_mark.mark, Some(1)).await
                            {
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
    pub async fn get_futures_by_node(
        &self,
        node: &str,
        limit: Option<usize>,
    ) -> Result<Vec<FuturesInfo>> {
        let full_url = format!(
            "{}?page=1&sort=position&asc=0&node={}&base=futures",
            SINA_FUTURES_LIST_API, node
        );
        println!("📡 请求期货列表 URL: {}", full_url);

        let response = self
            .client
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
        let preview: String = text.chars().take(300).collect();
        println!("📥 原始响应数据: {}", preview);

        let json_data: serde_json::Value =
            serde_json::from_str(&text).map_err(|e| anyhow!("解析JSON失败: {}", e))?;

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
    pub async fn get_main_contracts(&mut self, exchange: &str) -> Result<Vec<String>> {
        let exchange_symbols = self.get_exchange_symbols(exchange).await?;
        let mut main_contracts = Vec::new();

        for symbol_mark in &exchange_symbols {
            match self.get_futures_by_node(&symbol_mark.mark, Some(5)).await {
                Ok(futures) => {
                    if !futures.is_empty() {
                        if let Some(main) =
                            futures.iter().max_by_key(|f| f.open_interest.unwrap_or(0))
                        {
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

    // ==================== 合约详情 ====================

    /// 获取期货合约详情
    pub async fn get_contract_detail(&self, symbol: &str) -> Result<FuturesContractDetail> {
        let url = format!("{}/{}.shtml", SINA_CONTRACT_DETAIL_URL, symbol);
        println!("📡 请求合约详情 URL: {}", url);

        let response = self
            .client
            .get(&url)
            .header(
                "User-Agent",
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36",
            )
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(anyhow!("获取合约详情失败: {}", response.status()));
        }

        let bytes = response.bytes().await?;
        let text = encoding_rs::GBK.decode(&bytes).0.to_string();

        self.parse_contract_detail(&text, symbol)
    }

    /// 解析合约详情HTML
    fn parse_contract_detail(&self, html: &str, symbol: &str) -> Result<FuturesContractDetail> {
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
    fn format_symbol_for_realtime(&self, symbol: &str) -> String {
        let symbol_upper = symbol.to_uppercase();

        if symbol_upper.starts_with("NF_") {
            return format!("nf_{}", &symbol_upper[3..]);
        }
        if symbol_upper.starts_with("CFF_") {
            return format!("CFF_{}", &symbol_upper[4..]);
        }

        if self.is_cffex_symbol(&symbol_upper) {
            format!("CFF_{}", symbol_upper)
        } else {
            format!("nf_{}", symbol_upper)
        }
    }

    /// 判断是否为中金所合约
    fn is_cffex_symbol(&self, symbol: &str) -> bool {
        let cffex_products = ["IF", "IC", "IH", "IM", "T", "TF", "TS", "TL"];
        cffex_products
            .iter()
            .any(|&product| symbol.starts_with(product))
    }

    /// 解析新浪期货实时数据
    pub fn parse_sina_realtime_data(
        &self,
        data: &str,
        original_symbol: &str,
    ) -> Result<FuturesInfo> {
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
                return Err(anyhow!(
                    "数据字段不足: 期望至少15个，实际{}个",
                    fields.len()
                ));
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
    fn parse_multiple_realtime_data(
        &self,
        data: &str,
        original_symbols: &[String],
    ) -> Result<Vec<FuturesInfo>> {
        let mut results = Vec::new();

        let items: Vec<&str> = data.split(';').filter(|s| !s.trim().is_empty()).collect();

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
        let current_price = item["trade"]
            .as_str()
            .unwrap_or("0")
            .parse::<f64>()
            .unwrap_or(0.0);
        let prev_settlement = item["presettlement"]
            .as_str()
            .unwrap_or("0")
            .parse::<f64>()
            .unwrap_or(0.0);
        let open = item["open"]
            .as_str()
            .unwrap_or("0")
            .parse::<f64>()
            .unwrap_or(0.0);
        let high = item["high"]
            .as_str()
            .unwrap_or("0")
            .parse::<f64>()
            .unwrap_or(0.0);
        let low = item["low"]
            .as_str()
            .unwrap_or("0")
            .parse::<f64>()
            .unwrap_or(0.0);
        let volume = item["volume"]
            .as_str()
            .unwrap_or("0")
            .parse::<u64>()
            .unwrap_or(0);
        let open_interest = item["position"].as_str().unwrap_or("0").parse::<u64>().ok();
        let settlement = item["settlement"]
            .as_str()
            .unwrap_or("0")
            .parse::<f64>()
            .ok();

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
