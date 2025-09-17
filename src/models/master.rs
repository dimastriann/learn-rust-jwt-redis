use crate::models::base_model::Timestamps;

#[derive(Debug)]
pub struct ProductCategory {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    
}

#[derive(Debug)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub sale_price: f64,
    pub cost: f64,
    pub stock: f64,
    pub description: Option<String>,
    pub timestamps: Timestamps
}

pub struct PaymentMethod {
    pub id: i32,
    pub name: String,
    pub method: String,
    pub timestamps: Timestamps
}
