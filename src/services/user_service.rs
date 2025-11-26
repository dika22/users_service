use crate::{
    models::users::{CreateUserDto, User, UserAuthDto},
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

    pub async fn update_user(db: &PgPool, id: Uuid, dto: CreateUserDto) -> Result<User, String> {
        let user = UserRepository::update(db, id, dto)
            .await
            .map_err(|e| e.to_string())?;

        Ok(user)
    }

    pub async fn login_user(db: &PgPool, dto: UserAuthDto) -> Result<User, String> {
        let user_auth = UserRepository::login(db, dto.clone())
            .await
            .map_err(|e| e.to_string())?;

        let email = user_auth.email.ok_or("User has no email")?;

        let user = UserRepository::get_by_email(db, &email)
            .await
            .map_err(|e| e.to_string())?
            .ok_or("User profile not found")?;

        Ok(user)
    }
}
