use tauri::{
    menu::{Menu, MenuItem, PredefinedMenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Emitter, Manager, Wry,
};

use crate::AppState;
use crate::features::settings::core::service::SettingService;
use crate::features::window::manager as window;

const WEBSERVER_STATUS_CHANGED_EVENT: &str = "webserver-status-changed";

/// 托盘管理器
pub struct TrayManager;

impl TrayManager {
    pub fn new() -> Self {
        Self
    }

    /// 创建系统托盘
    pub fn create_tray(&self, app: &AppHandle<Wry>) -> tauri::Result<()> {
        // 从数据库读取初始状态
        let app_handle = app.clone();
        let initial_server_running = tauri::async_runtime::block_on(async move {
            if let Some(state) = app_handle.try_state::<AppState>() {
                match SettingService::get_bool(state.db(), "webserver.auto_start", false).await {
                    Ok(auto_start) => auto_start,
                    Err(e) => {
                        eprintln!("Failed to read auto-start setting for tray: {}", e);
                        false
                    }
                }
            } else {
                false
            }
        });

        // 创建初始菜单
        let menu = Self::build_tray_menu(app, initial_server_running)?;

        let _tray = TrayIconBuilder::with_id("main")
            .icon(app.default_window_icon().unwrap().clone())
            .menu(&menu)
            .show_menu_on_left_click(false) // 左键不显示菜单
            .on_tray_icon_event(|tray, event| {
                if let TrayIconEvent::Click {
                    button: MouseButton::Left,
                    button_state: MouseButtonState::Up,
                    ..
                } = event
                {
                    let app = tray.app_handle();
                    let _ = window::show_main_window(&app);
                }
            })
            .on_menu_event(move |app, event| {
                match event.id.as_ref() {
                    "toggle" => {
                        let _ = window::toggle_main_window(&app);
                    }
                    "start_server" => {
                        let app_handle = app.clone();
                        tauri::async_runtime::spawn(async move {
                            if let Some(state) = app_handle.try_state::<AppState>() {
                                match state
                                    .webserver_manager()
                                    .start(state.db().clone(), app_handle.clone(), None)
                                    .await
                                {
                                    Ok(_) => {
                                        println!("Web server started successfully");
                                        // 保存设置
                                        let _ = SettingService::set_bool(
                                            state.db(),
                                            "webserver.auto_start",
                                            true,
                                        )
                                        .await;
                                        // 更新托盘菜单
                                        let _ = Self::update_tray_menu(&app_handle, true);
                                        // 通知前端状态变化（直接使用 Tauri Event）
                                        let _ = app_handle.emit(WEBSERVER_STATUS_CHANGED_EVENT, true);
                                    }
                                    Err(e) => {
                                        eprintln!("Failed to start web server: {}", e);
                                    }
                                }
                            }
                        });
                    }
                    "stop_server" => {
                        let app_handle = app.clone();
                        tauri::async_runtime::spawn(async move {
                            if let Some(state) = app_handle.try_state::<AppState>() {
                                match state.webserver_manager().stop().await {
                                    Ok(_) => {
                                        println!("Web server stopped successfully");
                                        // 保存设置
                                        let _ = SettingService::set_bool(
                                            state.db(),
                                            "webserver.auto_start",
                                            false,
                                        )
                                        .await;
                                        // 更新托盘菜单
                                        let _ = Self::update_tray_menu(&app_handle, false);
                                        // 通知前端状态变化（直接使用 Tauri Event）
                                        let _ = app_handle.emit(WEBSERVER_STATUS_CHANGED_EVENT, false);
                                    }
                                    Err(e) => {
                                        eprintln!("Failed to stop web server: {}", e);
                                    }
                                }
                            }
                        });
                    }
                    "quit" => {
                        // 先停止 web server
                        let app_handle = app.clone();
                        tauri::async_runtime::spawn(async move {
                            if let Some(state) = app_handle.try_state::<AppState>() {
                                let _ = state.webserver_manager().stop().await;
                            }
                            // 退出应用
                            app_handle.exit(0);
                        });
                    }
                    _ => {}
                }
            })
            .build(app)?;

        Ok(())
    }

    /// 更新托盘菜单
    pub fn update_menu(app: &AppHandle<Wry>, server_running: bool) -> tauri::Result<()> {
        Self::update_tray_menu(app, server_running)
    }

    /// 根据服务器状态构建托盘菜单
    fn build_tray_menu(app: &AppHandle<Wry>, server_running: bool) -> tauri::Result<Menu<Wry>> {
        let toggle_i = MenuItem::with_id(app, "toggle", "显示/隐藏窗口", true, None::<&str>)?;
        let separator_1 = PredefinedMenuItem::separator(app)?;
        let separator_2 = PredefinedMenuItem::separator(app)?;
        let quit_i = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;

        let menu = if server_running {
            // 服务器运行中，只显示停止按钮
            let stop_server_i =
                MenuItem::with_id(app, "stop_server", "停止 WebSocket API", true, None::<&str>)?;
            Menu::with_items(
                app,
                &[
                    &toggle_i,
                    &separator_1,
                    &stop_server_i,
                    &separator_2,
                    &quit_i,
                ],
            )?
        } else {
            // 服务器未运行，只显示启动按钮
            let start_server_i = MenuItem::with_id(
                app,
                "start_server",
                "启动 WebSocket API",
                true,
                None::<&str>,
            )?;
            Menu::with_items(
                app,
                &[
                    &toggle_i,
                    &separator_1,
                    &start_server_i,
                    &separator_2,
                    &quit_i,
                ],
            )?
        };

        Ok(menu)
    }

    /// 更新托盘菜单（内部实现）
    fn update_tray_menu(app: &AppHandle<Wry>, server_running: bool) -> tauri::Result<()> {
        if let Some(tray) = app.tray_by_id("main") {
            let new_menu = Self::build_tray_menu(app, server_running)?;
            tray.set_menu(Some(new_menu))?;
        }
        Ok(())
    }
}

impl Default for TrayManager {
    fn default() -> Self {
        Self::new()
    }
}
