use std::error::Error;

use mysql::*;
use mysql::prelude::*;

use crate::models::users::{ User, RequestCreateUser, RequestUpdateUser, create_users_table_query };

use super::BasicQueries;

pub struct UserQueries {}

impl BasicQueries for UserQueries {
    type Model = User;

    type CreateDto = RequestCreateUser;

    type UpdateDto = RequestUpdateUser;

    fn table_name() -> String {
        "users".to_string()
    }

    fn create_table(conn: &mut PooledConn) -> Result<(), Box<dyn Error>> {
        let query = create_users_table_query();
        let stmt = conn.prep(query)?;
        conn.exec_drop(stmt, ())?;

        Ok(())
    }

    fn create_entity(
        conn: &mut PooledConn,
        create_dto: Self::CreateDto
    ) -> Result<i32, Box<dyn Error>> {
        // Now let's insert payments to the database
        conn.exec_drop(
            r"INSERT INTO users (email, password, first_name, last_name, date_of_birth, phone_number)
              VALUES (:email, :password, :first_name, :last_name, :date_of_birth, :phone_number);",
            params! {
                "email" => &create_dto.email,
                "password" => &create_dto.password,
                "first_name" => &create_dto.first_name,
                "last_name" => &create_dto.last_name,
                "date_of_birth" => create_dto.date_of_birth.to_string(),
                "phone_number" => &create_dto.phone_number,}
        )?;
        Ok(conn.last_insert_id().try_into()?)
    }

    fn update_entity(
        conn: &mut PooledConn,
        update_dto: Self::UpdateDto
    ) -> Result<u64, Box<dyn Error>> {
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

    fn delete_entity(conn: &mut PooledConn, id: i32) -> Result<u64, Box<dyn Error>> {
        let query_result = conn.query_iter(format!("DELETE FROM users WHERE id = {}", id))?;

        Ok(query_result.affected_rows())
    }
}

#[cfg(test)]
mod test {
    use chrono::NaiveDate;

    use crate::tests::{ initialize_test_db, cleanup_test_db };

    use super::*;

    #[test]
    fn test_users_table() -> Result<(), Box<dyn std::error::Error>> {
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

        for user in &users {
            UserQueries::create_entity(&mut conn, user.clone())?;
        }

        // Let's select payments from database. Type inference should do the trick here.
        let selected_users: Vec<User> = conn.query("SELECT * from users;")?;

        assert_eq!(selected_users.len(), users.len());

        // Update a user
        // Example: Update user1's name
        let affected_rows = UserQueries::update_entity(&mut conn, RequestUpdateUser {
            id: selected_users[0].id,
            first_name: Some("Johnny".to_string()),
            ..Default::default()
        })?;
        println!("affected_row {affected_rows}");
        assert_eq!(affected_rows, 1);

        // Select user1 from the database to check the updated email
        let updated_user = UserQueries::find_by_id(&mut conn, selected_users[0].id)?;

        // Assert that the email has been updated correctly

        assert_eq!(updated_user.first_name, "Johnny");

        // Delete a user
        // Example: Delete user1
        let deleted_rows = UserQueries::delete_entity(&mut conn, selected_users[0].id)?;
        assert_eq!(deleted_rows, 1);

        conn.exec_drop("DELETE FROM users;", ())?;

        // Clean up: Drop the database
        cleanup_test_db(conn)?;

        Ok(())
    }
}
