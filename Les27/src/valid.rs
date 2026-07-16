use std::borrow::Cow::Borrowed;

use axum::{Json, extract::rejection::JsonRejection, http::StatusCode, response::{IntoResponse, Response}};
use axum_valid::{Valid, ValidRejection};
use chrono::{Local, NaiveDate};
use validator::ValidationError;

use crate::{errors::CustomJsonError, models::task::{TaskAdd, TaskUpdate}};

pub fn valid_body<T>(body: Result<Valid<Json<T>>, ValidRejection<JsonRejection>>) -> Result<T, Response>{
    let Valid(Json(body)) = match body {
        Ok(v) => v,
        Err(ValidRejection::Inner(e)) => {
            return Err(CustomJsonError(e).into_response());
        },
        Err(ValidRejection::Valid(v_err)) => {
            return Err((StatusCode::BAD_REQUEST, Json(serde_json::json!({"message": v_err.to_string()}))).into_response())
        }
    };

    Ok(body)
}


pub fn validate_not_blank(text: &str) -> Result<(), ValidationError>{
    if text.trim().is_empty(){
        let mut error = ValidationError::new("not_blank");
        error.message = Some(Borrowed("This field is required."));
        return Err(error);
    }

    Ok(())
}

pub fn validate_date(text: &str) -> Result<(), ValidationError>{
    let date = NaiveDate::parse_from_str(text, "%Y-%m-%d")
        .map_err(|_| {
            let mut error = ValidationError::new("invalid_date_format");
            error.message = Some(Borrowed("Invalid date format."));
            error
        })?;
    
    if date < Local::now().date_naive(){
        let mut error = ValidationError::new("invalid_date");
        error.message = Some(Borrowed("You cannot provide a past date."));
        return Err(error);
    }

    Ok(())
}

pub fn validate_date_range(task: &TaskAdd) -> Result<(), ValidationError>{
    if let (Some(start), Some(end)) = (task.start_date.as_ref(), task.end_date.as_ref()){
        let start_date = NaiveDate::parse_from_str(start, "%Y-%m-%d").ok();
        let end_date = NaiveDate::parse_from_str(end, "%Y-%m-%d").ok();
        if start_date > end_date {
            let mut error = ValidationError::new("invalid_date_range");
            error.message = Some(Borrowed("Start Date should be before End Date."));
            return Err(error);
        }
    }

    Ok(())
}

pub fn validate_task_update(task: &TaskUpdate) -> Result<(), ValidationError>{
    if let Some(Some(desc)) = &task.description{
        if desc.len() > 500{
            let mut error = ValidationError::new("desc_length");
            error.message = Some(Borrowed("Description cannot exceed 500 characters."));
            return Err(error);
        }
    }

    if let Some(Some(start_date)) = &task.start_date{
        if NaiveDate::parse_from_str(start_date, "%Y-%m-%d").is_err() {
            let mut error = ValidationError::new("invalid_date_format");
            error.message = Some(Borrowed("Start Date: Invalid date format."));
            return Err(error);
        }
    }

    if let Some(Some(end_date)) = &task.end_date{
        if NaiveDate::parse_from_str(end_date, "%Y-%m-%d").is_err() {
            let mut error = ValidationError::new("invalid_date_format");
            error.message = Some(Borrowed("End Date: Invalid date format."));
            return Err(error);
        }
    }
    
    if let (Some(Some(start)), Some(Some(end))) = (&task.start_date, &task.end_date){
        let start_date = NaiveDate::parse_from_str(start, "%Y-%m-%d").ok();
        let end_date = NaiveDate::parse_from_str(end, "%Y-%m-%d").ok();

        if start_date.unwrap() < Local::now().date_naive(){
            let mut error = ValidationError::new("invalid_date");
            error.message = Some(Borrowed("You cannot provide a past start date."));
            return Err(error);
        }else if end_date.unwrap() < Local::now().date_naive() {
            let mut error = ValidationError::new("invalid_date");
            error.message = Some(Borrowed("You cannot provide a past end date."));
            return Err(error);
        }

        if start_date > end_date{
            let mut error = ValidationError::new("invalid_date_range");
            error.message = Some(Borrowed("Start Date should be before End Date."));
            return Err(error);
        }
    }

    if let Some(Some(score)) = task.priority_score{
        if score < 1 || score > 10{
            let mut error = ValidationError::new("prio_score_range");
            error.message = Some(Borrowed("Priority Score must be between 1 and 10."));
            return Err(error);
        }
    }

    if let Some(Some(status)) = &task.status{
        let s = status.to_uppercase();
        if s != "PENDING" && s != "IN PROGRESS" && s != "COMPLETED"{
            let mut error = ValidationError::new("invalid_status");
            error.message = Some(Borrowed("The provided status valis is invalid. Use Pending, In Progress, or Completed."));
            return Err(error);
        }
    }

    Ok(())
}