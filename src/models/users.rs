use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
}

#[derive(Deserialize)]
pub struct CreateUserDto {
    pub name: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserAuth {
    pub id: i64,
    pub username: Option<String>,
    pub email: Option<String>,
    pub password_hash: String,
}

#[derive(Deserialize, Clone)]
pub struct UserAuthDto {
    pub email: String,
    pub password_hash: String,
}
