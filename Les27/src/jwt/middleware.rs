use axum::{Json, body::Body, extract::State, http::{Request, StatusCode, header::SET_COOKIE}, middleware::Next, response::{IntoResponse, Response}};
use chrono::{Duration, Utc};
use sqlx::PgPool;

use crate::{jwt::{claims::Claims, token::{generate_token, verify_refresh_token, verify_token}}, repositories::user_repository::get_user_by_id};



pub async fn auth_middleware(
    State(pool): State<PgPool>,
    mut req: Request<Body>,
    next: Next
) -> Result<Response, Response>{

    let cookies = req
        .headers()
        .get("cookie")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    let mut access_token = None;
    let mut refresh_token = None;

    for cookie in cookies.split(";"){
        let parts:Vec<&str> = cookie.trim().split("=").collect();

        if parts.len() == 2 {
            match parts[0] {
                "token" => access_token = Some(parts[1]),
                "refresh_token" => refresh_token = Some(parts[1]),
                _ => {}
            }
        }
    }

    let mut resolved_claims = None;
    let mut new_token_cookie = None;

    if let Some(token_str) = access_token{
        if let Ok(claims) = verify_token(token_str){
            resolved_claims = Some(claims);
        }
    }

    if resolved_claims.is_none(){
        if let Some(ref_token_str) = refresh_token{
            if let Ok(claims) = verify_refresh_token(ref_token_str){
                let user_id = claims.sub;
                let user = get_user_by_id(&pool, user_id)
                .await.map_err(|(code, msg)|  {
                    (
                        code,
                        Json(serde_json::json!({"message": msg}))
                    ).into_response()
                })?;

                let role = user.role;

                if let Ok(new_token) = generate_token(user_id, &role){
                    let now = Utc::now();
                    let iat:usize = now.timestamp() as usize;
                    let exp:usize = (now + Duration::minutes(10)).timestamp() as usize;

                    resolved_claims = Some(Claims{
                        sub: user_id,
                        role: role,
                        exp: exp,
                        iat: iat
                    });

                    new_token_cookie = Some(format!(
                        "token={}; Path=/; HttpOnly; Secure; SameSite=Strict; Max-Age=600",
                        new_token
                    ));
                }
            }
        }
    }

    let claims = match resolved_claims{
        Some(v) => v,
        None => {
            let error_response = (
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({"message": "Your session has expired or is invalid. Please log in again."}))
            ).into_response();
            return Err(error_response);
        }
    };
    
    req.extensions_mut().insert(claims);

    let mut response = next.run(req).await;

    if let Some(cookie_value) = new_token_cookie{
        response.headers_mut().append(
            SET_COOKIE,
            cookie_value.parse().unwrap(),
        );
    }

    Ok(response)
}