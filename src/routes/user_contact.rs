use crate::db::DbConnection;
use crate::models::user::{NewUser, UpdateUser, NewContact, UpdateContact};
use crate::routes::api_response::api_response;
use crate::repositories::user_repo::{UserRepository, ContactRepository};
use actix_web::{Responder, Result, get, post, route, web};

/* CONTACT Endpoint and Config Routes */
#[get("/contacts")]
async fn read_contact(mut conn: DbConnection) -> Result<impl Responder> {
    let result = ContactRepository::list(&mut conn.0);
    api_response(result)
}

#[get("/{contact_id}")]
async fn read_contact_by_id(
    contact_id: web::Path<i32>,
    mut conn: DbConnection,
) -> Result<impl Responder> {
    let result = ContactRepository::find_by_id(&mut conn.0, contact_id.into_inner());
    api_response(result)
}

#[post("/create")]
async fn create_contact(
    payload: web::Json<Vec<NewContact>>,
    mut conn: DbConnection,
) -> Result<impl Responder> {
    let result = ContactRepository::create(&mut conn.0, payload.into_inner());
    api_response(result)
}

#[route("/update/{contact_id}", method = "PUT")]
async fn update_contact(
    payload: web::Json<UpdateContact>,
    mut conn: DbConnection,
) -> Result<impl Responder> {
    let result = ContactRepository::update(&mut conn.0, payload.into_inner());
    api_response(result)
}

#[route("/delete/{contact_id}", method = "DELETE")]
async fn delete_contact(
    contact_id: web::Path<i32>,
    mut conn: DbConnection,
) -> Result<impl Responder> {
    let result = ContactRepository::delete(&mut conn.0, contact_id.into_inner());
    api_response(result)
}

pub fn contact_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/contact")
            .service(read_contact)
            .service(read_contact_by_id)
            .service(create_contact)
            .service(update_contact)
            .service(delete_contact),
    );
}

/* USER Endpoint and Config Routes */
#[get("/users")]
async fn read_user(mut conn: DbConnection) -> Result<impl Responder> {
    let result = UserRepository::list(&mut conn.0);
    api_response(result)
}

#[get("/{user_id}")]
async fn read_user_by_id(
    user_id: web::Path<i32>,
    mut conn: DbConnection,
) -> Result<impl Responder> {
    let result = UserRepository::find_by_id(&mut conn.0, user_id.into_inner());
    api_response(result)
}

#[post("/create")]
async fn create_user(
    payload: web::Json<NewUser>,
    mut conn: DbConnection,
) -> Result<impl Responder> {
    let result = UserRepository::create(&mut conn.0, payload.into_inner());
    api_response(result)
}

#[route("/update/{user_id}", method = "PUT")]
async fn update_user(
    payload: web::Json<UpdateUser>,
    mut conn: DbConnection,
) -> Result<impl Responder> {
    let result = UserRepository::update(&mut conn.0, payload.into_inner());
    api_response(result)
}

#[route("/delete/{user_id}", method = "DELETE")]
async fn delete_user(
    user_id: web::Path<i32>,
    mut conn: DbConnection,
) -> Result<impl Responder> {
    let result = UserRepository::delete(&mut conn.0, user_id.into_inner());
    api_response(result)
}

pub fn user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/user")
            .service(read_user)
            .service(read_user_by_id)
            .service(create_user)
            .service(update_user)
            .service(delete_user),
    );
}
