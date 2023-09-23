use color_eyre::Result;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub async fn pool() -> Result<Pool<Postgres>> {
    let url = std::env::var("DATABASE_URL")?;
    let pool = PgPoolOptions::new().connect(url.as_str()).await?;
    Ok(pool)
}

