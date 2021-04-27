use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TitanicData {
    #[serde(alias = "PassengerId")]
    pub passenger_id: i32,
    #[serde(alias = "Survived")]
    pub survived: i32,
    #[serde(alias = "Pclass")]
    pub pclass: i32,
    #[serde(alias = "Name")]
    pub name: String,
    #[serde(alias = "Sex")]
    pub sex: String,
    #[serde(alias = "Age")]
    pub age: Option<i32>,
    #[serde(alias = "SibSp")]
    pub sibsp: i32,
    #[serde(alias = "Parch")]
    pub parch: i32,
    #[serde(alias = "Ticket")]
    pub ticket: String,
    #[serde(alias = "Fare")]
    pub fare: i32,
    #[serde(alias = "Cabin")]
    pub cabin: Option<String>,
    #[serde(alias = "Embarked")]
    pub embarked: String
}

/*#[table_name = "titanic_analytics"]
pub struct TitanicAnalytic<'a> {
    pub passenger_id: &'a u32,
    pub survived: Option<&'a u32>,
    pub pclass: Option<&'a u32>,
    pub name: Option<&'a str>,
    pub sex: Option<&'a str>,
    pub age: Option<&'a f32>,
    pub sibsp: Option<&'a u32>,
    pub parch: Option<&'a u32>,
    pub ticket: Option<&'a str>,
    pub fare: Option<&'a f32>,
    pub cabin: Option<&'a str>,
    pub embarked: Option<&'a str>
}
*/


/*pub fn create_titanic_analytic(data: TitanicData) {}*/


