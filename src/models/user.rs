use crate::models::base_model::Timestamps;

#[derive(Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub email: String,
    pub full_name: Option<String>,
    pub timestamps: Timestamps
}