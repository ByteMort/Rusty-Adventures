use axum::{Extension, Json, extract::{Path, Query, State, rejection::JsonRejection}, http::{HeaderMap, StatusCode, header::SET_COOKIE}, response::{IntoResponse, Response}};
use axum_valid::{Valid, ValidRejection};
use sqlx::{PgPool};

use crate::{models::user::{VerifyCode, VerifyEmail}, page::{PaginationParams, page_limit_control}, services::user_service::{get_all_users, resend_user_code, verify_user_code}};
use crate::{jwt::claims::Claims, models::user::{LoginUser, RegisterUser, Role}, services::user_service::{delete_me, delete_user_as_admin, get_all_users_paginated, get_me, login_user}, valid::valid_body};
use crate::services::user_service::{register_user};

pub async fn register_user_handler(
    State(state): State<PgPool>,
    payload: Result<Valid<Json<RegisterUser>>, ValidRejection<JsonRejection>>,
) -> Response{
   
    let payload = match valid_body(payload){
        Ok(v) => v,
        Err(resp) => return resp,
    };

    match register_user(&state, payload).await {
        Ok(u) => {
            (StatusCode::CREATED, Json(u)).into_response()
        },
        Err((code, msg)) => {
            (code, Json(serde_json::json!({"message": msg}))).into_response()
        }
    }
}

pub async fn login_user_handler(
    State(state): State<PgPool>,
    payload: Result<Valid<Json<LoginUser>>, ValidRejection<JsonRejection>> 
) -> Response{
    let payload:LoginUser = match valid_body(payload){
        Ok(v) => v,
        Err(resp) => return resp,
    };

    match login_user(&state, payload).await {
        Ok(s) => {
            let mut headers = HeaderMap::new();
            let cookie_value = format!(
                "token={}; Path=/; HttpOnly; Secure; SameSite=Strict; Max-Age=600"
                , s.0
            );

            headers.insert(SET_COOKIE, cookie_value.parse().unwrap());

            let cookie_value2 = format!(
                "refresh_token={}; Path=/; HttpOnly; Secure; SameSite=Strict; Max-Age=432000"
                , s.1
            );

            headers.append(SET_COOKIE, cookie_value2.parse().unwrap());

            (StatusCode::OK, headers, Json(serde_json::json!({"message": "Login Successfully!"}))).into_response()
        },
        Err((code, msg)) => {
            (code, Json(serde_json::json!({"message": msg}))).into_response()
        }
    }
}

fn clear_auth_cookies() -> HeaderMap{
    let mut headers = HeaderMap::new();

    headers.append(
        SET_COOKIE,
        "token=; Path=/; HttpOnly; Secure; SameSite=Strict; Max-Age=0"
        .parse().unwrap()
    );

    headers.append(
        SET_COOKIE,
        "refresh_token=; Path=/; HttpOnly; Secure; SameSite=Strict; Max-Age=0"
        .parse().unwrap()
    );

    headers
}

pub async fn logout_handler() -> impl IntoResponse{
    (
        StatusCode::OK,
        clear_auth_cookies(),
        Json(serde_json::json!({"message": "Logout Successfully"}))
    )
}

pub async fn get_all_users_paginated_handler(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
    Query(params): Query<PaginationParams>
) -> Response{

    if claims.role != Role::RoleAdmin {
        (
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({"message": "You are not admin."}))
        ).into_response()
    }else{
        match (params.page, params.limit){
            (None, None) => {
                match get_all_users(&pool).await{
                    Ok(v) => {
                        (
                            StatusCode::OK,
                            Json(v)
                        ).into_response()
                    },
                    Err((code, msg)) => {
                        return (
                            code,
                            Json(serde_json::json!({"message": msg}))
                        ).into_response();
                    }
                }
            },
            _ => {
                let page = params.page.unwrap_or(1);
                let limit = params.limit.unwrap_or(3);

                if let Err((code, msg)) = page_limit_control(page, limit){
                    return (
                        code,
                        Json(serde_json::json!({"message": msg}))
                    ).into_response();
                }

                match get_all_users_paginated(&pool, page, limit).await{
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
        }
    }
}

pub async fn get_me_handler(
    State(pool): State<PgPool>,
    Extension(claim): Extension<Claims>
) 
-> Response{
    match get_me(&pool, claim.sub).await{
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

pub async fn delete_me_handler(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>
)
-> Response{
    match delete_me(&pool, claims.sub).await{
        Ok(s) => {
            (
                StatusCode::OK,
                clear_auth_cookies(),
                Json(serde_json::json!({"message": s}))
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

pub async fn delete_user_as_admin_handler(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
    Path(user_id): Path<i32>,
) 
-> Response{
    if claims.role != Role::RoleAdmin{
        (
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({"message": "You are not admin."}))
        ).into_response()
    }else{
        match delete_user_as_admin(&pool, user_id).await{
            Ok(s) => {
                (
                    StatusCode::OK,
                    Json(serde_json::json!({"message": s}))
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
}

pub async fn resend_user_code_handler(
    State(pool): State<PgPool>,
    payload: Result<Valid<Json<VerifyEmail>>, ValidRejection<JsonRejection>>
) -> Response{
    let body = match valid_body(payload){
        Ok(v) => v,
        Err(e) => return e
    };

    match resend_user_code(&pool, body.email).await{
        Ok(v) => {
            (
                StatusCode::OK,
                Json(serde_json::json!({"message": v}))
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

pub async fn verify_user_code_handler(
    State(pool): State<PgPool>,
    payload: Result<Valid<Json<VerifyCode>>, ValidRejection<JsonRejection>> 
) -> Response{
    let body = match valid_body(payload){
        Ok(v) => v,
        Err(e) => return e
    };

    match verify_user_code(&pool, body).await{
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