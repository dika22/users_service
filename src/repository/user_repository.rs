use crate::models::users::{CreateUserDto, User, UserAuth, UserAuthDto};
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

    pub async fn get_by_email(db: &PgPool, email: &str) -> sqlx::Result<Option<User>> {
        let user = sqlx::query_as!(
            User,
            r#"SELECT id, name, email FROM users WHERE email = $1"#,
            email
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

    pub async fn update(db: &PgPool, id: Uuid, dto: CreateUserDto) -> sqlx::Result<User> {
        let user = sqlx::query_as!(
            User,
            r#"
            UPDATE users
            SET name = $2, email = $3
            WHERE id = $1
            RETURNING id, name, email
            "#,
            id,
            dto.name,
            dto.email
        )
        .fetch_one(db)
        .await?;

        Ok(user)
    }

    pub async fn login(db: &PgPool, dto: UserAuthDto) -> sqlx::Result<UserAuth> {
        let user_auth = sqlx::query_as!(
            UserAuth,
            r#"
            SELECT id, username, email, password_hash
            FROM users_auth
            WHERE email = $1 AND password_hash = $2
            "#,
            dto.email,
            dto.password_hash
        )
        .fetch_one(db)
        .await?;

        Ok(user_auth)
    }
}
