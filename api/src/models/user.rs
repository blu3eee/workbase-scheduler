use chrono::{ NaiveDate, NaiveDateTime };
use serde::{ Serialize, Deserialize };

use mysql::*;
use mysql::prelude::*;

use crate::snowflake::SnowflakeId;
use crate::utilities::parse_chrono::{ convert_to_naive_date, convert_to_naive_date_time };

/// SQL statement to create the table `users`
pub fn create_users_table_query() -> String {
    "
    CREATE TABLE IF NOT EXISTS users (
        id BIGINT NOT NULL PRIMARY KEY,
        email VARCHAR(100) NOT NULL UNIQUE,
        encrypted_password VARCHAR(255) NOT NULL,
        first_name VARCHAR(100) NOT NULL,
        last_name VARCHAR(100) NOT NULL,
        date_of_birth DATE NOT NULL,
        phone_number VARCHAR(20) NULL UNIQUE,
        avatar VARCHAR(255) NULL,
        is_active BOOLEAN NOT NULL DEFAULT TRUE,
        deleted_at TIMESTAMP NULL,
    );
    ".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: SnowflakeId,
    pub email: String,
    pub encrypted_password: String,
    pub first_name: String,
    pub last_name: String,
    pub date_of_birth: NaiveDate,
    pub phone_number: Option<String>,
    pub avatar: Option<String>,
    pub is_active: bool,
    pub deleted_at: Option<NaiveDateTime>,
}

impl FromRow for User {
    fn from_row_opt(row: Row) -> Result<Self, FromRowError> {
        let deleted_at = if
            let Some(timestamp) = row.get("deleted_at").ok_or(FromRowError(row.clone()))?
        {
            Some(convert_to_naive_date_time(timestamp).map_err(|_| FromRowError(row.clone()))?)
        } else {
            None
        };

        Ok(User {
            id: row.get("id").ok_or(FromRowError(row.clone()))?,
            email: row.get("email").ok_or(FromRowError(row.clone()))?,
            encrypted_password: row.get("encrypted_password").ok_or(FromRowError(row.clone()))?,
            first_name: row.get("first_name").ok_or(FromRowError(row.clone()))?,
            last_name: row.get("last_name").ok_or(FromRowError(row.clone()))?,
            date_of_birth: convert_to_naive_date(
                row.get("date_of_birth").ok_or(FromRowError(row.clone()))?
            ).map_err(|_| FromRowError(row.clone()))?,
            phone_number: row.get("phone_number").ok_or(FromRowError(row.clone()))?,
            avatar: row.get("avatar").ok_or(FromRowError(row.clone()))?,
            is_active: row.get("is_active").ok_or(FromRowError(row.clone()))?,
            deleted_at,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartialUser {
    pub id: i64,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub date_of_birth: NaiveDate,
    pub phone_number: Option<String>,
    pub avatar: Option<String>,
    pub is_active: bool,
}

impl FromRow for PartialUser {
    fn from_row_opt(row: Row) -> Result<Self, FromRowError> {
        Ok(PartialUser {
            id: row.get("id").ok_or(FromRowError(row.clone()))?,
            email: row.get("email").ok_or(FromRowError(row.clone()))?,
            first_name: row.get("first_name").ok_or(FromRowError(row.clone()))?,
            last_name: row.get("last_name").ok_or(FromRowError(row.clone()))?,
            date_of_birth: convert_to_naive_date(
                row.get("date_of_birth").ok_or(FromRowError(row.clone()))?
            ).map_err(|_| FromRowError(row.clone()))?,
            phone_number: row.get("phone_number").ok_or(FromRowError(row.clone()))?,
            avatar: row.get("avatar").ok_or(FromRowError(row.clone()))?,
            is_active: row.get("is_active").ok_or(FromRowError(row.clone()))?,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestCreateUser {
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub date_of_birth: NaiveDate,
    pub phone_number: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RequestUpdateUser {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub date_of_birth: Option<NaiveDate>,
    pub phone_number: Option<String>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestUpdateUserCredentials {
    pub email: Option<String>,
    pub password: Option<String>,
}
