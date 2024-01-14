use std::str::FromStr;

use chrono::NaiveTime;
use serde::{ Serialize, Deserialize };
use mysql::*;
use mysql::prelude::*;

use crate::models::schedule::availability_detail::DayOfWeek;
use crate::snowflake::SnowflakeId;
use crate::utilities::parse_chrono::convert_to_naive_time;

pub fn create_company_location_operation_hours_table_query() -> String {
    "
    CREATE TABLE IF NOT EXISTS location_operation_hours (
        id BIGINT NOT NULL PRIMARY KEY,
        location_id BIGINT NOT NULL,
        day_of_week ENUM('MONDAY', 'TUESDAY', 'WEDNESDAY', 'THURSDAY', 'FRIDAY', 'SATURDAY', 'SUNDAY') NOT NULL,
        is_closed BOOLEAN NOT NULL DEFAULT TRUE,
        open_time TIME,
        close_time TIME,
        FOREIGN KEY (location_id) REFERENCES company_locations(id) ON DELETE CASCADE,
        UNIQUE KEY daily_operation_hour (location_id, day_of_week)
    );
    ".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationHolidaySetting {
    pub id: SnowflakeId,
    pub location_id: SnowflakeId,
    pub day_of_week: DayOfWeek,
    pub is_closed: bool,
    pub open_time: Option<NaiveTime>,
    pub close_time: Option<NaiveTime>,
}

impl FromRow for LocationHolidaySetting {
    fn from_row_opt(row: Row) -> Result<Self, FromRowError> {
        let open_time = if
            let Some(start_time) = row.get("preferred_start_time").ok_or(FromRowError(row.clone()))?
        {
            Some(convert_to_naive_time(start_time).map_err(|_| FromRowError(row.clone()))?)
        } else {
            None
        };
        let close_time = if
            let Some(end_time) = row.get("preferred_end_time").ok_or(FromRowError(row.clone()))?
        {
            Some(convert_to_naive_time(end_time).map_err(|_| FromRowError(row.clone()))?)
        } else {
            None
        };
        let day_of_week: String = row.get("day_of_week").ok_or(FromRowError(row.clone()))?;
        Ok(LocationHolidaySetting {
            id: row.get("id").ok_or(FromRowError(row.clone()))?,
            location_id: row.get("location_id").ok_or(FromRowError(row.clone()))?,
            day_of_week: DayOfWeek::from_str(&day_of_week).map_err(|_| FromRowError(row.clone()))?,
            is_closed: row.get("is_closed").ok_or(FromRowError(row.clone()))?,
            open_time,
            close_time,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestCreateLocationHolidaySetting {
    pub location_id: SnowflakeId,
    pub day_of_week: String,
    pub is_closed: bool,
    pub open_time: Option<NaiveTime>,
    pub close_time: Option<NaiveTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RequestUpdateLocationHolidaySetting {
    pub location_id: SnowflakeId,
    pub day_of_week: String,
    pub is_closed: Option<bool>,
    pub open_time: Option<NaiveTime>,
    pub close_time: Option<NaiveTime>,
}
