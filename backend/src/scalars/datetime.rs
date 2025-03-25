use async_graphql::{InputValueError, InputValueResult, Scalar, ScalarType, Value};
use chrono::{DateTime as ChronoDateTime, Utc};

#[derive(Debug, PartialEq)]
pub struct DateTime(pub ChronoDateTime<Utc>);

#[Scalar]
impl ScalarType for DateTime {
    fn parse(value: Value) -> InputValueResult<Self> {
        if let Value::String(value) = &value {
            let datetime = value
                .parse::<ChronoDateTime<Utc>>()
                .map_err(|e| InputValueError::custom(format!("無効な DateTime: {}", e)))?;
            Ok(DateTime(datetime))
        } else {
            Err(InputValueError::expected_type(value))
        }
    }

    fn to_value(&self) -> Value {
        Value::String(self.0.to_rfc3339())
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;
    use async_graphql::Value;

    #[test]
    fn parse_有効な日時の場合_エラーにならないこと() {
        let value = Value::String("2025-01-01 00:00:00+00:00".to_string());

        let result = DateTime::parse(value);

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            DateTime(ChronoDateTime::from_str("2025-01-01 00:00:00+00:00").unwrap())
        );
    }

    #[test]
    fn parse_無効な日時の場合_エラーになること() {
        let value = Value::String("2025-01-32 00:00:00+00:00".to_string());

        let result = DateTime::parse(value);

        assert!(result.is_err());
    }

    #[test]
    fn to_value_文字列を返すこと() {
        let datetime = DateTime(ChronoDateTime::from_str("2025-01-01 00:00:00+00:00").unwrap());

        let value = datetime.to_value();

        assert_eq!(
            value,
            Value::String("2025-01-01T00:00:00+00:00".to_string())
        );
    }
}
