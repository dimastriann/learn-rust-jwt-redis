use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::{PgConnection};
use crate::utils::Config;
use dotenv::dotenv;

type DatabasePool = Pool<ConnectionManager<PgConnection>>;
pub fn establish_db_connection() -> DatabasePool {
    dotenv().ok();
    let db_url: String = Config::from_env().database_url;
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}

pub fn pooled_conn(db_pool: &DatabasePool) -> Result<PooledConnection<ConnectionManager<PgConnection>>, actix_web::Error> {
    db_pool.get().map_err(|e| {
        log::error!("DB pool error: {:?}", e);
        actix_web::error::ErrorInternalServerError("Failed to get DB connection")
    })
}
