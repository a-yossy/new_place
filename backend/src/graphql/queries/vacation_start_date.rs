use async_graphql::{Context, Object, Result};
use sqlx::{MySql, Pool};

use crate::{
    clients::holidays::fetch_holidays, graphql::scalars::date::Date,
    models::resignation::Resignation as ResignationModel,
};

#[derive(Default)]
pub struct VacationStartDateQuery;

#[Object]
impl VacationStartDateQuery {
    async fn vacation_start_date(&self, ctx: &Context<'_>) -> Result<Date> {
        let holidays = fetch_holidays().await?;
        let pool = ctx.data::<Pool<MySql>>().unwrap();
        let vacation_start_date = ResignationModel::fetch_latest(pool)
            .await?
            .vacation_start_date(&holidays)
            .await?;

        Ok(Date(vacation_start_date))
    }
}
