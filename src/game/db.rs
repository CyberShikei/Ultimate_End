use dotenvy::dotenv;
use sqlx::PgPool;
use std::env;

#[tokio::main]
pub async fn connect_to_db() -> Result<PgPool, sqlx::Error> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&db_url).await?;
    Ok(pool)
}
