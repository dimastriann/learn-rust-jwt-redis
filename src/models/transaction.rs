use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct Order {
    pub id: i32,
    pub name: String,
    pub customer: i32,
    pub amount_total: f64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug)]
pub struct OrderDetail {
    pub id: i32,
    pub name: String,
    pub order_id: i32,
    pub product_id: i32,
    pub quantity: f64,
    pub price_unit: f64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug)]
pub struct PaymentOrder {
    pub id: i32,
    pub name: String,
    pub payment_method: i32,
    pub order_id: i32,
    pub amount_paid: f64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
