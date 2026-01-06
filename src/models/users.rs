use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
}

#[derive(Deserialize, ToSchema)]
pub struct CreateUserDto {
    pub name: String,
    pub email: String,
}

use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Debug)]
pub struct UserAuth {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub created_by: String,
}

#[derive(Deserialize, Clone, ToSchema)]
pub struct UserAuthDto {
    pub email: String,
    pub password_hash: String,
}

#[derive(Deserialize, Clone, ToSchema)]
pub struct RegisterUserDto {
    pub username: String,
    pub email: String,
    pub password: String,
    pub created_by: String,
}
