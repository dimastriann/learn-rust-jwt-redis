use actix_web::{HttpResponse, Responder, Result, route, web};

#[route("/orders", method = "GET")]
async fn read_orders() -> Result<impl Responder> {
    Ok(HttpResponse::Ok())
}

#[route("/order_by_id/{order_id}", method = "GET")]
async fn read_order_by_id() -> Result<impl Responder> {
    Ok(HttpResponse::Ok())
}

#[route("/create", method = "POST")]
async fn create_order() -> Result<impl Responder> {
    Ok(HttpResponse::Ok())
}

#[route("/update", method = "PUT")]
async fn update_order() -> Result<impl Responder> {
    Ok(HttpResponse::Ok())
}

#[route("/delete", method = "DELETE")]
async fn delete_order() -> Result<impl Responder> {
    Ok(HttpResponse::Ok())
}

pub fn order_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/order")
            .service(read_orders)
            .service(read_order_by_id)
            .service(create_order)
            .service(update_order)
            .service(delete_order),
    );
}
