use serde::{ Serialize, Deserialize };
use mysql::*;
use mysql::prelude::*;

use crate::snowflake::SnowflakeId;

pub fn create_company_location_departments_table_query() -> String {
    "
    CREATE TABLE IF NOT EXISTS location_departments (
        id BIGINT NOT NULL PRIMARY KEY,
        location_id BIGINT NOT NULL,
        name VARCHAR(100) NOT NULL,
        is_active BOOLEAN NOT NULL DEFAULT TRUE,
        FOREIGN KEY (location_id) REFERENCES company_locations(id) ON DELETE CASCADE
    );
    ".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationDepartment {
    pub id: SnowflakeId,
    pub location_id: SnowflakeId,
    pub name: String,
    pub is_active: bool,
}

impl FromRow for LocationDepartment {
    fn from_row_opt(row: Row) -> Result<Self, FromRowError> {
        Ok(LocationDepartment {
            id: row.get("id").ok_or(FromRowError(row.clone()))?,
            name: row.get("name").ok_or(FromRowError(row.clone()))?,
            location_id: row.get("location_id").ok_or(FromRowError(row.clone()))?,
            is_active: row.get("is_active").ok_or(FromRowError(row.clone()))?,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestCreateLocationDepartment {
    pub location_id: SnowflakeId,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RequestUpdateLocationDepartment {
    pub id: SnowflakeId,
    pub name: Option<String>,
    pub is_active: Option<bool>,
}
