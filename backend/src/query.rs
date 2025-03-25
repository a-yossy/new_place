use async_graphql::{Object, SimpleObject};
use chrono::{NaiveDate, Utc};

use crate::{
    resignation::Resignation,
    scalars::{date::Date, datetime::DateTime},
};

pub struct QueryRoot;

#[derive(SimpleObject)]
struct Test {
    date: Date,
}

#[Object]
impl QueryRoot {
    async fn resignation(&self) -> Resignation {
        Resignation::new(
            Date(NaiveDate::from_ymd_opt(2025, 4, 30).unwrap()),
            40,
            DateTime(Utc::now().into()),
        )
    }
}
