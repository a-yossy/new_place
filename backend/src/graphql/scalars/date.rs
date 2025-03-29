use async_graphql::{InputValueError, InputValueResult, Scalar, ScalarType, Value};
use chrono::NaiveDate;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use async_graphql::Value;
    use serde_json::Number;

    #[test]
    fn parse_有効な日付の場合_エラーにならないこと() {
        let value = Value::String("2025-01-01".to_string());

        let result = Date::parse(value);

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Date(NaiveDate::from_ymd_opt(2025, 1, 1).unwrap())
        );
    }

    #[test]
    fn parse_無効な日付の場合_エラーになること() {
        let value = Value::String("2025-01-32".to_string());

        let result = Date::parse(value);

        assert!(result.is_err());
    }

    #[test]
    fn parse_文字列以外の場合_エラーになること() {
        let value = Value::Number(Number::from_i128(20250101).unwrap());

        let result = Date::parse(value);

        assert!(result.is_err());
    }

    #[test]
    fn to_value_文字列を返すこと() {
        let date = Date(NaiveDate::from_ymd_opt(2025, 1, 1).unwrap());

        let value = date.to_value();

        assert_eq!(value, Value::String("2025-01-01".to_string()));
    }
}
