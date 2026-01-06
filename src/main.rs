use axum::serve;
use sqlx::PgPool;
use std::sync::Arc;
use tokio::net::TcpListener;

mod docs;
mod handlers;
mod models;
mod repository;
mod routes;
mod services;
mod state;

use state::AppState;

#[tokio::main]
async fn main() {
    // load .env
    dotenvy::dotenv().ok();

    // ambil DATABASE_URL
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");

    // connect pool
    let db = PgPool::connect(&database_url)
        .await
        .expect("DB connection failed");

    let state = Arc::new(AppState { db });

    use docs::ApiDoc;
    use utoipa::OpenApi;
    use utoipa_swagger_ui::SwaggerUi;

    let app = routes::create_router(state)
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()));

    let port = std::env::var("PORT").unwrap_or("3001".to_string());
    let host = std::env::var("HOST").unwrap_or("0.0.0.0".to_string());
    let addr = format!("{}:{}", host, port);

    let listener = TcpListener::bind(&addr).await.unwrap();

    println!("🚀 Server running on {}", addr);

    serve(listener, app).await.unwrap();
}
