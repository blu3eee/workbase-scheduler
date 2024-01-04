use chrono::NaiveDateTime;
use mysql::*;
use mysql::prelude::*;
use serde::{ Serialize, Deserialize };

use crate::utilities::parse_chrono::convert_to_naive_date_time;

pub fn create_org_job_table() -> String {
    "
    CREATE TABLE IF NOT EXISTS org_jobs (
        id BIGINT NOT NULL PRIMARY KEY,
        org_id BIGINT,
        name VARCHAR(100) NOT NULL,
        description TEXT,
        base_pay_rate FLOAT,
        color VARCHAR(6),
        update_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
        FOREIGN KEY (org_id) REFERENCES organizations(id) ON DELETE CASCADE
    );
    ".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrgJob {
    pub id: i64,
    pub org_id: i64,
    pub name: String,
    pub description: Option<String>,
    pub color: Option<String>,
    pub base_pay_rate: f32,
    pub update_at: NaiveDateTime,
}

impl FromRow for OrgJob {
    fn from_row_opt(row: Row) -> Result<Self, FromRowError> {
        Ok(OrgJob {
            id: row.get("id").ok_or(FromRowError(row.clone()))?,
            org_id: row.get("org_id").ok_or(FromRowError(row.clone()))?,
            name: row.get("name").ok_or(FromRowError(row.clone()))?,
            description: row.get("description").ok_or(FromRowError(row.clone()))?,
            base_pay_rate: row.get("base_pay_rate").ok_or(FromRowError(row.clone()))?,
            color: row.get("color").ok_or(FromRowError(row.clone()))?,
            update_at: convert_to_naive_date_time(
                row.get("update_at").ok_or(FromRowError(row.clone()))?
            ).map_err(|_| FromRowError(row.clone()))?,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestCreateOrgJob {
    pub org_id: i64,
    pub name: String,
    pub description: Option<String>,
    pub base_pay_rate: f32,
    pub color: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RequestUpdateOrgJob {
    pub name: Option<String>,
    pub description: Option<String>,
    pub base_pay_rate: Option<f32>,
    pub color: Option<String>,
}
