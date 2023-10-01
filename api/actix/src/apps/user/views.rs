use actix_web::{
    web::{Data, ReqData},
    HttpResponse, Responder,
};

use super::models::User;
use crate::prelude::{AppState, TokenClaims};

pub async fn user_info(
    req_user: Option<ReqData<TokenClaims>>,
    state: Data<AppState>,
) -> impl Responder {
    match req_user {
        Some(user) => {
            let query = sqlx::query_as!(
                User,
                r#"
                SELECT id, username, email, first_name, last_name, date_joined, last_login
                  FROM "User"
                  WHERE id = $1
                "#,
                user.id
            )
            .fetch_one(&state.pool)
            .await;
            match query {
                Ok(user) => HttpResponse::Ok().json(user),
                Err(e) => HttpResponse::InternalServerError().json(format!("{:?}", e)),
            }
        }
        None => HttpResponse::Unauthorized().json("Unable to verify identity"),
    }
}
