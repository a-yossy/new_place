use async_graphql::{InputObject, Object, Result};

use crate::{
    resignation::Resignation, scalars::date::Date, validations::date::FutureDateValidator,
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
        let resignation = Resignation::new(input.retirement_date, input.remaining_paid_leave_days);

        Ok(resignation)
    }
}
