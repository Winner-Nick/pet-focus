use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// 专注记录（原 pomodoro_sessions）
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "pomodoro_records")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    /// focus | rest
    pub kind: String,
    /// completed | stopped | skipped
    pub status: String,
    pub round: i32,
    pub start_at: DateTimeUtc,
    pub end_at: DateTimeUtc,
    /// seconds
    pub elapsed_seconds: i32,
    pub related_todo_id: Option<i32>,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::pomodoro_session_records::Entity")]
    SessionRecords,
}

impl Related<super::pomodoro_session_records::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SessionRecords.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
