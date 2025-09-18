use actix_web::{route, web, HttpResponse, Responder, Result, HttpRequest, get, post};
use crate::models::master::{Product};

/* PRODUCT ENDPOINT */
#[route("/products", method="GET")]
async fn read_product(request: HttpRequest) -> Result<impl Responder> {
    Ok(HttpResponse::Ok().json(vec![{}]))
}

#[route("/product/{product_id}", method="GET")]
async fn read_product_by_id(product_id: web::Path<String>) -> Result<impl Responder> {
    println!("Getting Product Successfully, {:?}", product_id.to_string());
    let product = Product {
        id: 1,
        name: "Coffee".to_string(),
        sale_price: 150000.0,
        cost: 5000.0,
        stock: 500.0,
        description: Option::from("".to_string()),
    };
    Ok(HttpResponse::Ok().json(product))
}

#[route("/create-product", method="POST")]
async fn create_product(request: HttpRequest) -> Result<impl Responder> {
    println!("{:?}", request);
    Ok(HttpResponse::Ok().body("Product created"))
}

#[route("/update-product", method="PUT")]
async fn update_product() -> Result<impl Responder> {
    Ok(HttpResponse::Ok().body("Product updated"))
}

#[route("/delete-product", method="DELETE")]
async fn delete_product() -> Result<impl Responder> {
    Ok(HttpResponse::Ok().body("Product Deleted"))
}

pub fn product_routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(read_product)
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

#[route("/update-category", method="PUT", method="POST")]
async fn update_category() -> Result<impl Responder> {
    Ok(HttpResponse::Ok())
}

#[route("/delete-category", method="DELETE", method="POST")]
async fn delete_category() -> Result<impl Responder> {
    Ok(HttpResponse::Ok())
}

pub fn category_routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(read_categories)
        .service(create_category)
        .service(update_category)
        .service(delete_category);
}