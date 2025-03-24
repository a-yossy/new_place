use async_graphql::SimpleObject;

use crate::scalars::date::Date;

#[derive(SimpleObject)]
pub struct Resignation {
    retirement_date: Date,
    remaining_paid_leave_days: u32,
}

impl Resignation {
    pub fn new(retirement_date: Date, remaining_paid_leave_days: u32) -> Self {
        Resignation {
            retirement_date,
            remaining_paid_leave_days,
        }
    }
}
