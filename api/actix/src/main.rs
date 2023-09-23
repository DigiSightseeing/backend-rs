use api_db::{migrate, pool};
use color_eyre::Result;

#[actix_web::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    dotenv::dotenv().ok();

    let pool = pool().await?;
    migrate(pool).await?;
    Ok(())
}
