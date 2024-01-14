use std::str::FromStr;

use chrono::NaiveDateTime;
use serde::{ Serialize, Deserialize };
use mysql::{ *, prelude::FromRow };

use crate::utilities::parse_chrono::convert_to_naive_date_time;

use super::ShiftRequestStatus;

pub fn create_shift_trades_table_query() -> String {
    "
    CREATE TABLE IF NOT EXISTS shift_trades (
        id BIGINT NOT NULL PRIMARY KEY,
        shift1_id BIGINT NOT NULL,
        shift2_id BIGINT NOT NULL,
        status ENUM('PENDING', 'PEER_ACCEPTED', 'PEER_DECLINED', 'APPROVED', 'DECLINED', 'CANCELLED') NOT NULL DEFAULT 'PENDING',
        admin_id BIGINT,
        note TEXT,
        updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
        FOREIGN KEY (shift1_id) REFERENCES shifts(id) ON DELETE CASCADE,
        FOREIGN KEY (shift2_id) REFERENCES shifts(id) ON DELETE CASCADE,
        FOREIGN KEY (admin_id) REFERENCES users(id) ON DELETE SET NULL
    );
    ".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShiftTrade {
    pub id: i64,
    pub shift1_id: i64,
    pub shift2_id: i64,
    pub status: ShiftRequestStatus,
    pub admin_id: Option<i64>,
    pub note: Option<String>,
    pub updated_at: NaiveDateTime,
}

impl FromRow for ShiftTrade {
    fn from_row_opt(row: Row) -> Result<Self, FromRowError> {
        let status: String = row.get("status").ok_or(FromRowError(row.clone()))?;

        Ok(ShiftTrade {
            id: row.get("id").ok_or(FromRowError(row.clone()))?,
            shift1_id: row.get("shift1_id").ok_or(FromRowError(row.clone()))?,
            shift2_id: row.get("shift2_id").ok_or(FromRowError(row.clone()))?,
            status: ShiftRequestStatus::from_str(&status).map_err(|_| FromRowError(row.clone()))?,
            admin_id: row.get("admin_id").ok_or(FromRowError(row.clone()))?,
            note: row.get("note").ok_or(FromRowError(row.clone()))?,
            updated_at: convert_to_naive_date_time(
                row.get("updated_at").ok_or(FromRowError(row.clone()))?
            ).map_err(|_| FromRowError(row.clone()))?,
        })
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct RequestCreateShiftTrade {
    pub shift1_id: i64,
    pub shift2_id: i64,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct RequestUpdateShiftCover {
    pub id: i64,
    pub status: Option<ShiftRequestStatus>,
    pub note: Option<String>,
    pub admin_id: Option<i64>,
}
