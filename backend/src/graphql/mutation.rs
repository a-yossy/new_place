use async_graphql::{Context, ID, InputObject, Object, Result};
use sqlx::{MySql, Pool};

use crate::{
    graphql::objects::resignation::Resignation,
    graphql::scalars::{date::Date, datetime::DateTime},
    validations::date::FutureDateValidator,
};

pub struct MutationRoot;

#[derive(InputObject)]
struct PostResignationInput {
    #[graphql(validator(custom = "FutureDateValidator"))]
    retirement_date: Date,
    remaining_paid_leave_days: u32,
}

#[Object]
impl MutationRoot {
    async fn post_resignation(
        &self,
        ctx: &Context<'_>,
        input: PostResignationInput,
    ) -> Result<Resignation> {
        let pool = ctx.data::<Pool<MySql>>().unwrap();
        let now = DateTime::now();
        let id = sqlx::query!(
            r#"
            INSERT INTO
                resignation (retirement_date, remaining_paid_leave_days, created_at)
            VALUES
                (?, ?, ?)
            "#,
            input.retirement_date.0.to_string(),
            input.remaining_paid_leave_days,
            now.0.format("%Y-%m-%d %H:%M:%S").to_string()
        )
        .execute(pool)
        .await?
        .last_insert_id();

        let resignation = Resignation::new(
            ID(id.to_string()),
            input.retirement_date,
            input.remaining_paid_leave_days,
            now,
        );

        Ok(resignation)
    }
}
