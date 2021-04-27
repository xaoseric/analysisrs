use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TitanicData {
    #[serde(alias = "PassengerId")]
    pub passenger_id: u32,
    #[serde(alias = "Survived")]
    pub survived: u32,
    #[serde(alias = "Pclass")]
    pub pclass: u32,
    #[serde(alias = "Name")]
    pub name: String,
    #[serde(alias = "Sex")]
    pub sex: String,
    #[serde(alias = "Age")]
    pub age: Option<f32>,
    #[serde(alias = "SibSp")]
    pub sibsp: u32,
    #[serde(alias = "Parch")]
    pub parch: u32,
    #[serde(alias = "Ticket")]
    pub ticket: String,
    #[serde(alias = "Fare")]
    pub fare: f32,
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



pub fn create_titanic_analytic(conn: &MysqlConnection, data: TitanicData) -> TitanicAnalytic {
    let new_titanic_data = TitanicAnalytic {
        passenger_id: &data.passenger_id,
        survived: Option::from(data.survived),
        pclass: Option::from(data.pclass),
        name: Option::from(data.name),
        sex: Option::from(data.sex),
        age: &data.age,
        sibsp: Option::from(data.sibsp),
        parch: Option::from(data.parch),
        ticket: Option::from(data.ticket),
        fare: Option::from(data.fare),
        cabin: &data.cabin,
        embarked: Option::from(data.embarked)
    };

    diesel::insert_into(titanic_analytics::table)
        .values(&new_titanic_data)
        .expect("Error saving new titanic analytic data")
}*/


