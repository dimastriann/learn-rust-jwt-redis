use crate::db::DbConnection;
use crate::models::master::{NewUom, NewWarehouse, NewLocation, NewStockQuant};
use crate::routes::api_response::api_response;
use crate::repositories::inventory_repo::InventoryRepository;
use actix_web::{Responder, Result, get, post, web};

#[get("/uoms")]
async fn list_uoms(mut conn: DbConnection) -> Result<impl Responder> {
    let result = InventoryRepository::list_uoms(&mut conn.0);
    api_response(result)
}

#[post("/uom/create")]
async fn create_uom(
    payload: web::Json<NewUom>,
    mut conn: DbConnection,
) -> Result<impl Responder> {
    let result = InventoryRepository::create_uom(&mut conn.0, payload.into_inner());
    api_response(result)
}

#[get("/warehouses")]
async fn list_warehouses(mut conn: DbConnection) -> Result<impl Responder> {
    let result = InventoryRepository::list_warehouses(&mut conn.0);
    api_response(result)
}

#[post("/warehouse/create")]
async fn create_warehouse(
    payload: web::Json<NewWarehouse>,
    mut conn: DbConnection,
) -> Result<impl Responder> {
    let result = InventoryRepository::create_warehouse(&mut conn.0, payload.into_inner());
    api_response(result)
}

#[get("/locations")]
async fn list_locations(mut conn: DbConnection) -> Result<impl Responder> {
    let result = InventoryRepository::list_locations(&mut conn.0);
    api_response(result)
}

#[post("/location/create")]
async fn create_location(
    payload: web::Json<NewLocation>,
    mut conn: DbConnection,
) -> Result<impl Responder> {
    let result = InventoryRepository::create_location(&mut conn.0, payload.into_inner());
    api_response(result)
}

#[get("/stock")]
async fn list_stock(mut conn: DbConnection) -> Result<impl Responder> {
    let result = InventoryRepository::list_stock(&mut conn.0);
    api_response(result)
}

#[post("/stock/update")]
async fn update_stock(
    payload: web::Json<NewStockQuant>,
    mut conn: DbConnection,
) -> Result<impl Responder> {
    let result = InventoryRepository::update_stock(&mut conn.0, payload.into_inner());
    api_response(result)
}

pub fn inventory_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/inventory")
            .service(list_uoms)
            .service(create_uom)
            .service(list_warehouses)
            .service(create_warehouse)
            .service(list_locations)
            .service(create_location)
            .service(list_stock)
            .service(update_stock),
    );
}
