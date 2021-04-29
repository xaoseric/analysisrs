use serde::Deserialize;
use diesel::mysql::MysqlConnection;
use crate::schema::cruise_data;
use diesel::RunQueryDsl;

#[derive(Debug, Insertable, Deserialize)]
#[table_name="cruise_data"]
pub struct CruiseData {
    #[serde(alias = "Brand")]
    pub brand: String,
    #[serde(alias = "Total Passengers")]
    pub total_passengers: f32,
    #[serde(alias = "% of Passengers")]
    pub passengers_percentage: f32,
    #[serde(alias = "Revenue")]
    pub revenue: f32,
    #[serde(alias = "% of Revenue")]
    pub revenue_percentage: f32
}

pub fn create_cruise_data(conn: &MysqlConnection, data: CruiseData) {

    diesel::insert_into(cruise_data::table)
        .values(&data)
        .execute(conn)
        .expect("Could not create new Stock Data record");
}