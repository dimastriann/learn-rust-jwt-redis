use crate::db::pooled_conn;
use crate::models::app_state::AppState;
use crate::models::master::{
    NewProduct, NewProductCategory, Product, ProductCategory, UpdateProduct,
};
use crate::routes::api_response::api_response;
use crate::schema::product_categories::dsl::*;
use crate::schema::products::dsl::*;
use actix_web::{HttpResponse, Responder, Result, get, post, route, web};
use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};
use log::info;
/* PRODUCT ENDPOINT */
#[route("/products", method = "GET")]
async fn read_product(state: web::Data<AppState>) -> Result<impl Responder> {
    let result = products
        .select(Product::as_select())
        .load::<Product>(&mut pooled_conn(&state.pool)?);
    api_response(result)
}

#[route("/{product_id}", method = "GET")]
async fn read_product_by_id(
    product_id: web::Path<i32>,
    state: web::Data<AppState>,
) -> Result<impl Responder> {
    let product = products
        .find(product_id.into_inner())
        .first::<Product>(&mut pooled_conn(&state.pool)?);
    api_response(product)
}

#[route("/create", method = "POST")]
async fn create_product(
    payload: web::Json<Vec<NewProduct>>,
    state: web::Data<AppState>,
) -> Result<impl Responder> {
    let product = payload.into_inner();
    info!("{:?}", product);
    let result = diesel::insert_into(products)
        .values(&product)
        .returning(Product::as_returning())
        .get_results(&mut pooled_conn(&state.pool)?);
    api_response(result)
}

#[route("/update/{product_id}", method = "PUT")]
async fn update_product(
    payload: web::Json<UpdateProduct>,
    state: web::Data<AppState>,
) -> Result<impl Responder> {
    let product = payload.into_inner();
    let result = diesel::update(products.find(product.id))
        .set(&product)
        .returning(Product::as_returning())
        .get_result(&mut pooled_conn(&state.pool)?);
    api_response(result)
}

#[route("/delete/{product_id}", method = "DELETE")]
async fn delete_product(
    product_id: web::Path<i32>,
    state: web::Data<AppState>,
) -> Result<impl Responder> {
    let product = diesel::delete(products.find(*product_id))
        .get_result::<Product>(&mut pooled_conn(&state.pool)?);
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
async fn read_categories(state: web::Data<AppState>) -> Result<impl Responder> {
    let categories = product_categories
        .select(ProductCategory::as_select())
        .load::<ProductCategory>(&mut pooled_conn(&state.pool)?);
    api_response(categories)
}

#[get("/{product_category_id}")]
async fn read_category_by_id(
    product_category_id: web::Path<i32>,
    state: web::Data<AppState>,
) -> Result<impl Responder> {
    let category = product_categories
        .find(product_category_id.into_inner())
        .first::<ProductCategory>(&mut pooled_conn(&state.pool)?);
    api_response(category)
}

#[post("/create")]
async fn create_category(
    payload: web::Json<Vec<NewProductCategory>>,
    state: web::Data<AppState>,
) -> Result<impl Responder> {
    let category = payload.into_inner();
    let result = diesel::insert_into(product_categories)
        .values(&category)
        .returning(ProductCategory::as_returning())
        .get_results(&mut pooled_conn(&state.pool)?);
    api_response(result)
}

#[route("/update", method = "PUT", method = "POST")]
async fn update_category() -> Result<impl Responder> {
    Ok(HttpResponse::Ok())
}

#[route("/delete", method = "DELETE", method = "POST")]
async fn delete_category() -> Result<impl Responder> {
    Ok(HttpResponse::Ok())
}

pub fn category_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/product-category")
            .service(read_categories)
            .service(create_category)
            .service(update_category)
            .service(delete_category),
    );
}
