use mysql::*;
use mysql::prelude::*;

use crate::{
    models::{
        company_member::{
            CompanyMember,
            RequestCreateCompanyMember,
            RequestUpdateCompanyMember,
            create_company_members_table_query,
        },
        result::Result,
    },
    prototypes::create_table::DatabaseTable,
};

pub struct CompanyMemberQueries {}

impl DatabaseTable for CompanyMemberQueries {
    fn create_table(&self, conn: &mut PooledConn) -> Result<()> {
        let query = create_company_members_table_query();
        let stmt = conn.prep(query)?;
        conn.exec_drop(stmt, ())?;

        Ok(())
    }
}

impl CompanyMemberQueries {
    fn table_name() -> String {
        "company_members".to_string()
    }

    fn insert_statement() -> String {
        format!(
            "INSERT INTO {} (user_id, company_id, job_id) VALUES (:user_id, :company_id, :job_id)",
            Self::table_name()
        )
    }

    fn insert_params(create_dto: &RequestCreateCompanyMember) -> Result<Params> {
        Ok(
            params! {
                "user_id" => create_dto.user_id,
                "company_id" => create_dto.company_id,
                "job_id" => create_dto.job_id,
            }
        )
    }

    pub fn create_entity(
        conn: &mut PooledConn,
        create_dto: RequestCreateCompanyMember
    ) -> Result<()> {
        conn.exec_drop(Self::insert_statement(), Self::insert_params(&create_dto)?)?;
        Ok(())
    }

    pub fn update_entity(
        conn: &mut PooledConn,
        update_dto: RequestUpdateCompanyMember
    ) -> Result<u64> {
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
            &format!(
                " WHERE company_id = {} AND user_id = {};",
                update_dto.company_id,
                update_dto.user_id
            )
        );

        // Convert Vec to Params::Named
        let params = Params::from(params);

        // Use exec_fold to execute the query and process the result
        let query_result = conn.exec_iter(&query, params)?;

        // Return the number of affected rows
        Ok(query_result.affected_rows())
    }

    pub fn delete_entity(conn: &mut PooledConn, company_id: i64, user_id: i64) -> Result<u64> {
        let query_result = conn.query_iter(
            format!(
                "DELETE FROM {} WHERE company_id = {} AND user_id = {};",
                Self::table_name(),
                company_id,
                user_id
            )
        )?;

        Ok(query_result.affected_rows())
    }

    pub fn find_by_id(
        conn: &mut PooledConn,
        company_id: i64,
        user_id: i64
    ) -> Result<CompanyMember> {
        // SQL query to select an User in the Company
        let query = format!(
            "SELECT * FROM {} WHERE company_id = {} AND user_id = {};",
            Self::table_name(),
            company_id,
            user_id
        );

        // Execute the query
        let result: Option<CompanyMember> = conn.exec_first(query, ())?;

        // Extract the first row from the result (if any)
        if let Some(model) = result {
            // Convert the row into a Self::Model struct
            Ok(model)
        } else {
            // Return an error if no user is found
            Err(From::from("User not found"))
        }
    }

    pub fn find_company_members(
        conn: &mut PooledConn,
        company_id: i64
    ) -> Result<Vec<CompanyMember>> {
        Ok(
            conn.query(
                format!("SELECT * FROM {} WHERE company_id = {};", Self::table_name(), company_id)
            )?
        )
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use chrono::NaiveDate;

    use super::*;
    use crate::models::company_job::RequestCreateCompanyJob;
    use crate::prototypes::basic_queries::BasicQueries;
    use crate::queries::company_job::CompanyJobQueries;
    use crate::queries::company::CompanyQueries;
    use crate::queries::user::UserQueries;
    use crate::snowflake::SnowflakeGenerator;
    use crate::tests::{ initialize_test_db, cleanup_test_db };
    use crate::models::user::RequestCreateUser;
    use crate::models::company::RequestCreateCompany;

    #[test]
    fn test_company_member_queries() -> Result<()> {
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

        // Insert an company
        let company = RequestCreateCompany {
            name: "Test Company".to_string(),
            description: Some("A test company".to_string()),
            owner_id: user_id, // Assume the user is the owner
            timezone: None,
            icon: None,
        };

        let company_id: i64 = CompanyQueries::create_entity(
            &mut conn,
            snowflake_generator.clone(),
            company
        )?;

        let job_id: i64 = CompanyJobQueries::create_entity(
            &mut conn,
            snowflake_generator.clone(),
            RequestCreateCompanyJob {
                company_id,
                name: "Cashier".to_string(),
                description: None,
                base_pay_rate: 15.5,
                color: None,
            }
        )?;

        CompanyMemberQueries::update_entity(&mut conn, RequestUpdateCompanyMember {
            company_id,
            user_id,
            job_id: Some(job_id),
        })?;

        let member = CompanyMemberQueries::find_by_id(&mut conn, company_id, user_id)?;
        assert_eq!(member.job_id, job_id);

        // Delete company member
        let deleted_rows = CompanyMemberQueries::delete_entity(&mut conn, company_id, user_id)?;
        assert_eq!(deleted_rows, 1);

        cleanup_test_db(conn)?;

        Ok(())
    }
}
