use axum::http::StatusCode;
use sqlx::PgPool;
use crate::models::task::{SortBy, SortOrder, TaskFilterQuery};
use crate::repositories::task_repository::get_tasks_with_filter;
use crate::{services::user_service::validate_user};
use crate::{models::task::{Task, TaskAdd, TaskUpdate}, repositories::{task_repository::{self, delete_task_by_id, get_task_by_id, get_tasks, save_task}}};



pub async fn add_task(
    pool: &PgPool,
    user_id: i32,
    payload: &TaskAdd
) -> Result<Task, (StatusCode, String)>{

    validate_user(pool, user_id, None).await?;

    return save_task(pool, user_id, payload).await;
}

pub async fn get_my_task(pool: &PgPool, user_id: i32) -> Result<Vec<Task>, (StatusCode, String)>{
    validate_user(pool, user_id, None).await?;

    return get_tasks(pool, user_id).await;
}

pub async fn delete_my_task(pool: &PgPool, user_id: i32, task_id: i32) -> Result<String, (StatusCode, String)>{
    validate_user(pool, user_id, None).await?;

    let task = get_task_by_id(pool, task_id).await?;

    if task.user_id != user_id{
        tracing::error!("You({}) do not have permission to delete this task.", user_id);
        return Err((StatusCode::UNAUTHORIZED, "You do not have permission to delete this task.".to_string()));
    }

    return delete_task_by_id(pool, task_id).await;
}

pub async fn update_task(pool: &PgPool, user_id: i32, task_id: i32, body: TaskUpdate) 
-> Result<Task, (StatusCode, String)>
{
    validate_user(pool, user_id, None).await?;

    let task = get_task_by_id(pool, task_id).await?;

    if task.user_id != user_id{
        tracing::error!("You({}) do not have permission to update this task.", user_id);
        return Err((StatusCode::UNAUTHORIZED, "You do not have permission to update this task.".to_string()));
    }
    
    task_repository::update_task(pool, task_id, body).await?;

    return get_task_by_id(pool, task_id).await;
}

pub async fn delete_completed_tasks(pool: &PgPool, user_id: i32)
-> Result<String, (StatusCode, String)>{
    validate_user(pool, user_id, None).await?;

    return task_repository::delete_completed_tasks(pool, user_id).await;
}

pub async fn get_tasks_filter(
    pool: &PgPool,
    user_id: i32,
    query: TaskFilterQuery
) -> Result<Vec<Task>, (StatusCode, String)>{
    validate_user(pool, user_id, None).await?;

    let sort_by = query.sort_by.unwrap_or(SortBy::CreatedAt);
    let order = query.order.unwrap_or(SortOrder::Asc);

    return get_tasks_with_filter(pool, user_id, sort_by, order).await;
}