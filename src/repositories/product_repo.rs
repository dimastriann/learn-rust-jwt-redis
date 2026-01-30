use diesel::prelude::*;
use crate::schema::{products, product_categories, payment_methods};
use crate::models::master::{
    Product, NewProduct, UpdateProduct, 
    ProductCategory, NewProductCategory, UpdateProductCategory,
    PaymentMethod, NewPaymentMethod, UpdatePaymentMethod
};
use crate::db::DbPooledConn as DbConn;

pub struct ProductRepository;

impl ProductRepository {
    pub fn list(conn: &mut DbConn) -> QueryResult<Vec<Product>> {
        products::table.select(Product::as_select()).load(conn)
    }

    pub fn find_by_id(conn: &mut DbConn, id: i32) -> QueryResult<Product> {
        products::table.find(id).first(conn)
    }

    pub fn create(conn: &mut DbConn, new_products: Vec<NewProduct>) -> QueryResult<Vec<Product>> {
        diesel::insert_into(products::table)
            .values(&new_products)
            .returning(Product::as_returning())
            .get_results(conn)
    }

    pub fn update(conn: &mut DbConn, product: UpdateProduct) -> QueryResult<Product> {
        diesel::update(products::table.find(product.id))
            .set(&product)
            .returning(Product::as_returning())
            .get_result(conn)
    }

    pub fn delete(conn: &mut DbConn, id: i32) -> QueryResult<Product> {
        diesel::delete(products::table.find(id))
            .get_result(conn)
    }
}

pub struct CategoryRepository;

impl CategoryRepository {
    pub fn list(conn: &mut DbConn) -> QueryResult<Vec<ProductCategory>> {
        product_categories::table.select(ProductCategory::as_select()).load(conn)
    }

    pub fn find_by_id(conn: &mut DbConn, id: i32) -> QueryResult<ProductCategory> {
        product_categories::table.find(id).first(conn)
    }

    pub fn create(conn: &mut DbConn, new_categories: Vec<NewProductCategory>) -> QueryResult<Vec<ProductCategory>> {
        diesel::insert_into(product_categories::table)
            .values(&new_categories)
            .returning(ProductCategory::as_returning())
            .get_results(conn)
    }

    pub fn update(conn: &mut DbConn, category: UpdateProductCategory) -> QueryResult<ProductCategory> {
        diesel::update(product_categories::table.find(category.id))
            .set(&category)
            .returning(ProductCategory::as_returning())
            .get_result(conn)
    }

    pub fn delete(conn: &mut DbConn, id: i32) -> QueryResult<ProductCategory> {
        diesel::delete(product_categories::table.find(id))
            .get_result(conn)
    }
}

pub struct PaymentRepository;

impl PaymentRepository {
    pub fn list(conn: &mut DbConn) -> QueryResult<Vec<PaymentMethod>> {
        payment_methods::table.select(PaymentMethod::as_select()).load(conn)
    }

    pub fn find_by_id(conn: &mut DbConn, id: i32) -> QueryResult<PaymentMethod> {
        payment_methods::table.find(id).first(conn)
    }

    pub fn create(conn: &mut DbConn, new_payment: NewPaymentMethod) -> QueryResult<PaymentMethod> {
        diesel::insert_into(payment_methods::table)
            .values(&new_payment)
            .returning(PaymentMethod::as_returning())
            .get_result(conn)
    }

    pub fn update(conn: &mut DbConn, payment: UpdatePaymentMethod) -> QueryResult<PaymentMethod> {
        diesel::update(payment_methods::table.find(payment.id))
            .set(&payment)
            .returning(PaymentMethod::as_returning())
            .get_result(conn)
    }

    pub fn delete(conn: &mut DbConn, id: i32) -> QueryResult<PaymentMethod> {
        diesel::delete(payment_methods::table.find(id))
            .get_result(conn)
    }
}
