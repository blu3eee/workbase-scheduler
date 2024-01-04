use std::str::FromStr;

use chrono::{ NaiveDate, NaiveDateTime, NaiveTime };
use mysql::Value;

use crate::models::result::Result;

use super::app_error::BoxedError;

pub fn convert_to_naive_date(value: Value) -> Result<NaiveDate> {
    match value {
        Value::Bytes(date) => {
            let date_str = String::from_utf8(date).map_err(|e| Box::new(e) as BoxedError)?; // Specify error type explicitly

            NaiveDate::from_str(&date_str).map_err(|_| "Failed to parse value to NaiveDate".into())
        }
        Value::Date(year, month, day, _, _, _, _) => {
            NaiveDate::from_ymd_opt(year as i32, month as u32, day as u32).ok_or_else(||
                "Invalid date components".into()
            )
        }
        _ => Err("Unsupported value type for date conversion".into()),
    }
}

pub fn convert_to_naive_time(value: Value) -> Result<NaiveTime> {
    match value {
        Value::Bytes(time) => {
            let time_str = String::from_utf8(time).map_err(|e| Box::new(e) as BoxedError)?; // Specify error type explicitly

            NaiveTime::parse_from_str(&time_str, "%H:%M:%S").map_err(|e| Box::new(e) as BoxedError)
        }
        Value::Date(_, _, _, hour, minute, second, _) => {
            NaiveTime::from_hms_opt(hour as u32, minute as u32, second as u32).ok_or_else(||
                "Invalid time components".into()
            )
        }
        _ => Err("Unsupported value type for date conversion".into()),
    }
}

pub fn convert_to_naive_date_time(value: Value) -> Result<NaiveDateTime> {
    let naive_datetime = match value {
        Value::Bytes(bytes) => {
            let date_str = String::from_utf8_lossy(&bytes);
            // println!("Attempting to parse NaiveDateTime from string: '{}'", date_str);
            NaiveDateTime::parse_from_str(&date_str, "%Y-%m-%d %H:%M:%S").map_err(
                |e| Box::new(e) as BoxedError
            )?
        }
        Value::Date(year, month, day, hour, minutes, seconds, micro_seconds) => {
            let naive_date = NaiveDate::from_ymd_opt(
                year as i32,
                month as u32,
                day as u32
            ).unwrap_or_else(|| NaiveDate::from_ymd_opt(1970, 1, 1).unwrap());
            let naive_time = NaiveDateTime::new(
                naive_date,
                chrono::NaiveTime
                    ::from_hms_micro_opt(
                        hour as u32,
                        minutes as u32,
                        seconds as u32,
                        micro_seconds as u32
                    )
                    .unwrap_or_else(|| chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap())
            );

            naive_time
        }
        _ => NaiveDateTime::from_timestamp_opt(0, 0).unwrap(),
    };

    Ok(naive_datetime)
}

pub fn parse_naive_date_time_from_str(date_str: &str) -> Result<NaiveDateTime> {
    NaiveDateTime::parse_from_str(date_str, "%Y-%m-%d %H:%M:%S").map_err(
        |e| Box::new(e) as BoxedError
    )
}
