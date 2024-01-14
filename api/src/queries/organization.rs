use std::sync::Arc;

use mysql::*;
use mysql::prelude::*;

use crate::{
    models::{
        company::{
            Company,
            RequestCreateCompany,
            RequestUpdateCompany,
            create_companies_table_query,
        },
        result::Result,
        company_job::RequestCreateCompanyJob,
        company_member::RequestCreateCompanyMember,
    },
    prototypes::{ basic_queries::BasicQueries, create_table::DatabaseTable },
    snowflake::SnowflakeGenerator,
};

use super::{ company_job::CompanyJobQueries, company_member::CompanyMemberQueries };

pub struct CompanyQueries {}

impl CompanyQueries {
    pub fn find_by_id_with_owner(conn: &mut PooledConn, id: i64) -> Result<Company> {
        // SQL query to select a user by ID
        let query =
            format!("
            SELECT  
                company.id as id,
                company.name as name,
                company.description as description,
                company.updated_at as updated_at,
                company.is_active as is_active,
                company.owner_id as owner_id,
                company.icon as icon,
                user.email as owner_email,
                user.first_name as owner_first_name,
                user.last_name as owner_last_name,
                user.date_of_birth as owner_date_of_birth,
                user.phone_number as owner_phone_number,
                user.avatar as owner_avatar,
                user.is_active as owner_is_active
            FROM companies company
            LEFT JOIN users user ON user.id = company.owner_id 
            WHERE company.id = {};", id);

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

impl DatabaseTable for CompanyQueries {
    fn create_table(&self, conn: &mut PooledConn) -> Result<()> {
        let query = create_companies_table_query();
        let stmt = conn.prep(query)?;
        conn.exec_drop(stmt, ())?;

        Ok(())
    }
}

impl BasicQueries for CompanyQueries {
    type Model = Company;

    type CreateDto = RequestCreateCompany;

    type UpdateDto = RequestUpdateCompany;

    fn table_name() -> String {
        "companies".to_string()
    }

    fn insert_statement() -> String {
        format!(
            "INSERT INTO {} (id, name, description, owner_id) VALUES (:id, :name, :description, :owner_id)",
            Self::table_name()
        )
    }

    fn insert_params(create_dto: &Self::CreateDto) -> Result<Params> {
        Ok(
            params! {
                "name" => &create_dto.name,
                "description" => &create_dto.description,
                "owner_id" => create_dto.owner_id,
            }
        )
    }

    fn create_entity_postprocessor(
        conn: &mut PooledConn,
        snowflake_generator: Arc<SnowflakeGenerator>,
        create_dto: Self::CreateDto,
        id: i64
    ) -> Result<i64> {
        let company_id = id;

        let job_id = CompanyJobQueries::create_entity(
            conn,
            snowflake_generator.clone(),
            RequestCreateCompanyJob {
                company_id,
                name: "Dummy".to_string(),
                description: Some("Dummy for job placeholders".to_string()),
                base_pay_rate: 0.0,
                color: None,
            }
        )?;

        let _ = CompanyMemberQueries::create_entity(conn, RequestCreateCompanyMember {
            company_id,
            user_id: create_dto.owner_id,
            job_id,
        })?;

        Ok(company_id)
    }

    fn update_entity(conn: &mut PooledConn, id: i64, update_dto: Self::UpdateDto) -> Result<u64> {
        let mut query = "UPDATE companies SET ".to_string();
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

#[cfg(test)]
mod tests {
    use crate::models::user::RequestCreateUser;
    use crate::queries::user::UserQueries;
    use crate::tests::{ initialize_test_db, cleanup_test_db };

    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_company_workflow() -> Result<()> {
        // Setup database connection
        let pool = initialize_test_db()?;
        let mut conn = pool.get_conn()?;

        let snowflake_generator = Arc::new(SnowflakeGenerator::new(1));

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
        let user_ids: Vec<i64> = users
            .iter()
            .filter_map(|user| {
                if
                    let Some(user_id) = UserQueries::create_entity(
                        &mut conn,
                        snowflake_generator.clone(),
                        user.clone()
                    ).ok()
                {
                    Some(user_id)
                } else {
                    None
                }
            })
            .collect::<Vec<i64>>();

        assert_eq!(user_ids.len(), users.len());

        let owner_user_id = user_ids[0];

        // Create an company with the dummy user as owner
        let company_id: i64 = CompanyQueries::create_entity(
            &mut conn,
            snowflake_generator.clone(),
            RequestCreateCompany {
                name: "Dummy Company".to_string(),
                description: Some("A test company".to_string()),
                owner_id: owner_user_id,
                timezone: None,
                icon: None,
            }
        )?;

        // Assert that the company is linked to the owner
        let company = CompanyQueries::find_by_id_with_owner(&mut conn, company_id)?;

        assert_eq!(company.id, company_id);
        assert_eq!(company.owner_id, owner_user_id);
        assert_eq!(company.name, "Dummy Company".to_string());
        assert_eq!(company.description, Some("A test company".to_string()));

        // Clean up: Drop the database
        cleanup_test_db(conn)?;

        Ok(())
    }
}
