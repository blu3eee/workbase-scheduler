use serde::{ Serialize, Deserialize };
use mysql::*;
use mysql::prelude::*;

use crate::snowflake::SnowflakeId;

pub fn create_department_roles_table_query() -> String {
    "
    CREATE TABLE IF NOT EXISTS department_roles (
        id BIGINT NOT NULL PRIMARY KEY,
        department_id BIGINT NOT NULL,
        name VARCHAR(100) NOT NULL,
        wage FLOAT NOT NULL,
        color VARCHAR(6) NOT NULL DEFAULT FFFFFF,
        is_active BOOLEAN NOT NULL DEFAULT TRUE,
        FOREIGN KEY (department_id) REFERENCES location_departments(id) ON DELETE CASCADE
    );
    ".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepartmentRole {
    pub id: SnowflakeId,
    pub department_id: SnowflakeId,
    pub name: String,
    pub wage: f32,
    pub color: String,
    pub is_active: bool,
}

impl FromRow for DepartmentRole {
    fn from_row_opt(row: Row) -> Result<Self, FromRowError> {
        Ok(DepartmentRole {
            id: row.get("id").ok_or(FromRowError(row.clone()))?,
            department_id: row.get("department_id").ok_or(FromRowError(row.clone()))?,
            name: row.get("name").ok_or(FromRowError(row.clone()))?,
            wage: row.get("wage").ok_or(FromRowError(row.clone()))?,
            color: row.get("color").ok_or(FromRowError(row.clone()))?,
            is_active: row.get("is_active").ok_or(FromRowError(row.clone()))?,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestCreateDepartmentRole {
    pub location_id: SnowflakeId,
    pub name: String,
    pub wage: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RequestUpdateDepartmentRole {
    pub id: SnowflakeId,
    pub name: Option<String>,
    pub wage: Option<f32>,
    pub color: Option<String>,
    pub is_active: Option<bool>,
}
