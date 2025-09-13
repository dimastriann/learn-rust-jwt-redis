use actix_web::{Responder, get, web};

#[get("/")]
async fn hello() -> impl Responder {
    "Hello World!".to_string()
}

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

pub fn hello_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(hello).service(greet);
}
