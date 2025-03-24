use async_graphql::{InputObject, Object, Result};
use chrono::Local;

use crate::{
    date::{Date, FutureDateValidator},
    resignation::Resignation,
};

pub struct MutationRoot;

#[derive(InputObject)]
struct PostResignationInput {
    #[graphql(validator(custom = "FutureDateValidator::new(Date(Local::now().date_naive()))"))]
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
