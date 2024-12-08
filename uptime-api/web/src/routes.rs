use crate::state::AppState;
use axum::Router;

use crate::controllers::users::user_routes;
use std::sync::Arc;
use tower_http::services::ServeDir;

/// Initializes the application's routes.
///
/// This function maps paths (e.g. "/greet") and HTTP methods (e.g. "GET") to functions in [`crate::controllers`] as well as includes middlewares defined in [`crate::middlewares`] into the routing layer (see [`axum::Router`]).
pub fn init_routes(app_state: AppState) -> Router {
    let shared_app_state = Arc::new(app_state);

    Router::new().nest_service("/users", user_routes(Arc::clone(&shared_app_state)))
}
