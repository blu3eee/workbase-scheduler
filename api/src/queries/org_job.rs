use mysql::*;
use mysql::prelude::*;

use crate::{
    models::{
        org_job::{ OrgJob, RequestCreateOrgJob, RequestUpdateOrgJob, create_org_job_table },
        result::Result,
    },
    prototypes::create_table::DatabaseTable,
};

use crate::prototypes::basic_queries::BasicQueries;

pub struct OrgJobQueries {}

impl DatabaseTable for OrgJobQueries {
    fn create_table(&self, conn: &mut PooledConn) -> Result<()> {
        let query = create_org_job_table();
        conn.query_drop(query)?;
        Ok(())
    }
}

impl BasicQueries for OrgJobQueries {
    type Model = OrgJob;
    type CreateDto = RequestCreateOrgJob;
    type UpdateDto = RequestUpdateOrgJob;

    fn table_name() -> String {
        "org_jobs".to_string()
    }

    fn insert_statement() -> String {
        format!(
            "INSERT INTO {} (id, org_id, name, description, base_pay_rate) VALUES (:id, :org_id, :name, :description, :base_pay_rate)",
            Self::table_name()
        )
    }

    fn insert_params(create_dto: &Self::CreateDto) -> Result<Params> {
        Ok(
            params! {
                "org_id" => create_dto.org_id,
                "name" => &create_dto.name,
                "description" => &create_dto.description,
                "base_pay_rate" => create_dto.base_pay_rate,
            }
        )
    }

    fn update_entity(conn: &mut PooledConn, id: i64, update_dto: Self::UpdateDto) -> Result<u64> {
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
        params.push(("id".to_string(), id.into()));

        let params = Params::from(params);
        let query_result = conn.exec_iter(&query, params)?;

        Ok(query_result.affected_rows())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;
    use crate::models::organization::RequestCreateOrganization;
    use crate::models::user::RequestCreateUser;
    use crate::models::org_job::RequestCreateOrgJob;
    use crate::queries::organization::OrgQueries;
    use crate::queries::user::UserQueries;
    use crate::snowflake::SnowflakeGenerator;
    use crate::tests::{ initialize_test_db, cleanup_test_db };

    use chrono::NaiveDate;

    #[test]
    fn test_org_job_queries() -> Result<()> {
        let pool = initialize_test_db()?;
        let mut conn = pool.get_conn()?;

        let snowflake_generator = Arc::new(SnowflakeGenerator::new(1));

        // Create a user
        let user = RequestCreateUser {
            email: "owner@example.com".to_string(),
            password: "password".to_string(),
            first_name: "Owner".to_string(),
            last_name: "User".to_string(),
            date_of_birth: NaiveDate::from_ymd_opt(1980, 1, 1).unwrap(),
            phone_number: Some("1234567890".to_string()),
        };

        let owner_user_id: i64 = UserQueries::create_entity(
            &mut conn,
            snowflake_generator.clone(),
            user
        )?;

        // Create an organization
        let org = RequestCreateOrganization {
            name: "Test Organization".to_string(),
            description: Some("A test organization".to_string()),
            owner_id: owner_user_id,
            timezone: None,
            icon: None,
        };

        let org_id: i64 = OrgQueries::create_entity(&mut conn, snowflake_generator.clone(), org)?;

        // Test creating a job
        let job = RequestCreateOrgJob {
            org_id,
            name: "Developer".to_string(),
            description: Some("Develops software".to_string()),
            base_pay_rate: 50.0,
            color: None,
        };

        let job_id = OrgJobQueries::create_entity(&mut conn, snowflake_generator.clone(), job)?;

        // Test updating the job
        let affected_rows = OrgJobQueries::update_entity(&mut conn, job_id, RequestUpdateOrgJob {
            name: Some("Senior Developer".to_string()),
            description: None,
            base_pay_rate: Some(60.0),
            ..Default::default()
        })?;
        assert_eq!(affected_rows, 1);

        let job: OrgJob = OrgJobQueries::find_by_id(&mut conn, job_id)?;
        assert_eq!(job.name, "Senior Developer");
        assert_eq!(job.description, Some("Develops software".to_string()));
        assert_eq!(job.base_pay_rate, 60.0);

        // Test deleting the job
        let deleted_rows = OrgJobQueries::delete_entity(&mut conn, job_id)?;
        assert_eq!(deleted_rows, 1);

        // Cleanup: Drop the database
        cleanup_test_db(conn)?;

        Ok(())
    }
}
