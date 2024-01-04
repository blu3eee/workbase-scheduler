use std::{ sync::Arc, env };
use dotenv::dotenv;
use mysql::{ *, prelude::Queryable };

use crate::{
    models::result::Result,
    app::AppState,
    snowflake::SnowflakeGenerator,
    queries::create_tables,
};

fn initialize_test_db() -> Result<()> {
    dotenv().ok();
    // Setup database connection (replace with your test database credentials)
    let url: String = env
        ::var("DATABASE_URL_TEST")
        .expect("DATABASE_URL_TEST must be set in the .env file");

    let pool = Pool::new(url.as_str())?;
    let mut conn: PooledConn = pool.get_conn()?;

    conn.query_drop("DROP DATABASE IF EXISTS worktest;")?;
    conn.query_drop(
        "CREATE DATABASE IF NOT EXISTS worktest DEFAULT CHARSET = utf8mb4 DEFAULT COLLATE = utf8mb4_unicode_ci;"
    )?;
    conn.query_drop("USE worktest;")?;

    let _ = create_tables(&mut conn);

    Ok(())
}

fn connect_to_db() -> Result<Pool> {
    initialize_test_db()?;
    let url: String = env
        ::var("DATABASE_URL_TEST")
        .expect("DATABASE_URL_TEST must be set in the .env file");

    let pool = Pool::new(format!("{url}/worktest").as_str())?;

    Ok(pool)
}

pub async fn initialize_test_app_state() -> Result<AppState> {
    // Set up test database and other configurations
    // Replace with actual test database initialization
    let pool = connect_to_db()?;
    let state = AppState {
        db_pool: pool,
        snowflake_generator: Arc::new(SnowflakeGenerator::new(1)),
    };

    Ok(state)
}
