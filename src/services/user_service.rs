use crate::{
    models::users::{CreateUserDto, User},
    repository::user_repository::UserRepository,
};
use sqlx::PgPool;
use uuid::Uuid;

pub struct UserService;

impl UserService {
    pub async fn create_user(db: &PgPool, dto: CreateUserDto) -> Result<User, String> {
        // contoh validasi bisnis
        if dto.name.trim().is_empty() {
            return Err("Name must not be empty".into());
        }

        let user = UserRepository::create(db, dto)
            .await
            .map_err(|e| e.to_string())?;

        Ok(user)
    }

    pub async fn get_users(db: &PgPool) -> Result<Vec<User>, String> {
        UserRepository::get_all(db).await.map_err(|e| e.to_string())
    }

    pub async fn get_user_by_id(db: &PgPool, id: Uuid) -> Result<Option<User>, String> {
        UserRepository::get_by_id(db, id)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn delete_user(db: &PgPool, id: Uuid) -> Result<bool, String> {
        let rows = UserRepository::delete(db, id)
            .await
            .map_err(|e| e.to_string())?;

        Ok(rows > 0)
    }
}
