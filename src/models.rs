use crate::schema::portfolio_table;
use chrono::NaiveDate;

#[derive(Serialize, Deserialize, Queryable)]
pub struct portfolio_table_view {
    pub id: i32,
}

// #[derive(Clone, Serialize, Deserialize, Insertable)]
// #[table_name = "portfolio_table"]
// pub struct insert_portfolio_table {
// }
