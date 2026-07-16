use axum::http::StatusCode;
use chrono::{DateTime, Utc};
use sqlx::query_scalar;
use sqlx::{PgPool, query_as, query};

use crate::models::user::RegisterUser;
use crate::models::user::User;
use crate::models::user::Role;


pub async fn save_user(pool: &PgPool, payload: &RegisterUser) -> Result<User, (StatusCode, String)>{
    let saved_user = query_as!(User, 
        r#"
        INSERT INTO users(username, email, password) 
        VALUES($1, $2, $3)
        RETURNING id, username, email, password,
        created_at as "created_at: chrono::DateTime<chrono::Utc>",
        is_verified, role as "role: Role"
        "#, 
        payload.username,
        payload.email,
        payload.password
    )
    .fetch_one(pool)
    .await;

    match saved_user {
        Ok(u) => {
            tracing::info!("The account was successfully registered!");
            Ok(u)
        },
        Err(e) => {
            tracing::error!("Something went wrong while saving user: {}", e);

            if let Some(db_err) = e.as_database_error(){
                if db_err.code() == Some(std::borrow::Cow::Borrowed("23505")){
                    return Err(
                        (
                            StatusCode::CONFLICT,
                            format!("This email address is already registered.")
                        )
                    );
                }
            }

            Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Something went wrong while saving user.")))
        }
    }
}

pub async fn get_user_by_email(pool: &PgPool, email:String) -> Result<User, (StatusCode, String)>{
    let db_user = query_as!(User, 
    r#"
        SELECT 
            id,
            username,
            email,
            password,
            role as "role: Role",
            is_verified,
            created_at as "created_at: chrono::DateTime<chrono::Utc>"
        FROM users
        WHERE email = $1
    "#, email)
    .fetch_one(pool)
    .await;
    
    match db_user {
        Ok(v) => {
            tracing::info!("User found!");
            Ok(v)
        },
        Err(e) => {
            tracing::error!("No user was found with this email address: {}", e);
            Err((StatusCode::NOT_FOUND, format!("No user was found with this email address.")))
        }
    }
}

pub async fn get_all_users(pool: &PgPool)
-> Result<Vec<User>, (StatusCode, String)>{
    let users = query_as!(User,
    r#"
        SELECT
            id,
            username,
            email,
            password,
            role as "role:Role",
            is_verified,
            created_at as "created_at: chrono::DateTime<chrono::Utc>"
        FROM users;
    "#)
    .fetch_all(pool)
    .await;

    match users{
        Ok(u) => {
            if u.is_empty() {
                tracing::error!("No User found in db: Table is empty");
                return Err((StatusCode::NOT_FOUND, format!("No User found in db.")));
            }
            tracing::info!("Users found!");
            Ok(u)
        },
        Err(e) => {
            tracing::error!("An internal error occured: {}", e);
            return Err((StatusCode::INTERNAL_SERVER_ERROR, format!("An internal error occured!")));
        }
    }
}

pub async fn get_all_users_paginated(pool: &PgPool, page: i64, limit: i64) -> Result<Vec<User>, (StatusCode, String)> {
    let offset = (page.saturating_sub(1)) * limit;

    let users = query_as!(User,
    r#"
        SELECT
            id,
            username,
            email,
            password,
            role as "role:Role",
            is_verified,
            created_at as "created_at: chrono::DateTime<chrono::Utc>"
        FROM users
        ORDER BY id
        LIMIT $1 OFFSET $2;
    "#, limit, offset)
    .fetch_all(pool)
    .await;

    match users{
        Ok(u) => {
            tracing::info!("Users found!");
            Ok(u)
        },
        Err(e) => {
            tracing::error!("An internal error occured: {}", e);
            return Err((StatusCode::INTERNAL_SERVER_ERROR, format!("An internal error occured!")));
        }
    }
}

pub async fn get_users_total(pool: &PgPool)
-> Result<i64, (StatusCode, String)>{
    let total = query_scalar!(
        "SELECT COUNT(*) FROM users"
    )
    .fetch_one(pool)
    .await;

    match total{
        Ok(v) => {
            let count = v.unwrap_or(0);
            if count == 0 {
                tracing::error!("No User found in db.");
                return Err((
                    StatusCode::NOT_FOUND,
                    "".to_string()
                ));
            }
            tracing::info!("Users Count found.");
            Ok(count)
        },
        Err(e) => {
            tracing::error!("An internal error occured: {}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "An internal error occured!".to_string()
            ))
        }
    }
}

pub async fn user_exists(pool: &PgPool, user_id: i32) -> Result<bool, (StatusCode, String)>{
    let user = query_scalar!(
    r#"
        SELECT EXISTS(
            SELECT 1
            FROM users
            WHERE id = $1
        )
        as "exists!"
    "#, user_id)
    .fetch_one(pool)
    .await;

    match user{
        Ok(v) => {
            tracing::info!("User Exists!");
            Ok(v)
        },
        Err(e) => {
            tracing::error!("An internal error occured: {}", e);
            Err((StatusCode::INTERNAL_SERVER_ERROR, format!("An internal error occured!")))
        }
    }
}

pub async fn get_user_by_id(pool: &PgPool, user_id: i32) -> Result<User, (StatusCode, String)>{
    let db_user = query_as!(User, 
    r#"
        SELECT
        id,
        username,
        email,
        password,
        role as "role:Role",
        is_verified,
        created_at
        FROM users
        WHERE id = $1
    "#, user_id)
    .fetch_one(pool)
    .await;

    match db_user{
        Ok(v) => {
            tracing::info!("User found!");
            return Ok(v);
        },
        Err(e) => {
            tracing::info!("No user was found: {}", e);
            return Err((StatusCode::NOT_FOUND, "No user was found.".to_string()));
        }
    }
}

pub async fn delete_user(pool: &PgPool, user_id: i32)
-> Result<String, (StatusCode, String)>{
    let delete = query!(r#"
        DELETE
        FROM users
        WHERE id = $1
    "#, user_id)
    .execute(pool)
    .await;

    match delete {
        Ok(r) => {
            if r.rows_affected() == 0 {
                tracing::error!("User could not be found or was already deleted.");
                return Err((StatusCode::NOT_FOUND, "User could not be found or was already deleted.".to_string()));
            }
            tracing::info!("User deleted successfully");
            Ok("User deleted successfully.".to_string())
        },
        Err(e) => { 
            tracing::error!("An internal error occured: {}", e);
            return Err((StatusCode::INTERNAL_SERVER_ERROR, "An internal error occured!".to_string()));
        }
    }
}

pub async fn save_code_for_user(
    pool: &PgPool,
    user_id: i32, 
    code: String, 
    expires_at: DateTime<Utc>
) -> Result<String, (StatusCode, String)>{
    let query = query!(r#"
        INSERT INTO
        user_codes(code, expires_at, user_id)
        VALUES($1, $2, $3);
    "#, code, expires_at, user_id)
    .execute(pool)
    .await;

    match query {
        Ok(v) => {
            if v.rows_affected() == 0{
                tracing::error!("We were unable to save a code for the user");
                return Err((
                    StatusCode::BAD_REQUEST,
                    "We were unable to save a code for the user".to_string()
                ));
            }
            tracing::info!("The code was successfully send to your email.");
            return Ok("The code was successfully sent to your email. It is valid for 5 minutes.".to_string());
        },
        Err(err) => {
            tracing::error!("An internal error occured: {}", err);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "An internal error occured.".to_string()
            ))
        }
    }
}

pub async fn get_last_user_code(pool: &PgPool, user_id: i32)
-> Result<Option<(DateTime<Utc>, String)>, (StatusCode, String)>{
    let query = query!(r#"
        SELECT expires_at AS "expires_at: DateTime<Utc>", code
        FROM user_codes
        WHERE user_id = $1
        ORDER BY id DESC
        LIMIT 1
    "#, user_id)
    .fetch_optional(pool)
    .await;

    match query {
        Ok(v) => {
            Ok(v.map(|row| (row.expires_at, row.code)))
        },
        Err(err) => {
            tracing::error!("An internal error occured: {}", err);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "An internal error occured.".to_string()
            ));
        }
    }
}

pub async fn verify_account(pool: &PgPool, user_id: i32)
-> Result<String, (StatusCode, String)>{
    let query = query!(r#"
        UPDATE users
        SET is_verified = true
        WHERE id = $1
    "#, user_id)
    .execute(pool)
    .await;

    match query{
        Ok(v) => {
            if v.rows_affected() == 0{
                tracing::error!("We couldn't verify your account due to an issue. Please try again.");
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "We couldn't verify your account due to an issue. Please try again.".to_string()
                ));
            }
            tracing::info!("Your account has been successfully verified.");
            Ok("Your account has been successfully verified.".to_string())
        },
        Err(err) => {
            tracing::error!("An internal error occured: {}", err);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "An internal error occured.".to_string()
            ));
        }
    }
}