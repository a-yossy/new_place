use sqlx::{MySql, Pool, mysql::MySqlPoolOptions};

pub async fn get_pool() -> Pool<MySql> {
    let database_url = dotenv::var("DATABASE_URL").unwrap();
    MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .unwrap()
}
