use axum::http::StatusCode;
use bcrypt::{DEFAULT_COST, hash};
use chrono::{Duration, Utc};
use crate::email::send_email;
use sqlx::PgPool;
use crate::jwt::token::{generate_refresh_token, generate_token};

use crate::page::PaginatedResponse;
use crate::models::user::{AdminUserView, VerifyCode};
use crate::repositories::user_repository::{self, delete_user, get_last_user_code, get_user_by_id, save_code_for_user, user_exists, verify_account};
use crate::{models::user::{LoginUser, RegisterUser, User}, repositories::user_repository::{get_user_by_email, save_user}};

pub async fn validate_user(pool: &PgPool, user_id: i32, txt: Option<String>) -> Result<(), (StatusCode, String)> {
    let user_status = match user_exists(pool, user_id).await{
        Ok(v) => v,
        Err((code, msg)) => {
            return Err((code, msg));
        }
    };

    if !user_status{
        let t = txt.unwrap_or("token".to_string());
        tracing::error!("No user associated with this {} was found.", t);
        return Err((StatusCode::NOT_FOUND, format!("No User associated with this {} was found.", t)));
    }

    Ok(())
}

pub async fn register_user(pool: &PgPool, payload: RegisterUser) -> Result<User, (StatusCode, String)>{

    let hashed_password = hash(payload.password, DEFAULT_COST)
        .map_err(|e| {
            tracing::error!("Hashing Password Failed! {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, format!("Hashing Password Failed!"))
        })?;    

    let save_dto:RegisterUser = RegisterUser{
        username: payload.username,
        email: payload.email,
        password: hashed_password,
    };

    return save_user(pool, &save_dto).await;
}

pub async fn login_user(pool: &PgPool, payload: LoginUser) -> Result<(String, String), (StatusCode, String)>{

    let user = match get_user_by_email(&pool, payload.email).await{
        Ok(u) => u,
        Err((code, msg)) => {
            return Err((code, msg));
        }
    };

    if !user.is_verified{
        return Err(
            (StatusCode::FORBIDDEN,
            "Please verify your email address to log in.".to_string())
        );
    }

    let is_valid = bcrypt::verify(&payload.password, &user.password)
        .map_err(|e| {
            tracing::error!("Password verification failed: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, format!("Internal server error: Password verification failed"))
        })?;

    if !is_valid{
        tracing::error!("User Password is not correct!");
        return Err((StatusCode::UNAUTHORIZED, "User Password is not correct!".to_string()));
    }

    let token:String = match generate_token(user.id, &user.role) {
        Ok(v) => {
            v
        },
        Err(e) => {
            tracing::error!("Something went wrong while generating token: {}", e);
            return Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Something went wrong while generating token.")));
        }
    };

    let refresh_token:String = match generate_refresh_token(user.id){
        Ok(v) => {
            v
        },
        Err(e) => {
            tracing::error!("Something went wrong while generating refresh token: {}", e);
            return Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Something went wrong while generating refresh token.")));
        }
    };

    Ok((token, refresh_token))
}

pub async fn get_all_users(pool: &PgPool)
-> Result<Vec<AdminUserView>, (StatusCode, String)>{
    return match user_repository::get_all_users(pool).await{
        Ok(v) => {
            Ok(v.into_iter()
            .map(AdminUserView::from)
            .collect::<Vec<AdminUserView>>())
        },
        Err(e) => {
            return Err(e);
        }
    };
}

pub async fn get_all_users_paginated(pool: &PgPool, page: i64, limit: i64) 
-> Result<PaginatedResponse<AdminUserView>, (StatusCode, String)>{
    let total_users = match user_repository::get_users_total(pool).await{
        Ok(v) => v,
        Err(e) => {
            return Err(e);
        }
    };
    
    let users = match user_repository::get_all_users_paginated(pool, page, limit).await{
        Ok(v) => {
            let admin_views = v
                .into_iter()
                .map(AdminUserView::from)
                .collect::<Vec<AdminUserView>>();

            admin_views
        },
        Err(e) => {
            return Err(e);
        }
    };

    let total_pages = if limit == 0{
        0
    } else{
        (total_users + limit -1) / limit
    };

    Ok(PaginatedResponse{
        data: users,
        page: page,
        limit: limit,
        total: total_users,
        total_pages: total_pages
    })
}

pub async fn get_me(
    pool: &PgPool,
    user_id: i32,
) 
-> Result<User, (StatusCode, String)>{
    return get_user_by_id(pool, user_id).await;
}

pub async fn delete_me(
    pool: &PgPool,
    user_id: i32
)
-> Result<String, (StatusCode, String)>{
    validate_user(pool, user_id, None).await?;

    return delete_user(pool, user_id).await;
}

pub async fn delete_user_as_admin(
    pool: &PgPool,
    user_id: i32
)
-> Result<String, (StatusCode, String)>{
    validate_user(pool, user_id, Some("id".to_string())).await?;

    return delete_user(pool, user_id).await;
}

pub async fn resend_user_code(
    pool: &PgPool,
    email: String
) -> Result<String, (StatusCode, String)> {
    let user: User = get_user_by_email(pool, email).await?;

    if user.is_verified{
        return Err((
            StatusCode::BAD_REQUEST,
            "This user has already been verified.".to_string()
        ));
    }

    match get_last_user_code(pool, user.id).await?{
        Some((expires_at, _code)) => {
            if expires_at > Utc::now(){
                return Err((
                    StatusCode::BAD_REQUEST,
                    "You already have an active code. Please check your email.".to_string()
                ));
            }else{
                return generate_code_and_send_email(pool, user.id, user.email).await;
            }
        },
        None => {
            return generate_code_and_send_email(pool, user.id, user.email).await;
        }
    }    
}


async fn generate_code_and_send_email(pool: &PgPool, user_id: i32, email: String) 
-> Result<String, (StatusCode, String)>{
    let generated_code: String = rand::random_range(100_000..=999_999).to_string();

    let expires_at = Utc::now() + Duration::minutes(5);
    
    match save_code_for_user(pool, user_id, generated_code.clone(), expires_at).await{
        Ok(v) => {
            send_email(email, generated_code).await?;           

            Ok(v)
        },
        Err((code, msg)) => {
            Err((code, msg))
        }
    }
}

pub async fn verify_user_code(
    pool: &PgPool,
    body: VerifyCode,
) -> Result<String, (StatusCode, String)>{

    let user: User = get_user_by_email(pool, body.email).await?;

    if user.is_verified{
        return Err((
            StatusCode::BAD_REQUEST,
            "This user has already been verified.".to_string()
        ));
    }

    match get_last_user_code(pool, user.id).await? {
        Some((expired_at, code)) => {
            if expired_at < Utc::now(){
                return Err((
                    StatusCode::NOT_FOUND,
                    "It looks like you don't have an active verification code yet. Please request a new one to continue.".to_string()
                ))
            }

            if code != body.code{
                return Err((
                    StatusCode::BAD_REQUEST,
                    "The verification code you entered is incorrect. Please check and try again.".to_string()
                ))
            }

            return verify_account(pool, user.id).await;
        },
        None => {
            return Err((
                StatusCode::NOT_FOUND,
                "It looks like you don't have an active verification code yet. Please request a new one to continue.".to_string()
            ))
        }
    }
}