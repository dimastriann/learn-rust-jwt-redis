use diesel::r2d2::{ConnectionManager, Pool};
use diesel::{PgConnection};
use dotenv::dotenv;
use std::env;

type DatabasePool = Pool<ConnectionManager<PgConnection>>;
pub fn establish_db_connection() -> DatabasePool {
    dotenv().ok();
    let db_url: String = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}
