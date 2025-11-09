use std::fs;

use anyhow::{Context, Result};
use directories::ProjectDirs;
use sea_orm::{ConnectionTrait, Database, DatabaseConnection, DbBackend, Statement};
use tauri::AppHandle;

const DB_FILENAME: &str = "pet_focus.sqlite";
const QUALIFIER: &str = "site";
const ORGANIZATION: &str = "yiges";
const APPLICATION: &str = "pet-focus";

/// 初始化数据库连接
/// 
/// 只负责创建连接和启用外键，不执行任何 Migration
/// Migration 由各个 Feature 通过 DatabaseRegistry 统一管理
pub async fn init_db(_app_handle: &AppHandle) -> Result<DatabaseConnection> {
    let project_dirs = ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION)
        .context("failed to resolve application data directory")?;
    let app_dir = project_dirs.data_dir();

    fs::create_dir_all(app_dir).context("failed to create application data directory")?;

    let db_path = app_dir.join(DB_FILENAME);
    let path = db_path.to_string_lossy().replace('\u{5c}', "/");
    let connection_url = format!("sqlite://{}?mode=rwc", path);

    let db = Database::connect(&connection_url)
        .await
        .with_context(|| format!("failed to connect to database at {}", connection_url))?;

    enable_foreign_keys(&db).await?;

    Ok(db)
}

async fn enable_foreign_keys(db: &DatabaseConnection) -> Result<()> {
    db.execute(Statement::from_string(
        DbBackend::Sqlite,
        "PRAGMA foreign_keys = ON".to_owned(),
    ))
    .await
    .context("failed to enable foreign key support")?;

    Ok(())
}
