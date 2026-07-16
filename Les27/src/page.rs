use axum::http::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct PaginationParams{
    pub page: Option<i64>,
    pub limit: Option<i64>
}

#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T>{
    pub data: Vec<T>,
    pub page: i64,
    pub limit: i64,
    pub total: i64,
    pub total_pages: i64
}

pub fn page_limit_control(page: i64, limit: i64)
-> Result<(), (StatusCode, String)>{
    if page < 1{
        return Err((
            StatusCode::BAD_REQUEST,
            "Page must be greater than or equal to 1.".to_string()
        ));
    }

    if limit < 1{
        return Err((
            StatusCode::BAD_REQUEST,
            "Limit must be greater than or equal to 1.".to_string()
        ));
    }

    Ok(())
}