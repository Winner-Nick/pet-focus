use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// Session 和 Record 的关联表
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "pomodoro_session_records")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub session_id: i32,
    pub record_id: i32,
    /// 记录在 session 中的顺序
    pub sequence: i32,
    pub created_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::pomodoro_sessions::Entity",
        from = "Column::SessionId",
        to = "super::pomodoro_sessions::Column::Id"
    )]
    Session,
    #[sea_orm(
        belongs_to = "super::pomodoro_records::Entity",
        from = "Column::RecordId",
        to = "super::pomodoro_records::Column::Id"
    )]
    Record,
}

impl Related<super::pomodoro_sessions::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Session.def()
    }
}

impl Related<super::pomodoro_records::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Record.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
