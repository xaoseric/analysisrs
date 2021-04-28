table! {
    titanic_analytics (passenger_id) {
        passenger_id -> Integer,
        survived -> Nullable<Integer>,
        pclass -> Nullable<Integer>,
        name -> Nullable<Varchar>,
        sex -> Nullable<Varchar>,
        age -> Nullable<Float>,
        sibsp -> Nullable<Integer>,
        parch -> Nullable<Integer>,
        ticket -> Nullable<Varchar>,
        fare -> Nullable<Float>,
        cabin -> Nullable<Varchar>,
        embarked -> Nullable<Varchar>,
    }
}
