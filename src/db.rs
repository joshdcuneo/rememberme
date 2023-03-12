use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};

pub async fn init() -> Result<SqlitePool, sqlx::Error> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = SqlitePoolOptions::new().connect(&database_url).await?;
    sqlx::migrate!().run(&pool).await?;
    Ok(pool)
}