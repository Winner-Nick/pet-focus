use tauri::{Emitter, State};

use crate::core::AppState;
use crate::infrastructure::webserver::types::WebServerStatus;
use crate::features::settings::service::SettingService;

const WEBSERVER_STATUS_CHANGED_EVENT: &str = "webserver-status-changed";

/// 启动 WebServer
#[tauri::command]
pub async fn start_web_server(
    state: State<'_, AppState>,
) -> Result<WebServerStatus, String> {
    let result = state
        .webserver_manager()
        .start(state.db().clone(), state.app_handle(), None)
        .await
        .map_err(|e| e.to_string());

    if result.is_ok() {
        // 保存设置
        let _ = SettingService::set_bool(state.db(), "webserver.auto_start", true).await;

        // 通知前端状态变化
        let _ = state.app_handle().emit(WEBSERVER_STATUS_CHANGED_EVENT, true);
        
        // 更新托盘菜单
        use crate::infrastructure::tray::TrayManager;
        let _ = TrayManager::update_menu(&state.app_handle(), true);
    }

    result
}

/// 停止 WebServer
#[tauri::command]
pub async fn stop_web_server(
    state: State<'_, AppState>,
) -> Result<WebServerStatus, String> {
    let result = state
        .webserver_manager()
        .stop()
        .await
        .map_err(|e| e.to_string());

    if result.is_ok() {
        // 保存设置
        let _ = SettingService::set_bool(state.db(), "webserver.auto_start", false).await;

        // 通知前端状态变化
        let _ = state.app_handle().emit(WEBSERVER_STATUS_CHANGED_EVENT, false);
        
        // 更新托盘菜单
        use crate::infrastructure::tray::TrayManager;
        let _ = TrayManager::update_menu(&state.app_handle(), false);
    }

    result
}

/// 获取 WebServer 状态
#[tauri::command]
pub async fn web_server_status(
    state: State<'_, AppState>,
) -> Result<WebServerStatus, String> {
    Ok(state.webserver_manager().status().await)
}
