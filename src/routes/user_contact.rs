use actix_web::{Error, HttpResponse, Responder, get, web};

/* CONTACT Endpoint and Config Routes */
#[get("/contacts")]
async fn read_contact() -> Result<impl Responder, Error> {
    Ok(HttpResponse::Ok().body("Contact list"))
}

#[get("/contact_by_id/{contact_id}")]
async fn read_contact_by_id() -> Result<impl Responder, Error> {
    Ok(HttpResponse::Ok().body("Contact By ID"))
}

pub fn contact_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/contact")
            .service(read_contact)
            .service(read_contact_by_id),
    );
}

/* USER Endpoint and Config Routes */
#[get("/users")]
async fn read_user() -> Result<impl Responder, Error> {
    Ok(HttpResponse::Ok().body("User list"))
}

#[get("/user_by_id/{user_id}")]
async fn read_user_by_id() -> Result<impl Responder, Error> {
    Ok(HttpResponse::Ok().body("User By ID"))
}

pub fn user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/user")
            .service(read_user)
            .service(read_user_by_id),
    );
}
