use std::ops::Deref;

use actix_web::{
    get, post,
    web::{Data, Json},
    HttpResponse, Responder,
};
use actix_web_httpauth::extractors::basic::BasicAuth;
use api_models::{
    user::{CreateUserBody, UserNoPassword},
    AppState, TokenClaims,
};
use argonautica::{Hasher, Verifier};
use chrono::Utc;
use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use sha2::Sha512;

#[post("/regiseter")]
pub async fn register(state: Data<AppState>, body: Json<CreateUserBody>) -> impl Responder {
    let user = body.into_inner();

    let mut hasher = Hasher::default();
    let hashed_password = hasher
        .with_password(user.password)
        .with_secret_key(shared::hash_secret().deref())
        .hash()
        .unwrap();

    let query = sqlx::query_as!(
        UserNoPassword,
        r#"
        INSERT INTO "User" (username, first_name, last_name, email, password)
          VALUES ($1, $2, $3, $4, $5) RETURNING id, username
        "#,
        user.username,
        user.first_name,
        user.last_name,
        user.email,
        hashed_password
    )
    .fetch_one(&state.pool)
    .await;

    match query {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::InternalServerError().json(format!("{:?}", e)),
    }
}

#[get("/login")]
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
