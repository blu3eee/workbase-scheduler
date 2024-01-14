use chrono::{ NaiveTime, NaiveDate };
use serde::{ Serialize, Deserialize };
use mysql::*;
use mysql::prelude::*;

use crate::snowflake::SnowflakeId;
use crate::utilities::parse_chrono::{ convert_to_naive_date, convert_to_naive_time };

pub fn create_company_location_holiday_settings_table_query() -> String {
    "
    CREATE TABLE IF NOT EXISTS location_holiday_settings (
        id BIGINT NOT NULL PRIMARY KEY,
        location_id BIGINT NOT NULL,
        holiday DATE NOT NULL,
        is_closed BOOLEAN NOT NULL DEFAULT TRUE,
        factor FLOAT,
        open_time TIME,
        close_time TIME,
        FOREIGN KEY (location_id) REFERENCES company_locations(id) ON DELETE CASCADE,
        UNIQUE KEY location_holiday_setting (location_id, holiday)
    );
    ".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationHolidaySetting {
    pub id: SnowflakeId,
    pub location_id: SnowflakeId,
    pub holiday: NaiveDate,
    pub is_closed: bool,
    pub factor: Option<f32>,
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
        Ok(LocationHolidaySetting {
            id: row.get("id").ok_or(FromRowError(row.clone()))?,
            location_id: row.get("location_id").ok_or(FromRowError(row.clone()))?,
            holiday: convert_to_naive_date(
                row.get("holiday").ok_or(FromRowError(row.clone()))?
            ).map_err(|_| FromRowError(row.clone()))?,
            is_closed: row.get("is_closed").ok_or(FromRowError(row.clone()))?,
            factor: row.get("factor").ok_or(FromRowError(row.clone()))?,
            open_time,
            close_time,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestCreateLocationHolidaySetting {
    pub location_id: SnowflakeId,
    pub holiday: String,
    pub is_closed: bool,
    pub factor: Option<f32>,
    pub open_time: Option<NaiveTime>,
    pub close_time: Option<NaiveTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RequestUpdateLocationHolidaySetting {
    pub location_id: SnowflakeId,
    pub holiday: String,
    pub is_closed: Option<bool>,
    pub factor: Option<f32>,
    pub open_time: Option<NaiveTime>,
    pub close_time: Option<NaiveTime>,
}
