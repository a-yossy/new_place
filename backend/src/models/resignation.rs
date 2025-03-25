use chrono::{NaiveDate, NaiveDateTime};

pub struct Resignation {
    pub retirement_date: NaiveDate,
    pub remaining_paid_leave_days: u32,
    pub created_at: NaiveDateTime,
}
