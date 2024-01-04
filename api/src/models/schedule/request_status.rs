use std::str::FromStr;

use serde::{ Serialize, Deserialize };

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ScheduleRequestStatus {
    PENDING,
    CANCELLED,
    DECLINED,
    APPROVED,
}

impl FromStr for ScheduleRequestStatus {
    type Err = ScheduleRequestStatusParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "PENDING" => Ok(ScheduleRequestStatus::PENDING),
            "APPROVED" => Ok(ScheduleRequestStatus::APPROVED),
            "DECLINED" => Ok(ScheduleRequestStatus::DECLINED),
            "CANCELLED" => Ok(ScheduleRequestStatus::CANCELLED),
            _ => Err(ScheduleRequestStatusParseError),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ScheduleRequestStatusParseError;

impl std::fmt::Display for ScheduleRequestStatusParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid value for ScheduleRequestStatus")
    }
}

impl std::error::Error for ScheduleRequestStatusParseError {}

impl ToString for ScheduleRequestStatus {
    fn to_string(&self) -> String {
        match self {
            ScheduleRequestStatus::PENDING => "PENDING".to_string(),
            ScheduleRequestStatus::APPROVED => "APPROVED".to_string(),
            ScheduleRequestStatus::DECLINED => "DECLINED".to_string(),
            ScheduleRequestStatus::CANCELLED => "CANCELLED".to_string(),
        }
    }
}
