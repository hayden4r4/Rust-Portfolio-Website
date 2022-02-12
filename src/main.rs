#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_json;
extern crate chrono;
extern crate dotenv;

pub use diesel::prelude::*;
pub use rocket::fs::{relative, FileServer};
pub use rocket_sync_db_pools::database;

pub use serde::{Deserialize, Serialize};
pub use serde_json::{json, Value};

pub mod apps;
pub mod db;
pub mod models;
pub mod routes;
pub mod schema;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/",
            routes![routes::get_portfolio_table, routes::index, routes::project1],
        )
        // routes::insert_to_portfolio_table
        .mount("/", FileServer::from(relative!("static")))
        .attach(db::DBPool::fairing())
}
