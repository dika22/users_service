use axum::serve;
use sqlx::PgPool;
use std::sync::Arc;
use tokio::net::TcpListener;

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

    let app = routes::create_router(state);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("🚀 Server running on 3000");

    serve(listener, app).await.unwrap();
}
