use chrono::NaiveDateTime;
use serde::{ Serialize, Deserialize };
use mysql::*;
use mysql::prelude::*;

use crate::utilities::parse_chrono::{ convert_to_naive_date, convert_to_naive_date_time };
use super::user::PartialUser;

pub fn create_organizations_table_query() -> String {
    "
    CREATE TABLE IF NOT EXISTS organizations (
        id BIGINT NOT NULL PRIMARY KEY,
        name VARCHAR(100) NOT NULL,
        description TEXT,
        owner_id BIGINT NOT NULL,
        timezone VARCHAR(50) NOT NULL DEFAULT 'America/Los_Angeles',
        icon VARCHAR(255) NULL,
        is_active BOOLEAN NOT NULL DEFAULT TRUE,
        updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
        FOREIGN KEY (owner_id) REFERENCES users(id) ON DELETE CASCADE
    );
    ".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Organization {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub owner_id: i64,
    pub timezone: String,
    pub icon: Option<String>,
    pub is_active: bool,
    pub updated_at: NaiveDateTime,
    pub owner: Option<PartialUser>,
}

impl FromRow for Organization {
    fn from_row_opt(row: Row) -> Result<Self, FromRowError> {
        let owner: Option<PartialUser> = if let Some(email) = row.get("owner_email") {
            Some(PartialUser {
                id: row.get("owner_id").ok_or(FromRowError(row.clone()))?,
                email,
                first_name: row.get("owner_first_name").ok_or(FromRowError(row.clone()))?,
                last_name: row.get("owner_last_name").ok_or(FromRowError(row.clone()))?,
                date_of_birth: convert_to_naive_date(
                    row.get("owner_date_of_birth").ok_or(FromRowError(row.clone()))?
                ).map_err(|_| FromRowError(row.clone()))?,
                phone_number: row.get("owner_phone_number").ok_or(FromRowError(row.clone()))?,
                avatar: row.get("owner_avatar").ok_or(FromRowError(row.clone()))?,
                is_active: row.get("owner_is_active").ok_or(FromRowError(row.clone()))?,
            })
        } else {
            None
        };

        Ok(Organization {
            id: row.get("id").ok_or(FromRowError(row.clone()))?,
            name: row.get("name").ok_or(FromRowError(row.clone()))?,
            description: row.get("description").ok_or(FromRowError(row.clone()))?,
            owner_id: row.get("owner_id").ok_or(FromRowError(row.clone()))?,
            timezone: row.get("timezone").ok_or(FromRowError(row.clone()))?,
            icon: row.get("icon").ok_or(FromRowError(row.clone()))?,
            is_active: row.get("is_active").ok_or(FromRowError(row.clone()))?,
            updated_at: convert_to_naive_date_time(
                row.get("updated_at").ok_or(FromRowError(row.clone()))?
            ).map_err(|_| FromRowError(row.clone()))?,
            owner,
        })
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct RequestCreateOrganization {
    pub name: String,
    pub description: Option<String>,
    pub owner_id: i64,
    pub timezone: Option<String>,
    pub icon: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct RequestUpdateOrganization {
    pub id: i64,
    pub name: Option<String>,
    pub description: Option<String>,
    pub owner_id: Option<i64>,
    pub timezone: Option<String>,
    pub icon: Option<String>,
}
