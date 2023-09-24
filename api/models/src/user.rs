use serde::{Deserialize, Serialize};
use sqlx::{types::Uuid, FromRow};

#[derive(Debug, Deserialize)]
pub struct CreateUserBody {
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, FromRow)]
pub struct UserNoPassword {
    pub id: Uuid,
    pub username: String,
}
