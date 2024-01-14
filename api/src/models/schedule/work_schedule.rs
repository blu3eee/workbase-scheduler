use chrono::{ NaiveDate, NaiveDateTime };
use serde::{ Serialize, Deserialize };
use mysql::*;
use mysql::prelude::*;

use crate::utilities::parse_chrono::{ convert_to_naive_date_time, convert_to_naive_date };

pub fn create_work_schedules_table_query() -> String {
    "
    CREATE TABLE IF NOT EXISTS work_schedules (
        id BIGINT NOT NULL PRIMARY KEY,
        company_id BIGINT NOT NULL,
        start_date DATE NOT NULL,
        end_date DATE NOT NULL,
        published BOOLEAN NOT NULL DEFAULT FALSE,
        updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
        FOREIGN KEY (company_id) REFERENCES companies(id) ON DELETE CASCADE,
        UNIQUE KEY company_work_schedule (company_id, start_date)
    );
    ".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkSchedule {
    pub id: i64,
    pub company_id: i64,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub published: bool,
    pub updated_at: NaiveDateTime,
}

impl FromRow for WorkSchedule {
    fn from_row_opt(row: Row) -> Result<Self, FromRowError> {
        Ok(WorkSchedule {
            id: row.get("id").ok_or(FromRowError(row.clone()))?,
            company_id: row.get("company_id").ok_or(FromRowError(row.clone()))?,
            published: row.get("published").ok_or(FromRowError(row.clone()))?,
            start_date: convert_to_naive_date(
                row.get("start_date").ok_or(FromRowError(row.clone()))?
            ).map_err(|_| FromRowError(row.clone()))?,
            end_date: convert_to_naive_date(
                row.get("end_date").ok_or(FromRowError(row.clone()))?
            ).map_err(|_| FromRowError(row.clone()))?,
            updated_at: convert_to_naive_date_time(
                row.get("updated_at").ok_or(FromRowError(row.clone()))?
            ).map_err(|_| FromRowError(row.clone()))?,
        })
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct RequestCreateWorkSchedule {
    pub company_id: i64,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct RequestUpdateWorkSchedule {
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub publish: Option<bool>,
}
