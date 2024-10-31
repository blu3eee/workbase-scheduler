use serde::{ Serialize, Deserialize };
use mysql::*;
use mysql::prelude::*;

use crate::snowflake::SnowflakeId;

// Snowflake ID
pub fn create_company_locations_table_query() -> String {
    "
    CREATE TABLE IF NOT EXISTS company_locations (
        id BIGINT NOT NULL PRIMARY KEY,
        company_id BIGINT NOT NULL,
        name VARCHAR(100) NOT NULL,
        timezone VARCHAR(50) NOT NULL DEFAULT 'America/Los_Angeles',
        address TEXT NOT NULL,
        is_active BOOLEAN NOT NULL DEFAULT TRUE,
        FOREIGN KEY (company_id) REFERENCES companies(id) ON DELETE CASCADE,
    );
    ".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompanyLocation {
    pub id: SnowflakeId,
    pub company_id: SnowflakeId,
    pub name: String,
    pub timezone: String,
    pub address: String,
    pub is_active: bool,
}

impl FromRow for CompanyLocation {
    fn from_row_opt(row: Row) -> Result<Self, FromRowError> {
        Ok(CompanyLocation {
            id: row.get("id").ok_or(FromRowError(row.clone()))?,
            company_id: row.get("company_id").ok_or(FromRowError(row.clone()))?,
            name: row.get("name").ok_or(FromRowError(row.clone()))?,
            timezone: row.get("timezone").ok_or(FromRowError(row.clone()))?,
            address: row.get("address").ok_or(FromRowError(row.clone()))?,
            is_active: row.get("is_active").ok_or(FromRowError(row.clone()))?,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestCreateCompanyLocation {
    pub company_id: SnowflakeId,
    pub name: String,
    pub timezone: String,
    pub address: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RequestUpdateCompanyLocation {
    // pub id: SnowflakeId,
    pub name: Option<String>,
    pub timezone: Option<String>,
    pub address: Option<String>,
    pub is_active: Option<bool>,
}
