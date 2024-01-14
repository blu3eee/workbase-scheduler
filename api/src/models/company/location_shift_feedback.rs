use serde::{ Serialize, Deserialize };
use mysql::*;
use mysql::prelude::*;

use crate::snowflake::SnowflakeId;

pub fn create_company_location_shift_feedback_settings_table_query() -> String {
    "
    CREATE TABLE IF NOT EXISTS location_shift_feedback_settings (
        location_id BIGINT NOT NULL PRIMARY KEY,
        enabled BOOLEAN NOT NULL DEFAULT FALSE,
        FOREIGN KEY (location_id) REFERENCES company_locations(id) ON DELETE CASCADE
    );
    ".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationShiftFeedbackSetting {
    pub location_id: SnowflakeId,
    pub enabled: bool,
}

impl FromRow for LocationShiftFeedbackSetting {
    fn from_row_opt(row: Row) -> Result<Self, FromRowError> {
        Ok(LocationShiftFeedbackSetting {
            location_id: row.get("location_id").ok_or(FromRowError(row.clone()))?,
            enabled: row.get("enabled").ok_or(FromRowError(row.clone()))?,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestCreateLocationShiftFeedbackSetting {
    pub location_id: SnowflakeId,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RequestUpdateLocationShiftFeedbackSetting {
    pub location_id: SnowflakeId,
    pub enabled: Option<bool>,
}
