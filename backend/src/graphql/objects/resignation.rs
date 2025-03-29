use async_graphql::{ID, SimpleObject};

use crate::graphql::scalars::{date::Date, datetime::DateTime};

#[derive(SimpleObject)]
pub struct Resignation {
    id: ID,
    retirement_date: Date,
    remaining_paid_leave_days: u32,
    created_at: DateTime,
}

impl Resignation {
    pub fn new(
        id: ID,
        retirement_date: Date,
        remaining_paid_leave_days: u32,
        created_at: DateTime,
    ) -> Self {
        Self {
            id,
            retirement_date,
            remaining_paid_leave_days,
            created_at,
        }
    }
}
