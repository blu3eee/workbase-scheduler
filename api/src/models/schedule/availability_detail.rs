use std::str::FromStr;

use chrono::NaiveTime;
use mysql::*;
use mysql::prelude::*;
use serde::{ Serialize, Deserialize };

use crate::{ utilities::parse_chrono::convert_to_naive_time, snowflake::SnowflakeId };

pub fn create_availability_details_table_query() -> String {
    "
    CREATE TABLE IF NOT EXISTS availability_details (
        request_id BIGINT NOT NULL,
        day_of_week ENUM('MONDAY', 'TUESDAY', 'WEDNESDAY', 'THURSDAY', 'FRIDAY', 'SATURDAY', 'SUNDAY') NOT NULL,
        is_available BOOLEAN NOT NULL,
        whole_day BOOLEAN NOT NULL DEFAULT FALSE,
        start_time TIME,
        end_time TIME,
        preferred_start_time TIME,
        preferred_end_time TIME,
        FOREIGN KEY (request_id) REFERENCES availability_requests(id) ON DELETE CASCADE
    );
    ".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvailabilityDetail {
    pub request_id: i64,
    pub day_of_week: DayOfWeek,
    pub is_available: bool,
    pub whole_day: bool,
    pub start_time: Option<NaiveTime>,
    pub end_time: Option<NaiveTime>,
    pub preferred_start_time: Option<NaiveTime>,
    pub preferred_end_time: Option<NaiveTime>,
}

impl FromRow for AvailabilityDetail {
    fn from_row_opt(row: Row) -> Result<Self, FromRowError> {
        let day_of_week: String = row.get("day_of_week").ok_or(FromRowError(row.clone()))?;

        let preferred_start_time = if
            let Some(start_time) = row.get("preferred_start_time").ok_or(FromRowError(row.clone()))?
        {
            Some(convert_to_naive_time(start_time).map_err(|_| FromRowError(row.clone()))?)
        } else {
            None
        };
        let preferred_end_time = if
            let Some(end_time) = row.get("preferred_end_time").ok_or(FromRowError(row.clone()))?
        {
            Some(convert_to_naive_time(end_time).map_err(|_| FromRowError(row.clone()))?)
        } else {
            None
        };

        let start_time = if
            let Some(start_time) = row.get("start_time").ok_or(FromRowError(row.clone()))?
        {
            Some(convert_to_naive_time(start_time).map_err(|_| FromRowError(row.clone()))?)
        } else {
            None
        };
        let end_time = if
            let Some(end_time) = row.get("end_time").ok_or(FromRowError(row.clone()))?
        {
            Some(convert_to_naive_time(end_time).map_err(|_| FromRowError(row.clone()))?)
        } else {
            None
        };

        Ok(AvailabilityDetail {
            request_id: row.get("request_id").ok_or(FromRowError(row.clone()))?,
            day_of_week: DayOfWeek::from_str(&day_of_week).map_err(|_| FromRowError(row.clone()))?,
            is_available: row.get("is_available").ok_or(FromRowError(row.clone()))?,
            whole_day: row.get("whole_day").ok_or(FromRowError(row.clone()))?,
            start_time,
            end_time,
            preferred_start_time,
            preferred_end_time,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct RequestCreateAvailabilityDetail {
    pub request_id: SnowflakeId,
    pub day_of_week: DayOfWeek,
    pub is_available: bool,
    pub whole_day: bool,
    pub start_time: Option<NaiveTime>,
    pub end_time: Option<NaiveTime>,
    pub preferred_start_time: Option<NaiveTime>,
    pub preferred_end_time: Option<NaiveTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum DayOfWeek {
    #[default]
    MONDAY,
    TUESDAY,
    WEDNESDAY,
    THURSDAY,
    FRIDAY,
    SATURDAY,
    SUNDAY,
}

impl FromStr for DayOfWeek {
    type Err = DayOfWeekParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "MONDAY" => Ok(DayOfWeek::MONDAY),
            "TUESDAY" => Ok(DayOfWeek::TUESDAY),
            "WEDNESDAY" => Ok(DayOfWeek::WEDNESDAY),
            "THURSDAY" => Ok(DayOfWeek::THURSDAY),
            "FRIDAY" => Ok(DayOfWeek::FRIDAY),
            "SATURDAY" => Ok(DayOfWeek::SATURDAY),
            "SUNDAY" => Ok(DayOfWeek::SUNDAY),
            _ => Err(DayOfWeekParseError),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DayOfWeekParseError;

impl std::fmt::Display for DayOfWeekParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid value for DayOfWeek")
    }
}

impl std::error::Error for DayOfWeekParseError {}

impl ToString for DayOfWeek {
    fn to_string(&self) -> String {
        match self {
            DayOfWeek::MONDAY => "MONDAY".to_string(),
            DayOfWeek::TUESDAY => "TUESDAY".to_string(),
            DayOfWeek::WEDNESDAY => "WEDNESDAY".to_string(),
            DayOfWeek::THURSDAY => "THURSDAY".to_string(),
            DayOfWeek::FRIDAY => "FRIDAY".to_string(),
            DayOfWeek::SATURDAY => "SATURDAY".to_string(),
            DayOfWeek::SUNDAY => "SUNDAY".to_string(),
        }
    }
}
