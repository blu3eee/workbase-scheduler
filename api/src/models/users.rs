use chrono::{ NaiveDateTime, NaiveDate };
use serde::{ Serialize, Deserialize };

use mysql::*;
use mysql::prelude::*;

use crate::models::convert_to_naive_date;

use super::convert_to_naive_date_time;

/// SQL statement to create the table `users`
pub fn create_users_table_query() -> String {
    "
    CREATE TABLE IF NOT EXISTS users (
        id INT AUTO_INCREMENT PRIMARY KEY,
        email VARCHAR(100) NOT NULL UNIQUE,
        password VARCHAR(255) NOT NULL,
        first_name VARCHAR(100) NOT NULL,
        last_name VARCHAR(100) NOT NULL,
        date_of_birth DATE NOT NULL,
        phone_number VARCHAR(20) NULL UNIQUE,
        created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
    );
    ".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub date_of_birth: NaiveDate,
    pub phone_number: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl FromRow for User {
    fn from_row_opt(row: Row) -> Result<Self, FromRowError> {
        Ok(User {
            id: row.get("id").ok_or(FromRowError(row.clone()))?,
            email: row.get("email").ok_or(FromRowError(row.clone()))?,
            password: row.get("password").ok_or(FromRowError(row.clone()))?,
            first_name: row.get("first_name").ok_or(FromRowError(row.clone()))?,
            last_name: row.get("last_name").ok_or(FromRowError(row.clone()))?,
            date_of_birth: convert_to_naive_date(
                row.get("date_of_birth").ok_or(FromRowError(row.clone()))?
            ),
            phone_number: row.get("phone_number").ok_or(FromRowError(row.clone()))?,
            created_at: convert_to_naive_date_time(
                row.get("created_at").ok_or(FromRowError(row.clone()))?
            ),
            updated_at: convert_to_naive_date_time(
                row.get("updated_at").ok_or(FromRowError(row.clone()))?
            ),
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartialUser {
    pub id: i32,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub date_of_birth: NaiveDate,
    pub phone_number: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl FromRow for PartialUser {
    fn from_row_opt(row: Row) -> Result<Self, FromRowError> {
        Ok(PartialUser {
            id: row.get("id").ok_or(FromRowError(row.clone()))?,
            email: row.get("email").ok_or(FromRowError(row.clone()))?,
            first_name: row.get("first_name").ok_or(FromRowError(row.clone()))?,
            last_name: row.get("last_name").ok_or(FromRowError(row.clone()))?,
            phone_number: row.get("phone_number").ok_or(FromRowError(row.clone()))?,
            date_of_birth: convert_to_naive_date(
                row.get("date_of_birth").ok_or(FromRowError(row.clone()))?
            ),
            created_at: convert_to_naive_date_time(
                row.get("created_at").ok_or(FromRowError(row.clone()))?
            ),
            updated_at: convert_to_naive_date_time(
                row.get("updated_at").ok_or(FromRowError(row.clone()))?
            ),
        })
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct RequestCreateUser {
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub date_of_birth: NaiveDate,
    pub phone_number: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct RequestUpdateUser {
    pub id: i32,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub date_of_birth: Option<NaiveDate>,
    pub phone_number: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RequestUpdateUserCredentials {
    pub email: Option<String>,
    pub password: Option<String>,
}
