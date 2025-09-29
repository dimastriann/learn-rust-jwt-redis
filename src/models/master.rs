use serde::{Deserialize, Serialize};
use crate::models::base_model::Timestamps;
use diesel::prelude::*;
use crate::schema::{products};

#[derive(Debug)]
pub struct ProductCategory {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    
}

#[derive(Debug, Serialize, Queryable, Selectable, AsChangeset, Deserialize)]
#[diesel(table_name = products)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub sale_price: f64,
    pub cost: f64,
    pub stock: f64,
    pub description: Option<String>,
    // pub timestamps: Timestamps
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = products)]
pub struct NewProduct {
    pub name: String,
    pub sale_price: f64,
    pub cost: f64,
    pub stock: f64,
    pub description: Option<String>,
}

pub struct PaymentMethod {
    pub id: i32,
    pub name: String,
    pub method: String,
    pub timestamps: Timestamps
}

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
}
