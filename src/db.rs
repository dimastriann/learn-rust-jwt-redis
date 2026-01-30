use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::{PgConnection};
use crate::utils::Config;
use dotenv::dotenv;
use actix_web::{FromRequest, HttpRequest, dev::Payload, web, Error};
use std::future::{Ready, ready};
use crate::models::app_state::AppState;

type DatabasePool = Pool<ConnectionManager<PgConnection>>;
pub type DbPooledConn = PooledConnection<ConnectionManager<PgConnection>>;

pub fn establish_db_connection() -> DatabasePool {
    dotenv().ok();
    let db_url: String = Config::from_env().database_url;
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}

pub fn pooled_conn(db_pool: &DatabasePool) -> Result<DbPooledConn, actix_web::Error> {
    db_pool.get().map_err(|e| {
        log::error!("DB pool error: {:?}", e);
        actix_web::error::ErrorInternalServerError("Failed to get DB connection")
    })
}

pub struct DbConnection(pub DbPooledConn);

impl FromRequest for DbConnection {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let state = req.app_data::<web::Data<AppState>>().expect("AppState not found");
        match pooled_conn(&state.pool) {
            Ok(conn) => ready(Ok(DbConnection(conn))),
            Err(e) => ready(Err(e)),
        }
    }
}
