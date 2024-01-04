use std::str::FromStr;

use serde::{ Serialize, Deserialize };

pub mod availability;
pub mod availability_detail;
pub mod request_status;
pub mod shift_cover;
pub mod shift_trade;
pub mod shift_pickup;
pub mod open_shift;
pub mod shift;
pub mod work_schedule;
pub mod timeoff_request;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ShiftRequestStatus {
    PENDING,
    PEER_ACCEPTED,
    PEER_DECLINED,
    APPROVED,
    DECLINED,
    CANCELLED,
}

impl FromStr for ShiftRequestStatus {
    type Err = ShiftRequestStatusParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "PENDING" => Ok(ShiftRequestStatus::PENDING),
            "APPROVED" => Ok(ShiftRequestStatus::APPROVED),
            "DECLINED" => Ok(ShiftRequestStatus::DECLINED),
            "CANCELLED" => Ok(ShiftRequestStatus::CANCELLED),
            "PEER_ACCEPTED" => Ok(ShiftRequestStatus::PEER_ACCEPTED),
            "PEER_DECLINED" => Ok(ShiftRequestStatus::PEER_DECLINED),
            _ => Err(ShiftRequestStatusParseError),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ShiftRequestStatusParseError;

impl std::fmt::Display for ShiftRequestStatusParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid value for ShiftRequestStatus")
    }
}

impl std::error::Error for ShiftRequestStatusParseError {}

impl ToString for ShiftRequestStatus {
    fn to_string(&self) -> String {
        match self {
            ShiftRequestStatus::PENDING => "PENDING".to_string(),
            ShiftRequestStatus::APPROVED => "APPROVED".to_string(),
            ShiftRequestStatus::DECLINED => "DECLINED".to_string(),
            ShiftRequestStatus::CANCELLED => "CANCELLED".to_string(),
            ShiftRequestStatus::PEER_ACCEPTED => "PEER_ACCEPTED".to_string(),
            ShiftRequestStatus::PEER_DECLINED => "PEER_DECLINED".to_string(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct RequestUpdateShiftRequest {
    pub status: Option<ShiftRequestStatus>, // Consider using an Enum in Rust for stronger type safety
    pub admin_id: Option<i64>,
    pub note: Option<String>,
}
