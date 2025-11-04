use std::borrow::Cow;

use anyhow::{anyhow, Context, Result};
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ActiveValue::{NotSet, Set}, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder};

use super::super::{
    entities::todo,
    models::todo::Todo,
};

pub async fn list_todos(db: &DatabaseConnection) -> Result<Vec<Todo>> {
    let todos = todo::Entity::find()
        .order_by_asc(todo::Column::Completed)
        .order_by_desc(todo::Column::UpdatedAt)
        .all(db)
        .await
        .context("failed to query todos")?;

    Ok(todos.into_iter().map(Into::into).collect())
}

pub async fn create_todo(db: &DatabaseConnection, title: Option<String>) -> Result<Todo> {
    let now = Utc::now();
    let normalized_title = normalize_title(title);

    let model = todo::ActiveModel {
        id: NotSet,
        title: Set(normalized_title.into_owned()),
        completed: Set(false),
        created_date: Set(now),
        modified_date: Set(now),
        due_date: Set(None),
        remind_before_minutes: Set(15),
        notified: Set(false),
        created_at: Set(now),
        updated_at: Set(now),
    }
    .insert(db)
    .await
    .context("failed to insert todo")?;

    Ok(model.into())
}

pub async fn get_todo(db: &DatabaseConnection, id: i32) -> Result<Todo> {
    todo::Entity::find_by_id(id)
        .one(db)
        .await
        .with_context(|| format!("failed to load todo {id}"))?
        .map(Into::into)
        .ok_or_else(|| anyhow!("todo {id} not found"))
}

pub async fn update_todo(
    db: &DatabaseConnection,
    id: i32,
    title: Option<String>,
    completed: Option<bool>,
) -> Result<Todo> {
    let model = todo::Entity::find_by_id(id)
        .one(db)
        .await
        .with_context(|| format!("failed to load todo {id}"))?
        .ok_or_else(|| anyhow!("todo {id} not found"))?;

    let mut active: todo::ActiveModel = model.into();

    if let Some(title) = title {
        let normalized = normalize_title(Some(title));
        active.title = Set(normalized.into_owned());
    }

    if let Some(completed) = completed {
        active.completed = Set(completed);
    }

    active.modified_date = Set(Utc::now());
    active.updated_at = Set(Utc::now());

    let updated = active
        .update(db)
        .await
        .with_context(|| format!("failed to update todo {id}"))?;

    Ok(updated.into())
}

pub async fn update_todo_due_date(
    db: &DatabaseConnection,
    id: i32,
    due_date: Option<String>,
) -> Result<Todo> {
    let model = todo::Entity::find_by_id(id)
        .one(db)
        .await
        .with_context(|| format!("failed to load todo {id}"))?
        .ok_or_else(|| anyhow!("todo {id} not found"))?;

    let mut active: todo::ActiveModel = model.into();

    // 解析 ISO 8601 日期字符串
    let parsed_due_date = if let Some(date_str) = due_date {
        Some(
            chrono::DateTime::parse_from_rfc3339(&date_str)
                .with_context(|| format!("failed to parse due date: {}", date_str))?
                .with_timezone(&Utc),
        )
    } else {
        None
    };

    active.due_date = Set(parsed_due_date);
    active.notified = Set(false); // 重置通知状态
    active.modified_date = Set(Utc::now());
    active.updated_at = Set(Utc::now());

    let updated = active
        .update(db)
        .await
        .with_context(|| format!("failed to update todo due date {id}"))?;

    Ok(updated.into())
}

pub async fn update_todo_remind_before(
    db: &DatabaseConnection,
    id: i32,
    remind_before_minutes: i32,
) -> Result<Todo> {
    let model = todo::Entity::find_by_id(id)
        .one(db)
        .await
        .with_context(|| format!("failed to load todo {id}"))?
        .ok_or_else(|| anyhow!("todo {id} not found"))?;

    let mut active: todo::ActiveModel = model.into();

    active.remind_before_minutes = Set(remind_before_minutes);
    active.modified_date = Set(Utc::now());
    active.updated_at = Set(Utc::now());

    let updated = active
        .update(db)
        .await
        .with_context(|| format!("failed to update todo remind before {id}"))?;

    Ok(updated.into())
}

pub async fn delete_todo(db: &DatabaseConnection, id: i32) -> Result<()> {
    let result = todo::Entity::delete_by_id(id)
        .exec(db)
        .await
        .with_context(|| format!("failed to delete todo {id}"))?;

    if result.rows_affected == 0 {
        return Err(anyhow!("todo {id} not found"));
    }

    Ok(())
}

pub async fn get_next_due_todo(db: &DatabaseConnection) -> Result<Option<todo::Model>> {
    let now = Utc::now();
    
    let todo = todo::Entity::find()
        .filter(todo::Column::Completed.eq(false))
        .filter(todo::Column::DueDate.is_not_null())
        .filter(todo::Column::DueDate.gt(now))
        .filter(todo::Column::Notified.eq(false)) // 只选择未通知的 todo
        .order_by_asc(todo::Column::DueDate)
        .one(db)
        .await
        .context("failed to query next due todo")?;

    Ok(todo)
}

/// 标记 todo 为已通知
pub async fn mark_todo_notified(db: &DatabaseConnection, id: i32) -> Result<()> {
    let model = todo::Entity::find_by_id(id)
        .one(db)
        .await
        .with_context(|| format!("failed to load todo {id}"))?
        .ok_or_else(|| anyhow!("todo {id} not found"))?;

    let mut active: todo::ActiveModel = model.into();
    active.notified = Set(true);

    active
        .update(db)
        .await
        .with_context(|| format!("failed to mark todo {id} as notified"))?;

    Ok(())
}

fn normalize_title(title: Option<String>) -> Cow<'static, str> {
    let Some(title) = title else {
        return Cow::Borrowed("New Todo");
    };

    let trimmed = title.trim();
    if trimmed.is_empty() {
        Cow::Borrowed("New Todo")
    } else {
        Cow::Owned(trimmed.to_string())
    }
}
