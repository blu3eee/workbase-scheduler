use axum::response::Response;
use mysql::Pool;

use std::sync::Arc;

use crate::{ snowflake::SnowflakeGenerator, utilities::app_error::AppError };

#[derive(Debug, Clone)]
pub struct AppState {
    pub db_pool: Pool,
    pub snowflake_generator: Arc<SnowflakeGenerator>,
}

pub type AppResult<T> = std::result::Result<T, AppError>;

pub type ApiResponse = AppResult<Response>;
