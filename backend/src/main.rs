use async_graphql::{EmptySubscription, Schema, http::GraphiQLSource};
use async_graphql_axum::GraphQL;
use axum::{
    Router,
    http::{HeaderValue, Method, header},
    response::{Html, IntoResponse},
    routing::get,
};
use backend::graphql::mutation::MutationRoot;
use backend::graphql::query::QueryRoot;
use sqlx::{MySqlPool, mysql::MySqlPoolOptions};
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

async fn graphiql() -> impl IntoResponse {
    Html(GraphiQLSource::build().endpoint("/graphql").finish())
}

fn app(pool: MySqlPool) -> Router {
    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(pool.clone())
        .finish();
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:9000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers([header::CONTENT_TYPE, header::ACCEPT]);

    Router::new()
        .route("/graphql", get(graphiql).post_service(GraphQL::new(schema)))
        .layer(cors)
        .with_state(pool)
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let database_url = dotenv::var("DATABASE_URL").unwrap();
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .unwrap();

    axum::serve(
        TcpListener::bind("127.0.0.1:8000").await.unwrap(),
        app(pool),
    )
    .await
    .unwrap();

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{fs, net::SocketAddr};

    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use backend::models::resignation::Resignation;
    use chrono::NaiveDate;
    use graphql_parser::parse_query;
    use http_body_util::BodyExt;
    use hyper_util::client::legacy::{Client, connect::HttpConnector};
    use serde_json::{Value, json};
    use sqlx::MySqlPool;
    use tokio::net::TcpListener;

    use crate::app;

    async fn client(pool: MySqlPool) -> (SocketAddr, Client<HttpConnector, Body>) {
        let listener = TcpListener::bind("0.0.0.0:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        tokio::spawn(async move {
            axum::serve(listener, app(pool)).await.unwrap();
        });
        let client =
            hyper_util::client::legacy::Client::builder(hyper_util::rt::TokioExecutor::new())
                .build_http();

        (addr, client)
    }

    #[sqlx::test(fixtures("resignations"))]
    async fn resignation_200(pool: MySqlPool) {
        let (addr, client) = client(pool).await;
        let query =
            parse_query::<String>(&fs::read_to_string("graphql/queries/resignation.gql").unwrap())
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
        let resignation = &data["resignation"];
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
            json!("2025-02-02 00:00:00")
        );
    }

    #[sqlx::test]
    async fn post_resignation_200(pool: MySqlPool) {
        let resignations = sqlx::query_as!(
            Resignation,
            "SELECT id, created_at, remaining_paid_leave_days, retirement_date FROM resignation"
        )
        .fetch_all(&pool)
        .await
        .unwrap();
        assert_eq!(resignations.len(), 0);
        let (addr, client) = client(pool.clone()).await;
        let query = parse_query::<String>(
            &fs::read_to_string("graphql/mutations/resignation.gql").unwrap(),
        )
        .unwrap()
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
                    ))
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
        .await
        .unwrap();
        assert_eq!(resignations.len(), 1);
        assert_eq!(resignations.first().unwrap().remaining_paid_leave_days, 10);
        assert_eq!(
            resignations.first().unwrap().retirement_date,
            NaiveDate::from_ymd_opt(9999, 1, 1).unwrap()
        );
    }

    #[sqlx::test]
    async fn post_resignation_200_error(pool: MySqlPool) {
        let resignations = sqlx::query_as!(
            Resignation,
            "SELECT id, created_at, remaining_paid_leave_days, retirement_date FROM resignation"
        )
        .fetch_all(&pool)
        .await
        .unwrap();
        assert_eq!(resignations.len(), 0);
        let (addr, client) = client(pool.clone()).await;
        let query = parse_query::<String>(
            &fs::read_to_string("graphql/mutations/resignation.gql").unwrap(),
        )
        .unwrap()
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
                    ))
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
        let resignations = sqlx::query_as!(
            Resignation,
            "SELECT id, created_at, remaining_paid_leave_days, retirement_date FROM resignation"
        )
        .fetch_all(&pool)
        .await
        .unwrap();
        assert_eq!(resignations.len(), 0);
    }
}
