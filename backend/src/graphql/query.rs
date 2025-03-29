use async_graphql::{Context, ID, Object, Result, SimpleObject};
use sqlx::{MySql, Pool};

use crate::{
    graphql::objects::resignation::Resignation as ResignationObject,
    graphql::scalars::{date::Date, datetime::DateTime},
    models::resignation::Resignation as ResignationModel,
};

pub struct QueryRoot;

#[derive(SimpleObject)]
struct Test {
    date: Date,
}

#[Object]
impl QueryRoot {
    async fn latest_resignation(&self, ctx: &Context<'_>) -> Result<ResignationObject> {
        let pool = ctx.data::<Pool<MySql>>().unwrap();
        let resignation = sqlx::query_as!(
            ResignationModel,
            r#"
                SELECT
                    id, retirement_date, remaining_paid_leave_days, created_at
                FROM
                    resignation
                ORDER BY
                    created_at DESC
                LIMIT 1
            "#
        )
        .fetch_one(pool)
        .await?;

        Ok(ResignationObject::new(
            ID(resignation.id.to_string()),
            Date(resignation.retirement_date),
            resignation.remaining_paid_leave_days,
            DateTime(resignation.created_at),
        ))
    }
}
