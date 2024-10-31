use mysql::*;
use mysql::prelude::*;

use crate::models::company::department_role::{
    create_department_roles_table_query,
    DepartmentRole,
    RequestCreateDepartmentRole,
    RequestUpdateDepartmentRole,
};
use crate::prototypes::basic_queries::BasicQueries;
use crate::prototypes::create_table::DatabaseTable;

pub struct DepartmentRoleQueries {}

impl DatabaseTable for DepartmentRoleQueries {
    fn create_table(&self, conn: &mut PooledConn) -> crate::models::result::Result<()> {
        let query = create_department_roles_table_query();
        let stmt = conn.prep(query)?;
        conn.exec_drop(stmt, ())?;

        Ok(())
    }
}

impl BasicQueries for DepartmentRoleQueries {
    type Model = DepartmentRole;

    type CreateDto = RequestCreateDepartmentRole;

    type UpdateDto = RequestUpdateDepartmentRole;

    fn table_name() -> String {
        "location_departments".to_string()
    }

    fn insert_statement() -> String {
        format!(
            "INSERT INTO {} (id, department_id, name, :wage) VALUES (:id, :department_id, :name, :wage)",
            Self::table_name()
        )
    }

    fn insert_params(create_dto: &Self::CreateDto) -> crate::models::result::Result<Params> {
        Ok(
            params! {
                "department_id" => &create_dto.department_id,
                "name" => &create_dto.name,
                "wage" => &create_dto.wage,
            }
        )
    }

    fn update_entity(
        conn: &mut PooledConn,
        id: i64,
        update_dto: Self::UpdateDto
    ) -> crate::models::result::Result<u64> {
        let mut query = format!("UPDATE {} SET ", Self::table_name());
        let mut params: Vec<(String, Value)> = Vec::new();

        if let Some(name) = update_dto.name {
            query.push_str("name = :name, ");
            params.push(("name".to_string(), name.into()));
        }
        if let Some(wage) = update_dto.wage {
            query.push_str("wage = :wage, ");
            params.push(("wage".to_string(), wage.into()));
        }
        if let Some(color) = update_dto.color {
            query.push_str("color = :color, ");
            params.push(("color".to_string(), color.into()));
        }
        if let Some(is_active) = update_dto.is_active {
            query.push_str("is_active = :is_active, ");
            params.push(("is_active".to_string(), is_active.into()));
        }

        // Remove last comma and space if there are updates
        if !params.is_empty() {
            query.pop();
            query.pop();
        } else {
            return Ok(0);
        }

        query.push_str(&format!(" WHERE id = {};", id));

        // Convert Vec to Params::Named
        let params = Params::from(params);

        // Use exec_fold to execute the query and process the result
        let query_result = conn.exec_iter(&query, params)?;

        // Return the number of affected rows
        Ok(query_result.affected_rows())
    }
}
