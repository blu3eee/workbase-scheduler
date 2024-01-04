use std::env;

use mysql::*;
use mysql::prelude::*;
use serde::Deserialize;
use bcrypt::{ hash, verify };

use crate::{
    models::{
        user::{ User, RequestCreateUser, RequestUpdateUser, create_users_table_query },
        result::Result,
    },
    prototypes::create_table::DatabaseTable,
};

use crate::prototypes::basic_queries::BasicQueries;

pub fn hash_password(password: &str) -> std::result::Result<String, bcrypt::BcryptError> {
    dotenv::dotenv().ok();
    let salt: u32 = env
        ::var("HASH_COST")
        .expect("HASH_COST must be set in the .env file")
        .parse::<u32>()
        .unwrap();
    let hashed_password = hash(password, salt)?;
    Ok(hashed_password)
}

pub fn verify_password(
    password: &str,
    hashed_password: &str
) -> std::result::Result<bool, bcrypt::BcryptError> {
    verify(password, hashed_password)
}

pub struct UserQueries {}

#[derive(Debug, Clone, Deserialize)]
pub struct LoginForm {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PasswordChangeForm {
    pub id: i64,
    pub old_password: String,
    pub new_password: String,
}

impl UserQueries {
    pub fn verify_password(conn: &mut PooledConn, login_form: LoginForm) -> Result<bool> {
        let encrypted_password: Option<String> = conn.exec_first(
            "SELECT encrypted_password FROM users WHERE email = :email;",
            params! { "email" => login_form.email }
        )?;
        if let Some(encrypted_password) = encrypted_password {
            Ok(verify_password(&login_form.password, &encrypted_password)?)
        } else {
            Err("Account does not exist.".into())
        }
    }

    pub fn change_password(conn: &mut PooledConn, change_form: PasswordChangeForm) -> Result<()> {
        let encrypted_password: Option<String> = conn.exec_first(
            "SELECT encrypted_password FROM users WHERE id = :id;",
            params! { "id" => change_form.id }
        )?;
        if let Some(encrypted_password) = encrypted_password {
            if verify_password(&change_form.old_password, &encrypted_password)? {
                let encrypted_password = hash_password(&change_form.new_password)?;
                conn.exec_drop(
                    "UPDATE users SET encrypted_password = :encrypted_password WHERE id = :id;",
                    params! {
                        "encrypted_password" => encrypted_password
                    }
                )?;
            } else {
                return Err("Wrong password.".into());
            }
        } else {
            return Err("Account does not exist.".into());
        }
        Ok(())
    }
}

impl DatabaseTable for UserQueries {
    fn create_table(&self, conn: &mut PooledConn) -> Result<()> {
        let query = create_users_table_query();
        let stmt = conn.prep(query)?;
        conn.exec_drop(stmt, ())?;

        Ok(())
    }
}

impl BasicQueries for UserQueries {
    type Model = User;

    type CreateDto = RequestCreateUser;

    type UpdateDto = RequestUpdateUser;

    fn table_name() -> String {
        "users".to_string()
    }

    fn insert_statement() -> String {
        format!(
            "INSERT INTO {} (id, email, encrypted_password, first_name, last_name, date_of_birth, phone_number)
            VALUES (:id, :email, :encrypted_password, :first_name, :last_name, :date_of_birth, :phone_number)",
            Self::table_name()
        )
    }

    fn insert_params(create_dto: &Self::CreateDto) -> Result<Params> {
        let encrypted_password = hash_password(&create_dto.password)?;
        Ok(
            params! {
                "email" => &create_dto.email,
                "encrypted_password" => &encrypted_password,
                "first_name" => &create_dto.first_name,
                "last_name" => &create_dto.last_name,
                "date_of_birth" => create_dto.date_of_birth.to_string(),
                "phone_number" => &create_dto.phone_number,
            }
        )
    }

    fn update_entity(conn: &mut PooledConn, update_dto: Self::UpdateDto) -> Result<u64> {
        let mut query = "UPDATE users SET ".to_string();
        let mut params: Vec<(String, Value)> = Vec::new();

        if let Some(first_name) = update_dto.first_name {
            query.push_str("first_name = :first_name, ");
            params.push(("first_name".to_string(), first_name.into()));
        }
        if let Some(last_name) = update_dto.last_name {
            query.push_str("last_name = :last_name, ");
            params.push(("last_name".to_string(), last_name.into()));
        }
        if let Some(date_of_birth) = update_dto.date_of_birth {
            query.push_str("date_of_birth = :date_of_birth, ");
            params.push(("date_of_birth".to_string(), date_of_birth.to_string().into()));
        }
        if let Some(phone_number) = update_dto.phone_number {
            query.push_str("phone_number = :phone_number, ");
            params.push(("phone_number".to_string(), phone_number.into()));
        }

        // Remove last comma and space if there are updates
        if !params.is_empty() {
            query.pop();
            query.pop();
        } else {
            return Ok(0);
        }

        query.push_str(&format!(" WHERE id = {};", update_dto.id));

        // Convert Vec to Params::Named
        let params = Params::from(params);

        // Use exec_fold to execute the query and process the result
        let query_result = conn.exec_iter(&query, params)?;

        // Return the number of affected rows
        Ok(query_result.affected_rows())
    }
}

#[cfg(test)]
mod test {
    use std::sync::Arc;

    use super::*;
    use chrono::NaiveDate;
    use crate::{ tests::{ initialize_test_db, cleanup_test_db }, snowflake::SnowflakeGenerator };

    #[test]
    fn test_users_table() -> Result<()> {
        // Setup database connection
        let mut conn = initialize_test_db()?;
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
                phone_number: None,
            }
        ];

        for user in &users {
            UserQueries::create_entity(&mut conn, snowflake_generator.clone(), user.clone())?;
        }

        assert!(
            UserQueries::verify_password(&mut conn, LoginForm {
                email: "user1@email.com".to_string(),
                password: "password1".to_string(),
            })?
        );
        assert!(
            UserQueries::verify_password(&mut conn, LoginForm {
                email: "user2@email.com".to_string(),
                password: "password2".to_string(),
            })?
        );

        // Let's select payments from database. Type inference should do the trick here.
        let all_users: Vec<User> = UserQueries::find_all(&mut conn)?;
        // println!("all users: {:?}", all_users);
        assert_eq!(all_users.len(), users.len());

        for user in &all_users {
            println!("{user:?}");
        }

        let user2: Option<User> = conn.exec_first(
            "SELECT * FROM users WHERE email = :email",
            params! { "email" => "user2@email.com" }
        )?;

        assert!(user2.is_some());
        let user2 = user2.unwrap();
        assert_eq!(user2.phone_number, None);

        // Update a user
        // Example: Update user1's name
        let affected_rows = UserQueries::update_entity(&mut conn, RequestUpdateUser {
            id: all_users[0].id,
            first_name: Some("Johnny".to_string()),
            ..Default::default()
        })?;
        println!("affected_row {affected_rows}");
        assert_eq!(affected_rows, 1);

        // Select user1 from the database to check the updated email
        let updated_user = UserQueries::find_by_id(&mut conn, all_users[0].id)?;

        // Assert that the email has been updated correctly
        assert_eq!(updated_user.first_name, "Johnny");

        // Delete a user
        // Example: Delete user1
        let deleted_rows = UserQueries::delete_entity(&mut conn, all_users[0].id)?;
        assert_eq!(deleted_rows, 1);

        // Clean up: Drop the database
        cleanup_test_db(conn)?;

        Ok(())
    }
}
