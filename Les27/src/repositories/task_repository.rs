use axum::http::StatusCode;
use chrono::NaiveDate;
use sqlx::{PgPool, Postgres, QueryBuilder, query, query_as};

use crate::models::task::{SortBy, SortOrder, Task, TaskAdd, TaskStatus, TaskUpdate};

fn parse_start_end_date(start_date: Option<&str>, end_date: Option<&str>) 
    -> Result<(Option<NaiveDate>, Option<NaiveDate>), String> {
    let s = start_date.as_deref()
        .map(|s| NaiveDate::parse_from_str(s, "%Y-%m-%d"))
        .transpose()
        .map_err(|e| {
            tracing::error!("Something went wrong while parsing start_date: {}", e);
            format!("Something went wrong while parsing start_date")
        })?;

    let e= end_date.as_deref()
        .map(|e| NaiveDate::parse_from_str(e, "%Y-%m-%d"))
        .transpose()
        .map_err(|e| {
            tracing::error!("Something went wrong while parsing end_date: {}", e);
            format!("Something went wrong while parsing end_date")
        })?;

    Ok((s, e))
}

pub async fn save_task(
    pool: &PgPool,
    user_id: i32,
    payload: &TaskAdd
) -> Result<Task, (StatusCode, String)>{
    
    let (start_date, end_date) = match 
    parse_start_end_date(payload.start_date.as_deref(), payload.end_date.as_deref()){
        Ok(v) => v,
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e))
    };

    let saved_task = query_as!(Task, 
    r#"
        INSERT INTO tasks(name, description, start_date, end_date, priority_score, user_id)
        VALUES($1, $2, $3, $4, $5, $6)
        RETURNING id, name, description, start_date, end_date,
        priority_score, created_at, user_id, status as "status: TaskStatus"
    "#,
    payload.name, payload.description, start_date, end_date, payload.priority_score, user_id)
    .fetch_one(pool)
    .await;

    match saved_task{
        Ok(t) => {
            tracing::info!("Task Added!");
            Ok(t)
        },
        Err(e) => {
            tracing::error!("Task could not be saved—database error {}", e);
            return Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Task could not be saved—database error.")));
        }
    }
}

pub async fn get_tasks(pool: &PgPool, user_id: i32) -> Result<Vec<Task>, (StatusCode, String)>{
    let tasks = query_as!(Task, 
    r#"
        SELECT
        id, name, description, start_date, end_date, priority_score, 
        created_at, user_id, status as "status: TaskStatus"
        FROM tasks
        WHERE user_id = $1
        ORDER BY created_at ASC
    "#, user_id)
    .fetch_all(pool)
    .await;

    match tasks{
        Ok(v) => {
            if v.is_empty(){
                tracing::error!("No Task found in db.");
                return Err((StatusCode::NOT_FOUND, "U don't have any task.".to_string()));
            }
            tracing::info!("Tasks found!");
            Ok(v)
        },
        Err(e) => {
            tracing::error!("An internal error occured: {}", e);
            return Err((StatusCode::INTERNAL_SERVER_ERROR, "An internal error occured!".to_string()));
        }
    }
}

pub async fn get_task_by_id(
    pool: &PgPool, task_id: i32
) -> Result<Task, (StatusCode, String)>{
    let task = query_as!(Task, 
    r#"
        SELECT 
        id, name, description, start_date, end_date, priority_score,
        created_at, user_id, status as "status: TaskStatus"
        FROM tasks
        WHERE id = $1
    "#, task_id)
    .fetch_one(pool)
    .await;

    match task {
        Ok(v) => {
            tracing::info!("Task Found!");
            Ok(v)
        },
        Err(e) => {
            tracing::error!("No Task with this id: {}", e);
            return Err((StatusCode::NOT_FOUND, "No Task with this id.".to_string()))
        }
    }
}

pub async fn delete_task_by_id(
    pool: &PgPool,
    task_id: i32
) -> Result<String, (StatusCode, String)>{
    let delete_query = query!(
    r#"
        DELETE
        FROM tasks
        WHERE id = $1
    "#, task_id)
    .execute(pool)
    .await;

    match delete_query {
        Ok(v) => {
            if v.rows_affected() == 0 {
                return Err((StatusCode::NOT_FOUND, "Task could not be found or was already deleted.".to_string()));
            }else{
                tracing::info!("Task Successfully Deleted!");
                Ok("Task deleted successfully.".to_string())
            }
        },
        Err(e) => {
            tracing::error!("An internal error occured: {}", e);
            return Err((StatusCode::INTERNAL_SERVER_ERROR, "An internal error occured!".to_string()));
        }
    }
}

pub async fn update_task(
    pool: &PgPool, task_id: i32, body: TaskUpdate
)
-> Result<(), (StatusCode, String)>
{
    let mut query_builder = QueryBuilder::<Postgres>::new("UPDATE tasks SET ");
    let mut seperated = query_builder.separated(", ");

    seperated.push("name = ");
    seperated.push_bind_unseparated(body.name);
    
    match body.description{
        Some(Some(desc)) => {
            seperated.push("description = ");
            seperated.push_bind_unseparated(desc);
        },
        Some(None) => {
            seperated.push("description = NULL");
        },
        None => {}
    }

    match body.start_date{
        Some(Some(start)) => {
            let date = NaiveDate::parse_from_str(&start, "%Y-%m-%d")
            .map_err(|_| {
                tracing::error!("Server error while parsing start date.");
                (StatusCode::INTERNAL_SERVER_ERROR, "Server error while parsing start date.".to_string())
            })?;

            seperated.push("start_date = ");
            seperated.push_bind_unseparated(date);
        },
        Some(None) => {
            seperated.push("start_date = NULL");
        },
        None => {}
    }

    match body.end_date{
        Some(Some(end)) => {
            let date = NaiveDate::parse_from_str(&end, "%Y-%m-%d")
            .map_err(|_| {
                tracing::error!("Server error while parsing end date.");
                (StatusCode::INTERNAL_SERVER_ERROR, "Server error while parsing end date.".to_string())
            })?;

            seperated.push("end_date = ");
            seperated.push_bind_unseparated(date);
        },
        Some(None) => {
            seperated.push("end_date = NULL");
        },
        None => {}
    }

    match body.priority_score{
        Some(Some(score)) => {
            seperated.push("priority_score = ");
            seperated.push_bind_unseparated(score);
        },
        Some(None) => {
            seperated.push("priority_score = NULL");
        },
        None => {}
    }

    match body.status{
        Some(Some(status_str)) => {
            let status = match status_str.to_uppercase().as_str(){
                "PENDING" => TaskStatus::PENDING,
                "IN PROGRESS" => TaskStatus::IN_PROGRESS,
                "COMPLETED" => TaskStatus::COMPLETED,
                _ => TaskStatus::PENDING
            };

            seperated.push("status = ");
            seperated.push_bind_unseparated(status);
        },
        Some(None) => {
            seperated.push("status = NULL");
        },
        None => {}
    }

    query_builder.push(" WHERE id = ");
    query_builder.push_bind(task_id);

    let query = query_builder.build();
    query.execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("An internal error occured!: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, "An internal error occured!".to_string())
    })?;
    
    tracing::info!("Task Update Successfull!");
    Ok(())
}

pub async fn delete_completed_tasks(pool: &PgPool, user_id: i32)
-> Result<String, (StatusCode, String)>{
    let query = query!(r#"
        DELETE
        FROM tasks
        WHERE user_id = $1 AND status = 'completed'
    "#, user_id)
    .execute(pool)
    .await;

    match query{
        Ok(v) => {
            if v.rows_affected() == 0 {
                return Err((StatusCode::NOT_FOUND, "No completed tasks found.".to_string()));
            }else{
                tracing::info!("Task Successfully Deleted!");
                Ok("Task deleted successfully.".to_string())
            }
        },
        Err(err) => {
            tracing::error!("An internal error occured!: {}", err);
            return Err((StatusCode::INTERNAL_SERVER_ERROR, "An internal error occured!".to_string()));
        }
    }
}

pub async fn get_tasks_with_filter(
    pool: &PgPool,
    user_id: i32, 
    sort_by: SortBy,
    order: SortOrder
)
-> Result<Vec<Task>, (StatusCode, String)>{
    let mut qb = QueryBuilder::<Postgres>::new(r#"
        SELECT
        id, name, description, start_date, end_date,
        priority_score, user_id, status,
        created_at
        FROM tasks
    "#);

    qb.push(" WHERE user_id = ");
    qb.push_bind(user_id);

    qb.push(" ORDER BY ")
    .push(sort_by.column())
    .push(" ")
    .push(order.as_sql());

    let tasks = qb.build_query_as::<Task>()
    .fetch_all(pool)
    .await;

    match tasks{
        Ok(v) => {
            if v.is_empty(){
                tracing::error!("No Task found in db.");
                return Err((StatusCode::NOT_FOUND, "U don't have any task.".to_string()));
            }
            tracing::info!("Tasks found!");
            Ok(v)
        },
        Err(e) => {
            tracing::error!("An internal error occured!: {}", e);
            return Err((StatusCode::INTERNAL_SERVER_ERROR, "An internal error occured!".to_string()));
        }
    }
}