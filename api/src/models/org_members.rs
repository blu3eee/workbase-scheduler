use chrono::NaiveDateTime;
use mysql::{ prelude::FromRow, FromRowError };
use serde::{ Serialize, Deserialize };

use super::convert_to_naive_date_time;

pub fn create_org_members_table_query() -> String {
    "
    CREATE TABLE IF NOT EXISTS org_members (
        id INT AUTO_INCREMENT PRIMARY KEY,
        user_id INT NOT NULL,
        org_id INT NOT NULL,
        joined_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
        FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
        FOREIGN KEY (org_id) REFERENCES organizations(id) ON DELETE CASCADE,
        UNIQUE KEY user_org_unique (user_id, org_id)
    );
    ".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrgMember {
    pub id: i32,
    pub user_id: i32,
    pub org_id: i32,
    pub joined_at: NaiveDateTime,
}

// For creating a new user-organization relationship
#[derive(Debug, Clone, Deserialize)]
pub struct RequestCreateOrgMember {
    pub user_id: i32,
    pub org_id: i32,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct RequestUpdateOrgMember {
    pub id: i32,
}

impl FromRow for OrgMember {
    fn from_row_opt(row: mysql::Row) -> Result<Self, mysql::FromRowError> where Self: Sized {
        Ok(OrgMember {
            id: row.get("id").ok_or(FromRowError(row.clone()))?,
            user_id: row.get("user_id").ok_or(FromRowError(row.clone()))?,
            org_id: row.get("org_id").ok_or(FromRowError(row.clone()))?,
            joined_at: convert_to_naive_date_time(
                row.get("joined_at").ok_or(FromRowError(row.clone()))?
            ),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::models::users::{ create_users_table_query, RequestCreateUser };
    use crate::models::organizations::{
        create_organizations_table_query,
        RequestCreateOrganization,
    };
    use crate::models::org_members::{ create_org_members_table_query, RequestCreateOrgMember };
    use crate::tests::{ cleanup_test_db, initialize_test_db };

    use chrono::NaiveDate;
    use mysql::*;
    use mysql::prelude::*;

    #[test]
    fn test_org_members() -> Result<(), Box<dyn std::error::Error>> {
        // Setup database connection
        let mut conn = initialize_test_db()?;

        // Create the tables and clean them
        conn.query_drop(create_users_table_query())?;
        conn.query_drop(create_organizations_table_query())?;
        conn.query_drop(create_org_members_table_query())?;

        // Insert a user
        let user = RequestCreateUser {
            email: "user1@email.com".to_string(),
            password: "password".to_string(),
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            date_of_birth: NaiveDate::from_ymd_opt(2000, 1, 1).unwrap(), // Example date
            phone_number: Some("1234567890".to_string()),
        };

        conn.exec_drop(
            r"INSERT INTO users (email, password, first_name, last_name, date_of_birth, phone_number)
              VALUES (:email, :password, :first_name, :last_name, :date_of_birth, :phone_number);",
            params! {
                "email" => &user.email,
                "password" => &user.password,
                "first_name" => &user.first_name,
                "last_name" => &user.last_name,
                "date_of_birth" => user.date_of_birth.to_string(),
                "phone_number" => &user.phone_number,
            }
        )?;

        let user_id: i32 = conn.last_insert_id().try_into()?;
        println!("inserted owner_id {}", user_id);

        // Insert an organization
        let org = RequestCreateOrganization {
            name: "Test Organization".to_string(),
            description: Some("A test organization".to_string()),
            owner_id: user_id, // Assume the user is the owner
        };

        conn.exec_drop(
            "INSERT INTO organizations (name, description, owner_id) VALUES (:name, :description, :owner_id)",
            params! {
                "name" => &org.name,
                "description" => &org.description,
                "owner_id" => org.owner_id,
            }
        )?;

        let org_id: i32 = conn.last_insert_id().try_into()?;
        println!("inserted org_id {}", org_id);

        // Insert a org_members relationship
        let org_members = RequestCreateOrgMember {
            user_id,
            org_id,
        };

        conn.exec_drop(
            "INSERT INTO org_members (user_id, org_id) VALUES (:user_id, :org_id)",
            params! {
                "user_id" => org_members.user_id,
                "org_id" => org_members.org_id,
                
            }
        )?;

        // Assert the org_members relationship
        let result: Option<(i32, i32)> = conn.exec_first(
            "SELECT user_id, org_id FROM org_members WHERE user_id = :user_id AND org_id = :org_id",
            params! {
                "user_id" => org_members.user_id,
                "org_id" => org_members.org_id,
            }
        )?;

        assert_eq!(result, Some((user_id, org_id)));

        // Clean up

        // Drop the database
        cleanup_test_db(conn)?;

        Ok(())
    }
}
