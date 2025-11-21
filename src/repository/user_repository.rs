use crate::models::users::{CreateUserDto, User};
use sqlx::PgPool;
use uuid::Uuid;

pub struct UserRepository;

impl UserRepository {
    pub async fn create(db: &PgPool, dto: CreateUserDto) -> sqlx::Result<User> {
        let user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (name, email)
            VALUES ($1, $2)
            RETURNING id, name, email
            "#,
            dto.name,
            dto.email
        )
        .fetch_one(db)
        .await?;

        Ok(user)
    }

    pub async fn get_all(db: &PgPool) -> sqlx::Result<Vec<User>> {
        let users = sqlx::query_as!(User, r#"SELECT id, name, email FROM users ORDER BY name"#,)
            .fetch_all(db)
            .await?;

        Ok(users)
    }

    pub async fn get_by_id(db: &PgPool, id: Uuid) -> sqlx::Result<Option<User>> {
        let user = sqlx::query_as!(
            User,
            r#"SELECT id, name, email FROM users WHERE id = $1"#,
            id
        )
        .fetch_optional(db)
        .await?;

        Ok(user)
    }

    pub async fn delete(db: &PgPool, id: Uuid) -> sqlx::Result<u64> {
        let result = sqlx::query!(r#"DELETE FROM users WHERE id = $1"#, id)
            .execute(db)
            .await?;

        Ok(result.rows_affected())
    }
}
