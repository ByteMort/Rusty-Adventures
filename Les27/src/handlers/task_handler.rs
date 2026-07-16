use axum::{Extension, Json, extract::{Path, Query, State, rejection::JsonRejection}, http::StatusCode, response::{IntoResponse, Response}};
use axum_valid::{Valid, ValidRejection};
use sqlx::PgPool;

use crate::{jwt::claims::Claims, models::task::{TaskAdd, TaskFilterQuery, TaskUpdate}, services::task_service::{add_task, delete_completed_tasks, delete_my_task, get_my_task, get_tasks_filter, update_task}, valid::valid_body};

pub async fn add_task_handler(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
    payload: Result<Valid<Json<TaskAdd>>, ValidRejection<JsonRejection>>
) -> Response{

    let payload = match valid_body(payload){
        Ok(v) => v,
        Err(e) => return e
    };

    match add_task(&pool, claims.sub, &payload).await{
        Ok(v) => {
            (
                StatusCode::CREATED,
                Json(v)
            ).into_response()
        },
        Err((code, msg)) => {
            (
                code,
                Json(serde_json::json!({"message": msg}))
            ).into_response()
        }
    }
}

pub async fn get_my_task_handler(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>
) -> Response{
    match get_my_task(&pool, claims.sub).await {
        Ok(v) => {
            (
                StatusCode::OK,
                Json(v)
            ).into_response()
        }
        Err((code, msg)) => {
            (
                code,
                Json(serde_json::json!({"message": msg}))
            ).into_response()
        }
    }
}

pub async fn delete_my_task_handler(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
    Path(task_id): Path<i32>
) -> Response{
    match delete_my_task(&pool, claims.sub, task_id).await {
        Ok(v) => {
            (
                StatusCode::OK,
                Json(serde_json::json!({"message": v}))
            ).into_response()
        },
        Err((code, msg)) => {
            (
                code,
                Json(serde_json::json!({"message": msg}))
            ).into_response()
        }
    }
}

pub async fn update_task_handler(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
    Path(task_id): Path<i32>,
    payload: Result<Valid<Json<TaskUpdate>>, ValidRejection<JsonRejection>>
)
-> Response{
    let body = match valid_body(payload){
        Ok(v) => v,
        Err(e) => return e
    };

    match update_task(&pool, claims.sub, task_id, body).await {
        Ok(t) => {
            (
                StatusCode::ACCEPTED,
                Json(t)
            ).into_response()
        },
        Err((code, msg)) =>{
            return (code, Json(serde_json::json!({"message":msg}))).into_response();
        }
    }
}

pub async fn delete_completed_tasks_handler(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>
) -> Response {
    match delete_completed_tasks(&pool, claims.sub).await{
        Ok(v) => {
            (
                StatusCode::OK,
                Json(serde_json::json!({"message": v}))
            ).into_response()
        },
        Err((code, msg)) => {
            (
                code,
                Json(serde_json::json!({"message": msg}))
            ).into_response()
        }
    }
}

pub async fn get_tasks_filter_handler(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
    Query(query): Query<TaskFilterQuery>
) -> Response{
    match get_tasks_filter(&pool, claims.sub, query).await{
        Ok(v) => {
            (
                StatusCode::OK,
                Json(v)
            ).into_response()
        },
        Err((code, msg)) => {
            (
                code,
                Json(serde_json::json!({"message": msg}))
            ).into_response()
        }
    }
}