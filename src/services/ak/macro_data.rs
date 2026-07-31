//! 中国与全球宏观经济数据服务

use crate::models::ak::macro_data::MacroItem;
use std::collections::HashMap;

/// 金十 API 通用抓取函数
async fn fetch_jin10_macro_report(report_id: &str) -> Result<Vec<MacroItem>, String> {
    let url = "https://datacenter-api.jin10.com/reports/list_v2";
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .get(url)
        .header("x-app-id", "rU6QIu7JHe2gOUeR")
        .header("x-version", "1.0.0")
        .query(&[("report_id", report_id)])
        .send()
        .await
        .map_err(|e| format!("请求金十宏观接口失败: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("金十宏观接口状态码: {}", res.status()));
    }

    let json_val: serde_json::Value = res
        .json()
        .await
        .map_err(|e| format!("解析 JSON 失败: {}", e))?;
    let keys_arr = json_val["data"]["keys"]
        .as_array()
        .ok_or_else(|| "缺失 keys 字段".to_string())?;
    let values_arr = json_val["data"]["values"]
        .as_array()
        .ok_or_else(|| "缺失 values 字段".to_string())?;

    let mut col_names = Vec::new();
    for k in keys_arr {
        if let Some(name) = k["name"].as_str() {
            col_names.push(name.to_string());
        }
    }

    let mut result = Vec::new();
    for row in values_arr {
        if let Some(val_row) = row.as_array() {
            let mut data = HashMap::new();
            for (i, val) in val_row.iter().enumerate() {
                let name = col_names
                    .get(i)
                    .cloned()
                    .unwrap_or_else(|| format!("col_{}", i));
                data.insert(name, val.clone());
            }
            result.push(MacroItem { data });
        }
    }

    Ok(result)
}

/// 央行利率（按 attr_id 抓取金十数据）
async fn fetch_jin10_interest_rate(attr_id: &str, name: &str) -> Result<Vec<MacroItem>, String> {
    let url = "https://datacenter-api.jin10.com/reports/list_v2";
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .get(url)
        .header("x-app-id", "rU6QIu7JHe2gOUeR")
        .header("x-version", "1.0.0")
        .query(&[("category", "ec"), ("attr_id", attr_id)])
        .send()
        .await
        .map_err(|e| format!("请求金十央行利率接口失败: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("金十央行利率接口状态码: {}", res.status()));
    }

    let json_val: serde_json::Value = res
        .json()
        .await
        .map_err(|e| format!("解析 JSON 失败: {}", e))?;
    let values_arr = json_val["data"]["values"]
        .as_array()
        .ok_or_else(|| "缺失 values 字段".to_string())?;

    let mut result = Vec::new();
    for row in values_arr {
        if let Some(val_row) = row.as_array() {
            let mut data = HashMap::new();
            data.insert(
                "商品".to_string(),
                serde_json::Value::String(name.to_string()),
            );
            if let Some(v) = val_row.first() {
                data.insert("日期".to_string(), v.clone());
            }
            if let Some(v) = val_row.get(1) {
                data.insert("今值".to_string(), v.clone());
            }
            if let Some(v) = val_row.get(2) {
                data.insert("预测值".to_string(), v.clone());
            }
            if let Some(v) = val_row.get(3) {
                data.insert("前值".to_string(), v.clone());
            }
            result.push(MacroItem { data });
        }
    }

    Ok(result)
}

/// 1. 中国 GDP 年率/季率
pub async fn get_macro_china_gdp() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_macro_report("61").await
}

/// 2. 中国 CPI 年率
pub async fn get_macro_china_cpi() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_macro_report("62").await
}

/// 3. 中国 PPI 年率
pub async fn get_macro_china_ppi() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_macro_report("63").await
}

/// 4. 中国官方 PMI 数据
pub async fn get_macro_china_pmi() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_macro_report("64").await
}

/// 5. 中国社会融资规模
pub async fn get_macro_china_shrzgm() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_macro_report("67").await
}

/// 6. 中国 M2 货币供应量
pub async fn get_macro_china_m2() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_macro_report("68").await
}

/// 7. 中国 LPR 贷款市场报价利率 (东方财富/新浪)
pub async fn get_macro_china_lpr() -> Result<Vec<MacroItem>, String> {
    let url = "https://datacenter-web.eastmoney.com/api/data/v1/get?reportName=RPT_LPR_IPC&columns=TRADE_DATE,LPR_1Y,LPR_5Y&sortColumns=TRADE_DATE&sortTypes=-1&pageNumber=1&pageSize=500";
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .get(url)
        .send()
        .await
        .map_err(|e| format!("请求 LPR 失败: {}", e))?;
    if !res.status().is_success() {
        return Err(format!("东方财富 LPR 接口状态码: {}", res.status()));
    }

    let json_val: serde_json::Value = res
        .json()
        .await
        .map_err(|e| format!("解析 JSON 失败: {}", e))?;
    let data_arr = json_val["result"]["data"]
        .as_array()
        .ok_or_else(|| "缺失 result.data 字段".to_string())?;

    let mut result = Vec::new();
    for row in data_arr {
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

/// 8. 美国非农就业人口变动
pub async fn get_macro_usa_non_farm() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_macro_report("1").await
}

/// 9. 美国失业率
pub async fn get_macro_usa_unemployment() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_macro_report("2").await
}

/// 10. 美国 CPI 年率/月率
pub async fn get_macro_usa_cpi() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_macro_report("3").await
}

/// 11. 美国 GDP 年化季率
pub async fn get_macro_usa_gdp() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_macro_report("4").await
}

/// 12. 中国央行基准利率决议
pub async fn get_macro_bank_china_interest_rate() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_macro_report("65").await
}

/// 13. 美联储基准利率决议
pub async fn get_macro_bank_usa_interest_rate() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_macro_report("13").await
}

/// 14. 欧洲央行基准利率决议
pub async fn get_macro_bank_euro_interest_rate() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_macro_report("14").await
}

/// 15. 日本央行基准利率决议
pub async fn get_macro_bank_japan_interest_rate() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_macro_report("15").await
}

/// 16. 新西兰联储决议报告
pub async fn get_macro_bank_newzealand_interest_rate() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_interest_rate("23", "新西兰利率决议报告").await
}

/// 17. 瑞士央行决议报告
pub async fn get_macro_bank_switzerland_interest_rate() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_interest_rate("25", "瑞士央行决议报告").await
}

/// 18. 英国央行决议报告
pub async fn get_macro_bank_english_interest_rate() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_interest_rate("26", "英国央行决议报告").await
}

/// 19. 澳洲联储决议报告
pub async fn get_macro_bank_australia_interest_rate() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_interest_rate("27", "澳洲联储决议报告").await
}

/// 20. 俄罗斯央行决议报告
pub async fn get_macro_bank_russia_interest_rate() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_interest_rate("64", "俄罗斯央行决议报告").await
}

/// 21. 印度央行决议报告
pub async fn get_macro_bank_india_interest_rate() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_interest_rate("41", "印度央行决议报告").await
}

/// 22. 巴西央行决议报告
pub async fn get_macro_bank_brazil_interest_rate() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_interest_rate("37", "巴西央行决议报告").await
}

/// 23. 中国以美元计算出口年率报告
pub async fn get_macro_china_exports_yoy() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_interest_rate("66", "中国以美元计算出口年率报告").await
}

/// 24. 中国以美元计算进口年率报告
pub async fn get_macro_china_imports_yoy() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_interest_rate("77", "中国以美元计算进口年率报告").await
}

/// 25. 中国以美元计算贸易帐报告
pub async fn get_macro_china_trade_balance() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_interest_rate("61", "中国以美元计算贸易帐报告").await
}

/// 26. 中国规模以上工业增加值年率报告
pub async fn get_macro_china_industrial_production_yoy() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_interest_rate("58", "中国规模以上工业增加值年率报告").await
}

/// 27. 中国财新制造业 PMI 终值报告
pub async fn get_macro_china_cx_pmi_yearly() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_interest_rate("73", "中国财新制造业PMI终值报告").await
}

/// 28. 中国财新服务业 PMI 报告
pub async fn get_macro_china_cx_services_pmi_yearly() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_interest_rate("67", "中国财新服务业PMI报告").await
}

/// 29. 中国官方非制造业 PMI 报告
pub async fn get_macro_china_non_man_pmi() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_interest_rate("75", "中国官方非制造业PMI报告").await
}

/// 30. 中国外汇储备报告
pub async fn get_macro_china_fx_reserves_yearly() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_interest_rate("76", "中国外汇储备报告").await
}

/// 31. 美国 ADP 就业人数报告
pub async fn get_macro_usa_adp_employment() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_interest_rate("12", "美国ADP就业人数报告").await
}

/// 32. 美国初请失业金人数报告
pub async fn get_macro_usa_initial_jobless() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_interest_rate("19", "美国初请失业金人数报告").await
}

/// 33. 美国 PPI 生产者物价指数报告
pub async fn get_macro_usa_ppi() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_interest_rate("37", "美国生产者物价指数报告").await
}

/// 34. 美国 ISM 制造业 PMI 报告
pub async fn get_macro_usa_ism_pmi() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_interest_rate("28", "美国ISM制造业PMI报告").await
}

/// 35. 美国零售销售月率报告
pub async fn get_macro_usa_retail_sales() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_interest_rate("39", "美国零售销售月率报告").await
}

/// 36. 美国工业产出月率报告
pub async fn get_macro_usa_industrial_production() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_interest_rate("20", "美国工业产出月率报告").await
}

/// 37. 欧元区 GDP 季率报告
pub async fn get_macro_euro_gdp_yoy() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_interest_rate("46", "欧元区GDP季率报告").await
}

/// 38. 欧元区 CPI 年率报告
pub async fn get_macro_euro_cpi_yoy() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_interest_rate("43", "欧元区CPI年率报告").await
}

/// 39. 德国 IFO 商业景气指数报告
pub async fn get_macro_germany_ifo() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_interest_rate("50", "德国IFO商业景气指数报告").await
}

/// 40. 英国 GDP 季率报告
pub async fn get_macro_uk_gdp_quarterly() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_interest_rate("52", "英国GDP季率报告").await
}

/// 41. 澳大利亚失业率报告
pub async fn get_macro_australia_unemployment_rate() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_interest_rate("55", "澳大利亚失业率报告").await
}

/// 42. 黄金 ETF 持仓报告
pub async fn get_macro_cons_gold() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_interest_rate("80", "黄金ETF持仓报告").await
}

/// 43. 白银 ETF 持仓报告
pub async fn get_macro_cons_silver() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_interest_rate("81", "白银ETF持仓报告").await
}

/// 44. OPEC 月度原油产量报告
pub async fn get_macro_cons_opec_month() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_interest_rate("82", "OPEC月度原油产量报告").await
}

/// 45. 美国 CPI 月率报告
pub async fn get_macro_usa_cpi_monthly() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_interest_rate("9", "美国CPI月率报告").await
}

/// 46. 美国核心 CPI 月率报告
pub async fn get_macro_usa_core_cpi_monthly() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_interest_rate("10", "美国核心CPI月率报告").await
}

/// 47. 美国核心 PCE 物价指数年率报告
pub async fn get_macro_usa_core_pce_price() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_interest_rate("80", "美国核心PCE物价指数年率报告").await
}

/// 48. 美国贸易帐报告
pub async fn get_macro_usa_trade_balance() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_interest_rate("29", "美国贸易帐报告").await
}

/// 49. 美国 API 原油库存报告
pub async fn get_macro_usa_api_crude_stock() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_interest_rate("69", "美国API原油库存报告").await
}

/// 50. 美国 Markit 制造业 PMI 初值报告
pub async fn get_macro_usa_pmi() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_interest_rate("74", "美国Markit制造业PMI报告").await
}

/// 51. 美国 ISM 非制造业 PMI 报告
pub async fn get_macro_usa_ism_non_pmi() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_interest_rate("29", "美国ISM非制造业PMI报告").await
}

/// 52. 美国新屋开工总数年化报告
pub async fn get_macro_usa_house_starts() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_interest_rate("17", "美国新屋开工总数年化报告").await
}

/// 53. 美国新屋销售总数年化报告
pub async fn get_macro_usa_new_home_sales() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_interest_rate("32", "美国新屋销售总数年化报告").await
}

/// 54. 美国营建许可总数报告
pub async fn get_macro_usa_building_permits() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_interest_rate("3", "美国营建许可总数报告").await
}

/// 55. 美国谘商会消费者信心指数报告
pub async fn get_macro_usa_cb_consumer_confidence() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_interest_rate("5", "美国谘商会消费者信心指数").await
}

/// 56. 美国密歇根大学消费者信心指数初值报告
pub async fn get_macro_usa_michigan_consumer_sentiment() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_interest_rate("50", "美国密歇根大学消费者信心指数初值报告").await
}

/// 57. 欧元区工业产出月率报告
pub async fn get_macro_euro_industrial_production_mom() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_interest_rate("19", "欧元区工业产出月率报告").await
}

/// 58. 欧元区制造业 PMI 初值报告
pub async fn get_macro_euro_manufacturing_pmi() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_interest_rate("30", "欧元区制造业PMI初值报告").await
}

/// 59. 欧元区服务业 PMI 终值报告
pub async fn get_macro_euro_services_pmi() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_interest_rate("41", "欧元区服务业PMI终值报告").await
}

/// 60. 欧元区 ZEW 经济景气指数报告
pub async fn get_macro_euro_zew_economic_sentiment() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_interest_rate("44", "欧元区ZEW经济景气指数报告").await
}

/// 61. 德国 CPI 月率报告
pub async fn get_macro_germany_cpi_monthly() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_interest_rate("48", "德国CPI月率报告").await
}

/// 62. 德国 GDP 季率报告
pub async fn get_macro_germany_gdp() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_interest_rate("49", "德国GDP季率报告").await
}

/// 63. 英国 CPI 月率报告
pub async fn get_macro_uk_cpi_monthly() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_interest_rate("51", "英国CPI月率报告").await
}

/// 64. 英国失业率报告
pub async fn get_macro_uk_unemployment_rate() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_interest_rate("53", "英国失业率报告").await
}

/// 65. 澳大利亚零售销售月率报告
pub async fn get_macro_australia_retail_rate_monthly() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_interest_rate("56", "澳大利亚零售销售月率报告").await
}

/// 66. 澳大利亚 PPI 季率报告
pub async fn get_macro_australia_ppi_quarterly() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_interest_rate("57", "澳大利亚PPI季率报告").await
}

/// 67. 加拿大失业率报告
pub async fn get_macro_canada_unemployment_rate() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_interest_rate("58", "加拿大失业率报告").await
}

/// 68. 日本 CPI 年率报告
pub async fn get_macro_japan_cpi_yearly() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_interest_rate("59", "日本CPI年率报告").await
}

/// 69. 瑞士 CPI 年率报告
pub async fn get_macro_swiss_cpi_yearly() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_interest_rate("60", "瑞士CPI年率报告").await
}

/// 70. 瑞士 SVME 采购经理人指数报告
pub async fn get_macro_swiss_svme() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_interest_rate("61", "瑞士SVME采购经理人指数报告").await
}

/// 71. 中国城镇调查失业率报告
pub async fn get_macro_china_urban_unemployment() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_interest_rate("78", "中国城镇调查失业率报告").await
}

/// 72. 中国社会消费品零售总额年率报告
pub async fn get_macro_china_consumer_goods_retail() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_interest_rate("79", "中国社会消费品零售总额年率报告").await
}

/// 73. 中国 CPI 月率报告
pub async fn get_macro_china_cpi_monthly() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_interest_rate("83", "中国CPI月率报告").await
}

/// 74. 中国 PPI 年率报告
pub async fn get_macro_china_ppi_yearly() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_interest_rate("60", "中国PPI年率报告").await
}

/// 75. 中国官方制造业 PMI 报告
pub async fn get_macro_china_pmi_yearly() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_interest_rate("65", "中国官方制造业PMI报告").await
}

/// 76. 中国 M2 货币供应年率报告
pub async fn get_macro_china_m2_yearly() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_interest_rate("59", "中国M2货币供应年率报告").await
}

/// 77. 美国成屋签约销售指数报告
pub async fn get_macro_usa_pending_home_sales() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_interest_rate("38", "美国成屋签约销售指数报告").await
}

/// 78. 美国成屋销售总数年化报告
pub async fn get_macro_usa_exist_home_sales() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_interest_rate("14", "美国成屋销售总数年化报告").await
}

/// 79. 美国商业库存月率报告
pub async fn get_macro_usa_business_inventories() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_interest_rate("4", "美国商业库存月率报告").await
}

/// 80. 美国工厂订单月率报告
pub async fn get_macro_usa_factory_orders() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_interest_rate("15", "美国工厂订单月率报告").await
}

/// 81. 加拿大 CPI 年率报告
pub async fn get_macro_canada_cpi_yearly() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_interest_rate("62", "加拿大CPI年率报告").await
}

/// 82. 澳大利亚 CPI 季率报告
pub async fn get_macro_australia_cpi_quarterly() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_interest_rate("63", "澳大利亚CPI季率报告").await
}

/// 83. 英国贸易帐报告
pub async fn get_macro_uk_trade() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_interest_rate("64", "英国贸易帐报告").await
}

/// 84. 日本央行核心 CPI 年率报告
pub async fn get_macro_japan_core_cpi_yearly() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_interest_rate("66", "日本央行核心CPI年率报告").await
}

/// 85. 同花顺-数据中心-宏观数据-股票筹资
pub async fn get_macro_stock_finance() -> Result<Vec<MacroItem>, String> {
    let url = "https://data.10jqka.com.cn/macro/finance/";
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .get(url)
        .send()
        .await
        .map_err(|e| format!("请求同花顺股票筹资失败: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("同花顺股票筹资接口状态码: {}", res.status()));
    }

    let html_text = res
        .text()
        .await
        .map_err(|e| format!("读取响应失败: {}", e))?;
    let mut result = Vec::new();

    // 粗略解析 HTML 表格行
    let document = scraper::Html::parse_document(&html_text);
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
            data.insert(
                "月份".to_string(),
                serde_json::Value::String(cells[0].clone()),
            );
            data.insert(
                "募集资金".to_string(),
                serde_json::Value::String(cells[1].clone()),
            );
            data.insert(
                "首发募集资金".to_string(),
                serde_json::Value::String(cells[2].clone()),
            );
            data.insert(
                "增发募集资金".to_string(),
                serde_json::Value::String(cells[3].clone()),
            );
            data.insert(
                "配股募集资金".to_string(),
                serde_json::Value::String(cells[4].clone()),
            );
            result.push(MacroItem { data });
        }
    }

    Ok(result)
}

/// 86. 香港 CPI 年率报告
pub async fn get_macro_china_hk_cpi() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_interest_rate("68", "香港CPI年率报告").await
}

/// 87. 香港失业率报告
pub async fn get_macro_china_hk_rate_of_unemployment() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_interest_rate("69", "香港失业率报告").await
}

/// 88. 香港 GDP 年率报告
pub async fn get_macro_china_hk_gbp() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_interest_rate("70", "香港GDP年率报告").await
}

/// 89. 香港贸易帐报告
pub async fn get_macro_china_hk_trade_diff_ratio() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_interest_rate("75", "香港贸易帐报告").await
}

/// 90. 欧洲央行存款机制利率
pub async fn get_macro_euro_deposit_rate() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_interest_rate("14", "欧洲央行存款机制利率").await
}

/// 91. 欧洲央行边际贷款利率
pub async fn get_macro_euro_marginal_lending_rate() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_interest_rate("47", "欧洲央行边际贷款利率").await
}

/// 92. 加拿大央行利率决议报告
pub async fn get_macro_bank_canada_interest_rate() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_interest_rate("24", "加拿大央行利率决议报告").await
}

/// 93. 美国耐用品订单月率
pub async fn get_macro_usa_durable_goods_orders() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_interest_rate("22", "美国耐用品订单月率报告").await
}

/// 94. 美国个人支出月率
pub async fn get_macro_usa_personal_spending() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_interest_rate("11", "美国个人支出月率报告").await
}

/// 95. 中国城镇固定资产投资 (东方财富)
pub async fn get_macro_china_gdzctz() -> Result<Vec<MacroItem>, String> {
    let url = "https://datacenter-web.eastmoney.com/api/data/v1/get?reportName=RPT_ECONOMY_ASSET_INVEST&columns=REPORT_DATE,TIME,BASE,BASE_SAME,BASE_SEQUENTIAL,BASE_ACCUMULATE&sortColumns=REPORT_DATE&sortTypes=-1&pageNumber=1&pageSize=2000";
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .get(url)
        .send()
        .await
        .map_err(|e| format!("请求中国固定资产投资失败: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("东方财富固投接口状态码: {}", res.status()));
    }

    let json_val: serde_json::Value = res
        .json()
        .await
        .map_err(|e| format!("解析 JSON 失败: {}", e))?;
    let data_arr = json_val["result"]["data"]
        .as_array()
        .ok_or_else(|| "缺失 result.data 字段".to_string())?;

    let mut result = Vec::new();
    for row in data_arr {
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

/// 解析东方财富宏观数据 (通用)
async fn fetch_eastmoney_macro_data(
    report_name: &str,
    columns: &str,
) -> Result<Vec<MacroItem>, String> {
    let url = format!("https://datacenter-web.eastmoney.com/api/data/v1/get?reportName={}&columns={}&sortColumns=REPORT_DATE&sortTypes=-1&pageNumber=1&pageSize=2000", report_name, columns);
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("请求东方财富接口失败: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("东方财富接口状态码: {}", res.status()));
    }

    let json_val: serde_json::Value = res
        .json()
        .await
        .map_err(|e| format!("解析 JSON 失败: {}", e))?;
    let data_arr = json_val["result"]["data"]
        .as_array()
        .ok_or_else(|| "缺失 result.data 字段".to_string())?;

    let mut result = Vec::new();
    for row in data_arr {
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

/// 96. 中国海关进出口状况 (东方财富)
pub async fn get_macro_china_hgjck() -> Result<Vec<MacroItem>, String> {
    fetch_eastmoney_macro_data("RPT_ECONOMY_CUSTOMS", "REPORT_DATE,TIME,EXIT_INPUT_AMOUNT_SAME,EXIT_INPUT_AMOUNT_ACCUMULATE,EXIT_AMOUNT_SAME,EXIT_AMOUNT_ACCUMULATE,INPUT_AMOUNT_SAME,INPUT_AMOUNT_ACCUMULATE").await
}

/// 97. 中国财政收入 (东方财富)
pub async fn get_macro_china_czsr() -> Result<Vec<MacroItem>, String> {
    fetch_eastmoney_macro_data(
        "RPT_ECONOMY_REVENUE",
        "REPORT_DATE,TIME,BASE,BASE_SAME,BASE_SEQUENTIAL,BASE_ACCUMULATE",
    )
    .await
}

/// 98. 中国外汇信贷及外商投资 (东方财富)
pub async fn get_macro_china_whxd() -> Result<Vec<MacroItem>, String> {
    fetch_eastmoney_macro_data("RPT_ECONOMY_FOREIGN_LOAN", "REPORT_DATE,TIME,LOAN_AMOUNT,LOAN_AMOUNT_SAME,LOAN_AMOUNT_SEQUENTIAL,LOAN_AMOUNT_ACCUMULATE").await
}

/// 99. 中国消费者信心指数 (东方财富)
pub async fn get_macro_china_xfzxx() -> Result<Vec<MacroItem>, String> {
    fetch_eastmoney_macro_data("RPT_ECONOMY_CONSUME_TKI", "REPORT_DATE,TIME,CONSUMER_INDEX,CONSUMER_INDEX_SAME,CONSUMER_INDEX_SEQUENTIAL,SATISFY_INDEX").await
}

/// 100. 中国存款准备金率 (东方财富)
pub async fn get_macro_china_reserve_requirement_ratio() -> Result<Vec<MacroItem>, String> {
    fetch_eastmoney_macro_data("RPT_ECONOMY_RMB_DEPOSIT", "REPORT_DATE,TIME,LARGE_DEPOSIT,SMALL_DEPOSIT,CHANGED_REASON,BEFORE_LARGE_DEPOSIT,BEFORE_SMALL_DEPOSIT").await
}

/// 101. 加拿大零售销售月率报告
pub async fn get_macro_canada_retail_rate_monthly() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_interest_rate("55", "加拿大零售销售月率报告").await
}

/// 102. 德国零售销售月率报告
pub async fn get_macro_germany_retail_sale_monthly() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_interest_rate("47", "德国零售销售月率报告").await
}

/// 103. 英国零售销售月率报告
pub async fn get_macro_uk_retail_monthly() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_interest_rate("54", "英国零售销售月率报告").await
}

/// 104. 瑞士贸易帐报告
pub async fn get_macro_swiss_trade() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_interest_rate("58", "瑞士贸易帐报告").await
}

/// 105. 德国经调贸易帐报告
pub async fn get_macro_germany_trade_adjusted() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_interest_rate("44", "德国经调贸易帐报告").await
}

/// 106. 加拿大贸易帐报告
pub async fn get_macro_canada_trade() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_interest_rate("54", "加拿大贸易帐报告").await
}

/// 107. 瑞士 GBD 年率报告
pub async fn get_macro_swiss_gbd_yearly() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_interest_rate("60", "瑞士GBD年率报告").await
}

/// 东方财富英国深度宏观指标通用抓取函数
async fn fetch_eastmoney_uk_macro_data(symbol: &str) -> Result<Vec<MacroItem>, String> {
    let url = format!(
        "https://datacenter-web.eastmoney.com/api/data/v1/get?reportName=RPT_ECONOMICVALUE_BRITAIN&columns=ALL&filter=(INDICATOR_ID=%22{}%22)&pageNumber=1&pageSize=5000&sortColumns=REPORT_DATE&sortTypes=-1&source=WEB&client=WEB",
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
        .map_err(|e| format!("请求英国东财接口失败: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("东方财富英国接口状态码: {}", res.status()));
    }

    let json_val: serde_json::Value = res
        .json()
        .await
        .map_err(|e| format!("解析 JSON 失败: {}", e))?;
    let data_arr = json_val["result"]["data"]
        .as_array()
        .ok_or_else(|| "缺失 result.data 字段".to_string())?;

    let mut result = Vec::new();
    for row in data_arr {
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

/// 108. 英国 Halifax 房价指数月率报告
pub async fn get_macro_uk_halifax_monthly() -> Result<Vec<MacroItem>, String> {
    fetch_eastmoney_uk_macro_data("EMG00342256").await
}

/// 109. 英国 Halifax 房价指数年率报告
pub async fn get_macro_uk_halifax_yearly() -> Result<Vec<MacroItem>, String> {
    fetch_eastmoney_uk_macro_data("EMG00010370").await
}

/// 110. 英国 Rightmove 房价指数年率报告
pub async fn get_macro_uk_rightmove_yearly() -> Result<Vec<MacroItem>, String> {
    fetch_eastmoney_uk_macro_data("EMG00341608").await
}

/// 111. 英国 Rightmove 房价指数月率报告
pub async fn get_macro_uk_rightmove_monthly() -> Result<Vec<MacroItem>, String> {
    fetch_eastmoney_uk_macro_data("EMG00341607").await
}

/// 112. 德国零售销售年率报告
pub async fn get_macro_germany_retail_sale_yearly() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_interest_rate("46", "德国零售销售年率报告").await
}

/// 113. 加拿大新屋指数报告
pub async fn get_macro_canada_new_house_rate() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_interest_rate("53", "加拿大新屋指数报告").await
}

/// 114. 加拿大央行其他小类利率报告
pub async fn get_macro_canada_bank_rate() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_interest_rate("24", "加拿大央行其他小类利率报告").await
}

/// 115. 香港楼宇买卖合约（件数）报告
pub async fn get_macro_china_hk_building_volume() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_interest_rate("68", "香港楼宇买卖合约件数报告").await
}

/// 116. 香港楼宇买卖合约（金额）报告
pub async fn get_macro_china_hk_building_amount() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_interest_rate("69", "香港楼宇买卖合约金额报告").await
}

/// 117. 加拿大核心 CPI 年率报告
pub async fn get_macro_canada_core_cpi_yearly() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_interest_rate("61", "加拿大核心CPI年率报告").await
}

/// 118. 加拿大核心 CPI 月率报告
pub async fn get_macro_canada_core_cpi_monthly() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_interest_rate("60", "加拿大核心CPI月率报告").await
}

/// 119. 加拿大 CPI 月率报告
pub async fn get_macro_canada_cpi_monthly() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_interest_rate("59", "加拿大CPI月率报告").await
}

/// 120. 英国核心 CPI 年率报告
pub async fn get_macro_uk_core_cpi_yearly() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_interest_rate("50", "英国核心CPI年率报告").await
}

/// 121. 英国核心 CPI 月率报告
pub async fn get_macro_uk_core_cpi_monthly() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_interest_rate("49", "英国核心CPI月率报告").await
}

/// 122. 英国 CPI 年率报告
pub async fn get_macro_uk_cpi_yearly() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_interest_rate("48", "英国CPI年率报告").await
}

/// 123. 德国 GDP 年率报告
pub async fn get_macro_germany_gdp_yearly() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_interest_rate("43", "德国GDP年率报告").await
}

/// 124. 贝克休斯美国钻井总数报告
pub async fn get_macro_usa_rig_count() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_interest_rate("82", "贝克休斯美国钻井总数报告").await
}

/// 125. EIA原油库存油价变动幅报告
pub async fn get_macro_usa_eia_crude_rate() -> Result<Vec<MacroItem>, String> {
    fetch_jin10_interest_rate("71", "EIA原油库存变化报告").await
}

/// 126. 同花顺-数据中心-宏观数据-新增人民币贷款
pub async fn get_macro_rmb_loan() -> Result<Vec<MacroItem>, String> {
    let url = "https://data.10jqka.com.cn/macro/loan/";
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .get(url)
        .send()
        .await
        .map_err(|e| format!("请求同花顺贷款失败: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("同花顺贷款接口状态码: {}", res.status()));
    }

    let html_text = res
        .text()
        .await
        .map_err(|e| format!("读取响应失败: {}", e))?;
    let mut result = Vec::new();

    let document = scraper::Html::parse_document(&html_text);
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
            data.insert(
                "月份".to_string(),
                serde_json::Value::String(cells[0].clone()),
            );
            data.insert(
                "新增人民币贷款-总额".to_string(),
                serde_json::Value::String(cells[1].clone()),
            );
            data.insert(
                "新增人民币贷款-同比".to_string(),
                serde_json::Value::String(cells[2].clone()),
            );
            data.insert(
                "新增人民币贷款-环比".to_string(),
                serde_json::Value::String(cells[3].clone()),
            );
            data.insert(
                "累计人民币贷款-总额".to_string(),
                serde_json::Value::String(cells[4].clone()),
            );
            data.insert(
                "累计人民币贷款-同比".to_string(),
                serde_json::Value::String(cells[5].clone()),
            );
            result.push(MacroItem { data });
        }
    }

    Ok(result)
}

/// 127. 同花顺-数据中心-宏观数据-人民币存款余额
pub async fn get_macro_rmb_deposit() -> Result<Vec<MacroItem>, String> {
    let url = "https://data.10jqka.com.cn/macro/rmb/";
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .get(url)
        .send()
        .await
        .map_err(|e| format!("请求同花顺存款失败: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("同花顺存款接口状态码: {}", res.status()));
    }

    let html_text = res
        .text()
        .await
        .map_err(|e| format!("读取响应失败: {}", e))?;
    let mut result = Vec::new();

    let document = scraper::Html::parse_document(&html_text);
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
            data.insert(
                "月份".to_string(),
                serde_json::Value::String(cells[0].clone()),
            );
            data.insert(
                "新增存款-数量".to_string(),
                serde_json::Value::String(cells[1].clone()),
            );
            data.insert(
                "新增存款-同比".to_string(),
                serde_json::Value::String(cells[2].clone()),
            );
            data.insert(
                "新增存款-环比".to_string(),
                serde_json::Value::String(cells[3].clone()),
            );
            result.push(MacroItem { data });
        }
    }

    Ok(result)
}

/// 128. 华尔街见闻-经济日历/新闻网关
pub async fn get_macro_info_ws(date: &str) -> Result<Vec<MacroItem>, String> {
    // 转换 YYYYMMDD 至时间戳
    let year = &date[0..4];
    let month = &date[4..6];
    let day = &date[6..8];
    let start_time_str = format!("{}-{}-{} 00:00:00", year, month, day);
    let end_time_str = format!("{}-{}-{} 23:59:59", year, month, day);

    let start_timestamp =
        chrono::NaiveDateTime::parse_from_str(&start_time_str, "%Y-%m-%d %H:%M:%S")
            .map(|dt| dt.and_utc().timestamp())
            .map_err(|e| format!("日期解析失败: {}", e))?;
    let end_timestamp = chrono::NaiveDateTime::parse_from_str(&end_time_str, "%Y-%m-%d %H:%M:%S")
        .map(|dt| dt.and_utc().timestamp())
        .map_err(|e| format!("日期解析失败: {}", e))?;

    let url = "https://api-one-wscn.awtmt.com/apiv1/finance/macrodatas";
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .get(url)
        .query(&[("start", start_timestamp), ("end", end_timestamp)])
        .send()
        .await
        .map_err(|e| format!("请求日历网关失败: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("华尔街见闻日历接口状态码: {}", res.status()));
    }

    let json_val: serde_json::Value = res
        .json()
        .await
        .map_err(|e| format!("解析 JSON 失败: {}", e))?;
    let items_arr = json_val["data"]["items"]
        .as_array()
        .ok_or_else(|| "缺失 data.items 字段".to_string())?;

    let mut result = Vec::new();
    for row in items_arr {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_macro_china_lpr() {
        let res = get_macro_china_lpr().await;
        if let Ok(data) = res {
            println!("LPR 数据获取成功，条数: {}", data.len());
        }
    }

    #[tokio::test]
    async fn test_get_macro_bank_newzealand_interest_rate() {
        let res = get_macro_bank_newzealand_interest_rate().await;
        assert!(res.is_ok(), "获取新西兰利率失败: {:?}", res.err());
    }
}
