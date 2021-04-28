use serde::Deserialize;
use diesel::mysql::MysqlConnection;
use crate::schema::titanic_analytics;
use diesel::RunQueryDsl;

#[derive(Debug, Insertable, Deserialize)]
#[table_name = "titanic_analytics"]
pub struct TitanicAnalytic {
    #[serde(alias = "PassengerId")]
    pub passenger_id: i32,
    #[serde(alias = "Survived")]
    pub survived: Option<i32>,
    #[serde(alias = "Pclass")]
    pub pclass: Option<i32>,
    #[serde(alias = "Name")]
    pub name: Option<String>,
    #[serde(alias = "Sex")]
    pub sex: Option<String>,
    #[serde(alias = "Age")]
    pub age: Option<f32>,
    #[serde(alias = "SibSp")]
    pub sibsp: Option<i32>,
    #[serde(alias = "Parch")]
    pub parch: Option<i32>,
    #[serde(alias = "Ticket")]
    pub ticket: Option<String>,
    #[serde(alias = "Fare")]
    pub fare: Option<f32>,
    #[serde(alias = "Cabin")]
    pub cabin: Option<String>,
    #[serde(alias = "Embarked")]
    pub embarked: Option<String>
}

pub fn create_new_titanic_analytic(conn: &MysqlConnection, data: &TitanicAnalytic) {
    diesel::insert_into(titanic_analytics::table)
        .values(data)
        .execute(conn)
        .expect("Could not create new Titanic Analytic record");
}