use async_graphql::{Context, Error, Object, Result};
use chrono::{Datelike, Days, Weekday};
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
        let latest_resignation = ResignationModel::fetch_latest(pool).await?;
        let mut vacation_start_date = latest_resignation.retirement_date;
        let mut remaining_paid_leave_days = latest_resignation.remaining_paid_leave_days;

        if remaining_paid_leave_days == 0 {
            return Err(Error::new("有給がありません"));
        }

        remaining_paid_leave_days -= 1;

        while remaining_paid_leave_days > 0 {
            let new_date = vacation_start_date
                .checked_sub_days(Days::new(1))
                .ok_or("invalid date")?;
            vacation_start_date = new_date;
            if holidays.contains_key(&new_date.to_string()) {
                continue;
            }
            if new_date.weekday() == Weekday::Sat {
                continue;
            }
            if new_date.weekday() == Weekday::Sun {
                continue;
            }

            remaining_paid_leave_days -= 1;
        }

        Ok(Date(vacation_start_date))
    }
}
