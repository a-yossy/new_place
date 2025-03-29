use anyhow::Result;
use backend::infrastructure::{app::app, database::get_pool};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<()> {
    let pool = get_pool().await?;
    axum::serve(TcpListener::bind("127.0.0.1:8000").await?, app(pool)).await?;

    Ok(())
}
