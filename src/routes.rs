use crate::handlers::user_handler::*;
use crate::state::SharedState;
use axum::{
    Router,
    routing::{get, post, put},
};

pub fn create_router(state: SharedState) -> Router {
    Router::new()
        .route("/users", get(get_users).post(create_user))
        .route("/users/:id", get(get_user_by_id).delete(delete_user))
        .route("/users/:id", put(update_user))
        .route("/users/login", post(login_user))
        .with_state(state)
}
