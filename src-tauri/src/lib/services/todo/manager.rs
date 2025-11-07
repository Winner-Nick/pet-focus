use std::borrow::Cow;

use anyhow::{anyhow, Context, Result};
use chrono::{DateTime, Utc};
use sea_orm::{ActiveModelTrait, ActiveValue::{NotSet, Set}, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder};
use uuid::Uuid;

use crate::lib::{
    entities::todo,
    models::todo::Todo,
};

const DEFAULT_STATUS: &str = "NEEDS-ACTION";
const COMPLETED_STATUS: &str = "COMPLETED";
const DEFAULT_REMINDER_MINUTES: i32 = 15;

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
        uid: Set(Uuid::new_v4().to_string()),
        title: Set(normalized_title.into_owned()),
        description: Set(None),
        completed: Set(false),
        status: Set(DEFAULT_STATUS.to_string()),
    percent_complete: Set(Some(0)),
        priority: Set(None),
        location: Set(None),
        tags: Set(None),
        start_at: Set(now),
        last_modified_at: Set(now),
        due_date: Set(None),
        recurrence_rule: Set(None),
        reminder_offset_minutes: Set(DEFAULT_REMINDER_MINUTES),
        timezone: Set(None),
        reminder_method: Set(Some("display".to_string())),
        reminder_last_triggered_at: Set(None),
        completed_at: Set(None),
        notified: Set(false),
        dirty: Set(true),
        remote_url: Set(None),
        remote_etag: Set(None),
        remote_calendar_url: Set(None),
        sync_token: Set(None),
        last_synced_at: Set(None),
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
    let now = Utc::now();

    if let Some(title) = title {
        let normalized = normalize_title(Some(title));
        active.title = Set(normalized.into_owned());
    }

    if let Some(completed) = completed {
        active.completed = Set(completed);
        if completed {
            active.status = Set(COMPLETED_STATUS.to_string());
            active.percent_complete = Set(Some(100));
            active.completed_at = Set(Some(now));
        } else {
            active.status = Set(DEFAULT_STATUS.to_string());
            active.percent_complete = Set(Some(0));
            active.completed_at = Set(None);
        }
    }

    active.last_modified_at = Set(now);
    active.updated_at = Set(now);
    active.dirty = Set(true);

    let updated = active
        .update(db)
        .await
        .with_context(|| format!("failed to update todo {id}"))?;

    Ok(updated.into())
}

#[allow(clippy::too_many_arguments)]
pub async fn update_todo_details(
    db: &DatabaseConnection,
    id: i32,
    description: Option<String>,
    priority: Option<i32>,
    location: Option<String>,
    tags: Vec<String>,
    start_at: Option<String>,
    due_date: Option<String>,
    recurrence_rule: Option<String>,
    reminder_offset_minutes: Option<i32>,
    reminder_method: Option<String>,
    timezone: Option<String>,
) -> Result<Todo> {
    let model = todo::Entity::find_by_id(id)
        .one(db)
        .await
        .with_context(|| format!("failed to load todo {id}"))?
        .ok_or_else(|| anyhow!("todo {id} not found"))?;

    let previous_due_date = model.due_date;
    let previous_reminder_offset = model.reminder_offset_minutes;

    let mut active: todo::ActiveModel = model.into();
    let now = Utc::now();

    active.description = Set(description);
    active.priority = Set(priority);
    active.location = Set(location);

    if tags.is_empty() {
        active.tags = Set(None);
    } else {
        let serialized = serde_json::to_string(&tags).context("failed to serialize tags")?;
        active.tags = Set(Some(serialized));
    }

    if let Some(value) = start_at {
        let parsed = parse_datetime(&value)?;
        active.start_at = Set(parsed);
    }

    let parsed_due = parse_datetime_opt(due_date)?;
    if parsed_due != previous_due_date {
        active.due_date = Set(parsed_due);
        active.notified = Set(false);
    }

    active.recurrence_rule = Set(recurrence_rule);

    if let Some(minutes) = reminder_offset_minutes {
        if minutes != previous_reminder_offset {
            active.reminder_offset_minutes = Set(minutes);
            active.notified = Set(false);
        }
    }

    active.reminder_method = Set(reminder_method);
    active.timezone = Set(timezone);

    active.last_modified_at = Set(now);
    active.updated_at = Set(now);
    active.dirty = Set(true);

    let updated = active
        .update(db)
        .await
        .with_context(|| format!("failed to update todo details {id}"))?;

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
        .filter(todo::Column::Notified.eq(false))
        .order_by_asc(todo::Column::DueDate)
        .one(db)
        .await
        .context("failed to query next due todo")?;

    Ok(todo)
}

pub async fn mark_todo_notified(db: &DatabaseConnection, id: i32) -> Result<()> {
    let model = todo::Entity::find_by_id(id)
        .one(db)
        .await
        .with_context(|| format!("failed to load todo {id}"))?
        .ok_or_else(|| anyhow!("todo {id} not found"))?;

    let mut active: todo::ActiveModel = model.into();
    active.notified = Set(true);
    active.reminder_last_triggered_at = Set(Some(Utc::now()));

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

fn parse_datetime(value: &str) -> Result<DateTime<Utc>> {
    chrono::DateTime::parse_from_rfc3339(value)
        .map(|dt| dt.with_timezone(&Utc))
        .with_context(|| format!("failed to parse datetime: {value}"))
}

fn parse_datetime_opt(value: Option<String>) -> Result<Option<DateTime<Utc>>> {
    match value {
        Some(v) if !v.trim().is_empty() => parse_datetime(&v).map(Some),
        _ => Ok(None),
    }
}
