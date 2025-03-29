use backend::infrastructure::{app::app, database::get_pool};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = get_pool().await;
    axum::serve(
        TcpListener::bind("127.0.0.1:8000").await.unwrap(),
        app(pool),
    )
    .await
    .unwrap();

    Ok(())
}
