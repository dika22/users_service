use crate::models::users::{CreateUserDto, RegisterUserDto, User, UserAuth, UserAuthDto};
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
            SELECT id, username, email, password_hash, created_at, created_by
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

    pub async fn register(
        db: &PgPool,
        dto: RegisterUserDto,
        hash: String,
    ) -> sqlx::Result<UserAuth> {
        let user_auth = sqlx::query_as!(
            UserAuth,
            r#"
            INSERT INTO users_auth (id, username, email, password_hash, created_at, created_by)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, username, email, password_hash, created_at, created_by
            "#,
            Uuid::new_v4(),
            dto.username,
            dto.email,
            hash,
            chrono::Utc::now(),
            dto.created_by
        )
        .fetch_one(db)
        .await?;

        Ok(user_auth)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenvy::dotenv;
    use sqlx::postgres::PgPoolOptions;
    use std::env;

    #[tokio::test]
    async fn test_compile_register() {
        dotenv().ok();
        // This test mostly checks compilation and basic DB connectivity if env is set
        let db_url = env::var("DATABASE_URL")
            .unwrap_or("postgres://adhika:adhika@localhost/news_service".to_string());
        // We won't actually fail if DB is down, just checking if code compiles and runs up to DB call
        // But let's try to connect
        if let Ok(pool) = PgPoolOptions::new().connect(&db_url).await {
            let dto = RegisterUserDto {
                username: "test_user".to_string(),
                email: format!("test_{}@example.com", Uuid::new_v4()),
                password: "password".to_string(),
                created_by: "tester".to_string(),
            };
            let _ = UserRepository::register(&pool, dto, "hash".to_string()).await;
        }
    }
}
