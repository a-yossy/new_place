use async_graphql::{CustomValidator, InputValueError};
use chrono::Local;

use crate::graphql::scalars::date::Date;

pub struct FutureDateValidator;

impl FutureDateValidator {
    pub fn new() -> Self {
        Self
    }
}

impl CustomValidator<Date> for FutureDateValidator {
    fn check(&self, value: &Date) -> Result<(), InputValueError<Date>> {
        let today = Date(Local::now().date_naive());
        if value.gt(&today) {
            Ok(())
        } else {
            Err(InputValueError::custom(format!(
                "please set a future date, actual: {}",
                value.0
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::Duration;

    use super::*;

    #[test]
    fn 日付が明日の場合_エラーにならないこと() {
        let tomorrow = Date(Local::now().date_naive() + Duration::days(1));
        let validator = FutureDateValidator::new();

        let result = validator.check(&tomorrow);

        assert!(result.is_ok());
    }

    #[test]
    fn 日付が今日の場合_エラーになること() {
        let today = Date(Local::now().date_naive());
        let validator = FutureDateValidator::new();

        let result = validator.check(&today);

        assert!(result.is_err());
    }

    #[test]
    fn 日付が昨日の場合_エラーになること() {
        let today = Date(Local::now().date_naive() + Duration::days(-1));
        let validator = FutureDateValidator::new();

        let result = validator.check(&today);

        assert!(result.is_err());
    }
}
