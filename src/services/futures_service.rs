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
    // 遍历交易所下的多个品种，获取每个品种的合约列表
    pub async fn list_main_futures(&self, query: &FuturesQuery) -> Result<Vec<FuturesInfo>> {
        match query.exchange.as_deref() {
            Some(exchange) => {
                // 获取该交易所的所有品种node
                let nodes = self.get_exchange_nodes(exchange);
                let mut all_futures = Vec::new();
                let limit_per_node = query.limit.map(|l| (l / nodes.len().max(1)).max(1));
                
                for node in nodes {
                    match self.get_futures_by_node(node, limit_per_node).await {
                        Ok(mut futures) => all_futures.append(&mut futures),
                        Err(e) => log::warn!("获取品种 {} 数据失败: {}", node, e),
                    }
                    // 如果已经获取足够数据，提前退出
                    if let Some(limit) = query.limit {
                        if all_futures.len() >= limit {
                            break;
                        }
                    }
                }
                
                // 按持仓量排序
                all_futures.sort_by(|a, b| b.open_interest.cmp(&a.open_interest));
                
                if let Some(limit) = query.limit {
                    all_futures.truncate(limit);
                }
                Ok(all_futures)
            }
            None => {
                // 获取所有交易所的主力合约
                let mut all_futures = Vec::new();
                let exchanges = vec!["SHFE", "DCE", "CZCE", "CFFEX"];
                
                for exchange in exchanges {
                    let nodes = self.get_exchange_nodes(exchange);
                    // 每个交易所取前2个品种
                    for node in nodes.iter().take(2) {
                        match self.get_futures_by_node(node, Some(1)).await {
                            Ok(mut futures) => all_futures.append(&mut futures),
                            Err(e) => log::warn!("获取品种 {} 数据失败: {}", node, e),
                        }
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
            return Err(anyhow!("Failed to fetch futures list: {}", response.status()));
        }

        let text = response.text().await?;
        println!("📥 原始响应数据: {}", &text[..std::cmp::min(500, text.len())]);
        
        let json_data: serde_json::Value = serde_json::from_str(&text)
            .map_err(|e| anyhow!("Failed to parse JSON: {}", e))?;
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

    // 获取交易所对应的品种node列表
    // 新浪API的node参数是具体品种，不是交易所代码
    fn get_exchange_nodes(&self, exchange: &str) -> Vec<&'static str> {
        match exchange.to_uppercase().as_str() {
            // 大连商品交易所品种
            "DCE" => vec![
                "pvc_qh",   // PVC
                "zly_qh",   // 棕榈油
                "de_qh",    // 豆二
                "dp_qh",    // 豆粕
                "jd_qh",    // 鸡蛋
                "lldpe_qh", // 塑料
                "jbx_qh",   // PP
                "dy_qh",    // 豆油
                "jt_qh",    // 焦炭
                "jm_qh",    // 焦煤
                "gm_qh",    // 硅锰
                "pg_qh",    // 液化石油气
                "lh_qh",    // 生猪
            ],
            // 郑州商品交易所品种
            "CZCE" => vec![
                "pta_qh",   // PTA
                "czy_qh",   // 菜籽油
                "qm_qh",    // 强麦
                "mh_qh",    // 棉花
                "zc_qh",    // 郑煤
                "bl_qh",    // 玻璃
                "ms_qh",    // 棉纱
                "xpg_qh",   // 鲜苹果
                "cj_qh",    // 红枣
                "pk_qh",    // 花生
            ],
            // 上海期货交易所品种
            "SHFE" => vec![
                "ry_qh",    // 燃油
                "lv_qh",    // 铝
                "xj_qh",    // 橡胶
                "tong_qh",  // 铜
                "hj_qh",    // 黄金
                "lwg_qh",   // 螺纹钢
                "xc_qh",    // 线材
                "qian_qh",  // 铅
                "by_qh",    // 白银
                "ni_qh",    // 镍
                "xi_qh",    // 锡
                "zj_qh",    // 纸浆
            ],
            // 中国金融期货交易所品种
            "CFFEX" => vec![
                "qz_qh",    // 沪深300指数期货
                "gz_qh",    // 5年期国债期货
                "sngz_qh",  // 10年期国债期货
                "szgz_qh",  // 上证50指数期货
                "zzgz_qh",  // 中证500指数期货
                "im_qh",    // 中证1000指数期货
            ],
            // 上海国际能源交易中心
            "INE" => vec![
                "yy_qh",    // 原油
            ],
            // 广州期货交易所
            "GFEX" => vec![
                "si_qh",    // 工业硅
                "lc_qh",    // 碳酸锂
            ],
            _ => vec!["tong_qh"], // 默认铜
        }
    }

    // 获取交易所对应的node参数（保留兼容性，返回第一个品种）
    #[allow(dead_code)]
    fn get_exchange_node(&self, exchange: &str) -> String {
        self.get_exchange_nodes(exchange).first().unwrap_or(&"tong_qh").to_string()
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
    let base_url = "https://stock2.finance.sina.com.cn/futures/api/jsonp.php/var%20_temp=/InnerFuturesNewService.getDailyKLine";
    
    // 构建完整URL并输出
    let full_url = format!("{}?symbol={}", base_url, symbol);
    println!("📡 请求日K线数据 URL: {}", full_url);
    
    let response = client
        .get(base_url)
        .query(&[("symbol", symbol)])
        .header("Referer", "https://finance.sina.com.cn/")
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(anyhow!("Failed to fetch history data: {}", response.status()));
    }

    let text = response.text().await?;
    println!("📥 原始响应数据: {}", &text[..std::cmp::min(500, text.len())]);
    parse_sina_history_data(&text, symbol, limit)
}

// 获取期货分钟数据
// period: "1", "5", "15", "30", "60" 分钟
#[allow(dead_code)]
pub async fn get_futures_minute_data(symbol: &str, period: &str) -> Result<Vec<FuturesHistoryData>> {
    let client = Client::new();
    
    // 新浪期货分钟K线API基础URL
    let base_url = "https://stock2.finance.sina.com.cn/futures/api/jsonp.php/=/InnerFuturesNewService.getFewMinLine";
    
    // 构建完整URL并输出
    let full_url = format!("{}?symbol={}&type={}", base_url, symbol, period);
    log::info!("请求分钟K线数据 URL: {}", full_url);
    println!("📡 请求URL: {}", full_url);
    
    let response = client
        .get(base_url)
        .query(&[("symbol", symbol), ("type", period)])
        .header("Referer", "https://finance.sina.com.cn/")
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(anyhow!("Failed to fetch minute data: {}", response.status()));
    }

    let text = response.text().await?;
    println!("📥 原始响应数据: {}", &text[..std::cmp::min(500, text.len())]);
    parse_sina_minute_data(&text, symbol)
}

// 解析新浪期货日线历史数据
// 实际返回格式: var _temp=([{"d":"2025-01-16","o":"76660.000","h":"76820.000","l":"76460.000","c":"76820.000","v":"29","p":"25","s":"76710.000"},...])
fn parse_sina_history_data(data: &str, symbol: &str, limit: usize) -> Result<Vec<FuturesHistoryData>> {
    let mut history = Vec::new();
    
    // 提取JSON数组部分
    let start = data.find("([");
    let end = data.rfind("])");
    
    if start.is_none() || end.is_none() {
        println!("❌ 未找到有效的JSON数据边界");
        return Err(anyhow!("Invalid history data format: cannot find JSON boundaries"));
    }
    
    let json_str = &data[start.unwrap() + 1..end.unwrap() + 1];
    println!("📊 解析JSON数据，长度: {} 字节", json_str.len());
    
    let json_data: serde_json::Value = serde_json::from_str(json_str)
        .map_err(|e| anyhow!("Failed to parse JSON: {}", e))?;
    
    if let Some(arr) = json_data.as_array() {
        println!("📈 解析到 {} 条K线数据", arr.len());
        
        // 取最后 limit 条数据（最新的）
        let start_idx = if arr.len() > limit { arr.len() - limit } else { 0 };
        
        for item in arr.iter().skip(start_idx) {
            // 新格式：JSON对象 {"d": "日期", "o": "开盘", "h": "最高", "l": "最低", "c": "收盘", "v": "成交量", "p": "持仓", "s": "结算"}
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
            // 兼容旧格式：二维数组
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

// 解析新浪期货分钟数据
// 实际返回格式: =([{"d":"2025-12-16 21:05:00","o":"92080.000","h":"92160.000","l":"91800.000","c":"91820.000","v":"1987","p":"145118"},...])
fn parse_sina_minute_data(data: &str, symbol: &str) -> Result<Vec<FuturesHistoryData>> {
    let mut history = Vec::new();
    
    // 查找JSON数组的起始和结束位置
    let start = data.find("([");
    let end = data.rfind("])");
    
    if start.is_none() || end.is_none() {
        println!("❌ 未找到有效的JSON数据边界");
        return Err(anyhow!("Invalid minute data format: cannot find JSON boundaries"));
    }
    
    // 提取JSON数组部分（包含方括号）
    let json_str = &data[start.unwrap() + 1..end.unwrap() + 1];
    println!("📊 解析JSON数据，长度: {} 字节", json_str.len());
    
    let json_data: serde_json::Value = serde_json::from_str(json_str)
        .map_err(|e| anyhow!("Failed to parse JSON: {}", e))?;
    
    if let Some(arr) = json_data.as_array() {
        println!("📈 解析到 {} 条K线数据", arr.len());
        
        for item in arr.iter() {
            // 新格式：JSON对象 {"d": "日期", "o": "开盘", "h": "最高", "l": "最低", "c": "收盘", "v": "成交量", "p": "持仓"}
            if item.is_object() {
                let date = item["d"].as_str().unwrap_or("").to_string();
                let open = item["o"].as_str().unwrap_or("0").parse().unwrap_or(0.0);
                let high = item["h"].as_str().unwrap_or("0").parse().unwrap_or(0.0);
                let low = item["l"].as_str().unwrap_or("0").parse().unwrap_or(0.0);
                let close = item["c"].as_str().unwrap_or("0").parse().unwrap_or(0.0);
                let volume = item["v"].as_str().unwrap_or("0").parse().unwrap_or(0);
                let open_interest = item["p"].as_str().unwrap_or("0").parse().ok();
                
                history.push(FuturesHistoryData {
                    symbol: symbol.to_string(),
                    date,
                    open,
                    high,
                    low,
                    close,
                    volume,
                    open_interest,
                    settlement: None,
                });
            }
            // 兼容旧格式：二维数组 [["日期","开盘","最高","最低","收盘","成交量","持仓"],...]
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


#[cfg(test)]
mod tests {
    use super::*;

    /// 测试商品期货合约代码格式化
    /// 商品期货使用小写 nf_ 前缀
    #[test]
    fn test_format_symbol_for_realtime_commodity() {
        println!("\n========== 测试商品期货合约代码格式化 ==========");
        let service = FuturesService::new();
        
        let test_cases = vec![
            ("CU2405", "nf_CU2405"),  // 铜
            ("AL2405", "nf_AL2405"),  // 铝
            ("RB2405", "nf_RB2405"),  // 螺纹钢
            ("V2309", "nf_V2309"),    // PVC
        ];
        
        for (input, expected) in &test_cases {
            let result = service.format_symbol_for_realtime(input);
            println!("输入: {} -> 输出: {} (期望: {})", input, result, expected);
            assert_eq!(result, *expected);
        }
        println!("✅ 商品期货格式化测试通过！");
    }

    /// 测试金融期货合约代码格式化
    /// 金融期货使用 CFF_ 前缀
    #[test]
    fn test_format_symbol_for_realtime_financial() {
        println!("\n========== 测试金融期货合约代码格式化 ==========");
        let service = FuturesService::new();
        
        let test_cases = vec![
            ("IF2401", "CFF_IF2401"),  // 沪深300股指
            ("IC2401", "CFF_IC2401"),  // 中证500股指
            ("IH2401", "CFF_IH2401"),  // 上证50股指
            ("T2406", "CFF_T2406"),    // 10年期国债
            ("TF2406", "CFF_TF2406"),  // 5年期国债
        ];
        
        for (input, expected) in &test_cases {
            let result = service.format_symbol_for_realtime(input);
            println!("输入: {} -> 输出: {} (期望: {})", input, result, expected);
            assert_eq!(result, *expected);
        }
        println!("✅ 金融期货格式化测试通过！");
    }

    /// 测试已格式化的合约代码
    /// 已有前缀的代码应保持不变
    #[test]
    fn test_format_symbol_already_formatted() {
        println!("\n========== 测试已格式化的合约代码 ==========");
        let service = FuturesService::new();
        
        let test_cases = vec![
            ("nf_CU2405", "nf_CU2405"),   // 小写前缀
            ("NF_CU2405", "nf_CU2405"),   // 大写前缀转小写
            ("CFF_IF2401", "CFF_IF2401"), // 金融期货前缀
        ];
        
        for (input, expected) in &test_cases {
            let result = service.format_symbol_for_realtime(input);
            println!("输入: {} -> 输出: {} (期望: {})", input, result, expected);
            assert_eq!(result, *expected);
        }
        println!("✅ 已格式化代码测试通过！");
    }

    /// 测试中金所合约判断
    /// 判断合约是否属于中国金融期货交易所
    #[test]
    fn test_is_cffex_symbol() {
        println!("\n========== 测试中金所合约判断 ==========");
        let service = FuturesService::new();
        
        // 金融期货品种（应返回 true）
        let cffex_symbols = vec!["IF2401", "IC2401", "IH2401", "T2406", "TF2406", "TS2406"];
        println!("金融期货品种测试:");
        for symbol in &cffex_symbols {
            let result = service.is_cffex_symbol(symbol);
            println!("  {} -> {} (期望: true)", symbol, result);
            assert!(result);
        }
        
        // 商品期货品种（应返回 false）
        let commodity_symbols = vec!["CU2405", "AL2405", "RB2405"];
        println!("商品期货品种测试:");
        for symbol in &commodity_symbols {
            let result = service.is_cffex_symbol(symbol);
            println!("  {} -> {} (期望: false)", symbol, result);
            assert!(!result);
        }
        println!("✅ 中金所合约判断测试通过！");
    }

    /// 测试交易所品种节点映射
    /// 将交易所代码映射为新浪API的品种node列表
    #[test]
    fn test_get_exchange_nodes() {
        println!("\n========== 测试交易所品种节点映射 ==========");
        let service = FuturesService::new();
        
        // 测试各交易所返回的品种列表
        let dce_nodes = service.get_exchange_nodes("DCE");
        println!("大商所品种数量: {}", dce_nodes.len());
        println!("  品种列表: {:?}", &dce_nodes[..std::cmp::min(5, dce_nodes.len())]);
        assert!(dce_nodes.contains(&"pvc_qh"));
        assert!(dce_nodes.contains(&"jt_qh"));
        
        let czce_nodes = service.get_exchange_nodes("CZCE");
        println!("郑商所品种数量: {}", czce_nodes.len());
        assert!(czce_nodes.contains(&"pta_qh"));
        
        let shfe_nodes = service.get_exchange_nodes("SHFE");
        println!("上期所品种数量: {}", shfe_nodes.len());
        assert!(shfe_nodes.contains(&"tong_qh"));
        assert!(shfe_nodes.contains(&"hj_qh"));
        
        let cffex_nodes = service.get_exchange_nodes("CFFEX");
        println!("中金所品种数量: {}", cffex_nodes.len());
        assert!(cffex_nodes.contains(&"qz_qh"));
        
        // 测试 get_exchange_node 返回第一个品种
        let first_node = service.get_exchange_node("DCE");
        println!("大商所第一个品种: {}", first_node);
        assert_eq!(first_node, dce_nodes[0]);
        
        println!("✅ 交易所品种节点映射测试通过！");
    }

    /// 测试随机码生成
    /// 生成用于新浪API的rn参数
    #[test]
    fn test_generate_random_code() {
        println!("\n========== 测试随机码生成 ==========");
        let service = FuturesService::new();
        
        let code1 = service.generate_random_code();
        let code2 = service.generate_random_code();
        
        println!("生成的随机码1: {}", code1);
        println!("生成的随机码2: {}", code2);
        println!("验证: 都是十六进制字符串");
        
        assert!(code1.chars().all(|c| c.is_ascii_hexdigit()));
        assert!(code2.chars().all(|c| c.is_ascii_hexdigit()));
        println!("✅ 随机码生成测试通过！");
    }

    /// 测试获取交易所列表
    #[test]
    fn test_get_exchanges() {
        println!("\n========== 测试获取交易所列表 ==========");
        let service = FuturesService::new();
        let exchanges = service.get_exchanges();
        
        println!("交易所数量: {}", exchanges.len());
        for ex in &exchanges {
            println!("  【{}】{} - {}", ex.code, ex.name, ex.description);
        }
        
        assert_eq!(exchanges.len(), 5);
        
        let codes: Vec<&str> = exchanges.iter().map(|e| e.code.as_str()).collect();
        assert!(codes.contains(&"DCE"));
        assert!(codes.contains(&"CZCE"));
        assert!(codes.contains(&"SHFE"));
        assert!(codes.contains(&"INE"));
        assert!(codes.contains(&"CFFEX"));
        println!("✅ 交易所列表测试通过！");
    }

    /// 测试解析新浪实时数据（有效数据）
    #[test]
    fn test_parse_sina_realtime_data_valid() {
        println!("\n========== 测试解析新浪实时数据（有效数据） ==========");
        let service = FuturesService::new();
        
        // 模拟新浪API返回的数据格式
        let mock_data = r#"var hq_str_nf_CU2405="铜2405,09:00:00,75000,75500,74800,74900,75100,75200,75150,75100,74950,100,200,50000,100000,0,0,0,0,0,0,0,0,0,0,0,0,0";"#;
        println!("模拟数据: {}", mock_data);
        
        let result = service.parse_sina_realtime_data(mock_data, "CU2405");
        assert!(result.is_ok());
        
        let info = result.unwrap();
        println!("解析结果:");
        println!("  合约代码: {}", info.symbol);
        println!("  合约名称: {}", info.name);
        println!("  开盘价: {}", info.open);
        println!("  最高价: {}", info.high);
        println!("  最低价: {}", info.low);
        println!("  最新价: {}", info.current_price);
        println!("  昨结算: {:?}", info.prev_settlement);
        println!("  成交量: {}", info.volume);
        println!("  持仓量: {:?}", info.open_interest);
        
        assert_eq!(info.symbol, "CU2405");
        assert_eq!(info.name, "铜2405");
        assert_eq!(info.open, 75000.0);
        assert_eq!(info.high, 75500.0);
        assert_eq!(info.low, 74800.0);
        assert_eq!(info.current_price, 75150.0);
        assert_eq!(info.prev_settlement, Some(74950.0));
        assert_eq!(info.volume, 100000);
        assert_eq!(info.open_interest, Some(50000));
        println!("✅ 有效数据解析测试通过！");
    }

    /// 测试解析新浪实时数据（空数据）
    #[test]
    fn test_parse_sina_realtime_data_empty() {
        println!("\n========== 测试解析新浪实时数据（空数据） ==========");
        let service = FuturesService::new();
        
        let empty_data = r#"var hq_str_nf_CU2405="";"#;
        println!("模拟空数据: {}", empty_data);
        
        let result = service.parse_sina_realtime_data(empty_data, "CU2405");
        println!("解析结果: {:?}", result.is_err());
        
        assert!(result.is_err());
        println!("✅ 空数据处理测试通过（正确返回错误）！");
    }

    /// 测试解析新浪实时数据（字段不足）
    #[test]
    fn test_parse_sina_realtime_data_insufficient_fields() {
        println!("\n========== 测试解析新浪实时数据（字段不足） ==========");
        let service = FuturesService::new();
        
        let insufficient_data = r#"var hq_str_nf_CU2405="铜2405,09:00:00,75000";"#;
        println!("模拟不完整数据: {}", insufficient_data);
        
        let result = service.parse_sina_realtime_data(insufficient_data, "CU2405");
        println!("解析结果: {:?}", result.is_err());
        
        assert!(result.is_err());
        println!("✅ 字段不足处理测试通过（正确返回错误）！");
    }

    /// 测试解析多个合约实时数据
    #[test]
    fn test_parse_multiple_realtime_data() {
        println!("\n========== 测试解析多个合约实时数据 ==========");
        let service = FuturesService::new();
        
        let mock_data = r#"var hq_str_nf_CU2405="铜2405,09:00:00,75000,75500,74800,74900,75100,75200,75150,75100,74950,100,200,50000,100000,0,0,0,0,0,0,0,0,0,0,0,0,0";var hq_str_nf_AL2405="铝2405,09:00:00,19000,19200,18900,18950,19050,19100,19080,19050,18980,50,100,30000,80000,0,0,0,0,0,0,0,0,0,0,0,0,0";"#;
        println!("模拟多合约数据（铜、铝）");
        
        let symbols = vec!["CU2405".to_string(), "AL2405".to_string()];
        let result = service.parse_multiple_realtime_data(mock_data, &symbols);
        assert!(result.is_ok());
        
        let infos = result.unwrap();
        println!("解析结果: 共 {} 条数据", infos.len());
        for info in &infos {
            println!("  【{}】{} - 最新价: {}", info.symbol, info.name, info.current_price);
        }
        
        assert_eq!(infos.len(), 2);
        assert_eq!(infos[0].symbol, "CU2405");
        assert_eq!(infos[1].symbol, "AL2405");
        println!("✅ 多合约数据解析测试通过！");
    }

    /// 测试解析新浪期货列表数据
    #[test]
    fn test_parse_sina_list_data() {
        println!("\n========== 测试解析新浪期货列表数据 ==========");
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
        println!("模拟JSON数据: {}", mock_json);
        
        let result = service.parse_sina_list_data(&mock_json);
        assert!(result.is_ok());
        
        let info = result.unwrap();
        println!("解析结果:");
        println!("  合约代码: {}", info.symbol);
        println!("  合约名称: {}", info.name);
        println!("  最新价: {}", info.current_price);
        println!("  昨结算: {:?}", info.prev_settlement);
        println!("  开盘价: {}", info.open);
        println!("  最高价: {}", info.high);
        println!("  最低价: {}", info.low);
        println!("  成交量: {}", info.volume);
        println!("  持仓量: {:?}", info.open_interest);
        println!("  结算价: {:?}", info.settlement);
        
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
        println!("✅ 期货列表数据解析测试通过！");
    }

    /// 测试解析新浪历史K线数据
    #[test]
    fn test_parse_sina_history_data() {
        println!("\n========== 测试解析新浪历史K线数据 ==========");
        
        // 模拟新浪历史数据API返回格式
        let mock_data = r#"var _temp=([["2024-01-02","75000","75500","74800","75100","100000","50000","75050"],["2024-01-03","75100","75600","74900","75200","110000","51000","75150"]]);"#;
        println!("模拟历史数据格式");
        
        let result = parse_sina_history_data(mock_data, "CU2405", 10);
        assert!(result.is_ok());
        
        let history = result.unwrap();
        println!("解析结果: 共 {} 条K线数据", history.len());
        println!("{:<12} {:>10} {:>10} {:>10} {:>10} {:>10}", "日期", "开盘", "最高", "最低", "收盘", "成交量");
        println!("{}", "-".repeat(70));
        for data in &history {
            println!("{:<12} {:>10.0} {:>10.0} {:>10.0} {:>10.0} {:>10}", 
                data.date, data.open, data.high, data.low, data.close, data.volume);
        }
        
        assert_eq!(history.len(), 2);
        assert_eq!(history[0].date, "2024-01-02");
        assert_eq!(history[0].open, 75000.0);
        assert_eq!(history[0].high, 75500.0);
        assert_eq!(history[0].low, 74800.0);
        assert_eq!(history[0].close, 75100.0);
        assert_eq!(history[0].volume, 100000);
        println!("✅ 历史K线数据解析测试通过！");
    }

    /// 测试解析新浪分钟K线数据
    #[test]
    fn test_parse_sina_minute_data() {
        println!("\n========== 测试解析新浪分钟K线数据 ==========");
        
        // 模拟新浪分钟数据API返回格式
        let mock_data = r#"=([["2024-01-02 09:00","75000","75100","74950","75050","10000","50000"],["2024-01-02 09:01","75050","75150","75000","75100","8000","50100"]]);"#;
        println!("模拟分钟数据格式");
        
        let result = parse_sina_minute_data(mock_data, "CU2405");
        assert!(result.is_ok());
        
        let history = result.unwrap();
        println!("解析结果: 共 {} 条分钟数据", history.len());
        println!("{:<20} {:>10} {:>10} {:>10} {:>10}", "时间", "开盘", "最高", "最低", "收盘");
        println!("{}", "-".repeat(70));
        for data in &history {
            println!("{:<20} {:>10.0} {:>10.0} {:>10.0} {:>10.0}", 
                data.date, data.open, data.high, data.low, data.close);
        }
        
        assert_eq!(history.len(), 2);
        assert_eq!(history[0].date, "2024-01-02 09:00");
        assert_eq!(history[0].open, 75000.0);
        println!("✅ 分钟K线数据解析测试通过！");
    }

    /// 测试北京时间获取函数
    #[test]
    fn test_get_beijing_time() {
        println!("\n========== 测试北京时间获取 ==========");
        
        let beijing_time = get_beijing_time();
        println!("当前北京时间: {}", beijing_time.to_rfc3339());
        println!("时间戳: {}", beijing_time.timestamp());
        
        assert!(beijing_time.timestamp() > 0);
        println!("✅ 北京时间获取测试通过！");
    }

    // ==================== 异步集成测试 ====================
    // 以下测试会实际调用新浪API，需要网络连接
    // 运行命令: cargo test -- --nocapture

    /// 测试获取单个期货合约实时数据
    /// 调用新浪API获取铜期货(CU)的实时行情并输出
    #[tokio::test]
    async fn test_fetch_single_futures_realtime() {
        println!("\n========== 测试获取单个期货合约实时数据 ==========");
        
        let service = FuturesService::new();
        let symbol = "CU2501"; // 铜期货合约
        
        println!("正在获取合约 {} 的实时数据...", symbol);
        
        match service.get_futures_info(symbol).await {
            Ok(info) => {
                println!("✅ 获取成功！");
                println!("----------------------------------------");
                println!("合约代码: {}", info.symbol);
                println!("合约名称: {}", info.name);
                println!("最新价格: {:.2}", info.current_price);
                println!("涨跌额: {:.2}", info.change);
                println!("涨跌幅: {:.2}%", info.change_percent);
                println!("开盘价: {:.2}", info.open);
                println!("最高价: {:.2}", info.high);
                println!("最低价: {:.2}", info.low);
                println!("昨结算: {:?}", info.prev_settlement);
                println!("成交量: {}", info.volume);
                println!("持仓量: {:?}", info.open_interest);
                println!("更新时间: {}", info.updated_at);
                println!("----------------------------------------");
            }
            Err(e) => {
                println!("❌ 获取失败: {}", e);
                println!("提示: 可能是非交易时间或网络问题");
            }
        }
    }

    /// 测试批量获取多个期货合约实时数据
    /// 同时获取铜、铝、螺纹钢的实时行情
    #[tokio::test]
    async fn test_fetch_multiple_futures_realtime() {
        println!("\n========== 测试批量获取期货合约实时数据 ==========");
        
        let service = FuturesService::new();
        let symbols = vec![
            "CU2501".to_string(),  // 铜
            "AL2501".to_string(),  // 铝
            "RB2501".to_string(),  // 螺纹钢
        ];
        
        println!("正在批量获取合约 {:?} 的实时数据...", symbols);
        
        match service.get_multiple_futures(&symbols).await {
            Ok(infos) => {
                println!("✅ 获取成功！共 {} 条数据", infos.len());
                println!("========================================");
                
                for info in &infos {
                    println!("【{}】{}", info.symbol, info.name);
                    println!("  最新价: {:.2} | 涨跌: {:.2} ({:.2}%)", 
                        info.current_price, info.change, info.change_percent);
                    println!("  开: {:.2} | 高: {:.2} | 低: {:.2}", 
                        info.open, info.high, info.low);
                    println!("  成交量: {} | 持仓: {:?}", info.volume, info.open_interest);
                    println!("----------------------------------------");
                }
            }
            Err(e) => {
                println!("❌ 获取失败: {}", e);
            }
        }
    }

    /// 测试获取金融期货（股指期货）实时数据
    /// 金融期货使用 CFF_ 前缀
    #[tokio::test]
    async fn test_fetch_financial_futures_realtime() {
        println!("\n========== 测试获取金融期货实时数据 ==========");
        
        let service = FuturesService::new();
        let symbol = "IF2501"; // 沪深300股指期货
        
        println!("正在获取金融期货 {} 的实时数据...", symbol);
        println!("(金融期货使用 CFF_ 前缀)");
        
        match service.get_futures_info(symbol).await {
            Ok(info) => {
                println!("✅ 获取成功！");
                println!("----------------------------------------");
                println!("合约代码: {}", info.symbol);
                println!("合约名称: {}", info.name);
                println!("最新价格: {:.2}", info.current_price);
                println!("涨跌幅: {:.2}%", info.change_percent);
                println!("成交量: {}", info.volume);
                println!("----------------------------------------");
            }
            Err(e) => {
                println!("❌ 获取失败: {}", e);
                println!("提示: 金融期货交易时间为工作日 9:30-11:30, 13:00-15:00");
            }
        }
    }

    /// 测试获取期货列表（按交易所） ok
    /// 从新浪API获取指定交易所的期货品种列表
    #[tokio::test]
    async fn test_fetch_futures_list_by_exchange() {
        println!("\n========== 测试获取期货列表（按交易所） ==========");
        
        let service = FuturesService::new();
        
        // 测试获取大商所期货列表
        let query = FuturesQuery {
            symbol: None,
            exchange: Some("DCE".to_string()),
            category: None,
            limit: Some(5),
            start_date: None,
            end_date: None,
        };
        
        println!("正在获取大商所(DCE)期货列表，限制 {} 条...", query.limit.unwrap());
        
        match service.list_main_futures(&query).await {
            Ok(futures_list) => {
                println!("✅ 获取成功！共 {} 条数据", futures_list.len());
                println!("========================================");
                
                for (i, info) in futures_list.iter().enumerate() {
                    println!("{}. 【{}】{}", i + 1, info.symbol, info.name);
                    println!("   最新价: {:.2} | 涨跌幅: {:.2}%", 
                        info.current_price, info.change_percent);
                }
            }
            Err(e) => {
                println!("❌ 获取失败: {}", e);
            }
        }
    }

    /// 测试获取期货历史K线数据 ok
    /// 获取指定合约的日线历史数据
    #[tokio::test]
    async fn test_fetch_futures_history() {
        println!("\n========== 测试获取期货历史K线数据 ==========");
        
        let symbol = "CU2601";
        let query = FuturesQuery {
            symbol: None,
            exchange: None,
            category: None,
            limit: Some(10),
            start_date: None,
            end_date: None,
        };
        
        println!("正在获取 {} 的历史K线数据，限制 {} 条...", symbol, query.limit.unwrap());
        
        match get_futures_history(symbol, &query).await {
            Ok(history) => {
                println!("✅ 获取成功！共 {} 条数据", history.len());
                println!("========================================");
                println!("{:<12} {:>10} {:>10} {:>10} {:>10} {:>12}", 
                    "日期", "开盘", "最高", "最低", "收盘", "成交量");
                println!("----------------------------------------");
                
                for data in &history {
                    println!("{:<12} {:>10.2} {:>10.2} {:>10.2} {:>10.2} {:>12}", 
                        data.date, data.open, data.high, data.low, data.close, data.volume);
                }
            }
            Err(e) => {
                println!("❌ 获取失败: {}", e);
                println!("提示: 历史数据可能需要有效的合约代码");
            }
        }
    }

    /// 测试获取期货分钟K线数据 ok
    /// 获取指定合约的分钟级别数据
    #[tokio::test]
    async fn test_fetch_futures_minute_data() {
        println!("\n========== 测试获取期货分钟K线数据 ==========");
        
        let symbol = "CU2601";
        let period = "5"; // 5分钟K线
        
        println!("正在获取 {} 的 {}分钟 K线数据...", symbol, period);
        
        match get_futures_minute_data(symbol, period).await {
            Ok(history) => {
                println!("✅ 获取成功！共 {} 条数据", history.len());
                println!("========================================");
                
                // 只显示最近10条
                let display_count = std::cmp::min(10, history.len());
                println!("显示最近 {} 条数据:", display_count);
                println!("{:<20} {:>10} {:>10} {:>10} {:>10}", 
                    "时间", "开盘", "最高", "最低", "收盘");
                println!("----------------------------------------");
                
                for data in history.iter().rev().take(display_count) {
                    println!("{:<20} {:>10.2} {:>10.2} {:>10.2} {:>10.2}", 
                        data.date, data.open, data.high, data.low, data.close);
                }
            }
            Err(e) => {
                println!("❌ 获取失败: {}", e);
                println!("提示: 分钟数据可能只在交易时间内有效");
            }
        }
    }

    /// 测试获取所有交易所列表 通过
    #[tokio::test]
    async fn test_get_all_exchanges() {
        println!("\n========== 测试获取交易所列表 ==========");
        
        let service = FuturesService::new();
        let exchanges = service.get_exchanges();
        
        println!("✅ 支持的交易所列表:");
        println!("========================================");
        
        for exchange in &exchanges {
            println!("【{}】{}", exchange.code, exchange.name);
            println!("  英文: {}", exchange.description);
        }
    }
}