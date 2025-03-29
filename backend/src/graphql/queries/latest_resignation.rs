use async_graphql::{Context, ID, Object, Result};
use sqlx::{MySql, Pool};

use crate::{
    graphql::{
        objects::resignation::Resignation as ResignationObject,
        scalars::{date::Date, datetime::DateTime},
    },
    models::resignation::Resignation as ResignationModel,
};

#[derive(Default)]
pub struct LatestResignationQuery;

#[Object]
impl LatestResignationQuery {
    async fn latest_resignation(&self, ctx: &Context<'_>) -> Result<ResignationObject> {
        let pool = ctx.data::<Pool<MySql>>().unwrap();
        let latest_resignation = ResignationModel::fetch_latest(pool).await?;

        Ok(ResignationObject::new(
            ID(latest_resignation.id.to_string()),
            Date(latest_resignation.retirement_date),
            latest_resignation.remaining_paid_leave_days,
            DateTime(latest_resignation.created_at),
        ))
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use graphql_parser::parse_query;
    use http_body_util::BodyExt;
    use serde_json::{Value, json};
    use sqlx::MySqlPool;

    use crate::tests::utils::client::client;

    #[sqlx::test(fixtures("../../fixtures/resignation/resignations.sql"))]
    async fn latest_resignation_200(pool: MySqlPool) {
        let (addr, client) = client(pool).await;
        let query = parse_query::<String>(
            &fs::read_to_string("graphql/queries/latest_resignation.gql").unwrap(),
        )
        .unwrap()
        .to_string();
        let response = client
            .request(
                Request::builder()
                    .method("POST")
                    .uri(format!("http://{addr}/graphql"))
                    .header("Host", "localhost")
                    .header("Content-Type", "application/json")
                    .body(Body::from(json!({"query": query}).to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let bytes = response.into_body().collect().await.unwrap().to_bytes();
        let body: Value = serde_json::from_slice(&bytes).unwrap();
        assert!(body.is_object());
        let data = &body["data"];
        assert!(data.is_object());
        let resignation = &data["latestResignation"];
        assert_eq!(*resignation.get("id").unwrap(), json!("2222"));
        assert_eq!(
            *resignation.get("remainingPaidLeaveDays").unwrap(),
            json!(5)
        );
        assert_eq!(
            *resignation.get("retirementDate").unwrap(),
            json!("2025-01-01")
        );
        assert_eq!(
            *resignation.get("createdAt").unwrap(),
            json!("2025-02-01 00:00:00")
        );
    }
}
