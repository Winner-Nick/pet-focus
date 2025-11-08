use anyhow::Result;
use async_trait::async_trait;

use crate::core::AppState;
use crate::infrastructure::database::DatabaseRegistry;
#[cfg(not(any(target_os = "android", target_os = "ios")))]
use crate::infrastructure::tray::TrayRegistry;
#[cfg(not(any(target_os = "android", target_os = "ios")))]
use crate::infrastructure::webserver::ApiRegistry;

/// Feature trait - 所有业务功能模块必须实现此 trait
/// 
/// 每个 Feature 负责：
/// 1. 向基础设施注册自己的组件（数据库实体、API 路由、托盘菜单等）
/// 2. 提供自己的 Tauri Commands
/// 3. 实现初始化逻辑
#[async_trait]
pub trait Feature: Send + Sync {
    /// Feature 名称（用于日志和调试）
    fn name(&self) -> &'static str;

    /// 注册数据库 Migrations
    /// 
    /// 使用 registry.register_migration() 注册迁移函数
    fn register_database(&self, _registry: &mut DatabaseRegistry) {
        // 默认实现：不注册任何数据库组件
    }

    /// 返回此 Feature 的所有 Tauri Command 名称
    /// 
    /// 这些名称用于文档和调试，实际 Commands 需要在 lib.rs 中手动注册
    fn command_names(&self) -> Vec<&'static str> {
        vec![]
    }

    /// 注册 WebServer API 路由（仅桌面平台）
    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    fn register_api_routes(&self, _registry: &mut ApiRegistry) {
        // 默认实现：不注册任何 API 路由
    }

    /// 注册托盘菜单项（仅桌面平台）
    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    fn register_tray_items(&self, _registry: &mut TrayRegistry) {
        // 默认实现：不注册任何托盘菜单项
    }

    /// Feature 初始化（在数据库迁移完成后调用）
    /// 
    /// 可以在此处启动后台任务、订阅事件等
    async fn initialize(&self, _app_state: &AppState) -> Result<()> {
        // 默认实现：什么都不做
        Ok(())
    }

    /// Feature 清理（应用关闭时调用）
    async fn cleanup(&self) -> Result<()> {
        // 默认实现：什么都不做
        Ok(())
    }
}
