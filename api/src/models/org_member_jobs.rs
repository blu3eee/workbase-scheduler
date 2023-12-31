use chrono::NaiveDateTime;
use mysql::*;
use mysql::prelude::*;
use serde::{ Serialize, Deserialize };

use super::convert_to_naive_date_time;

pub fn create_org_member_job_table() -> String {
    "
    CREATE TABLE IF NOT EXISTS org_member_jobs (
        id INT AUTO_INCREMENT PRIMARY KEY,
        member_id INT NOT NULL,
        job_id INT NOT NULL,
        pay_rate FLOAT NULL,
        start_date TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
        end_date TIMESTAMP NULL,
        UNIQUE KEY member_job_start_date_unique (member_id, job_id, start_date),
        FOREIGN KEY (member_id) REFERENCES org_members(id) ON DELETE CASCADE,
        FOREIGN KEY (job_id) REFERENCES org_jobs(id) ON DELETE CASCADE
    );
    ".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrgMemberJob {
    pub id: i32,
    pub member_id: i32,
    pub job_id: i32,
    pub pay_rate: Option<f32>,
    pub start_date: NaiveDateTime,
    pub end_date: Option<NaiveDateTime>,
}

impl FromRow for OrgMemberJob {
    fn from_row_opt(row: Row) -> Result<Self, FromRowError> {
        Ok(OrgMemberJob {
            id: row.get("id").ok_or(FromRowError(row.clone()))?,
            member_id: row.get("member_id").ok_or(FromRowError(row.clone()))?,
            job_id: row.get("job_id").ok_or(FromRowError(row.clone()))?,
            pay_rate: row.get("pay_rate").ok_or(FromRowError(row.clone()))?,
            start_date: convert_to_naive_date_time(
                row.get("start_date").ok_or(FromRowError(row.clone()))?
            ),
            end_date: row.get("end_date").map(|val| convert_to_naive_date_time(val)),
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestCreateOrgMemberJob {
    pub member_id: i32,
    pub job_id: i32,
    pub pay_rate: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RequestUpdateOrgMemberJob {
    pub id: i32,
    pub pay_rate: Option<f32>,
    pub end_date: Option<NaiveDateTime>,
}
