use std::sync::Arc;

pub fn database_url() -> Arc<str> {
    std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set.")
        .into()
}

pub fn hash_secret() -> Arc<str> {
    std::env::var("HASH_SECRET")
        .expect("HASH_SECRET must be set!")
        .into()
}

pub fn jwt_secret() -> Arc<[u8]> {
    std::env::var("JWT_SECRET")
        .expect("JWT_SECRET must be set!")
        .as_bytes()
        .into()
}
