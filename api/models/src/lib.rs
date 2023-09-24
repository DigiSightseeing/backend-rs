use color_eyre::Result;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, types::Uuid, Pool, Postgres};

pub mod user;

pub struct AppState {
    pub pool: Pool<Postgres>,
}

impl AppState {
    pub fn new(db: Pool<Postgres>) -> Self {
        Self { pool: db }
    }
}

pub async fn pool() -> Result<Pool<Postgres>> {
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    let pool = PgPoolOptions::new().connect(url.as_str()).await?;
    Ok(pool)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenClaims {
    pub id: Uuid,
}

impl TokenClaims {
    pub fn new(id: Uuid) -> Self {
        Self { id }
    }
}
