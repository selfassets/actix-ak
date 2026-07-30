//! 国家金融监督管理总局（原银保监会）数据服务
//!
//! 提供行政处罚信息公开表等数据接口

use crate::models::ak::bank::{BankFjcfDetailItem, BankFjcfListItem};
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, USER_AGENT};

/// 获取 itemId 映射
fn get_item_id(item_name: &str) -> &'static str {
    match item_name {
        "机关" => "4113",
        "本级" => "4114",
        _ => "4115", // 默认 "分局本级"
    }
}

/// 构建请求头
fn build_nfra_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(
        USER_AGENT,
        HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36"),
    );
    headers.insert(
        ACCEPT,
        HeaderValue::from_static("application/json, text/plain, */*"),
    );
    headers
}

/// 1. 获取行政处罚数据总条数
pub async fn get_bank_fjcf_total_num(item: Option<String>) -> Result<i64, String> {
    let item_str = item.unwrap_or_else(|| "分局本级".to_string());
    let item_id = get_item_id(&item_str);

    let url = "https://www.nfra.gov.cn/cbircweb/DocInfo/SelectDocByItemIdAndChild";
    let client = reqwest::Client::builder()
        .default_headers(build_nfra_headers())
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let res = client
        .get(url)
        .query(&[("itemId", item_id), ("pageSize", "18"), ("pageIndex", "1")])
        .send()
        .await
        .map_err(|e| format!("请求 NFRA 接口失败: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("NFRA 接口响应状态码: {}", res.status()));
    }

    let json_val: serde_json::Value = res
        .json()
        .await
        .map_err(|e| format!("解析 NFRA JSON 响应失败: {}", e))?;

    json_val["data"]["total"]
        .as_i64()
        .ok_or_else(|| "缺失 total 字段".to_string())
}

/// 2. 获取行政处罚数据总页数
pub async fn get_bank_fjcf_total_page(
    item: Option<String>,
    begin: Option<i32>,
) -> Result<i64, String> {
    let total_num = get_bank_fjcf_total_num(item).await?;
    let begin_page = begin.unwrap_or(1) as i64;

    let page_size = 18;
    let mut total_pages = (total_num + page_size - 1) / page_size;
    if total_pages < begin_page {
        total_pages = begin_page;
    }

    Ok(total_pages)
}

/// 3. 获取行政处罚列表（概要）
pub async fn get_bank_fjcf_list(
    page: Option<i32>,
    item: Option<String>,
    begin: Option<i32>,
) -> Result<Vec<BankFjcfListItem>, String> {
    let item_str = item.unwrap_or_else(|| "分局本级".to_string());
    let item_id = get_item_id(&item_str);
    let fetch_pages = page.unwrap_or(1).max(1);
    let start_page = begin.unwrap_or(1).max(1);

    let url = "https://www.nfra.gov.cn/cbircweb/DocInfo/SelectDocByItemIdAndChild";
    let client = reqwest::Client::builder()
        .default_headers(build_nfra_headers())
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let mut result_list = Vec::new();

    for p in start_page..(start_page + fetch_pages) {
        let res = client
            .get(url)
            .query(&[
                ("itemId", item_id),
                ("pageSize", "18"),
                ("pageIndex", &p.to_string()),
            ])
            .send()
            .await;

        if let Ok(response) = res {
            if response.status().is_success() {
                if let Ok(json_val) = response.json::<serde_json::Value>().await {
                    if let Some(rows) = json_val["data"]["rows"].as_array() {
                        for row in rows {
                            result_list.push(BankFjcfListItem {
                                doc_id: row["docId"].as_i64().map(|id| id.to_string()),
                                doc_subtitle: row["docSubtitle"].as_str().map(|s| s.to_string()),
                                publish_date: row["publishDate"].as_str().map(|s| s.to_string()),
                                doc_file_url: row["docFileUrl"].as_str().map(|s| s.to_string()),
                                doc_title: row["docTitle"].as_str().map(|s| s.to_string()),
                                general_type: Some(row["generaltype"].clone()),
                            });
                        }
                    }
                }
            }
        }
    }

    Ok(result_list)
}

/// 4. 获取行政处罚信息公开表详情
pub async fn get_bank_fjcf_detail(
    page: Option<i32>,
    item: Option<String>,
    begin: Option<i32>,
) -> Result<Vec<BankFjcfDetailItem>, String> {
    let list_items = get_bank_fjcf_list(page, item, begin).await?;
    let client = reqwest::Client::builder()
        .default_headers(build_nfra_headers())
        .build()
        .map_err(|e| format!("构建 HTTP 客户端失败: {}", e))?;

    let mut detail_list = Vec::new();

    for list_item in list_items {
        let doc_id = match list_item.doc_id {
            Some(id) => id,
            None => continue,
        };

        let detail_url = format!(
            "https://www.nfra.gov.cn/cn/static/data/DocInfo/SelectByDocId/data_docId={}.json",
            doc_id
        );

        let res = client.get(&detail_url).send().await;
        if let Ok(response) = res {
            if response.status().is_success() {
                if let Ok(json_val) = response.json::<serde_json::Value>().await {
                    let doc_clob = json_val["data"]["docClob"].as_str().unwrap_or("");
                    let pub_date = json_val["data"]["publishDate"]
                        .as_str()
                        .map(|s| s.to_string());

                    if let Some(detail) = parse_doc_clob_html(doc_clob, &doc_id, pub_date) {
                        detail_list.push(detail);
                    }
                }
            }
        }
    }

    Ok(detail_list)
}

/// 解析单篇处罚公告 HTML 中的表格内容
fn parse_doc_clob_html(
    html_str: &str,
    doc_id: &str,
    pub_date: Option<String>,
) -> Option<BankFjcfDetailItem> {
    if html_str.is_empty() {
        return None;
    }

    let document = scraper::Html::parse_document(html_str);
    let tr_selector = scraper::Selector::parse("tr").ok()?;

    let mut rows_cells = Vec::new();

    for tr in document.select(&tr_selector) {
        let td_selector = scraper::Selector::parse("td, th").ok()?;
        let cell_texts: Vec<String> = tr
            .select(&td_selector)
            .map(|c| c.text().collect::<Vec<_>>().join("").trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        if !cell_texts.is_empty() {
            rows_cells.push(cell_texts);
        }
    }

    if rows_cells.is_empty() {
        return None;
    }

    let mut val_list = Vec::new();
    for row in rows_cells {
        if row.len() == 2 {
            val_list.push(row[1].clone());
        } else if row.len() >= 4 {
            val_list.push(row[row.len() - 1].clone());
        } else if row.len() == 1 {
            val_list.push(row[0].clone());
        }
    }

    if val_list.is_empty() {
        return None;
    }

    Some(BankFjcfDetailItem {
        doc_number: val_list.get(0).cloned(),
        name: val_list.get(1).cloned(),
        unit: val_list.get(2).cloned(),
        company_name: val_list.get(3).cloned(),
        principal_name: val_list.get(4).cloned(),
        main_facts: val_list.get(5).cloned(),
        penalty_basis: val_list.get(6).cloned(),
        penalty_decision: val_list.get(7).cloned(),
        agency_name: val_list.get(8).cloned(),
        decision_date: val_list.get(9).cloned(),
        penalty_id: Some(doc_id.to_string()),
        publish_date: pub_date,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_item_id() {
        assert_eq!(get_item_id("机关"), "4113");
        assert_eq!(get_item_id("本级"), "4114");
        assert_eq!(get_item_id("分局本级"), "4115");
    }
}
