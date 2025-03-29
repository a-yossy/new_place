use chrono::{NaiveDate, NaiveDateTime};

pub struct Resignation {
    pub id: i32,
    pub retirement_date: NaiveDate,
    pub remaining_paid_leave_days: u32,
    pub created_at: NaiveDateTime,
}
