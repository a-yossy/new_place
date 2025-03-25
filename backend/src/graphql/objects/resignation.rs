use async_graphql::SimpleObject;

use crate::graphql::scalars::{date::Date, datetime::DateTime};

#[derive(SimpleObject)]
pub struct Resignation {
    retirement_date: Date,
    remaining_paid_leave_days: u32,
    created_at: DateTime,
}

impl Resignation {
    pub fn new(
        retirement_date: Date,
        remaining_paid_leave_days: u32,
        created_at: DateTime,
    ) -> Self {
        Self {
            retirement_date,
            remaining_paid_leave_days,
            created_at,
        }
    }
}
