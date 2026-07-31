//! AkShare (AK) 数据服务
//!
//! 提供 AK 模块核心数据的获取与处理逻辑

pub mod bank;
pub mod bond;
pub mod cal;
pub mod crypto;
pub mod currency;
pub mod energy;
pub mod interest_rate;
pub mod macro_cnbs;
pub mod macro_data;

use crate::models::ak::{AkInfo, EpuIndexItem, FredItem, VolatilityItem};
use calamine::{DataType, Reader, Xlsx};
use std::collections::HashMap;
use std::io::Cursor;

/// 获取 AK 模块元数据及概览信息
pub async fn get_ak_info() -> Result<AkInfo, String> {
    Ok(AkInfo {
        name: "AkShare Rust Service".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        description: "AkShare financial data service API in Rust".to_string(),
        categories: vec![
            "stocks".to_string(),
            "futures".to_string(),
            "bond".to_string(),
            "fx".to_string(),
            "crypto".to_string(),
            "index".to_string(),
            "macro".to_string(),
            "article".to_string(),
        ],
    })
}

/// 获取经济政策不确定性指数（EPU Index）
///
/// 对应 akshare 中的 `article_epu_index`
pub async fn get_article_epu_index(symbol: Option<String>) -> Result<Vec<EpuIndexItem>, String> {
    let raw_symbol = symbol.unwrap_or_else(|| "China".to_string());
    let mut mapped_symbol = raw_symbol.as_str();

    if mapped_symbol == "China New" || mapped_symbol == "China" {
        mapped_symbol = "SCMP_China";
    } else if mapped_symbol == "USA" {
        mapped_symbol = "US";
    } else if mapped_symbol == "Hong Kong" {
        mapped_symbol = "HK";
    } else if mapped_symbol == "Germany" || mapped_symbol == "France" || mapped_symbol == "Italy" {
        mapped_symbol = "Europe";
    } else if mapped_symbol == "South Korea" {
        mapped_symbol = "Korea";
    } else if mapped_symbol == "Spain New" {
        mapped_symbol = "Spain";
    }

    let is_excel = matches!(
        raw_symbol.as_str(),
        "Hong Kong"
            | "Ireland"
            | "Chile"
            | "Colombia"
            | "Netherlands"
            | "Singapore"
            | "Sweden"
            | "Greece"
    );

    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64)")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    if is_excel {
        let url = if raw_symbol == "Greece" {
            format!(
                "http://www.policyuncertainty.com/media/FKT_{}_Policy_Uncertainty_Data.xlsx",
                raw_symbol
            )
        } else if raw_symbol == "Hong Kong" {
            format!(
                "http://www.policyuncertainty.com/media/{}_EPU_Data_Annotated.xlsx",
                mapped_symbol
            )
        } else {
            format!(
                "http://www.policyuncertainty.com/media/{}_Policy_Uncertainty_Data.xlsx",
                raw_symbol
            )
        };

        let response = client
            .get(&url)
            .send()
            .await
            .map_err(|e| format!("请求 EPU Excel 数据失败 [{}]: {}", url, e))?;

        if !response.status().is_success() {
            return Err(format!(
                "请求 EPU Excel 失败，HTTP 状态码: {}",
                response.status()
            ));
        }

        let bytes = response
            .bytes()
            .await
            .map_err(|e| format!("读取响应数据失败: {}", e))?;

        parse_epu_excel(&bytes)
    } else {
        let url = format!(
            "http://www.policyuncertainty.com/media/{}_Policy_Uncertainty_Data.csv",
            mapped_symbol
        );

        let response = client
            .get(&url)
            .send()
            .await
            .map_err(|e| format!("请求 EPU CSV 数据失败 [{}]: {}", url, e))?;

        if !response.status().is_success() {
            return Err(format!(
                "请求 EPU CSV 失败，HTTP 状态码: {}",
                response.status()
            ));
        }

        let text = response
            .text()
            .await
            .map_err(|e| format!("读取 CSV 文本失败: {}", e))?;

        parse_epu_csv(&text)
    }
}

/// 解析 EPU CSV 数据
fn parse_epu_csv(csv_content: &str) -> Result<Vec<EpuIndexItem>, String> {
    let mut reader = csv::ReaderBuilder::new()
        .flexible(true)
        .has_headers(true)
        .from_reader(csv_content.as_bytes());

    let headers = reader
        .headers()
        .map_err(|e| format!("读取 CSV 表头失败: {}", e))?
        .clone();

    let mut items = Vec::new();

    for result in reader.records() {
        let record = match result {
            Ok(rec) => rec,
            Err(_) => continue,
        };

        let mut year: Option<i32> = None;
        let mut month: Option<i32> = None;
        let mut epu: Option<f64> = None;
        let mut extra = HashMap::new();

        for (idx, field) in record.iter().enumerate() {
            let col_name = headers.get(idx).unwrap_or("").trim();
            if col_name.is_empty() {
                continue;
            }

            let field_trimmed = field.trim();
            let lower_col = col_name.to_lowercase();

            if lower_col == "year" || lower_col == "yyyy" {
                if let Ok(y) = field_trimmed.parse::<i32>() {
                    year = Some(y);
                }
            } else if lower_col == "month" || lower_col == "m" || lower_col == "mm" {
                if let Ok(m) = field_trimmed.parse::<i32>() {
                    month = Some(m);
                }
            } else if lower_col.contains("epu")
                || lower_col.contains("index")
                || lower_col == "scmp_china_epu"
            {
                if let Ok(v) = field_trimmed.parse::<f64>() {
                    if epu.is_none() {
                        epu = Some(v);
                    }
                }
            }

            if let Ok(v_f64) = field_trimmed.parse::<f64>() {
                extra.insert(col_name.to_string(), serde_json::Value::from(v_f64));
            } else if !field_trimmed.is_empty() {
                extra.insert(col_name.to_string(), serde_json::Value::from(field_trimmed));
            }
        }

        if year.is_some() || month.is_some() || !extra.is_empty() {
            items.push(EpuIndexItem {
                year,
                month,
                epu,
                extra,
            });
        }
    }

    Ok(items)
}

/// 解析 EPU Excel 数据
fn parse_epu_excel(bytes: &[u8]) -> Result<Vec<EpuIndexItem>, String> {
    let cursor = Cursor::new(bytes);
    let mut excel: Xlsx<_> =
        Xlsx::new(cursor).map_err(|e| format!("解析 Excel 格式失败: {}", e))?;

    let sheet_names = excel.sheet_names().to_vec();
    if sheet_names.is_empty() {
        return Err("Excel 文件中不包含工作表".to_string());
    }

    let range = excel
        .worksheet_range(&sheet_names[0])
        .map_err(|e| format!("读取工作表失败: {}", e))?;

    let mut rows = range.rows();
    let headers: Vec<String> = match rows.next() {
        Some(first_row) => first_row
            .iter()
            .map(|cell| match cell.as_string() {
                Some(s) => s.trim().to_string(),
                None => match cell.as_f64() {
                    Some(f) => f.to_string(),
                    None => match cell.as_i64() {
                        Some(i) => i.to_string(),
                        None => "".to_string(),
                    },
                },
            })
            .collect(),
        None => return Ok(Vec::new()),
    };

    let mut items = Vec::new();

    for row in rows {
        let mut year: Option<i32> = None;
        let mut month: Option<i32> = None;
        let mut epu: Option<f64> = None;
        let mut extra = HashMap::new();

        for (idx, cell) in row.iter().enumerate() {
            let col_name = headers.get(idx).map(|s| s.as_str()).unwrap_or("");
            if col_name.is_empty() {
                continue;
            }

            let lower_col = col_name.to_lowercase();

            if let Some(i) = cell.as_i64() {
                if lower_col == "year" || lower_col == "yyyy" {
                    year = Some(i as i32);
                } else if lower_col == "month" || lower_col == "m" || lower_col == "mm" {
                    month = Some(i as i32);
                }
                extra.insert(col_name.to_string(), serde_json::Value::from(i));
            } else if let Some(f) = cell.as_f64() {
                if lower_col == "year" || lower_col == "yyyy" {
                    year = Some(f as i32);
                } else if lower_col == "month" || lower_col == "m" || lower_col == "mm" {
                    month = Some(f as i32);
                } else if lower_col.contains("epu") || lower_col.contains("index") {
                    if epu.is_none() {
                        epu = Some(f);
                    }
                }
                extra.insert(col_name.to_string(), serde_json::Value::from(f));
            } else if let Some(s) = cell.as_string() {
                let s_trimmed = s.trim();
                if let Ok(v) = s_trimmed.parse::<f64>() {
                    if lower_col == "year" || lower_col == "yyyy" {
                        year = Some(v as i32);
                    } else if lower_col == "month" || lower_col == "m" || lower_col == "mm" {
                        month = Some(v as i32);
                    } else if lower_col.contains("epu") || lower_col.contains("index") {
                        if epu.is_none() {
                            epu = Some(v);
                        }
                    }
                    extra.insert(col_name.to_string(), serde_json::Value::from(v));
                } else if !s_trimmed.is_empty() {
                    extra.insert(col_name.to_string(), serde_json::Value::from(s_trimmed));
                }
            }
        }

        if year.is_some() || month.is_some() || !extra.is_empty() {
            items.push(EpuIndexItem {
                year,
                month,
                epu,
                extra,
            });
        }
    }

    Ok(items)
}

/// 美联储 FRED-MD (月度) 宏观经济数据
pub async fn fred_md(date: Option<String>) -> Result<Vec<FredItem>, String> {
    let date_str = date.unwrap_or_else(|| "2020-01".to_string());
    let url = format!(
        "https://s3.amazonaws.com/files.fred.stlouisfed.org/fred-md/monthly/{}.csv",
        date_str
    );
    fetch_and_parse_fred_csv(&url).await
}

/// 美联储 FRED-QD (季度) 宏观经济数据
pub async fn fred_qd(date: Option<String>) -> Result<Vec<FredItem>, String> {
    let date_str = date.unwrap_or_else(|| "2020-01".to_string());
    let url = format!(
        "https://s3.amazonaws.com/files.fred.stlouisfed.org/fred-md/quarterly/{}.csv",
        date_str
    );
    fetch_and_parse_fred_csv(&url).await
}

/// 请求并通用解析 FRED CSV 数据
async fn fetch_and_parse_fred_csv(url: &str) -> Result<Vec<FredItem>, String> {
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64)")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let response = client
        .get(url)
        .send()
        .await
        .map_err(|e| format!("请求 FRED 数据失败 [{}]: {}", url, e))?;

    if !response.status().is_success() {
        return Err(format!(
            "请求 FRED 数据失败，HTTP 状态码: {}",
            response.status()
        ));
    }

    let csv_content = response
        .text()
        .await
        .map_err(|e| format!("读取 CSV 文本失败: {}", e))?;

    parse_fred_csv(&csv_content)
}

/// 解析 FRED CSV 内容
fn parse_fred_csv(csv_content: &str) -> Result<Vec<FredItem>, String> {
    let mut reader = csv::ReaderBuilder::new()
        .flexible(true)
        .has_headers(true)
        .from_reader(csv_content.as_bytes());

    let headers = reader
        .headers()
        .map_err(|e| format!("读取 CSV 表头失败: {}", e))?
        .clone();

    let mut items = Vec::new();

    for result in reader.records() {
        let record = match result {
            Ok(rec) => rec,
            Err(_) => continue,
        };

        let mut data = HashMap::new();

        for (idx, field) in record.iter().enumerate() {
            let col_name = headers.get(idx).unwrap_or("").trim();
            if col_name.is_empty() {
                continue;
            }

            let field_trimmed = field.trim();
            if let Ok(v_f64) = field_trimmed.parse::<f64>() {
                data.insert(col_name.to_string(), serde_json::Value::from(v_f64));
            } else if !field_trimmed.is_empty() {
                data.insert(col_name.to_string(), serde_json::Value::from(field_trimmed));
            }
        }

        if !data.is_empty() {
            items.push(FredItem { data });
        }
    }

    Ok(items)
}

/// Oxford-Man 研究所 Realized Volatility 数据
pub async fn article_oman_rv(
    symbol: Option<String>,
    index: Option<String>,
) -> Result<Vec<VolatilityItem>, String> {
    let sym = symbol.unwrap_or_else(|| "FTSE".to_string());
    let idx = index.unwrap_or_else(|| "rk_th2".to_string());

    let url = "https://realized.oxford-man.ox.ac.uk/theme/js/visualization-data.js?20191111113154";
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64)")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .get(url)
        .send()
        .await
        .map_err(|e| format!("请求 Oxford-Man 数据失败: {}", e))?;

    if !res.status().is_success() {
        return Err(format!(
            "请求 Oxford-Man 数据失败, 状态码: {}",
            res.status()
        ));
    }

    let text = res
        .text()
        .await
        .map_err(|e| format!("读取 Oxford-Man 响应失败: {}", e))?;

    parse_oman_rv_js(&text, &sym, &idx)
}

/// 解析 Oxford-Man 脚本 JSON 变量
fn parse_oman_rv_js(text: &str, symbol: &str, index: &str) -> Result<Vec<VolatilityItem>, String> {
    let start_idx = text
        .find('{')
        .ok_or_else(|| "无法在响应脚本中定位 JSON 起始位置".to_string())?;
    let end_idx = text
        .rfind('}')
        .ok_or_else(|| "无法在响应脚本中定位 JSON 结束位置".to_string())?;

    let json_str = &text[start_idx..=end_idx];
    let v: serde_json::Value =
        serde_json::from_str(json_str).map_err(|e| format!("解析 Oxford-Man JSON 失败: {}", e))?;

    let target_key = format!(".{}", symbol);
    let sym_obj = v
        .get(&target_key)
        .ok_or_else(|| format!("未找到标的代码 [{}] 的数据", symbol))?;

    let dates = sym_obj
        .get("dates")
        .and_then(|d| d.as_array())
        .ok_or_else(|| format!("标的 [{}] 缺少 dates 数组", symbol))?;

    let index_data = sym_obj
        .get(index)
        .and_then(|i| i.get("data"))
        .and_then(|d| d.as_array())
        .ok_or_else(|| format!("标的 [{}] 缺少指标 [{}] 的 data 数组", symbol, index))?;

    let mut items = Vec::new();
    let count = dates.len().min(index_data.len());

    for i in 0..count {
        let timestamp_ms = dates[i].as_i64().unwrap_or(0);
        let val = index_data[i].as_f64();

        let date_str = chrono::DateTime::from_timestamp(timestamp_ms / 1000, 0)
            .map(|dt| dt.format("%Y-%m-%d").to_string())
            .unwrap_or_else(|| timestamp_ms.to_string());

        items.push(VolatilityItem {
            date: date_str,
            value: val,
        });
    }

    Ok(items)
}

/// Oxford-Man 研究所 Realized Volatility 简易图表数据
pub async fn article_oman_rv_short(symbol: Option<String>) -> Result<Vec<VolatilityItem>, String> {
    let sym = symbol.unwrap_or_else(|| "FTSE".to_string());
    let url = "https://realized.oxford-man.ox.ac.uk/theme/js/front-page-chart.js";

    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64)")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .get(url)
        .send()
        .await
        .map_err(|e| format!("请求 Oxford-Man 短曲线数据失败: {}", e))?;

    if !res.status().is_success() {
        return Err(format!(
            "请求 Oxford-Man 短曲线数据失败, 状态码: {}",
            res.status()
        ));
    }

    let text = res
        .text()
        .await
        .map_err(|e| format!("读取 Oxford-Man 响应失败: {}", e))?;

    let start_idx = text
        .find('{')
        .ok_or_else(|| "无法定位 JSON 起始".to_string())?;
    let end_idx = text
        .rfind('}')
        .ok_or_else(|| "无法定位 JSON 结束".to_string())?;

    let json_str = &text[start_idx..=end_idx];
    let v: serde_json::Value = serde_json::from_str(json_str)
        .map_err(|e| format!("解析 Oxford-Man 短曲线 JSON 失败: {}", e))?;

    let target_key = format!(".{}", sym);
    let data_arr = v
        .get(&target_key)
        .and_then(|o| o.get("data"))
        .and_then(|a| a.as_array())
        .ok_or_else(|| format!("未找到标的 [{}] 的短曲线数据", sym))?;

    let mut items = Vec::new();
    for row in data_arr {
        if let Some(arr) = row.as_array() {
            if arr.len() >= 2 {
                let timestamp_ms = arr[0].as_i64().unwrap_or(0);
                let val = arr[1].as_f64();
                let date_str = chrono::DateTime::from_timestamp(timestamp_ms / 1000, 0)
                    .map(|dt| dt.format("%Y-%m-%d").to_string())
                    .unwrap_or_else(|| timestamp_ms.to_string());

                items.push(VolatilityItem {
                    date: date_str,
                    value: val,
                });
            }
        }
    }

    Ok(items)
}

/// 修大成主页 Risk Lab - Realized Volatility 数据
pub async fn article_rlab_rv(symbol: Option<String>) -> Result<Vec<VolatilityItem>, String> {
    let sym = symbol.unwrap_or_else(|| "39693".to_string());
    let url = format!("https://dachxiu.chicagobooth.edu/data.php?ticker={}", sym);

    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64)")
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("请求 Risk Lab 失败: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("请求 Risk Lab 失败，状态码: {}", res.status()));
    }

    let text = res
        .text()
        .await
        .map_err(|e| format!("读取 Risk Lab 页面失败: {}", e))?;

    parse_rlab_rv_html(&text, &sym)
}

/// 解析 Risk Lab HTML/Text
fn parse_rlab_rv_html(html: &str, symbol: &str) -> Result<Vec<VolatilityItem>, String> {
    let document = scraper::Html::parse_document(html);
    let p_selector =
        scraper::Selector::parse("p").map_err(|_| "构建 CSS 选择器失败".to_string())?;

    let p_element = document
        .select(&p_selector)
        .next()
        .ok_or_else(|| "未找到包含数据的 <p> 标签".to_string())?;

    let p_text = p_element.text().collect::<Vec<_>>().join("");
    let parts: Vec<&str> = p_text.split(symbol).collect();

    if parts.len() < 3 {
        return Err("解析 Risk Lab 结构失败，分隔部分不足".to_string());
    }

    let mut items = Vec::new();

    for line in parts[2].lines() {
        let line_trimmed = line.trim();
        if line_trimmed.is_empty() {
            continue;
        }

        let tokens: Vec<&str> = line_trimmed.split_whitespace().collect();
        if tokens.len() >= 2 {
            let date_str = tokens[0].to_string();
            let val = tokens[1].parse::<f64>().ok();

            items.push(VolatilityItem {
                date: date_str,
                value: val,
            });
        }
    }

    Ok(items)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_ak_info() {
        let res = get_ak_info().await;
        assert!(res.is_ok());
        let info = res.unwrap();
        assert_eq!(info.name, "AkShare Rust Service");
        assert!(info.categories.contains(&"stocks".to_string()));
    }

    #[tokio::test]
    async fn test_parse_epu_csv() {
        let sample_csv = "Year,Month,China_EPU\n2023,1,120.5\n2023,2,115.3\n";
        let res = parse_epu_csv(sample_csv);
        assert!(res.is_ok());
        let items = res.unwrap();
        assert_eq!(items.len(), 2);
        assert_eq!(items[0].year, Some(2023));
        assert_eq!(items[0].month, Some(1));
        assert_eq!(items[0].epu, Some(120.5));
    }

    #[tokio::test]
    async fn test_parse_fred_csv() {
        let sample_csv = "sasdate,RPI,W875RX1\n1/1/1959,2437.296,2288.8\n2/1/1959,2446.91,2297.0\n";
        let res = parse_fred_csv(sample_csv);
        assert!(res.is_ok());
        let items = res.unwrap();
        assert_eq!(items.len(), 2);
        assert!(items[0].data.contains_key("sasdate"));
        assert!(items[0].data.contains_key("RPI"));
    }

    #[tokio::test]
    async fn test_parse_rlab_rv_html() {
        let sample_html = "<html><body><p>Header39693Line139693\n19960102 0.1234\n19960104 0.5678</p></body></html>";
        let res = parse_rlab_rv_html(sample_html, "39693");
        assert!(res.is_ok());
        let items = res.unwrap();
        assert_eq!(items.len(), 2);
        assert_eq!(items[0].date, "19960102");
        assert_eq!(items[0].value, Some(0.1234));
    }
}
