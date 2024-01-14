use chrono::NaiveDateTime;
use serde::{ Serialize, Deserialize };
use mysql::*;
use mysql::prelude::*;

use crate::utilities::parse_chrono::convert_to_naive_date_time;

pub fn create_shifts_table_query() -> String {
    "
    CREATE TABLE IF NOT EXISTS shifts (
        id BIGINT NOT NULL PRIMARY KEY,
        user_id BIGINT NOT NULL,
        schedule_id BIGINT NOT NULL,
        job_id BIGINT NOT NULL,
        start_time TIMESTAMP NOT NULL,
        end_time TIMESTAMP NOT NULL,
        pay_rate FLOAT,
        note TEXT,
        FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
        FOREIGN KEY (schedule_id) REFERENCES work_schedules(id) ON DELETE CASCADE,
        FOREIGN KEY (job_id) REFERENCES company_jobs(id) ON DELETE CASCADE
    );
    ".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shift {
    pub id: i64,
    pub user_id: i64,
    pub schedule_id: i64,
    pub job_id: i64,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub pay_rate: Option<f32>,
    pub note: Option<String>,
}

impl FromRow for Shift {
    fn from_row_opt(row: Row) -> Result<Self, FromRowError> {
        println!("{row:?}");
        Ok(Shift {
            id: row.get("id").ok_or(FromRowError(row.clone()))?,
            user_id: row.get("user_id").ok_or(FromRowError(row.clone()))?,
            schedule_id: row.get("schedule_id").ok_or(FromRowError(row.clone()))?,
            job_id: row.get("job_id").ok_or(FromRowError(row.clone()))?,
            start_time: convert_to_naive_date_time(
                row.get("start_time").ok_or(FromRowError(row.clone()))?
            ).map_err(|_| FromRowError(row.clone()))?,
            end_time: convert_to_naive_date_time(
                row.get("end_time").ok_or(FromRowError(row.clone()))?
            ).map_err(|_| FromRowError(row.clone()))?,
            pay_rate: row.get("pay_rate").ok_or(FromRowError(row.clone()))?,
            note: row.get("note").ok_or(FromRowError(row.clone()))?,
        })
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct RequestCreateShift {
    pub user_id: i64,
    pub schedule_id: i64,
    pub job_id: i64,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub pay_rate: Option<f32>,
    pub note: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct RequestUpdateShift {
    pub job_id: Option<i64>,
    pub start_time: Option<NaiveDateTime>,
    pub end_time: Option<NaiveDateTime>,
    pub pay_rate: Option<f32>,
    pub note: Option<String>,
}
