use std::collections::HashMap;

use async_graphql::{EmptySubscription, Schema, http::GraphiQLSource};
use async_graphql_axum::GraphQL;
use axum::{
    Json, Router,
    http::{HeaderValue, Method, header},
    response::{Html, IntoResponse},
    routing::get,
};

use sqlx::MySqlPool;
use tower_http::cors::CorsLayer;

use crate::graphql::{mutations::root::MutationRoot, queries::root::QueryRoot};

async fn graphiql() -> impl IntoResponse {
    Html(GraphiQLSource::build().endpoint("/graphql").finish())
}

async fn holidays() -> impl IntoResponse {
    let mut holidays = HashMap::new();
    holidays.insert("2025-01-01".to_string(), "休み".to_string());
    Json(holidays)
}

pub fn app(pool: MySqlPool) -> Router {
    let schema = Schema::build(
        QueryRoot::default(),
        MutationRoot::default(),
        EmptySubscription,
    )
    .data(pool.clone())
    .finish();
    let cors = CorsLayer::new()
        .allow_origin(
            "http://localhost:9000"
                .parse::<HeaderValue>()
                .unwrap_or_else(|_| HeaderValue::from_static("http://localhost:9000")),
        )
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers([header::CONTENT_TYPE, header::ACCEPT]);

    Router::new()
        .route("/graphql", get(graphiql).post_service(GraphQL::new(schema)))
        .route("/holidays", get(holidays))
        .layer(cors)
        .with_state(pool)
}
