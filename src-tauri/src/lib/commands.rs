use serde::Deserialize;
use tauri::{Emitter, State};

use super::{
    models::todo::Todo,
    services::{setting_service::SettingService, todo},
};
#[cfg(not(any(target_os = "android", target_os = "ios")))]
use super::webserver::WebServerStatus;
use crate::AppState;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
const WEBSERVER_STATUS_CHANGED_EVENT: &str = "webserver-status-changed";

#[derive(Debug, Default, Deserialize)]
pub struct CreateTodoPayload {
    pub title: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTodoPayload {
    pub id: i32,
    pub title: Option<String>,
    pub completed: Option<bool>,
}

#[derive(Debug, Default, Deserialize)]
pub struct UpdateTodoDetailsPayload {
    pub id: i32,
    pub description: Option<String>,
    pub priority: Option<i32>,
    pub location: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    pub start_at: Option<String>,
    pub due_date: Option<String>,
    pub recurrence_rule: Option<String>,
    pub reminder_offset_minutes: Option<i32>,
    pub reminder_method: Option<String>,
    pub timezone: Option<String>,
}

#[tauri::command]
pub async fn list_todos(state: State<'_, AppState>) -> Result<Vec<Todo>, String> {
    todo::list_todos(state.db())
        .await
        .map_err(|err| err.to_string())
}

#[tauri::command]
pub async fn create_todo(
    state: State<'_, AppState>,
    payload: Option<CreateTodoPayload>,
) -> Result<Todo, String> {
    let title = payload.and_then(|payload| payload.title);

    let result = todo::create_todo(state.db(), title)
        .await
        .map_err(|err| err.to_string())?;

    // 统一的变更通知（会自动触发 reschedule）
    state.notify_todo_change("created", Some(result.id)).await;

    Ok(result)
}

#[tauri::command]
pub async fn update_todo(
    state: State<'_, AppState>,
    payload: UpdateTodoPayload,
) -> Result<Todo, String> {
    let result = todo::update_todo(state.db(), payload.id, payload.title, payload.completed)
        .await
        .map_err(|err| err.to_string())?;

    // 统一的变更通知（会自动触发 reschedule）
    state.notify_todo_change("updated", Some(payload.id)).await;

    Ok(result)
}

#[tauri::command]
pub async fn delete_todo(state: State<'_, AppState>, id: i32) -> Result<(), String> {
    todo::delete_todo(state.db(), id)
        .await
        .map_err(|err| err.to_string())?;

    // 统一的变更通知（会自动触发 reschedule）
    state.notify_todo_change("deleted", Some(id)).await;

    Ok(())
}

#[tauri::command]
pub async fn update_todo_details(
    state: State<'_, AppState>,
    payload: UpdateTodoDetailsPayload,
) -> Result<Todo, String> {
    let result = todo::update_todo_details(
        state.db(),
        payload.id,
        payload.description,
        payload.priority,
        payload.location,
        payload.tags,
        payload.start_at,
        payload.due_date,
        payload.recurrence_rule,
        payload.reminder_offset_minutes,
        payload.reminder_method,
        payload.timezone,
    )
    .await
    .map_err(|err| err.to_string())?;

    state
        .notify_todo_change("updated", Some(payload.id))
        .await;

    Ok(result)
}

#[cfg(not(any(target_os = "android", target_os = "ios")))]
#[tauri::command]
pub async fn start_web_server(state: State<'_, AppState>) -> Result<WebServerStatus, String> {
    let result = state
        .web_server()
        .start(state.db().clone(), state.app_handle(), None)
        .await
        .map_err(|err| err.to_string());
    
    if result.is_ok() {
        // 保存设置
        let _ = SettingService::set_bool(state.db(), "webserver.auto_start", true).await;
        
        // 通知托盘菜单更新
        let _ = state.app_handle().emit(WEBSERVER_STATUS_CHANGED_EVENT, true);
        // 更新托盘菜单
        let _ = super::tray::update_tray_menu_from_app(&state.app_handle(), true);
    }
    
    result
}

#[cfg(not(any(target_os = "android", target_os = "ios")))]
#[tauri::command]
pub async fn stop_web_server(state: State<'_, AppState>) -> Result<WebServerStatus, String> {
    let result = state
        .web_server()
        .stop()
        .await
        .map_err(|err| err.to_string());
    
    if result.is_ok() {
        // 保存设置
        let _ = SettingService::set_bool(state.db(), "webserver.auto_start", false).await;
        
        // 通知托盘菜单更新
        let _ = state.app_handle().emit(WEBSERVER_STATUS_CHANGED_EVENT, false);
        // 更新托盘菜单
        let _ = super::tray::update_tray_menu_from_app(&state.app_handle(), false);
    }
    
    result
}

#[cfg(not(any(target_os = "android", target_os = "ios")))]
#[tauri::command]
pub async fn web_server_status(state: State<'_, AppState>) -> Result<WebServerStatus, String> {
    Ok(state.web_server().status().await)
}
