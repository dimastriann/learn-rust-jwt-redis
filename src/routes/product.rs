use crate::db::DbConnection;
use crate::models::master::{
    NewProduct, NewProductCategory, UpdateProduct, UpdateProductCategory,
    NewPaymentMethod, UpdatePaymentMethod
};
use crate::routes::api_response::api_response;
use crate::repositories::product_repo::{ProductRepository, CategoryRepository, PaymentRepository};
use actix_web::{Responder, Result, get, post, route, web};

/* PRODUCT ENDPOINT */
#[route("/products", method = "GET")]
async fn read_product(mut conn: DbConnection) -> Result<impl Responder> {
    let result = ProductRepository::list(&mut conn.0);
    api_response(result)
}

#[route("/{product_id}", method = "GET")]
async fn read_product_by_id(
    product_id: web::Path<i32>,
    mut conn: DbConnection,
) -> Result<impl Responder> {
    let product = ProductRepository::find_by_id(&mut conn.0, product_id.into_inner());
    api_response(product)
}

#[route("/create", method = "POST")]
async fn create_product(
    payload: web::Json<Vec<NewProduct>>,
    mut conn: DbConnection,
) -> Result<impl Responder> {
    let result = ProductRepository::create(&mut conn.0, payload.into_inner());
    api_response(result)
}

#[route("/update/{product_id}", method = "PUT")]
async fn update_product(
    payload: web::Json<UpdateProduct>,
    mut conn: DbConnection,
) -> Result<impl Responder> {
    let result = ProductRepository::update(&mut conn.0, payload.into_inner());
    api_response(result)
}

#[route("/delete/{product_id}", method = "DELETE")]
async fn delete_product(
    product_id: web::Path<i32>,
    mut conn: DbConnection,
) -> Result<impl Responder> {
    let product = ProductRepository::delete(&mut conn.0, product_id.into_inner());
    api_response(product)
}

pub fn product_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/product")
            .service(read_product)
            .service(read_product_by_id)
            .service(create_product)
            .service(update_product)
            .service(delete_product),
    );
}

/* PRODUCT CATEGORY ENDPOINT */
#[get("/categories")]
async fn read_categories(mut conn: DbConnection) -> Result<impl Responder> {
    let categories = CategoryRepository::list(&mut conn.0);
    api_response(categories)
}

#[get("/{product_category_id}")]
async fn read_category_by_id(
    product_category_id: web::Path<i32>,
    mut conn: DbConnection,
) -> Result<impl Responder> {
    let category = CategoryRepository::find_by_id(&mut conn.0, product_category_id.into_inner());
    api_response(category)
}

#[post("/create")]
async fn create_category(
    payload: web::Json<Vec<NewProductCategory>>,
    mut conn: DbConnection,
) -> Result<impl Responder> {
    let result = CategoryRepository::create(&mut conn.0, payload.into_inner());
    api_response(result)
}

#[route("/update/{category_id}", method = "PUT")]
async fn update_category(
    payload: web::Json<UpdateProductCategory>,
    mut conn: DbConnection,
) -> Result<impl Responder> {
    let result = CategoryRepository::update(&mut conn.0, payload.into_inner());
    api_response(result)
}

#[route("/delete/{category_id}", method = "DELETE")]
async fn delete_category(
    category_id: web::Path<i32>,
    mut conn: DbConnection,
) -> Result<impl Responder> {
    let result = CategoryRepository::delete(&mut conn.0, category_id.into_inner());
    api_response(result)
}

pub fn category_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/product-category")
            .service(read_categories)
            .service(read_category_by_id)
            .service(create_category)
            .service(update_category)
            .service(delete_category),
    );
}

/* PAYMENT METHOD ENDPOINT */
#[get("/methods")]
async fn read_payments(mut conn: DbConnection) -> Result<impl Responder> {
    let result = PaymentRepository::list(&mut conn.0);
    api_response(result)
}

#[get("/{payment_id}")]
async fn read_payment_by_id(
    payment_id: web::Path<i32>,
    mut conn: DbConnection,
) -> Result<impl Responder> {
    let result = PaymentRepository::find_by_id(&mut conn.0, payment_id.into_inner());
    api_response(result)
}

#[post("/create")]
async fn create_payment(
    payload: web::Json<NewPaymentMethod>,
    mut conn: DbConnection,
) -> Result<impl Responder> {
    let result = PaymentRepository::create(&mut conn.0, payload.into_inner());
    api_response(result)
}

#[route("/update/{payment_id}", method = "PUT")]
async fn update_payment(
    payload: web::Json<UpdatePaymentMethod>,
    mut conn: DbConnection,
) -> Result<impl Responder> {
    let result = PaymentRepository::update(&mut conn.0, payload.into_inner());
    api_response(result)
}

#[route("/delete/{payment_id}", method = "DELETE")]
async fn delete_payment(
    payment_id: web::Path<i32>,
    mut conn: DbConnection,
) -> Result<impl Responder> {
    let result = PaymentRepository::delete(&mut conn.0, payment_id.into_inner());
    api_response(result)
}

pub fn payment_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/payment")
            .service(read_payments)
            .service(read_payment_by_id)
            .service(create_payment)
            .service(update_payment)
            .service(delete_payment),
    );
}
