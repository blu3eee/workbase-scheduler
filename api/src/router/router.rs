use axum::{ middleware, routing::get, Router, Extension };

use crate::{ app::AppState, prototypes::uniqueid_routers::UniqueIdRouter };

use super::{ middlewares::log_route::log_route, routes::user::UserRouter };

pub async fn create_router(app_state: AppState) -> Router {
    Router::new()
        .merge(<UserRouter as UniqueIdRouter>::router().await)
        .layer(Extension(app_state.clone()))
        .layer(middleware::from_fn(log_route))
        .route(
            "/hello",
            get(|| async { "Hello, World!" })
        )
}
