# Pet Focus 后端架构文档

## 架构概览

Pet Focus 采用 **插件化 Feature 架构**，实现了业务功能与基础设施的清晰分离。

```
src-tauri/src/
├── lib.rs                   # 库入口，导出公共 API
├── core/                    # 核心抽象层
│   ├── feature.rs          # Feature trait 定义
│   └── app.rs              # AppState 管理器
├── infrastructure/          # 基础设施层（不包含业务逻辑）
│   ├── database/           # 数据库注册中心 + SeaORM 集成
│   ├── notification/       # 通知系统（WebSocket/Toast/Native）
│   ├── tray/               # 系统托盘管理（桌面平台）
│   ├── webserver/          # WebSocket API 服务器（桌面平台）
│   └── window.rs           # 窗口管理（显示/隐藏，macOS Dock 控制）
└── features/                # 业务功能层
    ├── todo/               # Todo 功能模块（含 CalDAV 同步）
    ├── settings/           # 设置功能模块
    └── pomodoro/           # 番茄钟功能模块（预留）
```

## 核心设计原则

### 1. Feature Trait

所有业务功能必须实现 `Feature` trait：

```rust
#[async_trait]
pub trait Feature: Send + Sync {
    fn name(&self) -> &str;
    fn command_names(&self) -> Vec<&str>;
    
    async fn register_database(&self, registry: &mut DatabaseRegistry) -> Result<()>;
    async fn register_commands(&self, app: &AppHandle) -> Result<()>;
    async fn register_notifications(&self, registry: &mut NotificationRegistry) -> Result<()>;
    async fn register_tray_items(&self, registry: &mut TrayRegistry) -> Result<()>;
    async fn register_api_routes(&self, router: &mut ApiRouter) -> Result<()>;
    
    async fn initialize(&self, app: &AppHandle) -> Result<()>;
    async fn shutdown(&self) -> Result<()>;
}
```

### 2. Infrastructure Registries

基础设施通过 Registry 模式提供服务：

- **DatabaseRegistry**: 动态注册 Entity 和 Migration
- **NotificationRegistry**: 统一管理通知事件（WebSocket/Toast/Native）
- **TrayRegistry**: 动态注册托盘菜单项
- **ApiRouter**: 动态注册 HTTP API 路由

### 3. 依赖注入

AppState 持有所有 Features，通过 Tauri 的 state 管理注入：

```rust
pub struct AppState {
    db: DatabaseConnection,
    features: Vec<Arc<dyn Feature>>,
}
```

## 如何添加新 Feature

### 步骤 1: 创建 Feature 目录

```bash
mkdir -p src/features/your_feature
cd src/features/your_feature
```

### 步骤 2: 定义数据模型

```rust
// entity.rs - SeaORM Entity 定义
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "your_table")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i32,
    pub name: String,
    // ...
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
```

```rust
// models.rs - API 响应模型
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct YourModel {
    pub id: i32,
    pub name: String,
}
```

### 步骤 3: 实现 Service

```rust
// service.rs
use super::entity::{self, Entity as YourEntity};
use sea_orm::*;

pub struct YourService;

impl YourService {
    pub async fn get_all(db: &DatabaseConnection) -> Result<Vec<entity::Model>, DbErr> {
        YourEntity::find().all(db).await
    }
    
    // 其他业务方法...
}
```

### 步骤 4: 定义 Tauri Commands

```rust
// commands.rs
use tauri::State;
use crate::core::AppState;
use super::models::YourModel;
use super::service::YourService;

#[tauri::command]
pub async fn get_your_items(
    state: State<'_, AppState>,
) -> Result<Vec<YourModel>, String> {
    let db = state.db();
    let items = YourService::get_all(db)
        .await
        .map_err(|e| e.to_string())?;
    
    Ok(items.into_iter().map(YourModel::from).collect())
}
```

### 步骤 5: 实现 Feature

```rust
// feature.rs
use crate::core::Feature;
use crate::infrastructure::database::{DatabaseRegistry, Migration};
use async_trait::async_trait;
use anyhow::Result;
use tauri::AppHandle;

pub struct YourFeature;

#[async_trait]
impl Feature for YourFeature {
    fn name(&self) -> &str {
        "your_feature"
    }
    
    fn command_names(&self) -> Vec<&str> {
        vec!["get_your_items"]
    }
    
    async fn register_database(&self, registry: &mut DatabaseRegistry) -> Result<()> {
        // 注册 Entity
        registry.register_entity(super::entity::Entity);
        
        // 注册 Migration
        registry.register_migration(Migration {
            version: "m20240101_000001_create_your_table",
            up: Box::new(|manager| {
                Box::pin(async move {
                    // 创建表的 SQL
                    manager.create_table(...).await
                })
            }),
            down: Box::new(|manager| {
                Box::pin(async move {
                    manager.drop_table(...).await
                })
            }),
        });
        
        Ok(())
    }
    
    async fn register_commands(&self, _app: &AppHandle) -> Result<()> {
        // Commands 在 lib.rs 的 invoke_handler 中注册
        Ok(())
    }
    
    async fn initialize(&self, _app: &AppHandle) -> Result<()> {
        println!("YourFeature initialized");
        Ok(())
    }
    
    async fn shutdown(&self) -> Result<()> {
        println!("YourFeature shutting down");
        Ok(())
    }
}
```

### 步骤 6: 注册到应用

在 `lib.rs` 中添加：

```rust
// 1. 导入模块
mod features {
    pub mod your_feature;
}

// 2. 在 init_features() 中添加
fn init_features() -> Vec<Arc<dyn Feature>> {
    vec![
        // ...existing features
        Arc::new(features::your_feature::YourFeature),
    ]
}

// 3. 在 invoke_handler 中注册命令
.invoke_handler(tauri::generate_handler![
    // ...existing commands
    features::your_feature::commands::get_your_items,
])
```

## Migration 系统

使用 `DatabaseRegistry` 动态注册 Migration：

```rust
registry.register_migration(Migration {
    version: "m20240101_000001_create_table",
    up: Box::new(|manager| {
        Box::pin(async move {
            manager
                .create_table(
                    Table::create()
                        .table(YourEntity)
                        .col(ColumnDef::new(Column::Id).integer().primary_key().auto_increment())
                        .col(ColumnDef::new(Column::Name).string().not_null())
                        .to_owned()
                )
                .await
        })
    }),
    down: Box::new(|manager| {
        Box::pin(async move {
            manager.drop_table(Table::drop().table(YourEntity).to_owned()).await
        })
    }),
});
```

### 重要技术细节：Higher-Ranked Trait Bounds

Migration 闭包使用了 `for<'a>` 语法来处理生命周期：

```rust
type MigrationFn = Box<
    dyn for<'a> Fn(&'a SchemaManager<'a>) 
        -> Pin<Box<dyn Future<Output = Result<(), DbErr>> + Send + 'a>> 
    + Send + Sync
>;
```

这确保了闭包可以接受任意生命周期的 `SchemaManager`。

## Notification 系统

通过 `NotificationRegistry` 注册通知事件：

```rust
registry.register_event("todo_updated", NotificationType::WebSocket);
registry.register_event("todo_reminder", NotificationType::Toast);
```

发送通知：

```rust
state.notification_registry()
    .send("todo_updated", serde_json::json!({ "id": 1 }))
    .await?;
```

## Tray 系统

动态注册托盘菜单项：

```rust
registry.register_item(TrayItem {
    id: "open_todo",
    label: "打开待办事项",
    icon: None,
    action: TrayAction::EmitEvent("open_todo_window"),
});
```

## WebServer API 路由

桌面端支持 HTTP API（移动端跳过）：

```rust
#[cfg(desktop)]
router.register_route(
    Method::GET,
    "/api/todos",
    Box::new(|req| Box::pin(async move {
        // 处理请求
        Ok(Response::new(Body::from("...")))
    }))
);
```

## 当前状态

### ✅ 已完成
- **Core 架构**：Feature trait + AppState 管理器
- **Infrastructure 完整实现**：
  - Database 注册中心（动态 Entity & Migration）
  - Notification 系统（WebSocket/Toast/Native）
  - Tray 管理器（动态菜单，根据 WebServer 状态更新）
  - WebServer 管理器（WebSocket API，自动启动）
  - Window 管理（显示/隐藏，macOS Dock 图标控制）
- **Todo Feature 完整迁移**：
  - CRUD 操作（创建、读取、更新、删除）
  - CalDAV 同步管理器（启动时自动初始化）
  - 完整的 CalDAV commands（status/config/sync）
- **Settings Feature 完整迁移**：
  - 主题设置（light/dark/system，自动规范化）
  - 使用 ui.theme 作为存储 key
- **Pomodoro 基础结构**（预留扩展）
- **所有旧代码已迁移**：legacy 目录已完全删除

### 🎯 完全运行中
- ✅ CalDAV 同步在应用启动时自动初始化
- ✅ WebServer 根据设置自动启动
- ✅ System Tray 完整功能（显示/隐藏窗口，启动/停止 API，退出）
- ✅ 窗口关闭时隐藏而非退出（后台运行）
- ✅ macOS Dock 图标跟随窗口显示/隐藏
- ✅ 编译成功（0 errors）

## 编译和运行

```bash
# 检查编译
cargo check

# 构建
cargo build

# 运行开发模式
pnpm tauri dev
```

## 技术栈

- **Rust**: 1.83+
- **Tauri**: 2.x
- **SeaORM**: 1.1
- **async-trait**: 0.1.89
- **axum**: 0.8 (桌面端 HTTP 服务器)

## 最佳实践

1. **每个 Feature 独立**: 不要在 Feature 之间直接引用
2. **通过 AppState 通信**: 使用 state.db() 访问数据库
3. **使用 Registry 注册**: 不要在 Infrastructure 中硬编码业务逻辑
4. **异步优先**: 所有 I/O 操作使用 async/await
5. **错误处理**: 使用 Result<T, anyhow::Error> 统一错误类型

## License

MIT
