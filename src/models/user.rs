use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::schema::{users, contacts};

#[derive(Debug, Serialize, Queryable, Selectable, Deserialize, Identifiable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub email: String,
    pub full_name: Option<String>,
    pub contact_id: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub password: String,
    pub email: String,
    pub full_name: Option<String>,
    pub contact_id: i32,
}

#[derive(Debug, AsChangeset, Deserialize)]
#[diesel(table_name = users)]
pub struct UpdateUser {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub full_name: Option<String>,
    pub contact_id: i32,
}

#[derive(Debug, Serialize, Queryable, Selectable, Deserialize, Identifiable)]
#[diesel(table_name = contacts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Contact {
    pub id: i32,
    pub name: String,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub mobile: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub zipcode: Option<String>,
    pub country: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = contacts)]
pub struct NewContact {
    pub name: String,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub mobile: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub zipcode: Option<String>,
    pub country: Option<String>,
}

#[derive(Debug, AsChangeset, Deserialize)]
#[diesel(table_name = contacts)]
pub struct UpdateContact {
    pub id: i32,
    pub name: String,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub mobile: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub zipcode: Option<String>,
    pub country: Option<String>,
}
