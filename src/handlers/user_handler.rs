use crate::{
    models::users::{CreateUserDto, RegisterUserDto, User, UserAuthDto},
    services::user_service::UserService,
    state::SharedState,
};
use axum::{
    Json,
    extract::{Path, State},
};
use uuid::Uuid;

#[utoipa::path(
    post,
    path = "/users",
    request_body = CreateUserDto,
    responses(
        (status = 200, description = "Create a new user", body = User)
    )
)]
pub async fn create_user(
    State(state): State<SharedState>,
    Json(dto): Json<CreateUserDto>,
) -> Json<User> {
    let user = UserService::create_user(&state.db, dto).await.unwrap();

    Json(user)
}

#[utoipa::path(
    get,
    path = "/users",
    responses(
        (status = 200, description = "List all users", body = [User])
    )
)]
pub async fn get_users(State(state): State<SharedState>) -> Json<Vec<User>> {
    let users = UserService::get_users(&state.db).await.unwrap();

    Json(users)
}

#[utoipa::path(
    get,
    path = "/users/{id}",
    params(
        ("id" = Uuid, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "Get user by ID", body = Option<User>)
    )
)]
pub async fn get_user_by_id(
    State(state): State<SharedState>,
    Path(id): Path<Uuid>,
) -> Json<Option<User>> {
    let user = UserService::get_user_by_id(&state.db, id).await.unwrap();

    Json(user)
}

#[utoipa::path(
    delete,
    path = "/users/{id}",
    params(
        ("id" = Uuid, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "Delete user by ID", body = String)
    )
)]
pub async fn delete_user(State(state): State<SharedState>, Path(id): Path<Uuid>) -> Json<String> {
    let deleted = UserService::delete_user(&state.db, id).await.unwrap();

    if deleted {
        Json("User deleted".to_string())
    } else {
        Json("User not found".to_string())
    }
}

#[utoipa::path(
    put,
    path = "/users/{id}",
    params(
        ("id" = Uuid, Path, description = "User ID")
    ),
    request_body = CreateUserDto,
    responses(
        (status = 200, description = "Update user by ID", body = User)
    )
)]
pub async fn update_user(
    State(state): State<SharedState>,
    Path(id): Path<Uuid>,
    Json(dto): Json<CreateUserDto>,
) -> Json<User> {
    let user = UserService::update_user(&state.db, id, dto).await.unwrap();

    Json(user)
}

#[utoipa::path(
    post,
    path = "/users/login",
    request_body = UserAuthDto,
    responses(
        (status = 200, description = "Login user", body = User)
    )
)]
pub async fn login_user(
    State(state): State<SharedState>,
    Json(dto): Json<UserAuthDto>,
) -> Json<User> {
    let user = UserService::login_user(&state.db, dto).await.unwrap();

    Json(user)
}

#[utoipa::path(
    post,
    path = "/users/register",
    request_body = RegisterUserDto,
    responses(
        (status = 200, description = "Register a new user", body = User)
    )
)]
pub async fn register_user(
    State(state): State<SharedState>,
    Json(dto): Json<RegisterUserDto>,
) -> Json<User> {
    let user_auth = UserService::register_user(&state.db, dto).await.unwrap();
    let user = User {
        id: user_auth.id,
        name: user_auth.username,
        email: user_auth.email,
    };

    Json(user)
}
