use axum::{
    Json,
    extract::{Path, State},
};
use uuid::Uuid;

use crate::{
    models::users::{CreateUserDto, User},
    services::user_service::UserService,
    state::SharedState,
};

pub async fn create_user(
    State(state): State<SharedState>,
    Json(dto): Json<CreateUserDto>,
) -> Json<User> {
    let user = UserService::create_user(&state.db, dto).await.unwrap();

    Json(user)
}

pub async fn get_users(State(state): State<SharedState>) -> Json<Vec<User>> {
    let users = UserService::get_users(&state.db).await.unwrap();

    Json(users)
}

pub async fn get_user_by_id(
    State(state): State<SharedState>,
    Path(id): Path<Uuid>,
) -> Json<Option<User>> {
    let user = UserService::get_user_by_id(&state.db, id).await.unwrap();

    Json(user)
}

pub async fn delete_user(State(state): State<SharedState>, Path(id): Path<Uuid>) -> Json<String> {
    let deleted = UserService::delete_user(&state.db, id).await.unwrap();

    if deleted {
        Json("User deleted".to_string())
    } else {
        Json("User not found".to_string())
    }
}
