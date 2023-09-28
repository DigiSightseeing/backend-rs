use actix_web::{web, Scope};

use super::views::{login, logout, register};

pub fn url() -> Scope {
    web::scope("")
        .service(web::resource("/register").route(web::post().to(register)))
        .service(web::resource("/login").route(web::get().to(login)))
        .service(web::resource("/logout").route(web::get().to(logout)))
}
