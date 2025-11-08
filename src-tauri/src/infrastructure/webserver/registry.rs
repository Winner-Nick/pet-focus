use std::sync::Arc;

use axum::{routing::MethodRouter, Router};

/// API 路由处理器类型
pub type ApiHandler = Arc<dyn Fn() -> MethodRouter + Send + Sync>;

/// API 注册表
/// 
/// 用于收集所有 Features 注册的 HTTP API 路由
pub struct ApiRegistry {
    routes: Vec<(String, ApiHandler)>,
}

impl Clone for ApiRegistry {
    fn clone(&self) -> Self {
        Self {
            routes: self.routes.clone(),
        }
    }
}

impl ApiRegistry {
    pub fn new() -> Self {
        Self { routes: Vec::new() }
    }

    /// 注册一个 API 路由
    /// 
    /// # 示例
    /// ```ignore
    /// registry.register_route("/todos", || {
    ///     axum::routing::get(list_todos).post(create_todo)
    /// });
    /// ```
    pub fn register_route<F>(&mut self, path: impl Into<String>, handler: F)
    where
        F: Fn() -> MethodRouter + Send + Sync + 'static,
    {
        self.routes.push((path.into(), Arc::new(handler)));
    }

    /// 构建 Axum Router
    pub fn build_router(&self) -> Router {
        let mut router = Router::new();
        
        for (path, handler) in &self.routes {
            router = router.route(path, handler());
        }
        
        router
    }
}

impl Default for ApiRegistry {
    fn default() -> Self {
        Self::new()
    }
}
