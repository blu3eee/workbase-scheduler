use serde::{ Serialize, Deserialize };
use mysql::*;
use mysql::prelude::*;

use crate::snowflake::SnowflakeId;
use crate::utilities::parse_chrono::convert_to_naive_date;
use super::user::PartialUser;

pub mod company_location;
pub mod location_department;
pub mod location_holiday_setting;
pub mod location_operation_hour;
pub mod location_shift_feedback;
pub mod company_employee;
pub mod company_onboarding_invite;
pub mod department_role;

pub fn create_companies_table_query() -> String {
    "
    CREATE TABLE IF NOT EXISTS companies (
        id BIGINT NOT NULL PRIMARY KEY,
        name VARCHAR(100) NOT NULL,
        description TEXT,
        icon VARCHAR(255) NULL,
        last_employee_id INT NOT NULL DEFAULT 0,
        owner_id BIGINT NOT NULL,
        FOREIGN KEY (owner_id) REFERENCES users(id) ON DELETE CASCADE
    );
    ".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Company {
    pub id: SnowflakeId,
    pub name: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub last_employee_id: i32,
    pub owner_id: i64,
    pub owner: Option<PartialUser>,
}

impl FromRow for Company {
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

        Ok(Company {
            id: row.get("id").ok_or(FromRowError(row.clone()))?,
            name: row.get("name").ok_or(FromRowError(row.clone()))?,
            description: row.get("description").ok_or(FromRowError(row.clone()))?,
            icon: row.get("icon").ok_or(FromRowError(row.clone()))?,
            last_employee_id: row.get("last_employee_id").ok_or(FromRowError(row.clone()))?,
            owner_id: row.get("owner_id").ok_or(FromRowError(row.clone()))?,
            owner,
        })
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct RequestCreateCompany {
    pub name: String,
    pub description: Option<String>,
    pub owner_id: i64,
    pub timezone: Option<String>,
    pub icon: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct RequestUpdateCompany {
    pub name: Option<String>,
    pub description: Option<String>,
    pub owner_id: Option<i64>,
    pub timezone: Option<String>,
    pub icon: Option<String>,
}
