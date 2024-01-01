use mysql::*;
use mysql::prelude::*;

use crate::models::{
    org_member_jobs::{
        OrgMemberJob,
        RequestCreateOrgMemberJob,
        RequestUpdateOrgMemberJob,
        create_org_member_job_table,
    },
    error::Result,
};

use super::BasicQueries;

pub struct OrgMemberJobQueries {}

impl BasicQueries for OrgMemberJobQueries {
    type Model = OrgMemberJob;
    type CreateDto = RequestCreateOrgMemberJob;
    type UpdateDto = RequestUpdateOrgMemberJob;

    fn table_name() -> String {
        "org_member_jobs".to_string()
    }

    fn create_table(conn: &mut PooledConn) -> Result<()> {
        let query = create_org_member_job_table();
        conn.query_drop(query)?;
        Ok(())
    }

    fn create_entity(conn: &mut PooledConn, create_dto: Self::CreateDto) -> Result<i32> {
        conn.exec_drop(
            format!(
                "INSERT INTO {} (member_id, job_id, pay_rate) VALUES (:member_id, :job_id, :pay_rate)",
                Self::table_name()
            ),
            params! {
                "member_id" => create_dto.member_id,
                "job_id" => create_dto.job_id,
                "pay_rate" => create_dto.pay_rate,
            }
        )?;
        Ok(conn.last_insert_id() as i32)
    }

    fn update_entity(conn: &mut PooledConn, update_dto: Self::UpdateDto) -> Result<u64> {
        let mut query = "UPDATE org_member_jobs SET ".to_string();
        let mut params: Vec<(String, Value)> = Vec::new();

        if let Some(pay_rate) = update_dto.pay_rate {
            query.push_str("pay_rate = :pay_rate, ");
            params.push(("pay_rate".to_string(), pay_rate.into()));
        }

        if let Some(end_date) = update_dto.end_date {
            query.push_str("end_date = :end_date, ");
            params.push((
                "end_date".to_string(),
                end_date.format("%Y-%m-%d %H:%M:%S").to_string().into(),
            ));
        }

        // Remove trailing comma and space
        query.pop();
        query.pop();

        query.push_str(&format!(" WHERE id = :id"));

        params.push(("id".to_string(), update_dto.id.into()));

        let params = Params::from(params);
        let query_result = conn.exec_iter(&query, params)?;

        Ok(query_result.affected_rows())
    }

    /// delete method not available (entity doesn't have unique ID), use `delete_entity(conn: &mut PooledConn, member_id: i32, job_id: i32)` instead
    fn delete_entity(conn: &mut PooledConn, id: i32) -> Result<u64> {
        let query = "DELETE FROM org_member_jobs WHERE id = :id;";
        let params = params! {
            "id" => id,
        };
        let query_result = conn.exec_iter(query, params)?;

        Ok(query_result.affected_rows())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    use crate::models::error::Result;
    use crate::models::org_jobs::RequestCreateOrgJob;
    use crate::models::org_member_jobs::RequestUpdateOrgMemberJob;
    use crate::queries::BasicQueries;
    use crate::queries::org_jobs::OrgJobQueries;
    use crate::queries::{
        users::UserQueries,
        organizations::OrgQueries,
        org_members::OrgMemberQueries,
    };
    use crate::models::{
        users::RequestCreateUser,
        organizations::RequestCreateOrganization,
        org_members::RequestCreateOrgMember,
        org_member_jobs::RequestCreateOrgMemberJob,
    };
    use crate::tests::{ initialize_test_db, cleanup_test_db };

    #[test]
    fn test_org_member_job_queries() -> Result<()> {
        let mut conn = initialize_test_db()?;

        // Create a user
        let user_id = UserQueries::create_entity(&mut conn, RequestCreateUser {
            email: "user1@email.com".to_string(),
            password: "password1".to_string(),
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            date_of_birth: NaiveDate::from_ymd_opt(2000, 1, 1).unwrap(), // Example date
            phone_number: Some("1234567890".to_string()),
        })?;

        // Create an organization
        let org_id = OrgQueries::create_entity(&mut conn, RequestCreateOrganization {
            name: "Dummy Organization".to_string(),
            description: Some("A test organization".to_string()),
            owner_id: user_id,
        })?;

        // Add user to organization
        let member_id = OrgMemberQueries::create_entity(&mut conn, RequestCreateOrgMember {
            user_id,
            org_id,
        })?;

        // add a job
        let job_id = OrgJobQueries::create_entity(&mut conn, RequestCreateOrgJob {
            org_id,
            name: "Manager".to_string(),
            description: None,
            base_pay_rate: 15.5,
        })?;

        // Assign a job to the organization member
        let member_job_id = OrgMemberJobQueries::create_entity(
            &mut conn,
            RequestCreateOrgMemberJob {
                member_id,
                job_id,
                pay_rate: None,
            }
        )?;

        // Update the job details
        let update_result = OrgMemberJobQueries::update_entity(
            &mut conn,
            RequestUpdateOrgMemberJob {
                id: job_id,
                pay_rate: Some(16 as f32),
                ..Default::default()
            }
        )?;
        assert!(update_result > 0, "Failed to update job details");

        // Delete the job entry
        let delete_result = OrgMemberJobQueries::delete_entity(&mut conn, member_job_id)?;
        assert_eq!(delete_result, 1, "Failed to delete job entry");

        cleanup_test_db(conn)?;

        Ok(())
    }
}
