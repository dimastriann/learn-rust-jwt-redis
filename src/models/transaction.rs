use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::schema::{orders, order_details, payment_orders};

#[derive(Debug, Serialize, Queryable, Selectable, Deserialize, Identifiable)]
#[diesel(table_name = orders)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Order {
    pub id: i32,
    pub name: String,
    pub customer_id: i32,
    pub amount_total: f64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = orders)]
pub struct NewOrder {
    pub name: String,
    pub customer_id: i32,
    pub amount_total: f64,
}

#[derive(Debug, Serialize, Queryable, Selectable, Deserialize, Identifiable)]
#[diesel(table_name = order_details)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct OrderDetail {
    pub id: i32,
    pub order_id: i32,
    pub product_id: i32,
    pub quantity: f64,
    pub price_unit: f64,
    pub amount_total: f64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = order_details)]
pub struct NewOrderDetail {
    pub order_id: i32,
    pub product_id: i32,
    pub quantity: f64,
    pub price_unit: f64,
    pub amount_total: f64,
}

#[derive(Debug, Serialize, Queryable, Selectable, Deserialize, Identifiable)]
#[diesel(table_name = payment_orders)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PaymentOrder {
    pub id: i32,
    pub order_id: i32,
    pub payment_method_id: i32,
    pub amount_paid: f64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = payment_orders)]
pub struct NewPaymentOrder {
    pub order_id: i32,
    pub payment_method_id: i32,
    pub amount_paid: f64,
}

#[derive(Debug, Deserialize)]
pub struct CreateOrderRequest {
    pub name: String,
    pub customer_id: i32,
    pub location_id: i32,
    pub items: Vec<OrderItemRequest>,
    pub payments: Vec<OrderPaymentRequest>,
}

#[derive(Debug, Deserialize)]
pub struct OrderItemRequest {
    pub product_id: i32,
    pub quantity: f64,
    pub price_unit: f64,
}

#[derive(Debug, Deserialize)]
pub struct OrderPaymentRequest {
    pub payment_method_id: i32,
    pub amount_paid: f64,
}
