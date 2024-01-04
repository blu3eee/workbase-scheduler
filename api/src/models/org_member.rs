use chrono::NaiveDateTime;
use mysql::{ prelude::FromRow, FromRowError };
use serde::{ Serialize, Deserialize };

use crate::utilities::parse_chrono::convert_to_naive_date_time;

pub fn create_org_members_table_query() -> String {
    "
    CREATE TABLE IF NOT EXISTS org_members (
        user_id BIGINT NOT NULL,
        org_id BIGINT NOT NULL,
        job_id BIGINT NOT NULL,
        joined_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
        FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
        FOREIGN KEY (org_id) REFERENCES organizations(id) ON DELETE CASCADE,
        FOREIGN KEY (job_id) REFERENCES org_jobs(id) ON DELETE CASCADE,
        UNIQUE KEY user_org_unique (user_id, org_id)
    );
    ".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrgMember {
    pub user_id: i64,
    pub org_id: i64,
    pub job_id: i64,
    pub joined_at: NaiveDateTime,
}

// For creating a new user-organization relationship
#[derive(Debug, Clone, Deserialize)]
pub struct RequestCreateOrgMember {
    pub user_id: i64,
    pub org_id: i64,
    pub job_id: i64,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct RequestUpdateOrgMember {
    pub user_id: i64,
    pub org_id: i64,
    pub job_id: Option<i64>,
}

impl FromRow for OrgMember {
    fn from_row_opt(row: mysql::Row) -> Result<Self, mysql::FromRowError> where Self: Sized {
        Ok(OrgMember {
            user_id: row.get("user_id").ok_or(FromRowError(row.clone()))?,
            org_id: row.get("org_id").ok_or(FromRowError(row.clone()))?,
            job_id: row.get("job_id").ok_or(FromRowError(row.clone()))?,
            joined_at: convert_to_naive_date_time(
                row.get("joined_at").ok_or(FromRowError(row.clone()))?
            ).map_err(|_| FromRowError(row.clone()))?,
        })
    }
}
