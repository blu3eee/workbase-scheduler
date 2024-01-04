use appstate::AppState;
use axum::Router;
use models::result::Result;

use crate::router::router::create_router;

pub mod utilities;
pub mod snowflake;
pub mod prototypes;
pub mod models;
pub mod router;
pub mod queries;
pub mod tests;
pub mod appstate;

/// Starts the Axum web server and sets up routing.
///
/// This function initializes the Axum router with the provided application state,
/// then binds and serves the application on a specified address.
pub async fn run(app_state: AppState) -> Result<()> {
    let app: Router = Router::new().nest("/api", create_router(app_state).await);

    println!("Starting server on 127.0.0.1:8080");
    let address = std::net::SocketAddr::from(([127, 0, 0, 1], 8080));

    // axum::serve::Serve::bind(&address).serve(app.into_make_service()).await.unwrap();
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, app.into_make_service()).await.unwrap();
    Ok(())
}
