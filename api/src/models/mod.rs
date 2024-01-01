use std::str::FromStr;

use chrono::{ NaiveDate, NaiveDateTime };
use mysql::{ PooledConn, prelude::Queryable, Value };

use self::{ users::create_users_table_query, error::Result };

pub mod error;
pub mod users;
pub mod organizations;
pub mod org_members;
pub mod org_jobs;
pub mod org_member_jobs;

/// create tables
pub async fn create_tables(mut conn: PooledConn) -> Result<()> {
    let stmt = conn.prep(create_users_table_query())?;
    conn.exec_drop(stmt, ())?;

    Ok(())
}

pub fn convert_to_naive_date(value: Value) -> NaiveDate {
    match value {
        Value::Bytes(date) => {
            NaiveDate::from_str(
                &date
                    .iter()
                    .map(|x| (*x as char).to_string())
                    .collect::<Vec<String>>()
                    .join("")
            ).expect("Failed to parse value to NaiveDate")
        }
        Value::Date(year, month, day, _, _, _, _) => {
            NaiveDate::from_ymd_opt(year as i32, month as u32, day as u32).unwrap()
        }
        _ => NaiveDate::from_ymd_opt(1970, 1, 1).unwrap(),
    }
}

pub fn convert_to_naive_date_time(value: Value) -> NaiveDateTime {
    match value {
        Value::Bytes(bytes) => {
            let date_str = String::from_utf8_lossy(&bytes);
            // println!("Attempting to parse NaiveDateTime from string: '{}'", date_str);
            NaiveDateTime::parse_from_str(&date_str, "%Y-%m-%d %H:%M:%S").unwrap_or_else(|err| {
                panic!("Failed to parse NaiveDateTime: '{}', error: {:?}", date_str, err)
            })
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
    }
}
