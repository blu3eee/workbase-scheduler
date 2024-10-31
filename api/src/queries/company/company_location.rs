use mysql::*;
use mysql::prelude::*;

use crate::models::company::company_location::{
    create_company_locations_table_query,
    CompanyLocation,
    RequestCreateCompanyLocation,
    RequestUpdateCompanyLocation,
};
use crate::prototypes::basic_queries::BasicQueries;
use crate::prototypes::create_table::DatabaseTable;

pub struct CompanyLocationQueries {}

impl DatabaseTable for CompanyLocationQueries {
    fn create_table(&self, conn: &mut PooledConn) -> crate::models::result::Result<()> {
        let query = create_company_locations_table_query();
        let stmt = conn.prep(query)?;
        conn.exec_drop(stmt, ())?;

        Ok(())
    }
}

impl BasicQueries for CompanyLocationQueries {
    type Model = CompanyLocation;

    type CreateDto = RequestCreateCompanyLocation;

    type UpdateDto = RequestUpdateCompanyLocation;

    fn table_name() -> String {
        "company_locations".to_string()
    }

    fn insert_statement() -> String {
        format!(
            "INSERT INTO {} (id, company_id, name, timezone, address) VALUES (:id, :company_id, :name, :timezone, :address)",
            Self::table_name()
        )
    }

    fn insert_params(create_dto: &Self::CreateDto) -> crate::models::result::Result<Params> {
        Ok(
            params! {
                "company_id" => &create_dto.company_id,
                "name" => &create_dto.name,
                "timezone" => &create_dto.timezone,
                "address" => &create_dto.address,
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
        if let Some(timezone) = update_dto.timezone {
            query.push_str("timezone = :timezone, ");
            params.push(("timezone".to_string(), timezone.into()));
        }
        if let Some(address) = update_dto.address {
            query.push_str("address = :address, ");
            params.push(("address".to_string(), address.into()));
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
