use crate::schema::{product_categories, products, payment_methods, uoms, warehouses, locations, stock_quants};
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

#[derive(Debug, AsChangeset, Deserialize)]
#[diesel(table_name = product_categories)]
pub struct UpdateProductCategory {
    pub id: i32,
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
    pub description: Option<String>,
    pub category_id: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub sku: Option<String>,
    pub uom_id: Option<i32>,
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = products)]
pub struct NewProduct {
    pub name: String,
    pub sale_price: f64,
    pub cost: f64,
    pub description: Option<String>,
    pub category_id: i32,
    pub sku: Option<String>,
    pub uom_id: Option<i32>,
}

#[derive(Debug, AsChangeset, Deserialize)]
#[diesel(table_name = products)]
pub struct UpdateProduct {
    pub id: i32,
    pub name: String,
    pub sale_price: f64,
    pub cost: f64,
    pub description: Option<String>,
    pub category_id: i32,
    pub sku: Option<String>,
    pub uom_id: Option<i32>,
}

#[derive(Debug, Serialize, Queryable, Selectable, Deserialize, Identifiable)]
#[diesel(table_name = uoms)]
pub struct Uom {
    pub id: i32,
    pub name: String,
    pub code: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = uoms)]
pub struct NewUom {
    pub name: String,
    pub code: String,
}

#[derive(Debug, Serialize, Queryable, Selectable, Deserialize, Identifiable)]
#[diesel(table_name = warehouses)]
pub struct Warehouse {
    pub id: i32,
    pub name: String,
    pub code: String,
    pub address: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = warehouses)]
pub struct NewWarehouse {
    pub name: String,
    pub code: String,
    pub address: Option<String>,
}

#[derive(Debug, Serialize, Queryable, Selectable, Deserialize, Identifiable)]
#[diesel(table_name = locations)]
#[diesel(belongs_to(Warehouse))]
pub struct Location {
    pub id: i32,
    pub name: String,
    pub warehouse_id: i32,
    pub is_scrap: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = locations)]
pub struct NewLocation {
    pub name: String,
    pub warehouse_id: i32,
    pub is_scrap: bool,
}

#[derive(Debug, Serialize, Queryable, Selectable, Deserialize, Identifiable)]
#[diesel(table_name = stock_quants)]
pub struct StockQuant {
    pub id: i32,
    pub product_id: i32,
    pub location_id: i32,
    pub quantity: f64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = stock_quants)]
pub struct NewStockQuant {
    pub product_id: i32,
    pub location_id: i32,
    pub quantity: f64,
}

/* OTHER MODELS */
#[derive(Debug, Serialize, Queryable, Selectable, Deserialize, Identifiable, AsChangeset)]
#[diesel(table_name = payment_methods)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PaymentMethod {
    pub id: i32,
    pub name: String,
    pub code: String,
    pub is_cash: Option<bool>,
    pub is_bank: Option<bool>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = payment_methods)]
pub struct NewPaymentMethod {
    pub name: String,
    pub code: String,
    pub is_cash: Option<bool>,
    pub is_bank: Option<bool>,
}

#[derive(Debug, AsChangeset, Deserialize)]
#[diesel(table_name = payment_methods)]
pub struct UpdatePaymentMethod {
    pub id: i32,
    pub name: String,
    pub code: String,
    pub is_cash: Option<bool>,
    pub is_bank: Option<bool>,
}
