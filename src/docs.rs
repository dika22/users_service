use crate::handlers::user_handler;
use crate::models::users::*;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        user_handler::create_user,
        user_handler::get_users,
        user_handler::get_user_by_id,
        user_handler::update_user,
        user_handler::delete_user,
        user_handler::login_user,
        user_handler::register_user,
    ),
    components(
        schemas(User, CreateUserDto, UserAuthDto, RegisterUserDto)
    ),
    tags(
        (name = "user", description = "User management endpoints")
    )
)]
pub struct ApiDoc;
