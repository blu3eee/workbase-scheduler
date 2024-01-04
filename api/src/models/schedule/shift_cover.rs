use std::str::FromStr;

use chrono::NaiveDateTime;
use serde::{ Serialize, Deserialize };
use mysql::{ *, prelude::FromRow };

use crate::utilities::parse_chrono::convert_to_naive_date_time;

use super::ShiftRequestStatus;

pub fn create_shift_covers_table_query() -> String {
    "
    CREATE TABLE IF NOT EXISTS shift_covers (
        id BIGINT NOT NULL PRIMARY KEY,
        shift_id BIGINT NOT NULL,
        cover_user_id BIGINT NOT NULL,
        status ENUM('PENDING', 'PEER_ACCEPTED', 'PEER_DECLINED', 'APPROVED', 'DECLINED', 'CANCELLED') NOT NULL DEFAULT 'PENDING',
        admin_id BIGINT,
        note TEXT,
        updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
        FOREIGN KEY (shift_id) REFERENCES shifts(id) ON DELETE CASCADE,
        FOREIGN KEY (cover_user_id) REFERENCES users(id) ON DELETE CASCADE,
        FOREIGN KEY (admin_id) REFERENCES users(id) ON DELETE SET NULL
    );
    ".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShiftCover {
    pub id: i64,
    pub shift_id: i64,
    pub cover_user_id: i64,
    pub status: ShiftRequestStatus,
    pub admin_id: Option<i64>,
    pub note: Option<String>,
    pub updated_at: NaiveDateTime,
}

impl FromRow for ShiftCover {
    fn from_row_opt(row: Row) -> Result<Self, FromRowError> {
        let status: String = row.get("status").ok_or(FromRowError(row.clone()))?;

        Ok(ShiftCover {
            id: row.get("id").ok_or(FromRowError(row.clone()))?,
            shift_id: row.get("shift_id").ok_or(FromRowError(row.clone()))?,
            cover_user_id: row.get("cover_user_id").ok_or(FromRowError(row.clone()))?,
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
pub struct RequestCreateShiftCover {
    pub shift_id: i64,
    pub cover_user_id: i64,
}
