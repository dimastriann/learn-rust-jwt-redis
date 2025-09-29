use std::borrow::Cow;
use chrono::{DateTime, Local};
use serde::Serialize;

#[derive(Debug)]
pub struct Timestamps {
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

#[derive(Serialize)]
pub struct ApiResponse<T> where T: Serialize {
    pub success: bool,
    pub data: T,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
