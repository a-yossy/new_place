use chrono::{FixedOffset, NaiveDateTime, Utc};

pub fn now() -> NaiveDateTime {
    let tokyo_offset = FixedOffset::east_opt(9 * 3600).unwrap();

    Utc::now().with_timezone(&tokyo_offset).naive_local()
}
