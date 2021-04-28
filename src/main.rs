#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate csv;
extern crate serde;

pub mod models;
pub mod schema;

use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use dotenv::dotenv;
use std::env;
use std::error::Error;
use models::titanic::{TitanicAnalytic, create_new_titanic_analytic};


/// Establishes the connection to teh database
fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

/// Reads the titanic csv file and inserts the data into MySQL backend
fn read_titanic_file(path: &str, conn: &MysqlConnection) -> Result<(), Box<dyn Error>> {
    // Creates a new csv `Reader` from a file
    let mut reader = csv::Reader::from_path(path)?;


    // `.deserialize` returns an iterator of the internal
    // record structure deserialized
    for result in reader.deserialize() {
        let record: TitanicAnalytic = result?;
        create_new_titanic_analytic(conn, &record);
        //println!("{:?}", record);
    }

    Ok(())
}

/*fn read_cruises_file(path: &str) -> Result<(), Box<dyn Error>> {
    // Creates a new csv `Reader` from a file
    let mut reader = csv::Reader::from_path(path)?;

    // Retrieve headers from file
    let headers = reader.headers();

    // `.deserialize` returns an iterator of the internal
    // record structure deserialized
    /*for result in reader.deserialize() {
        //todo: Read into a struct and insert record into database
    }*/

    Ok(())
}*/

fn main() {
    println!("Connecting to database...");

    // connect to database
    let connection = establish_connection();

    // read data from csv files and insert into database
    println!("Inserting Titanic Analysis Data...");

    // if an error occurs, print the error
    if let Err(e) = read_titanic_file("./data/titanic_analytics.csv", &connection) {
        eprintln!("{}", e);
    }

    println!("Titanic Analytic Data inserted successfully!");
    println!("Inserting Cruises Analysis Data...");
    //read_cruises_file("");
}
