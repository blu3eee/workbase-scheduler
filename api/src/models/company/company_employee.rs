use chrono::NaiveDate;
use mysql::{ prelude::FromRow, FromRowError };
use serde::{ Serialize, Deserialize };

use crate::utilities::parse_chrono::convert_to_naive_date;

pub fn create_company_employees_table_query() -> String {
    "
    CREATE TABLE IF NOT EXISTS company_employees (
        id INT NOT NULL,
        user_id BIGINT NOT NULL,
        company_id BIGINT NOT NULL,
        hired_date DATE NOT NULL DEFAULT (CURRENT_DATE),
        punch_id INT NOT NULL,
        notes TEXT,
        FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
        FOREIGN KEY (company_id) REFERENCES companies(id) ON DELETE CASCADE,
        UNIQUE KEY company_emp (user_id, company_id)
    );
    ".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompanyEmployee {
    pub id: i32,
    pub user_id: i64,
    pub company_id: i64,
    pub hired_date: NaiveDate,
    pub punch_id: i32,
    pub notes: Option<String>,
}

// For creating a new user-company relationship
#[derive(Debug, Clone, Deserialize)]
pub struct RequestCreateCompanyEmployee {
    pub user_id: i64,
    pub company_id: i64,
    pub punch_id: i32,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct RequestUpdateCompanyEmployee {
    // pub employee_id:
}

impl FromRow for CompanyEmployee {
    fn from_row_opt(row: mysql::Row) -> Result<Self, mysql::FromRowError> where Self: Sized {
        Ok(CompanyEmployee {
            id: row.get("id").ok_or(FromRowError(row.clone()))?,
            user_id: row.get("user_id").ok_or(FromRowError(row.clone()))?,
            company_id: row.get("company_id").ok_or(FromRowError(row.clone()))?,
            hired_date: convert_to_naive_date(
                row.get("joined_at").ok_or(FromRowError(row.clone()))?
            ).map_err(|_| FromRowError(row.clone()))?,
            punch_id: row.get("user_id").ok_or(FromRowError(row.clone()))?,
            notes: row.get("company_id").ok_or(FromRowError(row.clone()))?,
        })
    }
}
