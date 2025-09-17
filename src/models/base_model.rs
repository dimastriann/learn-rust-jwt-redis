use chrono::{DateTime, Local};

#[derive(Debug)]
pub struct Timestamps {
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}