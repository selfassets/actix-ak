//! 公共常量和辅助函数

use chrono::Utc;
use chrono_tz::Asia::Shanghai;
use regex::Regex;

// ==================== 新浪期货 API 常量 ====================

/// 新浪期货实时行情 API
pub const SINA_FUTURES_REALTIME_API: &str = "https://hq.sinajs.cn";
/// 新浪期货列表 API
pub const SINA_FUTURES_LIST_API: &str = "https://vip.stock.finance.sina.com.cn/quotes_service/api/json_v2.php/Market_Center.getHQFuturesData";
/// 新浪期货品种映射 JS 文件
pub const SINA_FUTURES_SYMBOL_URL: &str =
    "https://vip.stock.finance.sina.com.cn/quotes_service/view/js/qihuohangqing.js";
/// 新浪期货日K线 API
pub const SINA_FUTURES_DAILY_API: &str = "https://stock2.finance.sina.com.cn/futures/api/jsonp.php/var%20_temp=/InnerFuturesNewService.getDailyKLine";
/// 新浪期货分钟K线 API
pub const SINA_FUTURES_MINUTE_API: &str = "https://stock2.finance.sina.com.cn/futures/api/jsonp.php/=/InnerFuturesNewService.getFewMinLine";
/// 新浪期货合约详情页面
pub const SINA_CONTRACT_DETAIL_URL: &str = "https://finance.sina.com.cn/futures/quotes";
/// 外盘期货日K线API
pub const SINA_FOREIGN_DAILY_API: &str = "https://stock2.finance.sina.com.cn/futures/api/jsonp.php";
/// 新浪主力连续合约日K线API
pub const SINA_MAIN_DAILY_API: &str = "https://stock2.finance.sina.com.cn/futures/api/jsonp.php";
/// 新浪持仓排名API
pub const SINA_HOLD_POS_API: &str =
    "https://vip.stock.finance.sina.com.cn/q/view/vFutures_Positions_cjcc.php";

// ==================== 其他数据源常量 ====================

/// OpenCTP期货交易费用API
pub const OPENCTP_FEES_URL: &str = "http://openctp.cn/fees.html";
/// 九期网期货手续费API
pub const QIHUO_COMM_URL: &str = "https://www.9qihuo.com/qihuoshouxufei";
/// 国泰君安期货交易日历API
pub const GTJA_CALENDAR_URL: &str = "https://www.gtjaqh.com/pc/calendar";
/// 99期货网库存数据
pub const QH99_STOCK_URL: &str = "https://www.99qh.com/data/stockIn";
/// 现货价格数据
pub const SPOT_PRICE_URL: &str = "https://www.100ppi.com/sf";
/// 现货价格历史数据
pub const SPOT_PRICE_PREVIOUS_URL: &str = "https://www.100ppi.com/sf2";

// ==================== 交易所持仓排名API ====================

/// 上海期货交易所会员成交及持仓排名表API
pub const SHFE_VOL_RANK_URL: &str = "https://www.shfe.com.cn/data/tradedata/future/dailydata/pm";
/// 中国金融期货交易所持仓排名API
pub const CFFEX_VOL_RANK_URL: &str = "http://www.cffex.com.cn/sj/ccpm";
/// 郑州商品交易所持仓排名API
pub const CZCE_VOL_RANK_URL: &str = "http://www.czce.com.cn/cn/DFSStaticFiles/Future";
/// 大连商品交易所持仓排名API
pub const DCE_VOL_RANK_URL: &str =
    "http://www.dce.com.cn/dcereport/publicweb/dailystat/memberDealPosi/batchDownload";

/// 获取北京时间字符串（ISO 8601 格式，带+08:00时区）
pub fn get_beijing_time() -> String {
    Utc::now().with_timezone(&Shanghai).to_rfc3339()
}

/// 从合约代码中提取品种代码
pub fn extract_variety(symbol: &str) -> String {
    let re = Regex::new(r"^([A-Za-z]+)").unwrap();
    re.captures(symbol)
        .and_then(|c| c.get(1))
        .map(|m| m.as_str().to_uppercase())
        .unwrap_or_default()
}

/// 从字符串中提取字母部分
pub fn extract_letters(s: &str) -> String {
    s.chars()
        .filter(|c| c.is_ascii_alphabetic())
        .collect::<String>()
        .to_uppercase()
}

/// 从合约代码中提取月份
pub fn extract_contract_month(contract: &str) -> String {
    let digits: String = contract.chars().filter(|c| c.is_ascii_digit()).collect();
    if digits.len() >= 4 {
        digits[digits.len() - 4..].to_string()
    } else {
        digits
    }
}

/// 解析基差字符串，如 "-176-0.22%" 或 "80.03%"
pub fn parse_basis_string(s: &str) -> (f64, f64) {
    let s = s.trim();

    if s.is_empty() {
        return (0.0, 0.0);
    }

    if let Some(pct_pos) = s.rfind('%') {
        let before_pct = &s[..pct_pos];

        if let Ok(rate) = before_pct.parse::<f64>() {
            return (0.0, rate);
        }

        let chars: Vec<char> = before_pct.chars().collect();
        let mut split_pos = None;

        for i in (1..chars.len()).rev() {
            if (chars[i] == '-' || chars[i] == '+') && i > 0 && chars[i - 1].is_ascii_digit() {
                split_pos = Some(i);
                break;
            }
        }

        if let Some(pos) = split_pos {
            let basis_str: String = chars[..pos].iter().collect();
            let rate_str: String = chars[pos..].iter().collect();

            let basis = basis_str.parse::<f64>().unwrap_or(0.0);
            let rate = rate_str.parse::<f64>().unwrap_or(0.0);

            return (basis, rate);
        }

        let rate = before_pct.parse::<f64>().unwrap_or(0.0);
        return (0.0, rate);
    }

    let basis = s.parse::<f64>().unwrap_or(0.0);
    (basis, 0.0)
}

/// 中文品种名称到英文代码的映射
pub fn chinese_to_english(name: &str) -> Option<&'static str> {
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
        "PX" => Some("PX"),
        _ => None,
    };

    if result.is_some() {
        return result;
    }

    // 模糊匹配
    if name.contains("菜籽油") {
        return Some("OI");
    }
    if name.contains("甲醇") {
        return Some("MA");
    }
    if name.contains("强麦") {
        return Some("WH");
    }
    if name.contains("棉纱") {
        return Some("CY");
    }

    None
}
