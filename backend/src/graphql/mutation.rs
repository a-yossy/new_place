use async_graphql::{Context, ID, InputObject, Object, Result};
use sqlx::{MySql, Pool};

use crate::{
    graphql::{
        objects::resignation::Resignation as ResignationObject,
        scalars::{date::Date, datetime::DateTime},
    },
    models::resignation::{Resignation as ResignationModel, ResignationInput},
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
    ) -> Result<ResignationObject> {
        let pool = ctx.data::<Pool<MySql>>().unwrap();
        let resignation_input = ResignationInput {
            retirement_date: input.retirement_date.0,
            remaining_paid_leave_days: input.remaining_paid_leave_days,
        };
        let resignation = ResignationModel::insert(pool, &resignation_input).await?;

        let resignation = ResignationObject::new(
            ID(resignation.id.to_string()),
            Date(resignation.retirement_date),
            input.remaining_paid_leave_days,
            DateTime(resignation.created_at),
        );

        Ok(resignation)
    }
}
