use std::str::FromStr;

use serde::{ Serialize, Deserialize };

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GeneralStatus {
    PENDING,
    CANCELLED,
    DECLINED,
    APPROVED,
}

impl FromStr for GeneralStatus {
    type Err = GeneralStatusParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "PENDING" => Ok(GeneralStatus::PENDING),
            "APPROVED" => Ok(GeneralStatus::APPROVED),
            "DECLINED" => Ok(GeneralStatus::DECLINED),
            "CANCELLED" => Ok(GeneralStatus::CANCELLED),
            _ => Err(GeneralStatusParseError),
        }
    }
}

#[derive(Debug, Clone)]
pub struct GeneralStatusParseError;

impl std::fmt::Display for GeneralStatusParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid value for ScheduleRequestStatus")
    }
}

impl std::error::Error for GeneralStatusParseError {}

impl ToString for GeneralStatus {
    fn to_string(&self) -> String {
        match self {
            GeneralStatus::PENDING => "PENDING".to_string(),
            GeneralStatus::APPROVED => "APPROVED".to_string(),
            GeneralStatus::DECLINED => "DECLINED".to_string(),
            GeneralStatus::CANCELLED => "CANCELLED".to_string(),
        }
    }
}
