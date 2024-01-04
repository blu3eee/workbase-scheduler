use axum::{ middleware, routing::get, Router, Extension };

use crate::appstate::AppState;

use super::{ middlewares::log_route::log_route, routes::user::get_all_user_handler };

pub async fn create_router(app_state: AppState) -> Router {
    Router::new()
        .route("/users", get(get_all_user_handler))
        .layer(Extension(app_state.clone()))
        .layer(middleware::from_fn(log_route))
        .route(
            "/hello",
            get(|| async { "Hello, World!" })
        )
}
