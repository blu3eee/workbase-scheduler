use std::{ sync::Arc, env };

use mysql::*;

use dotenv::dotenv;

use api::{ run, queries::create_tables, snowflake::SnowflakeGenerator, app::AppState };

#[tokio::main]
async fn main() {
    dotenv().ok();
    let db_url: String = env
        ::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in the .env file");

    let pool = Pool::new(db_url.as_str()).expect("failed to create Pool from db_url");

    {
        let mut conn: PooledConn = pool
            .get_conn()
            .expect("'failed to establish connection with db'");
        let _ = create_tables(&mut conn);
    }

    let machine_id = 1; // Set this based on your deployment strategy
    let snowflake_generator = Arc::new(SnowflakeGenerator::new(machine_id));

    let app_state = AppState {
        db_pool: pool,
        snowflake_generator,
    };
    let _ = run(app_state).await;
}
