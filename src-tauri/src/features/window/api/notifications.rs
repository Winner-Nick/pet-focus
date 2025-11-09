use crate::infrastructure::notification::{NotificationManager, ToastLevel};

/// 窗口显示通知
pub fn notify_window_shown(notification_manager: &NotificationManager) {
    let _ = notification_manager.send_toast(
        "窗口已显示".to_string(),
        ToastLevel::Info,
    );
}

/// 窗口隐藏通知
pub fn notify_window_hidden(notification_manager: &NotificationManager) {
    let _ = notification_manager.send_toast(
        "窗口已隐藏".to_string(),
        ToastLevel::Info,
    );
}

/// 窗口最小化通知
pub fn notify_window_minimized(notification_manager: &NotificationManager) {
    let _ = notification_manager.send_toast(
        "窗口已最小化".to_string(),
        ToastLevel::Info,
    );
}

/// 窗口最大化通知
pub fn notify_window_maximized(notification_manager: &NotificationManager) {
    let _ = notification_manager.send_toast(
        "窗口已最大化".to_string(),
        ToastLevel::Info,
    );
}

/// 窗口全屏通知
pub fn notify_window_fullscreen(notification_manager: &NotificationManager, is_fullscreen: bool) {
    if is_fullscreen {
        let _ = notification_manager.send_toast(
            "已进入全屏模式".to_string(),
            ToastLevel::Info,
        );
    } else {
        let _ = notification_manager.send_toast(
            "已退出全屏模式".to_string(),
            ToastLevel::Info,
        );
    }
}
