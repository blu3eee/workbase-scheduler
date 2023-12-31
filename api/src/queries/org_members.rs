use std::error::Error;
use mysql::*;
use mysql::prelude::*;

use crate::models::org_members::{
    OrgMember,
    RequestCreateOrgMember,
    RequestUpdateOrgMember,
    create_org_members_table_query,
};

use super::BasicQueries;

pub struct OrgMemberQueries {}

impl BasicQueries for OrgMemberQueries {
    type Model = OrgMember;

    type CreateDto = RequestCreateOrgMember;

    type UpdateDto = RequestUpdateOrgMember;

    fn table_name() -> String {
        "org_members".to_string()
    }

    fn create_table(conn: &mut PooledConn) -> Result<(), Box<dyn Error>> {
        let query = create_org_members_table_query();
        let stmt = conn.prep(query)?;
        conn.exec_drop(stmt, ())?;

        Ok(())
    }

    fn create_entity(
        conn: &mut PooledConn,
        create_dto: Self::CreateDto
    ) -> Result<i32, Box<dyn Error>> {
        conn.exec_drop(
            format!(
                "INSERT INTO {} (user_id, org_id) VALUES (:user_id, :org_id)",
                Self::table_name()
            ),
            params! {
                "user_id" => create_dto.user_id,
                "org_id" => create_dto.org_id,
                
            }
        )?;
        Ok(conn.last_insert_id() as i32)
    }

    fn update_entity(
        _conn: &mut PooledConn,
        _update_dto: Self::UpdateDto
    ) -> Result<u64, Box<dyn Error>> {
        Ok(0)
    }

    fn delete_entity(conn: &mut PooledConn, id: i32) -> Result<u64, Box<dyn Error>> {
        let query_result = conn.query_iter(format!("DELETE FROM organizations WHERE id = {}", id))?;

        Ok(query_result.affected_rows())
    }
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    use super::*;
    use crate::queries::organizations::OrganizationQueries;
    use crate::queries::users::UserQueries;
    use crate::tests::{ initialize_test_db, cleanup_test_db };
    use crate::models::users::RequestCreateUser;
    use crate::models::organizations::RequestCreateOrganization;

    #[test]
    fn test_org_member_queries() -> Result<(), Box<dyn Error>> {
        let mut conn = initialize_test_db()?;

        // Create user
        let user_id = UserQueries::create_entity(&mut conn, RequestCreateUser {
            email: "testuser@example.com".to_string(),
            password: "password".to_string(),
            first_name: "Test".to_string(),
            last_name: "User".to_string(),
            date_of_birth: NaiveDate::from_ymd_opt(2000, 1, 1).unwrap(),
            phone_number: Some("1234567890".to_string()),
        })?;

        // Create organization
        let org_id = OrganizationQueries::create_entity(&mut conn, RequestCreateOrganization {
            name: "Test Organization".to_string(),
            description: Some("A test organization".to_string()),
            owner_id: user_id,
        })?;

        // Add user to organization
        let member_id = OrgMemberQueries::create_entity(&mut conn, RequestCreateOrgMember {
            user_id,
            org_id,
        })?;

        // Delete organization member
        let deleted_rows = OrgMemberQueries::delete_entity(&mut conn, member_id)?;
        assert_eq!(deleted_rows, 1);

        cleanup_test_db(conn)?;

        Ok(())
    }
}
