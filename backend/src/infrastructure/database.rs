use anyhow::Result;
use sqlx::{MySql, Pool, mysql::MySqlPoolOptions};

pub async fn get_pool() -> Result<Pool<MySql>> {
    let database_url = dotenv::var("DATABASE_URL")?;
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;
    Ok(pool)
}
