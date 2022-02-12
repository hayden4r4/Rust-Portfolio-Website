use dotenv::dotenv;
use rocket_sync_db_pools::database;
use rocket_sync_db_pools::diesel::prelude::*;
use rocket_sync_db_pools::diesel::PgConnection;
use std::env;

#[database("portfolio")]
pub struct DBPool(diesel::PgConnection);

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}
