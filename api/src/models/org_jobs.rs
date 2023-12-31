use chrono::NaiveDateTime;
use mysql::*;
use mysql::prelude::*;
use serde::{ Serialize, Deserialize };

pub fn create_org_job_table() -> String {
    "
    CREATE TABLE IF NOT EXISTS org_jobs (
        id INT AUTO_INCREMENT PRIMARY KEY,
        org_id INT,
        name VARCHAR(100) NOT NULL,
        description TEXT,
        base_pay_rate FLOAT,
        created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
        FOREIGN KEY (org_id) REFERENCES organizations(id) ON DELETE CASCADE
    );
    ".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrgJob {
    pub id: i32,
    pub org_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub base_pay_rate: f32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl FromRow for OrgJob {
    fn from_row(row: Row) -> OrgJob {
        let created_at: String = row.get("created_at").unwrap_or_default();
        let created_at = NaiveDateTime::parse_from_str(&created_at, "%Y-%m-%d %H:%M:%S").expect(
            "Failed to parse created_at"
        );
        let updated_at: String = row.get("updated_at").unwrap_or_default();
        let updated_at = NaiveDateTime::parse_from_str(&updated_at, "%Y-%m-%d %H:%M:%S").expect(
            "Failed to parse updated_at"
        );

        OrgJob {
            id: row.get("id").unwrap_or_default(),
            org_id: row.get("org_id").unwrap_or_default(),
            name: row.get("name").unwrap_or_default(),
            description: row.get("description").unwrap_or_default(),
            base_pay_rate: row.get("base_pay_rate").unwrap_or_default(),
            created_at,
            updated_at,
        }
    }

    fn from_row_opt(row: Row) -> Result<Self, FromRowError> {
        let id = row.get("id").ok_or(FromRowError(row.clone()))?;
        let org_id = row.get("org_id").ok_or(FromRowError(row.clone()))?;
        let name = row.get("name").ok_or(FromRowError(row.clone()))?;
        let description = row.get("description").ok_or(FromRowError(row.clone()))?;

        let base_pay_rate = row.get("base_pay_rate").ok_or(FromRowError(row.clone()))?;
        let created_at: String = row.get("created_at").ok_or(FromRowError(row.clone()))?;
        let created_at = NaiveDateTime::parse_from_str(&created_at, "%Y-%m-%d %H:%M:%S").expect(
            "Failed to parse created_at"
        );
        let updated_at: String = row.get("updated_at").ok_or(FromRowError(row.clone()))?;
        let updated_at: NaiveDateTime = NaiveDateTime::parse_from_str(
            &updated_at,
            "%Y-%m-%d %H:%M:%S"
        ).expect("Failed to parse updated_at");

        Ok(OrgJob {
            id,
            org_id,
            name,
            description,
            base_pay_rate,
            created_at,
            updated_at,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestCreateOrgJob {
    pub org_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub base_pay_rate: f32,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RequestUpdateOrgJob {
    pub id: i32,
    pub name: Option<String>,
    pub description: Option<String>,
    pub base_pay_rate: Option<f32>,
}

#[cfg(test)]
mod tests {
    use crate::models::organizations::{
        create_organizations_table_query,
        RequestCreateOrganization,
    };
    use crate::models::users::{ create_users_table_query, RequestCreateUser };
    use crate::models::org_jobs::{ create_org_job_table, RequestCreateOrgJob, OrgJob };
    use crate::tests::{ initialize_test_db, cleanup_test_db };

    use chrono::NaiveDate;
    use mysql::*;
    use mysql::prelude::*;

    #[test]
    fn test_org_custom_job() -> Result<(), Box<dyn std::error::Error>> {
        // Setup database connection
        let mut conn = initialize_test_db()?;

        // Create the tables and clean them
        conn.query_drop(create_users_table_query())?;
        conn.query_drop(create_organizations_table_query())?;
        conn.query_drop(create_org_job_table())?;

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
        let owner_user_id: i32 = conn.last_insert_id().try_into()?;

        // Insert an organization
        let org = RequestCreateOrganization {
            name: "Test Organization".to_string(),
            description: Some("A test organization".to_string()),
            owner_id: owner_user_id,
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

        // Insert a custom job
        let custom_job = RequestCreateOrgJob {
            org_id,
            name: "Manager".to_string(),
            description: Some("Manages the organization".to_string()),
            base_pay_rate: 30 as f32,
        };

        conn.exec_drop(
            "INSERT INTO org_jobs (org_id, name, description, base_pay_rate)
                  VALUES (:org_id, :name, :description, :base_pay_rate);",
            params! {
                "org_id" => custom_job.org_id,
                "name" => &custom_job.name,
                "description" => &custom_job.description,
                "base_pay_rate" => custom_job.base_pay_rate,
            }
        )?;
        let job_id: i32 = conn.last_insert_id().try_into()?;

        // Assert the custom job was correctly inserted
        let result: Vec<OrgJob> = conn.query(
            format!(
                "SELECT id, org_id, name, description, base_pay_rate, created_at, updated_at FROM org_jobs WHERE id = {job_id}"
            )
        )?;

        assert!(!result.is_empty());
        let inserted_job = result[0].clone();
        assert_eq!(inserted_job.org_id, org_id);
        assert_eq!(inserted_job.name, "Manager");
        assert_eq!(inserted_job.description, Some("Manages the organization".to_string()));
        assert_eq!(inserted_job.base_pay_rate, 30 as f32);

        // Clean up: Drop the database
        cleanup_test_db(conn)?;

        Ok(())
    }
}
