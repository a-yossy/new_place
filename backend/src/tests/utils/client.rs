use axum::body::Body;
use hyper_util::client::legacy::{Client, connect::HttpConnector};
use sqlx::MySqlPool;
use std::net::SocketAddr;
use tokio::net::TcpListener;

use crate::infrastructure::app::app;

pub async fn client(pool: MySqlPool) -> (SocketAddr, Client<HttpConnector, Body>) {
    let listener = TcpListener::bind("0.0.0.0:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move {
        axum::serve(listener, app(pool)).await.unwrap();
    });
    let client = hyper_util::client::legacy::Client::builder(hyper_util::rt::TokioExecutor::new())
        .build_http();

    (addr, client)
}
