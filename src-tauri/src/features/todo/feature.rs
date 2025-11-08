use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use sea_orm_migration::MigrationTrait;

use crate::core::{AppState, Feature};
use crate::infrastructure::database::DatabaseRegistry;

use super::migration;

/// Todo Feature
/// 
/// 负责 Todo 功能的所有逻辑，包括：
/// - 数据库 Entity 和 Migration
/// - CRUD Commands
/// - CalDAV 同步
pub struct TodoFeature;

impl TodoFeature {
    pub fn new() -> Arc<Self> {
        Arc::new(Self)
    }
}

#[async_trait]
impl Feature for TodoFeature {
    fn name(&self) -> &'static str {
        "todo"
    }

    fn register_database(&self, registry: &mut DatabaseRegistry) {
        // 注册 Todo 数据表迁移
        registry.register_migration("todo_migration", |manager| {
            let migration = migration::TodoMigration;
            Box::pin(async move {
                migration.up(manager).await
            })
        });
    }

    fn command_names(&self) -> Vec<&'static str> {
        vec![
            "list_todos",
            "create_todo",
            "update_todo",
            "delete_todo",
            "update_todo_details",
            "get_caldav_status",
            "save_caldav_config",
            "clear_caldav_config",
            "sync_caldav_now",
        ]
    }

    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    fn register_api_routes(&self, _registry: &mut crate::infrastructure::webserver::ApiRegistry) {
        // TODO: 注册 WebServer API 路由
    }

    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    fn register_tray_items(&self, _registry: &mut crate::infrastructure::tray::TrayRegistry) {
        // TODO: 注册托盘菜单项（如"新建待办"）
    }

    async fn initialize(&self, _app_state: &AppState) -> Result<()> {
        // TODO: 启动 CalDAV 同步管理器
        println!("[TodoFeature] Initialized");
        Ok(())
    }

    async fn cleanup(&self) -> Result<()> {
        println!("[TodoFeature] Cleaned up");
        Ok(())
    }
}

impl Default for TodoFeature {
    fn default() -> Self {
        Self
    }
}
