use std::str::FromStr;

use chrono::{ NaiveDate, NaiveDateTime };
use mysql::*;
use mysql::prelude::*;
use serde::{ Serialize, Deserialize };

use crate::utilities::parse_chrono::{ convert_to_naive_date, convert_to_naive_date_time };

use super::{
    request_status::ScheduleRequestStatus,
    availability_detail::RequestCreateAvailabilityDetail,
};

pub fn create_availability_requests_table_query() -> String {
    "
    CREATE TABLE IF NOT EXISTS availability_requests (
        id BIGINT NOT NULL PRIMARY KEY,
        user_id BIGINT NOT NULL,
        org_id BIGINT NOT NULL,
        start_date DATE NOT NULL,
        status ENUM('PENDING', 'CANCELLED', 'APPROVED', 'DENIED') NOT NULL DEFAULT 'PENDING',
        updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
        FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
        FOREIGN KEY (org_id) REFERENCES organizations(id) ON DELETE CASCADE
    );
    ".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvailabilityRequest {
    pub id: i64,
    pub user_id: i64,
    pub org_id: i64,
    pub start_date: NaiveDate,
    pub status: ScheduleRequestStatus,
    pub updated_at: NaiveDateTime,
}

impl FromRow for AvailabilityRequest {
    fn from_row_opt(row: Row) -> Result<Self, FromRowError> {
        let status: String = row.get("status").ok_or(FromRowError(row.clone()))?;
        Ok(AvailabilityRequest {
            id: row.get("id").ok_or(FromRowError(row.clone()))?,
            user_id: row.get("user_id").ok_or(FromRowError(row.clone()))?,
            org_id: row.get("org_id").ok_or(FromRowError(row.clone()))?,
            start_date: convert_to_naive_date(
                row.get("start_date").ok_or(FromRowError(row.clone()))?
            ).map_err(|_| FromRowError(row.clone()))?,
            status: ScheduleRequestStatus::from_str(&status).map_err(|_|
                FromRowError(row.clone())
            )?,
            updated_at: convert_to_naive_date_time(
                row.get("updated_at").ok_or(FromRowError(row.clone()))?
            ).map_err(|_| FromRowError(row.clone()))?,
        })
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct RequestCreateAvailability {
    pub user_id: i64,
    pub org_id: i64,
    pub start_date: NaiveDate,
    pub details: Vec<RequestCreateAvailabilityDetail>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RequestUpdateAvailability {
    pub status: Option<ScheduleRequestStatus>,
}
