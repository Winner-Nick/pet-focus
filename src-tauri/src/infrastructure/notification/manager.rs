use std::sync::Arc;

use tauri::{AppHandle, Emitter, Manager};

use super::WebSocketNotification;

/// 通知管理器
/// 
/// **专门用于向用户发送通知，而不是内部逻辑通信**
/// 
/// # 设计原则
/// 
/// - **Toast**: 前端弹出 Sonner Toast（通过 Tauri Event 发送给 NotificationCenter）
/// - **WebSocket**: 外部客户端弹出通知（通过 WebServer 的 WebSocket 广播）
/// - **Native**: 系统原生通知（TODO）
/// 
/// # 注意
/// 
/// 如果只是后端通知前端更新视图或同步状态，**不要使用 NotificationManager**！
/// 直接使用 `app_handle.emit()` 发送 Tauri Event。
#[derive(Clone)]
pub struct NotificationManager {
    app_handle: AppHandle,
}

impl NotificationManager {
    pub fn new(app_handle: AppHandle) -> Self {
        Self { app_handle }
    }

    /// 发送 WebSocket 通知
    /// 
    /// 用于通过 WebSocket 向外部 API 客户端广播通知
    /// 
    /// 注意：这会通过 WebServer 的 ConnectionManager 广播到订阅了该事件的所有客户端
    pub fn send_websocket(&self, notification: &WebSocketNotification) -> anyhow::Result<()> {
        // 克隆需要的数据以满足 'static 生命周期要求
        let event = notification.event.clone();
        let payload = notification.payload.clone();
        let app_handle = self.app_handle.clone();
        
        // 尝试获取 AppState 中的 WebServer ConnectionManager
        if let Some(state) = app_handle.try_state::<crate::core::AppState>() {
            if let Some(webserver) = state.webserver_manager().get_connection_manager() {
                // 通过 WebServer 的 ConnectionManager 广播
                let message = crate::infrastructure::webserver::WsMessage::event(
                    event.clone(),
                    payload,
                );
                
                tauri::async_runtime::spawn(async move {
                    webserver.broadcast_to_channel(&event, message).await;
                });
                
                Ok(())
            } else {
                // WebServer 未运行，降级为 Tauri Event
                eprintln!("WebServer not running, falling back to Tauri Event for notification: {}", event);
                let notification_copy = WebSocketNotification { event, payload };
                notification_copy.send(&app_handle)
            }
        } else {
            // 无法获取 AppState，降级为 Tauri Event
            eprintln!("Cannot access AppState, falling back to Tauri Event for notification: {}", event);
            let notification_copy = WebSocketNotification { event, payload };
            notification_copy.send(&app_handle)
        }
    }

    /// 发送 Toast 通知（由前端 NotificationCenter 处理）
    /// 
    /// 使用 Tauri Event 发送给前端，前端 NotificationCenter 监听并显示 Sonner Toast
    pub fn send_toast(&self, message: String, level: ToastLevel) -> anyhow::Result<()> {
        self.app_handle
            .emit(
                "toast-notification",
                serde_json::json!({
                    "message": message,
                    "level": level.as_str(),
                }),
            )
            .map_err(|e| anyhow::anyhow!("Failed to emit toast notification: {}", e))
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
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            ToastLevel::Info => "info",
            ToastLevel::Success => "success",
            ToastLevel::Warning => "warning",
            ToastLevel::Error => "error",
        }
    }
}
