use async_graphql::{CustomValidator, InputValueError};

use crate::scalars::date::Date;

pub struct FutureDateValidator {
    date: Date,
}

impl FutureDateValidator {
    pub fn new(date: Date) -> Self {
        Self { date }
    }
}

impl CustomValidator<Date> for FutureDateValidator {
    fn check(&self, value: &Date) -> Result<(), InputValueError<Date>> {
        if value.0.gt(&self.date.0) {
            Ok(())
        } else {
            Err(InputValueError::custom(format!(
                "please set a future date, actual: {}",
                value.0
            )))
        }
    }
}
