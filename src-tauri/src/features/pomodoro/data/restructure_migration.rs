use anyhow::Context;
use chrono::Utc;
use sea_orm::{ConnectionTrait, DatabaseBackend, DbBackend, Statement};
use sea_orm_migration::prelude::*;

/// 重构 Pomodoro 数据库结构
/// 1. 重命名 pomodoro_sessions -> pomodoro_records
/// 2. 创建新的 pomodoro_sessions 表（会话组）
/// 3. 创建关联表 pomodoro_session_records
/// 4. 迁移现有数据（按日分组）
#[derive(Debug, Clone, Copy)]
pub struct PomodoroRestructureMigration;

impl MigrationName for PomodoroRestructureMigration {
    fn name(&self) -> &str {
        "m20250115_000001_pomodoro_restructure"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for PomodoroRestructureMigration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        let backend = db.get_database_backend();

        // 步骤 1: 重命名表 pomodoro_sessions -> pomodoro_records（如果还没重命名）
        println!("Step 1: Renaming pomodoro_sessions to pomodoro_records (if needed)...");
        match backend {
            DbBackend::Sqlite => {
                // 检查 pomodoro_records 是否已存在
                let check_result = db.query_one(Statement::from_string(
                    DatabaseBackend::Sqlite,
                    "SELECT name FROM sqlite_master WHERE type='table' AND name='pomodoro_records';".to_string(),
                ))
                .await;
                
                // 如果 pomodoro_records 不存在，才进行重命名
                if check_result.is_err() || check_result.unwrap().is_none() {
                    println!("  -> Renaming table...");
                    db.execute(Statement::from_string(
                        DatabaseBackend::Sqlite,
                        "ALTER TABLE pomodoro_sessions RENAME TO pomodoro_records;".to_string(),
                    ))
                    .await
                    .context("failed to rename table")
                    .map_err(|e| DbErr::Custom(e.to_string()))?;
                } else {
                    println!("  -> Table already renamed, skipping...");
                }
            }
            _ => {
                return Err(DbErr::Custom("Unsupported database backend".to_string()));
            }
        }

        // 步骤 2: 创建新的 pomodoro_sessions 表（会话组）
        println!("Step 2: Creating new pomodoro_sessions table...");
        manager
            .create_table(
                Table::create()
                    .table(PomodoroSessions::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PomodoroSessions::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(PomodoroSessions::Note).text())
                    .col(
                        ColumnDef::new(PomodoroSessions::Archived)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(ColumnDef::new(PomodoroSessions::ArchivedAt).timestamp_with_time_zone())
                    .col(
                        ColumnDef::new(PomodoroSessions::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PomodoroSessions::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        // 步骤 3: 创建关联表 pomodoro_session_records
        println!("Step 3: Creating pomodoro_session_records table...");
        manager
            .create_table(
                Table::create()
                    .table(PomodoroSessionRecords::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PomodoroSessionRecords::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(PomodoroSessionRecords::SessionId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PomodoroSessionRecords::RecordId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PomodoroSessionRecords::Sequence)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PomodoroSessionRecords::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_session_records_session")
                            .from(PomodoroSessionRecords::Table, PomodoroSessionRecords::SessionId)
                            .to(PomodoroSessions::Table, PomodoroSessions::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_session_records_record")
                            .from(PomodoroSessionRecords::Table, PomodoroSessionRecords::RecordId)
                            .to(PomodoroRecords::Table, PomodoroRecords::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // 步骤 4: 迁移现有数据（按日分组）
        println!("Step 4: Migrating existing data...");
        migrate_existing_data(db).await?;

        println!("Migration completed successfully!");
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 回滚操作：删除新表，重命名回原表名
        manager
            .drop_table(Table::drop().table(PomodoroSessionRecords::Table).to_owned())
            .await?;
        
        manager
            .drop_table(Table::drop().table(PomodoroSessions::Table).to_owned())
            .await?;

        let db = manager.get_connection();
        db.execute(Statement::from_string(
            DatabaseBackend::Sqlite,
            "ALTER TABLE pomodoro_records RENAME TO pomodoro_sessions;".to_string(),
        ))
        .await
        .context("failed to rename table back")
        .map_err(|e| DbErr::Custom(e.to_string()))?;

        Ok(())
    }
}

/// 迁移现有数据：按日分组创建 sessions
async fn migrate_existing_data(db: &SchemaManagerConnection<'_>) -> Result<(), DbErr> {
    // 检查是否已经有关联数据（迁移已执行）
    let existing_links = db
        .query_one(Statement::from_string(
            DatabaseBackend::Sqlite,
            "SELECT COUNT(*) as count FROM pomodoro_session_records".to_string(),
        ))
        .await?;
    
    if let Some(row) = existing_links {
        let count: i64 = row.try_get("", "count")?;
        if count > 0 {
            println!("  -> Data already migrated (found {} links), skipping...", count);
            return Ok(());
        }
    }
    
    println!("  -> Migrating data...");
    
    // 查询所有现有记录按日期分组
    let records_by_date = db
        .query_all(Statement::from_string(
            DatabaseBackend::Sqlite,
            r#"
            SELECT 
                date(start_at, 'localtime') as record_date,
                GROUP_CONCAT(id) as record_ids,
                MIN(start_at) as first_start,
                MAX(end_at) as last_end
            FROM pomodoro_records
            GROUP BY date(start_at, 'localtime')
            ORDER BY record_date
            "#.to_string(),
        ))
        .await?;

    let now = Utc::now().to_rfc3339();
    
    for row in records_by_date {
        let record_date: String = row.try_get("", "record_date")?;
        let record_ids_str: String = row.try_get("", "record_ids")?;
        let record_ids: Vec<i32> = record_ids_str
            .split(',')
            .filter_map(|s| s.parse().ok())
            .collect();

        if record_ids.is_empty() {
            continue;
        }

        // 为每一天创建一个 session
        let insert_session = format!(
            r#"INSERT INTO pomodoro_sessions (note, archived, archived_at, created_at, updated_at)
               VALUES (NULL, 0, NULL, '{}', '{}')"#,
            now, now
        );
        
        db.execute(Statement::from_string(
            DatabaseBackend::Sqlite,
            insert_session,
        ))
        .await?;

        // 获取刚创建的 session_id
        let session_id_result = db
            .query_one(Statement::from_string(
                DatabaseBackend::Sqlite,
                "SELECT last_insert_rowid() as id".to_string(),
            ))
            .await?;

        if let Some(row) = session_id_result {
            let session_id: i32 = row.try_get("", "id")?;

            // 为每个 record 创建关联
            for (seq, record_id) in record_ids.iter().enumerate() {
                let insert_link = format!(
                    r#"INSERT INTO pomodoro_session_records (session_id, record_id, sequence, created_at)
                       VALUES ({}, {}, {}, '{}')"#,
                    session_id, record_id, seq, now
                );
                
                db.execute(Statement::from_string(
                    DatabaseBackend::Sqlite,
                    insert_link,
                ))
                .await?;
            }

            println!("  Created session {} for date {} with {} records", 
                     session_id, record_date, record_ids.len());
        }
    }

    Ok(())
}

#[derive(Iden)]
enum PomodoroSessions {
    Table,
    Id,
    Note,
    Archived,
    ArchivedAt,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum PomodoroRecords {
    Table,
    Id,
}

#[derive(Iden)]
enum PomodoroSessionRecords {
    Table,
    Id,
    SessionId,
    RecordId,
    Sequence,
    CreatedAt,
}
