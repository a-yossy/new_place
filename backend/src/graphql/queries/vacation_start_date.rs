use async_graphql::{Context, Object, Result};
use sqlx::{MySql, Pool};

use crate::{
    clients::holidays::fetch_holidays, graphql::scalars::date::Date,
    models::resignation::Resignation as ResignationModel,
};

#[derive(Default)]
pub struct VacationStartDateQuery;

#[Object]
impl VacationStartDateQuery {
    async fn vacation_start_date(&self, ctx: &Context<'_>) -> Result<Date> {
        let holidays = fetch_holidays().await?;
        let pool = ctx.data::<Pool<MySql>>().unwrap();
        let vacation_start_date = ResignationModel::fetch_latest(pool)
            .await?
            .vacation_start_date(&holidays)
            .await?;

        Ok(Date(vacation_start_date))
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

    use crate::tests::{mocks::server::MockServer, utils::client::client};

    #[sqlx::test(fixtures("vacation_start_date_200_data"))]
    async fn vacation_start_date_200_data(pool: MySqlPool) {
        let server = MockServer::new_async().await;
        let path = "/api/v1/date.json";
        let json_body = json!({
            "2025-01-01": "休み",
            "2024-12-31": "休み"
        });
        {
            let mut srv = server.0.lock().await;
            srv.mock("GET", path)
                .with_status(200)
                .with_header("content-type", "application/json")
                .with_body(json_body.to_string())
                .create_async()
                .await;
            let (addr, client) = client(pool).await;
            let query = parse_query::<String>(
                &fs::read_to_string("graphql/queries/vacation_start_date.gql").unwrap(),
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
            assert!(body.get("errors").is_none());
            let data = &body["data"];
            assert!(data.is_object());
            // 土曜日: 2024-12-28, 2024-12-21, 2024-12-14
            // 日曜日: 2024-12-29, 2024-12-22, 2024-12-15
            assert_eq!(data["vacationStartDate"], json!("2024-12-17"));
            srv.reset();
        }
    }

    #[sqlx::test()]
    async fn vacation_start_date_200_err(pool: MySqlPool) {
        let server = MockServer::new_async().await;
        let path = "/api/v1/date.json";
        let json_body = json!("invalid");
        {
            let mut srv = server.0.lock().await;
            srv.mock("GET", path)
                .with_status(200)
                .with_header("content-type", "application/json")
                .with_body(json_body.to_string())
                .create_async()
                .await;
            let (addr, client) = client(pool).await;
            let query = parse_query::<String>(
                &fs::read_to_string("graphql/queries/vacation_start_date.gql").unwrap(),
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
            assert!(data.is_null());
            let errors = &body["errors"];
            assert!(errors.is_array());
            assert!(errors.as_array().iter().len() > 0);
            srv.reset();
        }
    }
}
