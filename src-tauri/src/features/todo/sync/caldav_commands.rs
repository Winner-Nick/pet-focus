use serde::{Deserialize, Serialize};
use tauri::State;

use crate::core::AppState;
use super::{CalDavConfig, CalDavConfigService, CalDavSyncEvent};

#[derive(Debug, Deserialize)]
pub struct UpdateCalDavConfigPayload {
    pub url: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct CalDavStatus {
    pub configured: bool,
    pub url: Option<String>,
    pub username: Option<String>,
    pub last_sync_at: Option<String>,
    pub last_error: Option<String>,
    pub syncing: bool,
}

/// 获取 CalDAV 同步状态
#[tauri::command]
pub async fn get_caldav_status(state: State<'_, AppState>) -> Result<CalDavStatus, String> {
    // 获取配置
    let config = CalDavConfigService::get_config(state.db())
        .await
        .map_err(|e| e.to_string())?;

    let (configured, url, username) = if let Some(cfg) = config {
        (true, Some(cfg.url), Some(cfg.username))
    } else {
        (false, None, None)
    };

    // 获取最后同步时间
    let last_sync_at = CalDavConfigService::get_last_sync(state.db())
        .await
        .map_err(|e| e.to_string())?
        .map(|dt| dt.to_rfc3339());

    // 获取最后同步错误
    let last_error = CalDavConfigService::get_last_error(state.db())
        .await
        .map_err(|e| e.to_string())?;

    // 获取同步状态
    let syncing = state.caldav_sync_manager().is_running();

    Ok(CalDavStatus {
        configured,
        url,
        username,
        last_sync_at,
        last_error,
        syncing,
    })
}

/// 保存 CalDAV 配置
#[tauri::command]
pub async fn save_caldav_config(
    state: State<'_, AppState>,
    payload: UpdateCalDavConfigPayload,
) -> Result<CalDavStatus, String> {
    let config = CalDavConfig {
        url: payload.url.trim().to_string(),
        username: payload.username.trim().to_string(),
        password: payload.password,
    };

    if !config.is_valid() {
        return Err("CalDAV 配置信息不完整".to_string());
    }

    CalDavConfigService::set_config(state.db(), &config)
        .await
        .map_err(|e| e.to_string())?;
    
    CalDavConfigService::set_last_sync(state.db(), None)
        .await
        .map_err(|e| e.to_string())?;

    // 发送成功通知
    crate::features::todo::api::notifications::notify_caldav_config_saved(state.notification());

    // 触发同步
    use super::sync::SyncReason;
    state.caldav_sync_manager().trigger(SyncReason::ConfigUpdated);

    get_caldav_status(state).await
}

/// 清除 CalDAV 配置
#[tauri::command]
pub async fn clear_caldav_config(state: State<'_, AppState>) -> Result<CalDavStatus, String> {
    CalDavConfigService::clear_config(state.db())
        .await
        .map_err(|e| e.to_string())?;

    // 清理所有待删除的 todo
    crate::features::todo::core::service::cleanup_pending_deletes(state.db())
        .await
        .map_err(|e| e.to_string())?;

    // 发送成功通知
    crate::features::todo::api::notifications::notify_caldav_config_cleared(state.notification());

    // 触发同步
    use super::sync::SyncReason;
    state.caldav_sync_manager().trigger(SyncReason::ConfigUpdated);

    get_caldav_status(state).await
}

/// 立即执行 CalDAV 同步
#[tauri::command]
pub async fn sync_caldav_now(
    state: State<'_, AppState>,
) -> Result<CalDavSyncEvent, String> {
    use super::sync::SyncReason;
    
    state
        .caldav_sync_manager()
        .sync_now(SyncReason::Manual)
        .await
        .map_err(|e| e.to_string())
}
