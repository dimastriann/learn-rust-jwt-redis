use crate::db::DbConnection;
use crate::models::transaction::{CreateOrderRequest};
use crate::routes::api_response::api_response;
use crate::repositories::order_repo::OrderRepository;
use actix_web::{Responder, Result, get, post, route, web};

#[get("/orders")]
async fn read_orders(mut conn: DbConnection) -> Result<impl Responder> {
    let result = OrderRepository::list(&mut conn.0);
    api_response(result)
}

#[get("/{order_id}")]
async fn read_order_by_id(
    order_id: web::Path<i32>,
    mut conn: DbConnection,
) -> Result<impl Responder> {
    let result = OrderRepository::find_by_id(&mut conn.0, order_id.into_inner());
    api_response(result)
}

#[post("/create")]
async fn create_order(
    payload: web::Json<CreateOrderRequest>,
    mut conn: DbConnection,
) -> Result<impl Responder> {
    let result = OrderRepository::create_transactional(&mut conn.0, payload.into_inner());
    api_response(result)
}

#[route("/update", method = "PUT")]
async fn update_order() -> Result<impl Responder> {
    // Orders are usually not updated in this way in a POS, but placeholder remains
    Ok(actix_web::HttpResponse::Ok().body("Order update not implemented"))
}

#[route("/delete/{order_id}", method = "DELETE")]
async fn delete_order(
    order_id: web::Path<i32>,
    mut conn: DbConnection,
) -> Result<impl Responder> {
    let result = OrderRepository::delete(&mut conn.0, order_id.into_inner());
    api_response(result)
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
