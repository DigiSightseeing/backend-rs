use color_eyre::Result;
use sqlx::{migrate, postgres::PgPoolOptions, Pool, Postgres};

pub async fn pool() -> Result<Pool<Postgres>> {
    let url = std::env::var("DATABASE_URL")?;
    let pool = PgPoolOptions::new().connect(url.as_str()).await?;
    Ok(pool)
}

pub async fn migrate(pool: Pool<Postgres>) -> Result<()> {
    migrate!("./migrations").run(&pool).await?;
    Ok(())
}
