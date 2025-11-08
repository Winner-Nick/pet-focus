use tauri::AppHandle;

use super::WebSocketNotification;

/// 通知管理器
/// 
/// 统一管理所有通知渠道（WebSocket、Toast、Native）
pub struct NotificationManager {
    app_handle: AppHandle,
}

impl NotificationManager {
    pub fn new(app_handle: AppHandle) -> Self {
        Self { app_handle }
    }

    /// 发送 WebSocket 通知
    pub fn send_websocket(&self, notification: &WebSocketNotification) -> anyhow::Result<()> {
        notification.send(&self.app_handle)
    }

    /// 发送 Toast 通知（由前端 Sonner 处理）
    /// 
    /// 通过 WebSocket 发送特殊事件，前端监听后显示 Toast
    pub fn send_toast(&self, message: String, level: ToastLevel) -> anyhow::Result<()> {
        let notification = WebSocketNotification::new(
            "toast-notification",
            serde_json::json!({
                "message": message,
                "level": level.as_str(),
            }),
        );
        self.send_websocket(&notification)
    }

    /// 发送 Native 系统通知（预留）
    #[allow(dead_code)]
    pub fn send_native(&self, _title: String, _body: String) -> anyhow::Result<()> {
        // TODO: 使用 tauri-plugin-notification 实现
        Ok(())
    }
}

/// Toast 级别
#[derive(Debug, Clone, Copy)]
pub enum ToastLevel {
    Info,
    Success,
    Warning,
    Error,
}

impl ToastLevel {
    fn as_str(&self) -> &'static str {
        match self {
            ToastLevel::Info => "info",
            ToastLevel::Success => "success",
            ToastLevel::Warning => "warning",
            ToastLevel::Error => "error",
        }
    }
}
