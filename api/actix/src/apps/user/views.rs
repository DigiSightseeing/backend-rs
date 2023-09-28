use actix_web::{
    web::{self, Data},
    HttpResponse, Responder,
};
use sqlx::types::Uuid;

use super::models::User;
use crate::prelude::AppState;

pub async fn user_info(id: web::Path<Uuid>, state: Data<AppState>) -> impl Responder {
    let query = sqlx::query_as!(
        User,
        r#"
        SELECT id, username, email, first_name, last_name, date_joined, last_login
          FROM "User"
          WHERE id = $1
        "#,
        id.into_inner(),
    )
    .fetch_one(&state.pool)
    .await;
    match query {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::InternalServerError().json(format!("{:?}", e)),
    }
}
