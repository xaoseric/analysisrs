extern crate diesel;
extern crate dotenv;
extern crate csv;
extern crate serde;

use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use dotenv::dotenv;
use std::env;
use std::error::Error;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct TitanicAnalytic {
    #[serde(alias = "PassengerId")]
    passenger_id: u32,
    #[serde(alias = "Survived")]
    survived: u32,
    #[serde(alias = "Pclass")]
    pclass: u32,
    #[serde(alias = "Name")]
    name: String,
    #[serde(alias = "Sex")]
    sex: String,
    #[serde(alias = "Age")]
    age: Option<f32>,
    #[serde(alias = "SibSp")]
    sibsp: u32,
    #[serde(alias = "Parch")]
    parch: u32,
    #[serde(alias = "Ticket")]
    ticket: String,
    #[serde(alias = "Fare")]
    fare: f32,
    #[serde(alias = "Cabin")]
    cabin: Option<String>,
    #[serde(alias = "Embarked")]
    embarked: String
}

/// Establishes the connection to teh database
fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

fn read_titanic_file(path: &str) -> Result<(), Box<dyn Error>> {
    // Creates a new csv `Reader` from a file
    let mut reader = csv::Reader::from_path(path)?;

    // Retrieve headers from file
    let headers = reader.headers();

    // `.deserialize` returns an iterator of the internal
    // record structure deserialized
    for result in reader.deserialize() {
        let record: TitanicAnalytic = result?;
        println!("{:?}", record);
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
    establish_connection();

    // read data from csv files and insert into database
    println!("Inserting Titanic Analysis Data...");

    // if an error occurs, print the error
    if let Err(e) = read_titanic_file("./data/titanic_analytics.csv") {
        eprintln!("{}", e);
    }

    println!("Inserting Cruises Analysis Data...");
    //read_cruises_file("");
}
