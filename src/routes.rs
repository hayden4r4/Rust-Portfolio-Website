use crate::apps;
use crate::db::DBPool;
use crate::models;
use crate::models::portfolio_table_view; // insert_portfolio_table
use crate::schema;
use diesel::{self, prelude::*};
use schema::portfolio_table::dsl::*;

use rocket::fs::NamedFile;
use rocket::response::{status::Created, Debug};
use rocket::serde::{json::Json, Deserialize, Serialize};
use std::path::{Path, PathBuf};

use serde_json::Value;

type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;

// #[get("/")]
// pub fn index() -> Value {
//     apps::index::index()
// }

#[get("/")]
pub async fn index() -> Option<NamedFile> {
    apps::index::index().await
}

#[get("/project1")]
pub async fn project1() -> Option<NamedFile> {
    apps::project1::project1().await
}

// Returns a vector of jsons for the entire table portfolio_table
#[get("/get_table")]
pub async fn get_portfolio_table(conn: DBPool) -> Result<Json<Vec<portfolio_table_view>>> {
    let result: Json<Vec<portfolio_table_view>> =
        conn.run(move |c| portfolio_table.load(c)).await.map(Json)?;

    Ok(result)
}

// #[post("/insert", data = "<portfolio_table>")]
// pub async fn insert_to_portfolio_table(
//     conn: DBPool,
//     table: Json<Vec<insert_portfolio_table>>,
// ) -> Result<Created<Value>> {
//     let table_clone = table.clone();
//     let result = conn.run(move |c| -> QueryResult<usize>{
//         diesel::insert_into(schema::portfolio_table::table)
//             .values(&table_clone)
//             .execute(c)
//     }).await?;
//     let response: Value = json!(format!("{{ 'columns_inserted': {result} }}"));
//     Ok(Created::new("/insert").body(response))
// }
