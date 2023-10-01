use actix_web::{web, Scope};

use super::views::user_info;

pub fn url() -> Scope {
    web::scope("/user").service(web::resource("").route(web::get().to(user_info)))
}
