use chrono::NaiveDateTime;
use serde::{ Serialize, Deserialize };
use mysql::*;
use mysql::prelude::*;

use crate::utilities::parse_chrono::convert_to_naive_date_time;

pub fn create_open_shifts_table_query() -> String {
    "
    CREATE TABLE IF NOT EXISTS open_shifts (
        id BIGINT NOT NULL PRIMARY KEY,
        schedule_id BIGINT NOT NULL,
        job_id BIGINT NOT NULL,
        start_time TIME NOT NULL,
        end_time TIME NOT NULL,
        pay_rate FLOAT,
        FOREIGN KEY (schedule_id) REFERENCES work_schedules(id) ON DELETE CASCADE,
        FOREIGN KEY (job_id) REFERENCES org_jobs(id) ON DELETE CASCADE
    );
    ".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenShift {
    pub id: i64,
    pub schedule_id: i64,
    pub job_id: i64,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub pay_rate: Option<f32>,
}

impl FromRow for OpenShift {
    fn from_row_opt(row: Row) -> Result<Self, FromRowError> {
        Ok(OpenShift {
            id: row.get("id").ok_or(FromRowError(row.clone()))?,
            schedule_id: row.get("schedule_id").ok_or(FromRowError(row.clone()))?,
            job_id: row.get("job_id").ok_or(FromRowError(row.clone()))?,
            start_time: convert_to_naive_date_time(
                row.get("start_time").ok_or(FromRowError(row.clone()))?
            ).map_err(|_| FromRowError(row.clone()))?,
            end_time: convert_to_naive_date_time(
                row.get("end_time").ok_or(FromRowError(row.clone()))?
            ).map_err(|_| FromRowError(row.clone()))?,
            pay_rate: row.get("pay_rate").ok_or(FromRowError(row.clone()))?,
        })
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct RequestCreateOpenShift {
    pub user_id: i64,
    pub schedule_id: i64,
    pub job_id: i64,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub pay_rate: Option<f32>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct RequestUpdateOpenShift {
    pub job_id: Option<i64>,
    pub start_time: Option<NaiveDateTime>,
    pub end_time: Option<NaiveDateTime>,
    pub pay_rate: Option<f32>,
}
