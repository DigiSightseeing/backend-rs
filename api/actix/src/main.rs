mod routes;

use actix_web::{web::Data, App, HttpServer};
use api_models::{pool, AppState};
use routes::auth::{login, register};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    color_eyre::install().unwrap();
    dotenv::dotenv().ok();

    let pool = pool().await.unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState::new(pool.to_owned())))
            .service(login)
            .service(register)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
