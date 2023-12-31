use std::{ error::Error, fmt::Debug };

use mysql::{ PooledConn, prelude::{ FromRow, Queryable } };
use serde::{ de::DeserializeOwned, Serialize };

pub mod users;
pub mod organizations;
pub mod org_members;
pub mod org_member_jobs;
pub mod org_jobs;

pub trait BasicQueries {
    type Model: DeserializeOwned + Serialize + Send + Sync + Debug + Clone + FromRow;

    type CreateDto: DeserializeOwned + Send + Sync + Debug;
    type UpdateDto: DeserializeOwned + Send + Sync + Debug + Default;

    fn table_name() -> String;

    fn create_table(conn: &mut PooledConn) -> Result<(), Box<dyn Error>>;

    fn create_entity(
        conn: &mut PooledConn,
        create_dto: Self::CreateDto
    ) -> Result<i32, Box<dyn Error>>;

    fn update_entity(
        conn: &mut PooledConn,
        update_dto: Self::UpdateDto
    ) -> Result<u64, Box<dyn Error>>;

    fn delete_entity(conn: &mut PooledConn, id: i32) -> Result<u64, Box<dyn Error>>;

    fn find_all(conn: &mut PooledConn) -> Result<Vec<Self::Model>, Box<dyn Error>> {
        Ok(conn.query(format!("SELECT * FROM {};", Self::table_name()))?)
    }

    fn find_by_id(conn: &mut PooledConn, id: i32) -> Result<Self::Model, Box<dyn Error>> {
        // SQL query to select a user by ID
        let query = format!("SELECT * FROM {} WHERE id = {};", Self::table_name(), id);

        // Execute the query
        let result: Option<Self::Model> = conn.exec_first(query, ())?;

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
