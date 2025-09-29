use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

pub struct AppState {
    pub redis_client: redis::Client,
    pub jwt_secret_key: String,
    pub pool: Pool<ConnectionManager<PgConnection>>,
}
