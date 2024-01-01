use mysql::*;
use mysql::prelude::*;

use crate::models::{
    organizations::{
        Organization,
        RequestCreateOrganization,
        RequestUpdateOrganization,
        create_organizations_table_query,
    },
    error::Result,
};

use super::BasicQueries;

pub struct OrgQueries {}

impl OrgQueries {
    pub fn find_by_id_with_owner(conn: &mut PooledConn, id: i32) -> Result<Organization> {
        // SQL query to select a user by ID
        let query =
            format!("
            SELECT  
                org.id as id,
                org.name as name,
                org.description as description,
                org.created_at as created_at,
                org.updated_at as updated_at,
                org.is_active as is_active,
                org.owner_id as owner_id,
                user.email as owner_email,
                user.first_name as owner_first_name,
                user.last_name as owner_last_name,
                user.date_of_birth as owner_date_of_birth,
                user.phone_number as owner_phone_number,
                user.created_at as owner_created_at,
                user.updated_at as owner_updated_at
            FROM organizations org
            LEFT JOIN users user ON user.id = org.owner_id 
            WHERE org.id = {};", id);

        // Execute the query
        let result: Option<<Self as BasicQueries>::Model> = conn.exec_first(query, ())?;

        // Extract the first row from the result (if any)
        if let Some(model) = result {
            // Convert the row into a User struct
            Ok(model)
        } else {
            // Return an error if no user is found
            Err(From::from("User not found"))
        }
    }
}

impl BasicQueries for OrgQueries {
    type Model = Organization;

    type CreateDto = RequestCreateOrganization;

    type UpdateDto = RequestUpdateOrganization;

    fn table_name() -> String {
        "organizations".to_string()
    }

    fn create_table(conn: &mut PooledConn) -> Result<()> {
        let query = create_organizations_table_query();
        let stmt = conn.prep(query)?;
        conn.exec_drop(stmt, ())?;

        Ok(())
    }

    fn create_entity(conn: &mut PooledConn, create_dto: Self::CreateDto) -> Result<i32> {
        conn.exec_drop(
            r"INSERT INTO organizations (name, description, owner_id)
              VALUES (:name, :description, :owner_id)",
            params! {
                "name" => &create_dto.name,
                "description" => &create_dto.description,
                "owner_id" => create_dto.owner_id,
            }
        )?;
        Ok(conn.last_insert_id() as i32)
    }

    fn update_entity(conn: &mut PooledConn, update_dto: Self::UpdateDto) -> Result<u64> {
        let mut query = "UPDATE organizations SET ".to_string();
        let mut params: Vec<(String, Value)> = Vec::new();

        if let Some(name) = update_dto.name {
            query.push_str("name = :name, ");
            params.push(("name".to_string(), name.into()));
        }
        if let Some(description) = update_dto.description {
            query.push_str("description = :description, ");
            params.push(("description".to_string(), description.into()));
        }
        if let Some(owner_id) = update_dto.owner_id {
            query.push_str("owner_id = :owner_id, ");
            params.push(("owner_id".to_string(), owner_id.into()));
        }

        // Remove trailing comma and space
        query.pop();
        query.pop();
        query.push_str(&format!(" WHERE id = {};", update_dto.id));

        // Convert Vec to Params::Named
        let params = Params::from(params);

        // Use exec_fold to execute the query and process the result
        let query_result = conn.exec_iter(&query, params)?;

        // Return the number of affected rows
        Ok(query_result.affected_rows())
    }

    fn delete_entity(conn: &mut PooledConn, id: i32) -> Result<u64> {
        let query_result = conn.query_iter(format!("DELETE FROM organizations WHERE id = {}", id))?;

        Ok(query_result.affected_rows())
    }
}

#[cfg(test)]
mod tests {
    use crate::models::users::RequestCreateUser;
    use crate::queries::users::UserQueries;
    use crate::tests::{ initialize_test_db, cleanup_test_db };

    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_organization_workflow() -> Result<()> {
        // Setup database connection
        let mut conn = initialize_test_db()?;

        let users = vec![
            RequestCreateUser {
                email: "user1@email.com".to_string(),
                password: "password1".to_string(),
                first_name: "John".to_string(),
                last_name: "Doe".to_string(),
                date_of_birth: NaiveDate::from_ymd_opt(2000, 1, 1).unwrap(), // Example date
                phone_number: Some("1234567890".to_string()),
            },
            RequestCreateUser {
                email: "user2@email.com".to_string(),
                password: "password2".to_string(),
                first_name: "Jane".to_string(),
                last_name: "Doe".to_string(),
                date_of_birth: NaiveDate::from_ymd_opt(2000, 11, 2).unwrap(), // Example date
                phone_number: Some("9094610000".to_string()),
            }
        ];
        let user_ids: Vec<i32> = users
            .iter()
            .filter_map(|user| {
                if let Some(user_id) = UserQueries::create_entity(&mut conn, user.clone()).ok() {
                    Some(user_id)
                } else {
                    None
                }
            })
            .collect::<Vec<i32>>();

        assert_eq!(user_ids.len(), users.len());

        // Get the ID of the inserted user
        let owner_user_id: i32 = conn.last_insert_id().try_into()?;

        // Create an organization with the dummy user as owner
        let org_id: i32 = OrgQueries::create_entity(&mut conn, RequestCreateOrganization {
            name: "Dummy Organization".to_string(),
            description: Some("A test organization".to_string()),
            owner_id: owner_user_id,
        })?;

        // Assert that the organization is linked to the owner
        let org = OrgQueries::find_by_id_with_owner(&mut conn, org_id)?;

        assert_eq!(org.id, org_id);
        assert_eq!(org.owner_id, owner_user_id);
        assert_eq!(org.name, "Dummy Organization".to_string());
        assert_eq!(org.description, Some("A test organization".to_string()));

        // Clean up: Drop the database
        cleanup_test_db(conn)?;

        Ok(())
    }
}
