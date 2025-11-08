use std::collections::HashMap;
use std::sync::Arc;

use sea_orm::DatabaseConnection;
use tauri::{AppHandle, Wry};

use crate::core::Feature;
use crate::features::todo::sync::CalDavSyncManager;
use crate::infrastructure::webserver::WebServerManager;
#[cfg(not(any(target_os = "android", target_os = "ios")))]
use crate::infrastructure::tray::TrayManager;

/// 应用全局状态
/// 
/// 管理所有 Features 和基础设施组件
pub struct AppState {
    app_handle: AppHandle<Wry>,
    db: DatabaseConnection,
    features: HashMap<&'static str, Arc<dyn Feature>>,
    
    // CalDAV 同步管理器
    caldav_sync_manager: CalDavSyncManager,
    
    // WebServer 管理器（桌面平台）
    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    webserver_manager: WebServerManager,
    
    // 系统托盘管理器（桌面平台）
    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    tray_manager: TrayManager,
}

impl AppState {
    pub fn new(
        app_handle: AppHandle<Wry>,
        db: DatabaseConnection,
        features: Vec<Arc<dyn Feature>>,
    ) -> Self {
        let mut feature_map = HashMap::new();
        for feature in features {
            feature_map.insert(feature.name(), feature);
        }

        // 创建 CalDAV 同步管理器
        let caldav_sync_manager = CalDavSyncManager::new(db.clone(), app_handle.clone());

        Self {
            app_handle,
            db,
            features: feature_map,
            caldav_sync_manager,
            #[cfg(not(any(target_os = "android", target_os = "ios")))]
            webserver_manager: WebServerManager::new(),
            #[cfg(not(any(target_os = "android", target_os = "ios")))]
            tray_manager: TrayManager::new(),
        }
    }

    /// 获取数据库连接
    pub fn db(&self) -> &DatabaseConnection {
        &self.db
    }

    /// 获取 AppHandle
    pub fn app_handle(&self) -> AppHandle<Wry> {
        self.app_handle.clone()
    }

    /// 根据名称获取 Feature
    pub fn get_feature(&self, name: &str) -> Option<&Arc<dyn Feature>> {
        self.features.get(name)
    }

    /// 获取所有 Features
    pub fn features(&self) -> &HashMap<&'static str, Arc<dyn Feature>> {
        &self.features
    }

    /// 获取 CalDAV 同步管理器
    pub fn caldav_sync_manager(&self) -> &CalDavSyncManager {
        &self.caldav_sync_manager
    }

    /// 获取 WebServer 管理器（仅桌面平台）
    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    pub fn webserver_manager(&self) -> &WebServerManager {
        &self.webserver_manager
    }

    /// 获取 Tray 管理器（仅桌面平台）
    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    pub fn tray_manager(&self) -> &TrayManager {
        &self.tray_manager
    }
}
