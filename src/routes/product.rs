use crate::models::app_state::AppState;
use crate::models::master::{NewProduct, Product};
use crate::routes::api_response::api_response;
use crate::schema::products::dsl::*;
use actix_web::{HttpRequest, HttpResponse, Responder, Result, get, post, route, web};
use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};
/* PRODUCT ENDPOINT */
#[route("/products", method = "GET")]
async fn read_product(state: web::Data<AppState>) -> Result<impl Responder> {
    let mut conn = state.pool.get().unwrap();
    let result = products
        .select(Product::as_select())
        .load::<Product>(&mut conn);
    api_response(result)
}

#[route("/product/{product_id}", method = "GET")]
async fn read_product_by_id(
    product_id: web::Path<i32>,
    state: web::Data<AppState>,
) -> Result<impl Responder> {
    println!("Getting Product Successfully, {:?}", product_id.to_string());
    let mut conn = state.pool.get().unwrap();
    let product = products
        .find(product_id.into_inner())
        .first::<Product>(&mut conn);
    api_response(product)
}

#[route("/create-product", method = "POST")]
async fn create_product(
    request: HttpRequest,
    payload: web::Json<NewProduct>,
    state: web::Data<AppState>,
) -> Result<impl Responder> {
    println!("{:?}", request);
    let product = payload.into_inner();
    let mut conn = state.pool.get().unwrap();
    let result = diesel::insert_into(products)
        .values(product)
        .returning(Product::as_returning())
        .get_result(&mut conn);
    api_response(result)
}

#[route("/update-product", method = "PUT")]
async fn update_product(
    payload: web::Json<Product>,
    state: web::Data<AppState>,
) -> Result<impl Responder> {
    let mut conn = state.pool.get().unwrap();
    let product = payload.into_inner();
    let result = diesel::update(products.find(product.id))
        .set(product)
        .returning(Product::as_returning())
        .get_result(&mut conn);
    api_response(result)
}

#[route("/delete-product/{product_id}", method = "DELETE")]
async fn delete_product(
    product_id: web::Path<i32>,
    state: web::Data<AppState>,
) -> Result<impl Responder> {
    let mut conn = state.pool.get().unwrap();
    let product = diesel::delete(products.find(*product_id)).get_result::<Product>(&mut conn);
    api_response(product)
}

pub fn product_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(read_product)
        .service(read_product_by_id)
        .service(create_product)
        .service(update_product)
        .service(delete_product);
}

/* PRODUCT CATEGORY ENDPOINT */
#[get("/categories")]
async fn read_categories() -> Result<impl Responder> {
    Ok(HttpResponse::Ok())
}

#[get("/category/{category_id}")]
async fn read_category_by_id() -> Result<impl Responder> {
    Ok(HttpResponse::Ok())
}

#[post("/create-category")]
async fn create_category() -> Result<impl Responder> {
    Ok(HttpResponse::Ok())
}

#[route("/update-category", method = "PUT", method = "POST")]
async fn update_category() -> Result<impl Responder> {
    Ok(HttpResponse::Ok())
}

#[route("/delete-category", method = "DELETE", method = "POST")]
async fn delete_category() -> Result<impl Responder> {
    Ok(HttpResponse::Ok())
}

pub fn category_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(read_categories)
        .service(create_category)
        .service(update_category)
        .service(delete_category);
}
