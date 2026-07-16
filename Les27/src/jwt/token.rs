use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, encode, decode};
use chrono::{Duration, Utc};
use crate::{jwt::claims::{Claims, RefreshClaims}, models::user::Role};

fn get_jwt_secret() -> Vec<u8>{
    std::env::var("JWT_SECRET")
        .unwrap_or_else(|_e|{
            tracing::error!("JWT_SECRET must be set in .env file");
            panic!("JWT_SECRET must be set in .env file")
        })
        .into_bytes()
}

pub fn generate_token(user_id: i32, role: &Role) -> Result<String, jsonwebtoken::errors::Error>{
    let now = Utc::now();
    let iat:usize = now.timestamp() as usize;
    let exp:usize = (now + Duration::minutes(10)).timestamp() as usize;

    let claims:Claims = Claims { sub: user_id, role: role.clone(), exp, iat };

    let secret:Vec<u8> = get_jwt_secret();
    encode(&Header::default(), &claims, &EncodingKey::from_secret(&secret))
}

pub fn verify_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error>{
    let validation:Validation = Validation::default();

    let secret:Vec<u8> = get_jwt_secret();
    
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(&secret),
        &validation,
    )?;
    
    Ok(token_data.claims)
}

pub fn generate_refresh_token(user_id: i32) -> Result<String, jsonwebtoken::errors::Error>{
    let now = Utc::now();
    let exp:usize = (now + Duration::days(5)).timestamp() as usize;

    let claims = RefreshClaims{
        sub: user_id,
        exp: exp,
    };

    let secret:Vec<u8> = get_jwt_secret();

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(&secret)
    )
}

pub fn verify_refresh_token(refresh_token: &str) -> Result<RefreshClaims, jsonwebtoken::errors::Error>{
    let validation:Validation = Validation::default();

    let secret:Vec<u8> = get_jwt_secret();

    let token_data = decode
    (
        refresh_token,
        &DecodingKey::from_secret(&secret),
        &validation
    )?;

    Ok(token_data.claims)
}