use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Deserializer, Serialize};
use sqlx::prelude::FromRow;
use validator::Validate;
use crate::valid::{validate_date, validate_not_blank, validate_date_range, validate_task_update};

#[derive(Debug, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name="varchar")]
#[allow(non_camel_case_types)]
pub enum TaskStatus{
    #[sqlx(rename="pending")]
    #[serde(rename="Pending")]
    PENDING,
    #[sqlx(rename="in_progress")]
    #[serde(rename="In Progress", alias="in progress",
    alias="IN PROGRESS")]
    IN_PROGRESS,
    #[sqlx(rename="completed")]
    #[serde(rename="Completed")]
    COMPLETED
}

#[derive(Debug, FromRow, Serialize)]
#[allow(unused)]
pub struct Task{
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub priority_score: Option<i32>,
    pub created_at: DateTime<Utc>,
    #[serde(skip_serializing)]
    pub user_id: i32,
    pub status: TaskStatus
}

#[derive(Debug, Deserialize, Validate)]
#[validate(schema(function="validate_date_range"))]
pub struct TaskAdd{
    #[validate(custom(function="validate_not_blank"))]
    pub name: String,
    #[validate(length(max=500, message="Description cannot exceed 500 characters."))]
    pub description: Option<String>,
    #[validate(custom(function="validate_date"))]
    pub start_date: Option<String>,
    #[validate(custom(function="validate_date"))]
    pub end_date: Option<String>,
    #[validate(range(min=1, max=10, message="Priority Score must be between 1 and 10."))]
    pub priority_score: Option<i32>,
}

#[derive(Debug, Deserialize, Validate)]
#[validate(schema(function="validate_task_update"))]
pub struct TaskUpdate{
    #[validate(custom(function="validate_not_blank"))]
    pub name: String,

    #[serde(default, deserialize_with="deserialize_double_option")]
    pub description: Option<Option<String>>,
    
    #[serde(default, deserialize_with="deserialize_double_option")]
    pub start_date: Option<Option<String>>,

    #[serde(default, deserialize_with="deserialize_double_option")]
    pub end_date: Option<Option<String>>,

    #[serde(default, deserialize_with="deserialize_double_option")]
    pub priority_score: Option<Option<i32>>,

    #[serde(default, deserialize_with="deserialize_double_option")]
    pub status: Option<Option<String>>
}

fn deserialize_double_option<'de, T, D>(deserializer: D)
-> Result<Option<Option<T>>, D::Error>
where
    T: Deserialize<'de>,
    D: Deserializer<'de>,
{
    Option::deserialize(deserializer).map(Some)
}

#[derive(Debug, Deserialize)]
pub enum SortBy{
    Name,
    StartDate,
    EndDate,
    PriorityScore,
    CreatedAt,
}
impl SortBy{
    pub fn column(&self) -> &'static str{
        match self{
            SortBy::Name => "name",
            SortBy::StartDate => "start_date",
            SortBy::EndDate => "end_date",
            SortBy::PriorityScore => "priority_score",
            SortBy::CreatedAt => "created_at",
        }
    }
}

#[derive(Debug, Deserialize)]
pub enum SortOrder{
    Asc,
    Desc,
}
impl SortOrder{
    pub fn as_sql(&self) -> &'static str{
        match self{
            SortOrder::Asc => "ASC",
            SortOrder::Desc => "DESC",
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct TaskFilterQuery{
    pub sort_by: Option<SortBy>,
    pub order: Option<SortOrder>,
}