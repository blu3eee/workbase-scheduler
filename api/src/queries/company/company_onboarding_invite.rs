use mysql::*;
use mysql::prelude::*;

use crate::models::company::company_onboarding_invite::{
    create_company_onboarding_invites_table_query,
    CompanyOnboardingInvite,
    RequestCreateCompanyOnboardingInvite,
    RequestUpdateCompanyOnboardingInvite,
};
use crate::prototypes::basic_queries::BasicQueries;
use crate::prototypes::create_table::DatabaseTable;

pub struct CompanyOnboardingInviteQueries {}

impl DatabaseTable for CompanyOnboardingInviteQueries {
    fn create_table(&self, conn: &mut PooledConn) -> crate::models::result::Result<()> {
        let query = create_company_onboarding_invites_table_query();
        let stmt = conn.prep(query)?;
        conn.exec_drop(stmt, ())?;

        Ok(())
    }
}

impl BasicQueries for CompanyOnboardingInviteQueries {
    type Model = CompanyOnboardingInvite;

    type CreateDto = RequestCreateCompanyOnboardingInvite;

    type UpdateDto = RequestUpdateCompanyOnboardingInvite;

    fn table_name() -> String {
        "company_onboarding_invites".to_string()
    }

    fn insert_statement() -> String {
        format!(
            "INSERT INTO {} (id, company_id, location_id, email, name, role_id) VALUES (:id, :company_id, :location_id, :email, :name, :role_id)",
            Self::table_name()
        )
    }

    fn insert_params(create_dto: &Self::CreateDto) -> crate::models::result::Result<Params> {
        Ok(
            params! {
                "company_id" => &create_dto.company_id,
                "location_id" => &create_dto.location_id,
                "email" => &create_dto.email,
                "name" => &create_dto.name,
                "role_id" => &create_dto.role_id,
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

        if let Some(status) = update_dto.status {
            query.push_str("status = :status, ");
            params.push(("status".to_string(), status.to_string().into()));
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
