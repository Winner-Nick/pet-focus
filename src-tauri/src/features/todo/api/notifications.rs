use crate::infrastructure::notification::{NotificationManager, ToastLevel};

/// Todo Feature 的所有通知定义
/// 
/// 统一管理 Todo 相关的用户通知

/// 创建 Todo 成功通知
pub fn notify_todo_created(notification_manager: &NotificationManager, title: &str) {
    let _ = notification_manager.send_toast(
        format!("待办 \"{}\" 创建成功", title),
        ToastLevel::Success,
    );
}

/// 更新 Todo 成功通知
pub fn notify_todo_updated(notification_manager: &NotificationManager, title: &str) {
    let _ = notification_manager.send_toast(
        format!("待办 \"{}\" 更新成功", title),
        ToastLevel::Success,
    );
}

/// 删除 Todo 成功通知
pub fn notify_todo_deleted(notification_manager: &NotificationManager) {
    let _ = notification_manager.send_toast(
        "待办删除成功".to_string(),
        ToastLevel::Success,
    );
}

/// Todo 到期提醒通知
pub fn notify_todo_due(notification_manager: &NotificationManager, title: &str) {
    let _ = notification_manager.send_toast(
        format!("⏰ 待办 \"{}\" 已到期", title),
        ToastLevel::Warning,
    );
}

/// CalDAV 同步成功通知
pub fn notify_sync_success(
    notification_manager: &NotificationManager,
    created: usize,
    updated: usize,
    pushed: usize,
) {
    if created + updated + pushed > 0 {
        let _ = notification_manager.send_toast(
            format!(
                "同步完成：新建 {}，更新 {}，推送 {}",
                created, updated, pushed
            ),
            ToastLevel::Success,
        );
    }
}

/// CalDAV 同步失败通知
pub fn notify_sync_error(notification_manager: &NotificationManager, error: &str) {
    let _ = notification_manager.send_toast(
        format!("同步失败：{}", error),
        ToastLevel::Error,
    );
}
