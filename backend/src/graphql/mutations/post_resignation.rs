use async_graphql::{Context, ID, InputObject, Object, Result};
use sqlx::{MySql, Pool};

use crate::{
    graphql::{
        objects::resignation::Resignation as ResignationObject,
        scalars::{date::Date, datetime::DateTime},
        validations::date::FutureDateValidator,
    },
    models::resignation::{Resignation as ResignationModel, ResignationInput},
};

#[derive(Default)]
pub struct PostResignationMutation;

#[derive(InputObject)]
struct PostResignationInput {
    #[graphql(validator(custom = "FutureDateValidator"))]
    retirement_date: Date,
    remaining_paid_leave_days: u32,
}

#[Object]
impl PostResignationMutation {
    async fn post_resignation(
        &self,
        ctx: &Context<'_>,
        input: PostResignationInput,
    ) -> Result<ResignationObject> {
        let pool = ctx.data::<Pool<MySql>>()?;
        let resignation_input = ResignationInput {
            retirement_date: input.retirement_date.0,
            remaining_paid_leave_days: input.remaining_paid_leave_days,
        };
        let resignation = ResignationModel::insert(pool, &resignation_input).await?;

        let resignation = ResignationObject::new(
            ID(resignation.id.to_string()),
            Date(resignation.retirement_date),
            input.remaining_paid_leave_days,
            DateTime(resignation.created_at),
        );

        Ok(resignation)
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use anyhow::{Ok, Result};
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use chrono::NaiveDate;
    use graphql_parser::parse_query;
    use http_body_util::BodyExt;
    use serde_json::{Value, json};
    use sqlx::MySqlPool;

    use crate::{models::resignation::Resignation, tests::utils::client::client};

    #[sqlx::test]
    async fn post_resignation_200(pool: MySqlPool) -> Result<()> {
        let resignations = sqlx::query_as!(
            Resignation,
            "SELECT id, created_at, remaining_paid_leave_days, retirement_date FROM resignation"
        )
        .fetch_all(&pool)
        .await?;
        assert_eq!(resignations.len(), 0);
        let (addr, client) = client(pool.clone()).await;
        let query =
            parse_query::<String>(&fs::read_to_string("graphql/mutations/resignation.gql")?)?
                .to_string();
        let variables = json!({
            "input": {
                "retirementDate": "9999-01-01",
                "remainingPaidLeaveDays": 10
            }
        });

        let response = client
            .request(
                Request::builder()
                    .method("POST")
                    .uri(format!("http://{addr}/graphql"))
                    .header("Host", "localhost")
                    .header("Content-Type", "application/json")
                    .body(Body::from(
                        json!({
                            "query": query,
                            "variables": variables
                        })
                        .to_string(),
                    ))?,
            )
            .await?;

        assert_eq!(response.status(), StatusCode::OK);
        let bytes = response.into_body().collect().await?.to_bytes();
        let body: Value = serde_json::from_slice(&bytes)?;
        assert!(body.is_object());
        assert!(body.get("errors").is_none());
        let data = &body["data"];
        assert!(data.is_object());
        let resignation = &data["postResignation"];
        assert!(resignation.get("id").unwrap().is_string());
        assert_eq!(
            *resignation.get("remainingPaidLeaveDays").unwrap(),
            json!(10)
        );
        assert_eq!(
            *resignation.get("retirementDate").unwrap(),
            json!("9999-01-01")
        );
        assert!(resignation.get("createdAt").unwrap().is_string());
        let resignations = sqlx::query_as!(
            Resignation,
            "SELECT id, created_at, remaining_paid_leave_days, retirement_date FROM resignation"
        )
        .fetch_all(&pool)
        .await?;
        assert_eq!(resignations.len(), 1);
        assert_eq!(resignations.first().unwrap().remaining_paid_leave_days, 10);
        assert_eq!(
            resignations.first().unwrap().retirement_date,
            NaiveDate::from_ymd_opt(9999, 1, 1).unwrap()
        );

        Ok(())
    }

    #[sqlx::test]
    async fn post_resignation_200_error(pool: MySqlPool) -> Result<()> {
        let resignations = sqlx::query_as!(
            Resignation,
            "SELECT id, created_at, remaining_paid_leave_days, retirement_date FROM resignation"
        )
        .fetch_all(&pool)
        .await?;
        assert_eq!(resignations.len(), 0);
        let (addr, client) = client(pool.clone()).await;
        let query =
            parse_query::<String>(&fs::read_to_string("graphql/mutations/resignation.gql")?)?
                .to_string();
        let variables = json!({
            "input": {
                "retirementDate": "2000-01-01",
                "remainingPaidLeaveDays": 10
            }
        });

        let response = client
            .request(
                Request::builder()
                    .method("POST")
                    .uri(format!("http://{addr}/graphql"))
                    .header("Host", "localhost")
                    .header("Content-Type", "application/json")
                    .body(Body::from(
                        json!({
                            "query": query,
                            "variables": variables
                        })
                        .to_string(),
                    ))?,
            )
            .await?;

        assert_eq!(response.status(), StatusCode::OK);
        let bytes = response.into_body().collect().await?.to_bytes();
        let body: Value = serde_json::from_slice(&bytes)?;
        assert!(body.is_object());
        let data = &body["data"];
        assert!(data.is_null());
        let errors = &body["errors"];
        assert!(errors.is_array());
        assert!(errors.as_array().iter().len() > 0);
        let resignations = sqlx::query_as!(
            Resignation,
            "SELECT id, created_at, remaining_paid_leave_days, retirement_date FROM resignation"
        )
        .fetch_all(&pool)
        .await?;
        assert_eq!(resignations.len(), 0);

        Ok(())
    }
}
