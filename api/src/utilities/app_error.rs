// utilities/app_error.rs
use axum::{ http::StatusCode, response::IntoResponse, Json };
use serde::{ Deserialize, Serialize };
use std::fmt;
use std::error::Error;

pub type BoxedError = Box<dyn Error + Send + Sync + 'static>;

#[derive(Debug)]
pub struct AppError {
    code: StatusCode,
    message: String,
}

impl AppError {
    pub fn new(code: StatusCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }

    pub fn internal_server_error(message: impl Into<String>) -> Self {
        Self {
            code: StatusCode::INTERNAL_SERVER_ERROR,
            message: message.into(),
        }
    }

    pub fn bad_request(message: impl Into<String>) -> Self {
        Self {
            code: StatusCode::BAD_REQUEST,
            message: message.into(),
        }
    }

    pub fn not_found(message: impl Into<String>) -> Self {
        Self {
            code: StatusCode::NOT_FOUND,
            message: message.into(),
        }
    }

    pub fn db_error(message: impl Into<String>) -> Self {
        Self {
            code: StatusCode::INTERNAL_SERVER_ERROR,
            message: message.into(),
        }
    }
    // You can add other methods here for different types of errors
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        println!("{}", self.message);
        (
            self.code,
            Json(ErrorResponse {
                error: self.message.clone(),
            }),
        ).into_response()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ErrorResponse {
    error: String,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.code, self.message)
    }
}

impl Error for AppError {}

impl From<Box<dyn Error + Send + Sync + 'static>> for AppError {
    fn from(err: Box<dyn Error + Send + Sync + 'static>) -> Self {
        AppError::internal_server_error(format!("{}", err))
    }
}

impl From<mysql::Error> for AppError {
    fn from(error: mysql::Error) -> Self {
        match error {
            mysql::Error::IoError(err) => AppError::db_error(format!("I/O Error: {}", err)),
            mysql::Error::CodecError(err) => AppError::db_error(format!("Codec Error: {}", err)),
            mysql::Error::MySqlError(err) => AppError::db_error(format!("MySQL Error: {}", err)),
            mysql::Error::DriverError(err) => AppError::db_error(format!("Driver Error: {}", err)),
            mysql::Error::UrlError(err) => AppError::db_error(format!("URL Error: {}", err)),
            mysql::Error::TlsError(err) => AppError::db_error(format!("TLS Error: {}", err)),
            mysql::Error::FromValueError(_value) => AppError::db_error(format!("From Value Error")),
            mysql::Error::FromRowError(row) =>
                AppError::db_error(format!("From Row Error: {:?}", row)),
        }
    }
}
