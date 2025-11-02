use crate::schema::{product_categories, products};
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Queryable, Selectable, AsChangeset, Deserialize)]
#[diesel(table_name = product_categories)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ProductCategory {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = product_categories)]
pub struct NewProductCategory {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Queryable, Selectable, Deserialize, Identifiable)]
#[diesel(table_name = products)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub sale_price: f64,
    pub cost: f64,
    pub stock: f64,
    pub description: Option<String>,
    pub category_id: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = products)]
pub struct NewProduct {
    pub name: String,
    pub sale_price: f64,
    pub cost: f64,
    pub stock: f64,
    pub description: Option<String>,
    pub category_id: i32,
}

#[derive(Debug, AsChangeset, Deserialize)]
#[diesel(table_name = products)]
pub struct UpdateProduct {
    pub id: i32,
    pub name: String,
    pub sale_price: f64,
    pub cost: f64,
    pub stock: f64,
    pub description: Option<String>,
    pub category_id: i32,
}

// #[derive(Debug, Serialize, Queryable, Selectable, AsChangeset, Deserialize)]
pub struct PaymentMethod {
    pub id: i32,
    pub name: String,
    pub code: String,
    pub is_cash: Option<bool>,
    pub is_bank: Option<bool>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
