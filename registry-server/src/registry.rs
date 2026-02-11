//! 注册表核心服务
//!
//! 管理已注册服务实例的生命周期

use crate::models::{ServiceInstance, ServiceStatus};
use chrono::Utc;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// 服务注册表
#[derive(Debug, Clone)]
pub struct ServiceRegistry {
    instances: Arc<RwLock<HashMap<String, ServiceInstance>>>,
    heartbeat_timeout_secs: u64,
}

impl ServiceRegistry {
    pub fn new(heartbeat_timeout_secs: u64) -> Self {
        Self {
            instances: Arc::new(RwLock::new(HashMap::new())),
            heartbeat_timeout_secs,
        }
    }

    /// 注册服务实例
    pub async fn register(
        &self,
        instance_id: String,
        service_name: String,
        host: String,
        port: u16,
        metadata: HashMap<String, String>,
    ) -> String {
        let now = Utc::now();
        let instance = ServiceInstance {
            instance_id: instance_id.clone(),
            service_name,
            host,
            port,
            status: ServiceStatus::Up,
            last_heartbeat: now,
            registered_at: now,
            metadata,
        };

        let mut instances = self.instances.write().await;
        log::info!(
            "服务注册: {} ({}:{}), instance_id={}",
            instance.service_name,
            instance.host,
            instance.port,
            instance.instance_id
        );
        instances.insert(instance_id.clone(), instance);
        instance_id
    }

    /// 注销服务实例
    pub async fn deregister(&self, instance_id: &str) -> bool {
        let mut instances = self.instances.write().await;
        if instances.remove(instance_id).is_some() {
            log::info!("服务注销: instance_id={}", instance_id);
            true
        } else {
            log::warn!("注销失败，实例不存在: instance_id={}", instance_id);
            false
        }
    }

    /// 更新心跳
    pub async fn heartbeat(&self, instance_id: &str) -> bool {
        let mut instances = self.instances.write().await;
        if let Some(instance) = instances.get_mut(instance_id) {
            instance.last_heartbeat = Utc::now();
            instance.status = ServiceStatus::Up;
            log::debug!("心跳更新: instance_id={}", instance_id);
            true
        } else {
            log::warn!("心跳失败，实例不存在: instance_id={}", instance_id);
            false
        }
    }

    /// 获取所有服务实例
    pub async fn get_instances(&self) -> Vec<ServiceInstance> {
        let instances = self.instances.read().await;
        instances.values().cloned().collect()
    }

    /// 清理过期实例
    pub async fn cleanup_expired(&self) -> usize {
        let now = Utc::now();
        let timeout = chrono::Duration::seconds(self.heartbeat_timeout_secs as i64);
        let mut instances = self.instances.write().await;

        let expired_ids: Vec<String> = instances
            .iter()
            .filter(|(_, inst)| now - inst.last_heartbeat > timeout)
            .map(|(id, _)| id.clone())
            .collect();

        let count = expired_ids.len();
        for id in &expired_ids {
            log::warn!("清理过期实例: instance_id={}", id);
            instances.remove(id);
        }

        if count > 0 {
            log::info!("清理了 {} 个过期实例", count);
        }
        count
    }

    /// 启动后台过期清理定时任务
    pub fn start_cleanup_task(self) {
        let interval_secs = self.heartbeat_timeout_secs;
        tokio::spawn(async move {
            let mut interval =
                tokio::time::interval(tokio::time::Duration::from_secs(interval_secs));
            loop {
                interval.tick().await;
                self.cleanup_expired().await;
            }
        });
    }
}
