use std::str::FromStr;

use chrono::NaiveDateTime;
use mysql::*;
use mysql::prelude::*;
use serde::{ Serialize, Deserialize };

use crate::utilities::parse_chrono::convert_to_naive_date_time;

use super::request_status::ScheduleRequestStatus;

pub fn create_time_off_requests_table_query() -> String {
    "
    CREATE TABLE IF NOT EXISTS time_off_requests (
        id BIGINT NOT NULL PRIMARY KEY,
        user_id BIGINT NOT NULL,
        org_id BIGINT NOT NULL,
        start_time DATE NOT NULL,
        end_time DATE NOT NULL,
        reason TEXT,
        status ENUM('PENDING', 'CANCELLED', 'APPROVED', 'DENIED') NOT NULL DEFAULT 'PENDING',
        updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
        admin_id BIGINT,
        FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
        FOREIGN KEY (org_id) REFERENCES organizations(id) ON DELETE CASCADE
        FOREIGN KEY (admin_id) REFERENCES users(id) ON DELETE SET NULL
    );
    ".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeOffRequest {
    pub id: i64,
    pub user_id: i64,
    pub org_id: i64,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub reason: Option<String>,
    pub status: ScheduleRequestStatus,
    pub updated_at: NaiveDateTime,
    pub admin_id: Option<i64>,
}

impl FromRow for TimeOffRequest {
    fn from_row_opt(row: Row) -> Result<Self, FromRowError> {
        let status: String = row.get("status").ok_or(FromRowError(row.clone()))?;
        Ok(TimeOffRequest {
            id: row.get("id").ok_or(FromRowError(row.clone()))?,
            user_id: row.get("user_id").ok_or(FromRowError(row.clone()))?,
            org_id: row.get("org_id").ok_or(FromRowError(row.clone()))?,
            start_time: convert_to_naive_date_time(
                row.get("start_time").ok_or(FromRowError(row.clone()))?
            ).map_err(|_| FromRowError(row.clone()))?,
            end_time: convert_to_naive_date_time(
                row.get("end_time").ok_or(FromRowError(row.clone()))?
            ).map_err(|_| FromRowError(row.clone()))?,
            reason: row.get("reason").ok_or(FromRowError(row.clone()))?,
            status: ScheduleRequestStatus::from_str(&status).map_err(|_|
                FromRowError(row.clone())
            )?,
            updated_at: convert_to_naive_date_time(
                row.get("updated_at").ok_or(FromRowError(row.clone()))?
            ).map_err(|_| FromRowError(row.clone()))?,
            admin_id: row.get("admin_id").ok_or(FromRowError(row.clone()))?,
        })
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct RequestCreateTimeOff {
    pub user_id: i64,
    pub org_id: i64,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct RequestUpdateTimeOff {
    pub id: i64,
    pub status: Option<ScheduleRequestStatus>,
    pub admin_id: Option<i64>,
    pub reason: Option<String>,
}
