use std::error::Error;
use mysql::*;
use mysql::prelude::*;

use crate::models::org_jobs::{
    OrgJob,
    RequestCreateOrgJob,
    RequestUpdateOrgJob,
    create_org_job_table,
};

use super::BasicQueries;

pub struct OrgJobQueries {}

impl BasicQueries for OrgJobQueries {
    type Model = OrgJob;
    type CreateDto = RequestCreateOrgJob;
    type UpdateDto = RequestUpdateOrgJob;

    fn table_name() -> String {
        "org_jobs".to_string()
    }

    fn create_table(conn: &mut PooledConn) -> Result<(), Box<dyn Error>> {
        let query = create_org_job_table();
        conn.query_drop(query)?;
        Ok(())
    }

    fn create_entity(
        conn: &mut PooledConn,
        create_dto: Self::CreateDto
    ) -> Result<i32, Box<dyn Error>> {
        conn.exec_drop(
            format!(
                "INSERT INTO {} (org_id, name, description, base_pay_rate) VALUES (:org_id, :name, :description, :base_pay_rate)",
                Self::table_name()
            ),
            params! {
                "org_id" => create_dto.org_id,
                "name" => &create_dto.name,
                "description" => &create_dto.description,
                "base_pay_rate" => create_dto.base_pay_rate,
            }
        )?;
        Ok(conn.last_insert_id() as i32)
    }

    fn update_entity(
        conn: &mut PooledConn,
        update_dto: Self::UpdateDto
    ) -> Result<u64, Box<dyn Error>> {
        let mut query = format!("UPDATE {} SET ", Self::table_name());
        let mut params: Vec<(String, Value)> = Vec::new();

        if let Some(name) = update_dto.name {
            query.push_str("name = :name, ");
            params.push(("name".to_string(), name.into()));
        }
        if let Some(description) = update_dto.description {
            query.push_str("description = :description, ");
            params.push(("description".to_string(), description.into()));
        }
        if let Some(base_pay_rate) = update_dto.base_pay_rate {
            query.push_str("base_pay_rate = :base_pay_rate, ");
            params.push(("base_pay_rate".to_string(), base_pay_rate.into()));
        }

        // Remove trailing comma and space
        query.pop();
        query.pop();

        query.push_str(&format!(" WHERE id = :id;"));
        params.push(("id".to_string(), update_dto.id.into()));

        let params = Params::from(params);
        let query_result = conn.exec_iter(&query, params)?;

        Ok(query_result.affected_rows())
    }

    fn delete_entity(conn: &mut PooledConn, id: i32) -> Result<u64, Box<dyn Error>> {
        let query_result = conn.query_iter(
            format!("DELETE FROM {} WHERE id = {}", Self::table_name(), id)
        )?;
        Ok(query_result.affected_rows())
    }
}
