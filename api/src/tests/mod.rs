use std::env;
use dotenv::dotenv;
use mysql::*;
use mysql::prelude::*;

use crate::models::result::Result;
use crate::queries::create_tables;

pub fn initialize_test_db() -> Result<Pool> {
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

    Ok(pool)
}

pub fn cleanup_test_db(mut conn: PooledConn) -> Result<()> {
    conn.query_drop("DROP DATABASE IF EXISTS worktest;")?;

    Ok(())
}
