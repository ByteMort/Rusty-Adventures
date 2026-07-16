use sqlx::PgPool;
use axum::{Router, middleware, routing::{delete, get, post, put}};

use crate::{handlers::{task_handler::{add_task_handler, delete_completed_tasks_handler, delete_my_task_handler, get_my_task_handler, get_tasks_filter_handler, update_task_handler}, user_handler::{self, delete_me_handler, delete_user_as_admin_handler, get_all_users_paginated_handler, get_me_handler, login_user_handler, logout_handler, resend_user_code_handler, verify_user_code_handler}}, jwt::middleware::auth_middleware};

mod models;
mod handlers;
mod services;
mod repositories;
mod errors;
mod valid;
mod jwt;
mod page;
mod email;
pub mod background_jobs;

pub async fn run(pool: &PgPool){

    let protected_routes = Router::new()
        .route("/admin/users", get(get_all_users_paginated_handler))
        .route("/admin/user/{id}", delete(delete_user_as_admin_handler))
        .route("/me", get(get_me_handler))
        .route("/me", delete(delete_me_handler))
        .route("/logout", post(logout_handler))
        .route("/tasks/add", post(add_task_handler))
        .route("/tasks", get(get_my_task_handler))
        .route("/tasks/{id}", delete(delete_my_task_handler))
        .route("/tasks/{id}", put(update_task_handler))
        .route("/tasks/completed", delete(delete_completed_tasks_handler))
        .route("/tasks/filter", get(get_tasks_filter_handler))
        .route_layer(middleware::from_fn_with_state(pool.clone(), auth_middleware));

    let app_routes = Router::new()
        .route("/register", post(user_handler::register_user_handler))
        .route("/login", post(login_user_handler))
        .route("/resend", post(resend_user_code_handler))
        .route("/verify", post(verify_user_code_handler))
        .merge(protected_routes);

    let app = Router::new()
        .nest("/api", app_routes)
        .with_state(pool.clone());

    let addr:&str = "127.0.0.1:3000";
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .unwrap_or_else(|err|{
            tracing::error!("Axum Server couldn't start: {}", err);
            panic!("Axum Server stopped.");
        });
    tracing::info!("Axum server successfully started.");

    axum::serve(listener, app).await.unwrap();
}