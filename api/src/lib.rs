use std::env;
use std::error::Error;

use models::create_tables;
use mysql::*;
// use mysql::prelude::*;

pub mod models;
pub mod router;
pub mod queries;
pub mod tests;

/// Starts the Axum web server and sets up routing.
///
/// This function initializes the Axum router with the provided application state,
/// then binds and serves the application on a specified address.
pub async fn run() -> Result<(), Box<dyn Error>> {
    let db_url: String = env
        ::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in the .env file");

    let pool = Pool::new(db_url.as_str())?;

    let conn: PooledConn = pool.get_conn()?;
    let _ = create_tables(conn).await;
    Ok(())
}
