extern crate chrono;

use serde::Deserialize;
use diesel::mysql::MysqlConnection;
use crate::schema::stock_history;
use diesel::RunQueryDsl;

#[derive(Debug, Deserialize)]
pub struct StockData {
    #[serde(alias = "Date")]
    pub date: chrono::NaiveDate,
    #[serde(alias = "High")]
    pub high: f32,
    #[serde(alias = "Low")]
    pub low: f32,
    #[serde(alias = "Open")]
    pub open: f32,
    #[serde(alias = "Close")]
    pub close: f32,
    #[serde(alias = "Volume")]
    pub volume: f32,
    #[serde(alias = "Adj Close")]
    pub adj_close: f32
}

#[derive(Insertable)]
#[table_name="stock_history"]
pub struct StockAnalytic {
    pub ticker: String,
    pub date: chrono::NaiveDate,
    pub high: f32,
    pub low: f32,
    pub open: f32,
    pub close: f32,
    pub volume: f32,
    pub adj_close: f32
}

pub fn create_stock_history(conn: &MysqlConnection, ticker: &str, data: StockData) {
    let stock_data = StockAnalytic {
        ticker: ticker.to_string(),
        date: data.date,
        high: data.high,
        low: data.low,
        open: data.open,
        close: data.close,
        volume: data.volume,
        adj_close: data.adj_close
    };

    diesel::insert_into(stock_history::table)
        .values(stock_data)
        .execute(conn)
        .expect("Could not create new Stock Data record");
}