use std::str::FromStr;

use serde::{ Serialize, Deserialize };
use mysql::*;
use mysql::prelude::*;

use crate::models::schedule::request_status::GeneralStatus;
use crate::snowflake::SnowflakeId;

// Snowflake ID
pub fn create_company_onboarding_invites_table_query() -> String {
    "
    CREATE TABLE IF NOT EXISTS company_onboarding_invites (
        id BIGINT NOT NULL PRIMARY KEY,
        company_id BIGINT NOT NULL,
        location_id BIGINT NOT NULL,
        email VARCHAR(100) NOT NULL,
        name VARCHAR(100) NOT NULL,
        role_id BIGINT,
        status ENUM('PENDING', 'CANCELLED', 'APPROVED', 'DENIED') NOT NULL DEFAULT 'PENDING',
        FOREIGN KEY (company_id) REFERENCES companies(id) ON DELETE CASCADE,
        FOREIGN KEY (location_id) REFERENCES company_locations(id) ON DELETE CASCADE,
        FOREIGN KEY (role_id) REFERENCES department_roles(id) ON DELETE SET NULL
    );
    ".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompanyOnboardingInvite {
    pub id: SnowflakeId,
    pub company_id: SnowflakeId,
    pub location_id: SnowflakeId,
    pub email: String,
    pub name: String,
    pub role_id: Option<SnowflakeId>,
    pub status: GeneralStatus,
}

impl FromRow for CompanyOnboardingInvite {
    fn from_row_opt(row: Row) -> Result<Self, FromRowError> {
        let status: String = row.get("status").ok_or(FromRowError(row.clone()))?;
        Ok(CompanyOnboardingInvite {
            id: row.get("id").ok_or(FromRowError(row.clone()))?,
            company_id: row.get("company_id").ok_or(FromRowError(row.clone()))?,
            location_id: row.get("location_id").ok_or(FromRowError(row.clone()))?,
            name: row.get("name").ok_or(FromRowError(row.clone()))?,
            email: row.get("email").ok_or(FromRowError(row.clone()))?,
            role_id: row.get("role_id").ok_or(FromRowError(row.clone()))?,
            status: GeneralStatus::from_str(&status).map_err(|_| FromRowError(row.clone()))?,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestCreateCompanyOnboardingInvite {
    pub company_id: SnowflakeId,
    pub location_id: SnowflakeId,
    pub email: String,
    pub name: String,
    pub role_id: Option<SnowflakeId>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RequestUpdateCompanyOnboardingInvite {
    pub status: Option<GeneralStatus>,
}
