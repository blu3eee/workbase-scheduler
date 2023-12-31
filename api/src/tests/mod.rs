use std::env;
use dotenv::dotenv;
use mysql::*;
use mysql::prelude::*;

use crate::models::org_jobs::create_org_job_table;
use crate::models::org_member_jobs::create_org_member_job_table;
use crate::models::org_members::create_org_members_table_query;
use crate::models::organizations::create_organizations_table_query;
use crate::models::users::create_users_table_query;

pub fn initialize_test_db() -> Result<PooledConn, Box<dyn std::error::Error>> {
    dotenv().ok();
    // Setup database connection (replace with your test database credentials)
    let url: String = env
        ::var("DATABASE_URL_TEST")
        .expect("DATABASE_URL_TEST must be set in the .env file");

    let pool = Pool::new(url.as_str())?;
    let mut conn: PooledConn = pool.get_conn()?;

    conn.query_drop("DROP DATABASE IF EXISTS worktest;")?;
    conn.query_drop("CREATE DATABASE IF NOT EXISTS worktest;")?;
    conn.query_drop("USE worktest;")?;

    conn.query_drop(create_users_table_query())?;
    conn.query_drop(create_organizations_table_query())?;
    conn.query_drop(create_org_members_table_query())?;
    conn.query_drop(create_org_job_table())?;
    conn.query_drop(create_org_member_job_table())?;

    Ok(conn)
}

pub fn cleanup_test_db(mut conn: PooledConn) -> Result<(), Box<dyn std::error::Error>> {
    conn.query_drop("DROP DATABASE IF EXISTS worktest;")?;

    Ok(())
}
