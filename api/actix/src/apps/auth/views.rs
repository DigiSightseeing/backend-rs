use std::ops::Deref;

use actix_web::{
    web::{Data, Json},
    HttpResponse, Responder,
};
use actix_web_httpauth::extractors::basic::BasicAuth;
use argonautica::{Hasher, Verifier};
use chrono::Utc;
use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use sha2::Sha512;

use super::models::NewUser;
use crate::prelude::{AppState, TokenClaims};

pub async fn register(state: Data<AppState>, body: Json<NewUser>) -> impl Responder {
    let user = body.into_inner();

    let mut hasher = Hasher::default();
    let hashed_password = hasher
        .with_password(user.password)
        .with_secret_key(shared::hash_secret().deref())
        .hash()
        .unwrap();

    let query = sqlx::query!(
        r#"
        INSERT INTO "User" (username, email, password, last_login)
          VALUES ($1, $2, $3, $4) RETURNING id
        "#,
        user.username,
        user.email,
        hashed_password,
        Utc::now().naive_utc(),
    )
    .fetch_one(&state.pool)
    .await;

    match query {
        Ok(user) => HttpResponse::SeeOther()
            .append_header(("location", format!("/user/{}", user.id)))
            .finish(),
        Err(e) => HttpResponse::InternalServerError().json(format!("{:?}", e)),
    }
}

pub async fn login(state: Data<AppState>, credentials: BasicAuth) -> impl Responder {
    let jwt_secret: Hmac<Sha512> = Hmac::new_from_slice(shared::jwt_secret().deref()).unwrap();
    let username = credentials.user_id();
    let password = credentials.password();

    match password {
        Some(pass) => {
            let query = sqlx::query!(
                r#"
                SELECT id, username, password
                  FROM "User"
                  WHERE username = $1
                "#,
                username
            )
            .fetch_one(&state.pool)
            .await;
            match query {
                Ok(user) => {
                    let mut verifier = Verifier::default();
                    let is_invalid = verifier
                        .with_hash(user.password)
                        .with_password(pass)
                        .with_secret_key(shared::hash_secret().deref())
                        .verify()
                        .unwrap();

                    if is_invalid {
                        match sqlx::query!(
                            r#"
                            UPDATE "User"
                              SET last_login = $1
                              WHERE id = $2
                            "#,
                            Utc::now().naive_utc(),
                            user.id
                        )
                        .fetch_one(&state.pool)
                        .await
                        {
                            Ok(_) | Err(_) => {}
                        }
                        let claims = TokenClaims::new(user.id);
                        let token_str = claims.sign_with_key(&jwt_secret).unwrap();
                        HttpResponse::Ok().json(token_str)
                    } else {
                        HttpResponse::Unauthorized().json("Incorrect username or password")
                    }
                }
                Err(e) => HttpResponse::InternalServerError().json(format!("{:?}", e)),
            }
        }
        None => HttpResponse::Unauthorized().json("Must provide username and password"),
    }
}

pub async fn logout(state: Data<AppState>) -> impl Responder {
    format!("")
}
