use async_graphql::{EmptySubscription, Schema, http::GraphiQLSource};
use async_graphql_axum::GraphQL;
use axum::{
    Router,
    response::{Html, IntoResponse},
    routing::get,
};
use backend::graphql::mutation::MutationRoot;
use backend::graphql::query::QueryRoot;
use sqlx::{MySqlPool, mysql::MySqlPoolOptions};
use tokio::net::TcpListener;

async fn graphiql() -> impl IntoResponse {
    Html(GraphiQLSource::build().endpoint("/graphql").finish())
}

fn app(pool: MySqlPool) -> Router {
    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(pool.clone())
        .finish();

    Router::new()
        .route("/graphql", get(graphiql).post_service(GraphQL::new(schema)))
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
    use std::net::SocketAddr;

    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
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

        let response = client
            .request(
                Request::builder()
                    .method("POST")
                    .uri(format!("http://{addr}/graphql"))
                    .header("Host", "localhost")
                    .header("Content-Type", "application/json")
                    .body(Body::from(
                        r#"{"query":"query { resignation { id retirementDate remainingPaidLeaveDays createdAt } }"}"#,
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
}
