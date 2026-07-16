use serde::{Deserialize, Serialize};

use crate::models::user::Role;


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims{
    pub sub: i32,
    pub role: Role,
    pub exp: usize,
    pub iat: usize,    
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshClaims{
    pub sub: i32,
    pub exp: usize,
}