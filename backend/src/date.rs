use async_graphql::{
    CustomValidator, InputValueError, InputValueResult, Scalar, ScalarType, Value,
};
use chrono::NaiveDate;

pub struct Date(pub NaiveDate);

#[Scalar]
impl ScalarType for Date {
    fn parse(value: Value) -> InputValueResult<Self> {
        if let Value::String(value) = &value {
            let date = value
                .parse::<NaiveDate>()
                .map_err(|e| InputValueError::custom(format!("無効な Date: {}", e)))?;
            Ok(Date(date))
        } else {
            Err(InputValueError::expected_type(value))
        }
    }

    fn to_value(&self) -> Value {
        Value::String(self.0.to_string())
    }
}

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
