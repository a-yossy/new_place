use async_graphql::{InputObject, Object, Result};
use chrono::Utc;

use crate::{
    resignation::Resignation,
    scalars::{date::Date, datetime::DateTime},
    validations::date::FutureDateValidator,
};

pub struct MutationRoot;

#[derive(InputObject)]
struct PostResignationInput {
    #[graphql(validator(custom = "FutureDateValidator::new()"))]
    retirement_date: Date,
    remaining_paid_leave_days: u32,
}

#[Object]
impl MutationRoot {
    async fn post_resignation(&self, input: PostResignationInput) -> Result<Resignation> {
        let now = DateTime(Utc::now());
        let resignation =
            Resignation::new(input.retirement_date, input.remaining_paid_leave_days, now);

        Ok(resignation)
    }
}
