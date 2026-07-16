use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow};
use validator::Validate;
use crate::valid::validate_not_blank;

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterUser{
    #[validate(
        length(min=3, max=200, message="Username should be minimum 3 characters."),
        custom(function="validate_not_blank")
    )]
    pub username: String,
    #[validate(email(message="Email should be in email format."))]
    pub email: String,
    #[validate(length(min=8, message="Password should be minimum 8 characters."))]
    pub password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginUser{
    #[validate(email(message="Email should be in email format."))]
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize, sqlx::Type, Clone, PartialEq)]
#[sqlx(type_name="varchar")]
pub enum Role{
    #[sqlx(rename="role_user")]
    #[serde(rename="User")]
    RoleUser,
    #[sqlx(rename="role_admin")]
    #[serde(rename="Admin")]
    RoleAdmin
}

#[derive(Debug, FromRow, Serialize)]
pub struct User{
    #[serde(skip_serializing)]
    pub id: i32,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub created_at: DateTime<Utc>,
    #[serde(skip_serializing)]
    pub is_verified: bool,
    pub role: Role
}

#[derive(Debug, Serialize)]
pub struct AdminUserView{
    pub id: i32,
    pub username: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub is_verified: bool,
    pub role: Role
}

impl From<User> for AdminUserView{
    fn from(u: User) -> Self {
        Self {
            id: u.id, username: u.username, 
            email: u.email, created_at: u.created_at,
            is_verified: u.is_verified, role: u.role
        }
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct VerifyEmail{
    #[validate(email(message="Email should be in email format."))]
    pub email: String
}

#[derive(Debug, Deserialize, Validate)]
pub struct VerifyCode{
    #[validate(email(message="Email should be in email format."))]
    pub email: String,
    #[validate(length(equal=6, message="Code must be exactly 6 characters long."))]
    pub code: String,
}