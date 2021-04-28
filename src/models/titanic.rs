use serde::Deserialize;
use crate::schema;

#[derive(Debug, Queryable, Deserialize)]
pub struct TitanicAnalytic {
    #[serde(alias = "PassengerId")]
    pub passenger_id: u32,
    #[serde(alias = "Survived")]
    pub survived: Option<u32>,
    #[serde(alias = "Pclass")]
    pub pclass: Option<u32>,
    #[serde(alias = "Name")]
    pub name: Option<String>,
    #[serde(alias = "Sex")]
    pub sex: Option<String>,
    #[serde(alias = "Age")]
    pub age: Option<f32>,
    #[serde(alias = "SibSp")]
    pub sibsp: Option<u32>,
    #[serde(alias = "Parch")]
    pub parch: Option<u32>,
    #[serde(alias = "Ticket")]
    pub ticket: Option<String>,
    #[serde(alias = "Fare")]
    pub fare: Option<f32>,
    #[serde(alias = "Cabin")]
    pub cabin: Option<String>,
    #[serde(alias = "Embarked")]
    pub embarked: Option<String>
}