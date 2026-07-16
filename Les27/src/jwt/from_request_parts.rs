/*
use axum::{Json, extract::FromRequestParts, http::StatusCode, response::{IntoResponse, Response}};

use crate::jwt::{claims::Claims, token::verify_token};


impl<S> FromRequestParts<S> for Claims
where 
    S: Send + Sync,
{
    type Rejection = Response;

    fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _state: &S,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> + Send {
       
    }
}
*/