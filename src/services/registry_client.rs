//! 注册中心客户端
//!
//! 负责向远程注册中心注册当前服务并定时发送心跳

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// 注册请求体
#[derive(Debug, Serialize)]
struct RegisterPayload {
    service_name: String,
    host: String,
    port: u16,
    metadata: HashMap<String, String>,
}

/// 心跳请求体
#[derive(Debug, Serialize)]
struct HeartbeatPayload {
    instance_id: String,
}

/// 注册响应中的 data 字段
#[derive(Debug, Deserialize)]
struct RegisterResponseData {
    instance_id: String,
}

/// 注册中心 API 响应
#[derive(Debug, Deserialize)]
struct ApiResp<T> {
    success: bool,
    data: Option<T>,
    message: String,
}

/// 注册中心客户端
///
/// 管理当前服务在远程注册中心的注册状态和心跳
#[derive(Debug, Clone)]
pub struct RegistryClient {
    /// 注册中心 URL（如 http://127.0.0.1:8081）
    registry_url: String,
    /// 当前服务名称
    service_name: String,
    /// 当前服务主机
    host: String,
    /// 当前服务端口
    port: u16,
    /// 心跳发送间隔（秒）
    heartbeat_interval_secs: u64,
    /// 已注册的实例 ID
    instance_id: Arc<RwLock<Option<String>>>,
    /// API Key（用于请求认证）
    api_key: String,
}

impl RegistryClient {
    /// 创建客户端实例
    pub fn new(
        registry_url: String,
        service_name: String,
        host: String,
        port: u16,
        heartbeat_interval_secs: u64,
        api_key: String,
    ) -> Self {
        Self {
            registry_url,
            service_name,
            host,
            port,
            heartbeat_interval_secs,
            instance_id: Arc::new(RwLock::new(None)),
            api_key,
        }
    }

    /// 向注册中心注册当前服务
    pub async fn register(&self) -> anyhow::Result<String> {
        let url = format!("{}/api/v1/registry/register", self.registry_url);
        let payload = RegisterPayload {
            service_name: self.service_name.clone(),
            host: self.host.clone(),
            port: self.port,
            metadata: HashMap::new(),
        };

        let client = awc::Client::new();
        let mut resp = client
            .post(&url)
            .insert_header(("X-API-Key", self.api_key.as_str()))
            .send_json(&payload)
            .await
            .map_err(|e| anyhow::anyhow!("发送注册请求失败: {}", e))?;

        let body = resp
            .json::<ApiResp<RegisterResponseData>>()
            .await
            .map_err(|e| anyhow::anyhow!("解析注册响应失败: {}", e))?;

        if body.success {
            if let Some(data) = body.data {
                let id = data.instance_id;
                let mut instance_id = self.instance_id.write().await;
                *instance_id = Some(id.clone());
                log::info!("注册成功: instance_id={}", id);
                return Ok(id);
            }
        }

        anyhow::bail!("注册失败: {}", body.message)
    }

    /// 发送心跳
    pub async fn send_heartbeat(&self) -> anyhow::Result<()> {
        let instance_id = self.instance_id.read().await;
        let id = match instance_id.as_ref() {
            Some(id) => id.clone(),
            None => anyhow::bail!("尚未注册，无法发送心跳"),
        };
        drop(instance_id);

        let url = format!("{}/api/v1/registry/heartbeat", self.registry_url);
        let payload = HeartbeatPayload { instance_id: id };

        let client = awc::Client::new();
        let mut resp = client
            .post(&url)
            .insert_header(("X-API-Key", self.api_key.as_str()))
            .send_json(&payload)
            .await
            .map_err(|e| anyhow::anyhow!("发送心跳请求失败: {}", e))?;

        let body = resp
            .json::<ApiResp<serde_json::Value>>()
            .await
            .map_err(|e| anyhow::anyhow!("解析心跳响应失败: {}", e))?;

        if body.success {
            log::debug!("心跳发送成功");
            Ok(())
        } else {
            anyhow::bail!("心跳失败: {}", body.message)
        }
    }

    /// 注销当前服务
    #[allow(dead_code)]
    pub async fn deregister(&self) -> anyhow::Result<()> {
        let instance_id = self.instance_id.read().await;
        let id = match instance_id.as_ref() {
            Some(id) => id.clone(),
            None => return Ok(()),
        };
        drop(instance_id);

        let url = format!("{}/api/v1/registry/deregister/{}", self.registry_url, id);

        let client = awc::Client::new();
        let mut resp = client
            .delete(&url)
            .insert_header(("X-API-Key", self.api_key.as_str()))
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("发送注销请求失败: {}", e))?;

        let body = resp
            .json::<ApiResp<serde_json::Value>>()
            .await
            .map_err(|e| anyhow::anyhow!("解析注销响应失败: {}", e))?;

        if body.success {
            let mut instance_id = self.instance_id.write().await;
            *instance_id = None;
            log::info!("注销成功");
        }
        Ok(())
    }

    /// 启动后台心跳定时任务
    ///
    /// 首先注册到注册中心，然后每隔 `heartbeat_interval_secs` 秒发送一次心跳。
    /// 如果注册失败，会每隔 5 秒重试。
    pub fn start_heartbeat_task(self) {
        actix_web::rt::spawn(async move {
            // 先进行注册，失败则重试
            loop {
                match self.register().await {
                    Ok(_) => break,
                    Err(e) => {
                        log::error!("注册失败，5秒后重试: {}", e);
                        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                    }
                }
            }

            // 注册成功后，定时发送心跳
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(
                self.heartbeat_interval_secs,
            ));
            loop {
                interval.tick().await;
                if let Err(e) = self.send_heartbeat().await {
                    log::error!("心跳发送失败: {}", e);
                    // 心跳失败时尝试重新注册
                    match self.register().await {
                        Ok(_) => log::info!("重新注册成功"),
                        Err(e) => log::error!("重新注册失败: {}", e),
                    }
                }
            }
        });
    }
}
