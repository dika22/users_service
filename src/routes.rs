use crate::handlers::user_handler::*;
use crate::state::SharedState;
use axum::{Router, routing::get};

pub fn create_router(state: SharedState) -> Router {
    Router::new()
        .route("/users", get(get_users).post(create_user))
        .route("/users/:id", get(get_user_by_id).delete(delete_user))
        .with_state(state)
}
