use std::ops::Deref;

use actix_web::{dev::ServiceRequest, Error, HttpMessage};
use actix_web_httpauth::extractors::{
    bearer::{BearerAuth, Config},
    AuthenticationError,
};
use hmac::{Hmac, Mac};
use jwt::VerifyWithKey;
use serde::{Deserialize, Serialize};
use sha2::Sha512;
use sqlx::{postgres::PgPoolOptions, types::Uuid, Pool, Postgres};

pub struct AppState {
    pub pool: Pool<Postgres>,
}

impl AppState {
    pub const fn new(db: Pool<Postgres>) -> Self {
        Self { pool: db }
    }
}

pub async fn pool() -> color_eyre::Result<Pool<Postgres>> {
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    let pool = PgPoolOptions::new().connect(url.as_str()).await?;
    Ok(pool)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenClaims {
    pub id: Uuid,
}

impl TokenClaims {
    pub const fn new(id: Uuid) -> Self {
        Self { id }
    }
}

pub async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let token_string = credentials.token();
    let key: Hmac<Sha512> = Hmac::new_from_slice(shared::jwt_secret().deref()).unwrap();
    let claims: Result<TokenClaims, &str> = token_string
        .verify_with_key(&key)
        .map_err(|_| "Invalid token");

    match claims {
        Ok(value) => {
            req.extensions_mut().insert(value);
            Ok(req)
        }
        Err(_) => {
            let config = req
                .app_data::<Config>()
                .cloned()
                .unwrap_or_default()
                .scope("");
            Err((AuthenticationError::from(config).into(), req))
        }
    }
}
