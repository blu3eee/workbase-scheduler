use mysql::*;
use mysql::prelude::*;

use crate::{
    models::{
        org_member::{
            OrgMember,
            RequestCreateOrgMember,
            RequestUpdateOrgMember,
            create_org_members_table_query,
        },
        result::Result,
    },
    prototypes::create_table::DatabaseTable,
};

pub struct OrgMemberQueries {}

impl DatabaseTable for OrgMemberQueries {
    fn create_table(&self, conn: &mut PooledConn) -> Result<()> {
        let query = create_org_members_table_query();
        let stmt = conn.prep(query)?;
        conn.exec_drop(stmt, ())?;

        Ok(())
    }
}

impl OrgMemberQueries {
    fn table_name() -> String {
        "org_members".to_string()
    }

    fn insert_statement() -> String {
        format!(
            "INSERT INTO {} (user_id, org_id, job_id) VALUES (:user_id, :org_id, :job_id)",
            Self::table_name()
        )
    }

    fn insert_params(create_dto: &RequestCreateOrgMember) -> Result<Params> {
        Ok(
            params! {
                "user_id" => create_dto.user_id,
                "org_id" => create_dto.org_id,
                "job_id" => create_dto.job_id,
            }
        )
    }

    pub fn create_entity(conn: &mut PooledConn, create_dto: RequestCreateOrgMember) -> Result<()> {
        conn.exec_drop(Self::insert_statement(), Self::insert_params(&create_dto)?)?;
        Ok(())
    }

    pub fn update_entity(conn: &mut PooledConn, update_dto: RequestUpdateOrgMember) -> Result<u64> {
        let mut query = format!("UPDATE {} SET ", Self::table_name());
        let mut params: Vec<(String, Value)> = Vec::new();

        if let Some(job_id) = update_dto.job_id {
            query.push_str("job_id = :job_id, ");
            params.push(("job_id".to_string(), job_id.into()));
        }

        // Remove last comma and space if there are updates
        if !params.is_empty() {
            query.pop();
            query.pop();
        } else {
            return Ok(0);
        }

        query.push_str(
            &format!(" WHERE org_id = {} AND user_id = {};", update_dto.org_id, update_dto.user_id)
        );

        // Convert Vec to Params::Named
        let params = Params::from(params);

        // Use exec_fold to execute the query and process the result
        let query_result = conn.exec_iter(&query, params)?;

        // Return the number of affected rows
        Ok(query_result.affected_rows())
    }

    pub fn delete_entity(conn: &mut PooledConn, org_id: i64, user_id: i64) -> Result<u64> {
        let query_result = conn.query_iter(
            format!(
                "DELETE FROM {} WHERE org_id = {} AND user_id = {};",
                Self::table_name(),
                org_id,
                user_id
            )
        )?;

        Ok(query_result.affected_rows())
    }

    pub fn find_by_id(conn: &mut PooledConn, org_id: i64, user_id: i64) -> Result<OrgMember> {
        // SQL query to select an User in the Organization
        let query = format!(
            "SELECT * FROM {} WHERE org_id = {} AND user_id = {};",
            Self::table_name(),
            org_id,
            user_id
        );

        // Execute the query
        let result: Option<OrgMember> = conn.exec_first(query, ())?;

        // Extract the first row from the result (if any)
        if let Some(model) = result {
            // Convert the row into a Self::Model struct
            Ok(model)
        } else {
            // Return an error if no user is found
            Err(From::from("User not found"))
        }
    }

    pub fn find_org_members(conn: &mut PooledConn, org_id: i64) -> Result<Vec<OrgMember>> {
        Ok(conn.query(format!("SELECT * FROM {} WHERE org_id = {};", Self::table_name(), org_id))?)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use chrono::NaiveDate;

    use super::*;
    use crate::models::org_job::RequestCreateOrgJob;
    use crate::prototypes::basic_queries::BasicQueries;
    use crate::queries::org_job::OrgJobQueries;
    use crate::queries::organization::OrgQueries;
    use crate::queries::user::UserQueries;
    use crate::snowflake::SnowflakeGenerator;
    use crate::tests::{ initialize_test_db, cleanup_test_db };
    use crate::models::user::RequestCreateUser;
    use crate::models::organization::RequestCreateOrganization;

    #[test]
    fn test_org_member_queries() -> Result<()> {
        let pool = initialize_test_db()?;
        let mut conn = pool.get_conn()?;
        let snowflake_generator = Arc::new(SnowflakeGenerator::new(1));

        // Create user
        let user_id = UserQueries::create_entity(
            &mut conn,
            snowflake_generator.clone(),
            RequestCreateUser {
                email: "testuser@example.com".to_string(),
                password: "password".to_string(),
                first_name: "Test".to_string(),
                last_name: "User".to_string(),
                date_of_birth: NaiveDate::from_ymd_opt(2000, 1, 1).unwrap(),
                phone_number: Some("1234567890".to_string()),
            }
        )?;

        // Insert an organization
        let org = RequestCreateOrganization {
            name: "Test Organization".to_string(),
            description: Some("A test organization".to_string()),
            owner_id: user_id, // Assume the user is the owner
            timezone: None,
            icon: None,
        };

        let org_id: i64 = OrgQueries::create_entity(&mut conn, snowflake_generator.clone(), org)?;

        let job_id: i64 = OrgJobQueries::create_entity(
            &mut conn,
            snowflake_generator.clone(),
            RequestCreateOrgJob {
                org_id,
                name: "Cashier".to_string(),
                description: None,
                base_pay_rate: 15.5,
                color: None,
            }
        )?;

        OrgMemberQueries::update_entity(&mut conn, RequestUpdateOrgMember {
            org_id,
            user_id,
            job_id: Some(job_id),
        })?;

        let member = OrgMemberQueries::find_by_id(&mut conn, org_id, user_id)?;
        assert_eq!(member.job_id, job_id);

        // Delete organization member
        let deleted_rows = OrgMemberQueries::delete_entity(&mut conn, org_id, user_id)?;
        assert_eq!(deleted_rows, 1);

        cleanup_test_db(conn)?;

        Ok(())
    }
}
