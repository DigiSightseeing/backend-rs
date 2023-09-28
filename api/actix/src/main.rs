mod apps;
mod prelude;

use actix_web::{web::Data, App, HttpServer};
use apps::auth;
use prelude::{pool, AppState};
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    color_eyre::install().unwrap();
    dotenv::dotenv().ok();

    let pool = pool().await.unwrap();

    let mut base_path = env::current_dir().expect("Failed to get current directory");
    base_path.push("migrations");

    let migrator = sqlx::migrate::Migrator::new(base_path).await.unwrap();
    migrator.run(&pool).await.expect("Could not run migrations");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState::new(pool.to_owned())))
            .service(auth::url())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
