use serde::Serialize;
use tauri::{AppHandle, Emitter};

/// 通知渠道枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotificationChannel {
    /// WebSocket/Tauri Event 推送
    WebSocket,
    /// Sonner Toast (前端处理)
    Toast,
    /// Native 系统通知（预留）
    #[allow(dead_code)]
    Native,
}

/// WebSocket 通知消息
#[derive(Debug, Clone, Serialize)]
pub struct WebSocketNotification {
    /// 事件名称
    pub event: String,
    /// 消息负载
    pub payload: serde_json::Value,
}

impl WebSocketNotification {
    pub fn new<T: Serialize>(event: impl Into<String>, payload: T) -> Self {
        Self {
            event: event.into(),
            payload: serde_json::to_value(payload).unwrap_or(serde_json::Value::Null),
        }
    }

    /// 发送通知到前端
    pub fn send(&self, app_handle: &AppHandle) -> anyhow::Result<()> {
        app_handle
            .emit(&self.event, &self.payload)
            .map_err(|e| anyhow::anyhow!("Failed to emit event: {}", e))?;
        Ok(())
    }
}
