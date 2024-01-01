use std::fmt::Debug;

use mysql::{ PooledConn, prelude::{ FromRow, Queryable } };
use serde::{ de::DeserializeOwned, Serialize };

use crate::models::error::Result;

pub mod users;
pub mod organizations;
pub mod org_members;
pub mod org_member_jobs;
pub mod org_jobs;

/// The `BasicQueries` trait defines a set of basic operations for database interaction.
///
/// This trait is intended to be implemented for various models in the application,
/// providing a standardized interface for common CRUD operations.
pub trait BasicQueries {
    /// Specifies the model type associated with the query.
    ///
    /// This type should implement `DeserializeOwned`, `Serialize`, `Send`, `Sync`, `Debug`, `Clone`, and `FromRow` (crate `mysql`).
    type Model: DeserializeOwned + Serialize + Send + Sync + Debug + Clone + FromRow;

    /// Data Transfer Object (DTO) for creating entities.
    ///
    /// This type should implement `DeserializeOwned`, `Send`, `Sync`, and `Debug`.
    type CreateDto: DeserializeOwned + Send + Sync + Debug;
    /// DTO for updating entities.
    ///
    /// This type should implement `DeserializeOwned`, `Send`, `Sync`, `Debug`, and `Default`.
    type UpdateDto: DeserializeOwned + Send + Sync + Debug + Default;

    /// Returns the table name associated with the model.
    ///
    /// This method should provide the name of the database table that corresponds to the `Model`.
    fn table_name() -> String;

    /// Creates the table in the database.
    ///
    /// This method should handle the creation of the database table if it does not already exist.
    fn create_table(conn: &mut PooledConn) -> Result<()>;

    /// Creates a new entity in the database.
    ///
    /// This method should insert a new record into the database using the provided `create_dto`.
    fn create_entity(conn: &mut PooledConn, create_dto: Self::CreateDto) -> Result<i32>;

    /// Updates an existing entity in the database.
    ///
    /// This method should update an existing record in the database based on the provided `update_dto`.
    fn update_entity(conn: &mut PooledConn, update_dto: Self::UpdateDto) -> Result<u64>;

    /// Deletes an entity from the database.
    ///
    /// This method should remove a record from the database corresponding to the specified `id`.
    fn delete_entity(conn: &mut PooledConn, id: i32) -> Result<u64>;

    /// Retrieves all entities of the model from the database.
    ///
    /// This method should return a vector of all records for the `Model` from the database.
    fn find_all(conn: &mut PooledConn) -> Result<Vec<Self::Model>> {
        Ok(conn.query(format!("SELECT * FROM {};", Self::table_name()))?)
    }

    /// Retrieves a single entity by its ID.
    ///
    /// This method should return the record of the `Model` corresponding to the specified `id`.
    fn find_by_id(conn: &mut PooledConn, id: i32) -> Result<Self::Model> {
        // SQL query to select a row by ID
        let query = format!("SELECT * FROM {} WHERE id = {};", Self::table_name(), id);

        // Execute the query
        let result: Option<Self::Model> = conn.exec_first(query, ())?;

        // Extract the first row from the result (if any)
        if let Some(model) = result {
            // Convert the row into a Self::Model struct
            Ok(model)
        } else {
            // Return an error if no user is found
            Err(From::from("User not found"))
        }
    }
}
