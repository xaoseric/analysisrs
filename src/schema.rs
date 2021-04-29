table! {
    cruise_data (id) {
        id -> Unsigned<Integer>,
        brand -> Varchar,
        total_passengers -> Float,
        passengers_percentage -> Float,
        revenue -> Float,
        revenue_percentage -> Float,
    }
}

table! {
    stock_history (id) {
        id -> Unsigned<Integer>,
        ticker -> Varchar,
        date -> Date,
        high -> Float,
        low -> Float,
        open -> Float,
        close -> Float,
        volume -> Float,
        adj_close -> Float,
    }
}

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

allow_tables_to_appear_in_same_query!(
    cruise_data,
    stock_history,
    titanic_analytics,
);
