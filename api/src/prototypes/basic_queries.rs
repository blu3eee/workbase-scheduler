use std::{ fmt::Debug, sync::Arc };

use mysql::{ PooledConn, prelude::{ FromRow, Queryable }, Params, Value };
use serde::{ de::DeserializeOwned, Serialize };

use crate::{ models::result::Result, snowflake::SnowflakeGenerator };

/// The `BasicQueries` trait defines a set of basic CRUD (Create, Read, Update, Delete) operations for database interaction.
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
    /// # Returns
    /// A `String` representing the name of the database table for the `Model`.
    fn table_name() -> String;

    /// Provides an SQL insert statement for the model.
    ///
    /// # Returns
    /// A `String` containing an SQL insert statement used in the `create_entity_exec` method.
    fn insert_statement() -> String;

    /// Converts a DTO into a set of parameters for the SQL insert statement.
    ///
    /// # Arguments
    /// * `create_dto` - A reference to an instance of the `CreateDto` type.
    ///
    /// # Returns
    /// A `Result` wrapping `Params` used in the SQL insert statement execution.
    fn insert_params(create_dto: &Self::CreateDto) -> Result<Params>;

    /// Executes the SQL insert statement to create a new entity in the database.
    ///
    /// # Arguments
    /// * `conn` - A mutable reference to a pooled database connection.
    /// * `snowflake_generator` - A reference to a Snowflake ID generator.
    /// * `create_dto` - A reference to an instance of the `CreateDto` type.
    ///
    /// # Returns
    /// A `Result` containing the Snowflake ID of the newly created entity.
    fn create_entity_exec(
        conn: &mut PooledConn,
        snowflake_generator: Arc<SnowflakeGenerator>,
        create_dto: &Self::CreateDto
    ) -> Result<i64> {
        let id = snowflake_generator.generate_id();

        // Convert Params to Named if not already
        let mut params_map = if let Params::Named(map) = Self::insert_params(create_dto)? {
            map
        } else {
            panic!("Expected named parameters");
        };

        // Add Snowflake ID to parameters
        params_map.insert("id".to_string().into_bytes(), Value::from(id));

        // Execute the query with the updated parameters
        conn.exec_drop(Self::insert_statement(), Params::Named(params_map))?;

        Ok(id)
    }

    /// Optional method for post-processing after creating a new entity.
    ///
    /// # Arguments
    /// * `conn` - A mutable reference to a pooled database connection.
    /// * `snowflake_generator` - A reference to a Snowflake ID generator.
    /// * `create_dto` - An instance of the `CreateDto` type.
    /// * `id` - The Snowflake ID of the newly created entity.
    ///
    /// # Returns
    /// A `Result` containing the Snowflake ID of the newly created entity.
    fn create_entity_postprocessor(
        _conn: &mut PooledConn,
        _snowflake_generator: Arc<SnowflakeGenerator>,
        _create_dto: Self::CreateDto,
        id: i64
    ) -> Result<i64> {
        Ok(id)
    }

    /// Creates a new entity in the database.
    ///
    /// # Arguments
    /// * `conn` - A mutable reference to a pooled database connection.
    /// * `snowflake_generator` - A reference to a Snowflake ID generator.
    /// * `create_dto` - An instance of the `CreateDto` type.
    ///
    /// # Returns
    /// A `Result` containing the Snowflake ID of the newly created entity.
    fn create_entity(
        conn: &mut PooledConn,
        snowflake_generator: Arc<SnowflakeGenerator>,
        create_dto: Self::CreateDto
    ) -> Result<i64> {
        let id = Self::create_entity_exec(conn, snowflake_generator.clone(), &create_dto)?;

        Ok(Self::create_entity_postprocessor(conn, snowflake_generator, create_dto, id)?)
    }

    /// Inserts multiple new entities into the database.
    ///
    /// This method allows batch insertion of multiple entities. It uses the `exec_batch` method
    /// to execute the insert statement for each entity in the provided vector.
    ///
    /// # Arguments
    ///
    /// * `conn`: A mutable reference to a pooled database connection.
    /// * `create_dtos`: A vector of `CreateDto` instances to be inserted.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure of the operation.
    fn create_many(
        conn: &mut PooledConn,
        snowflake_generator: Arc<SnowflakeGenerator>,
        create_dtos: Vec<Self::CreateDto>
    ) -> Result<()> {
        let params_iter = create_dtos
            .iter()
            .map(|create_dto| {
                let mut params_map = if let Params::Named(map) = Self::insert_params(create_dto)? {
                    map
                } else {
                    panic!("Expected named parameters");
                };

                // Generate a unique Snowflake ID for each entity
                let id = snowflake_generator.generate_id();
                params_map.insert("id".to_string().into_bytes(), Value::from(id));

                Ok(Params::Named(params_map))
            })
            .collect::<Result<Vec<_>>>()?; // Collect into Result<Vec<Params>, _>

        conn.exec_batch(
            Self::insert_statement(),
            params_iter.into_iter() // Now params_iter is Iterator<Item = Params>
        )?;
        Ok(())
    }

    /// Updates an existing entity in the database.
    ///
    /// This method is responsible for updating a record in the database. The `update_dto` contains
    /// the updated values for the specified entity.
    ///
    /// # Arguments
    ///
    /// * `conn`: A mutable reference to a pooled database connection.
    /// * `update_dto`: An instance of the `UpdateDto` type containing the updated values.
    ///
    /// # Returns
    ///
    /// A `Result` wrapping the number of affected rows. If no rows are affected, it implies the update
    /// operation did not change any existing data or the specified ID does not exist.
    fn update_entity(conn: &mut PooledConn, id: i64, update_dto: Self::UpdateDto) -> Result<u64>;

    /// Deletes an entity from the database.
    ///
    /// This method removes a record from the database corresponding to the specified `id`. It is
    /// typically used to implement 'delete' operations in CRUD.
    ///
    /// # Arguments
    ///
    /// * `conn`: A mutable reference to a pooled database connection.
    /// * `id`: The ID of the entity to be deleted.
    ///
    /// # Returns
    ///
    /// A `Result` wrapping the number of affected rows. A result of zero implies no record was found
    /// with the given ID.
    fn delete_entity(conn: &mut PooledConn, id: i64) -> Result<u64> {
        let query = format!("DELETE FROM {} WHERE id = {}", Self::table_name(), id);
        let query_result = conn.query_iter(&query)?;
        Ok(query_result.affected_rows())
    }

    /// Retrieves all entities of a specific model from the database.
    ///
    /// This method fetches all records for the `Model` from the database, returning them as a vector.
    ///
    /// # Arguments
    ///
    /// * `conn`: A mutable reference to a pooled database connection.
    ///
    /// # Returns
    ///
    /// A `Result` wrapping a vector of `Model` instances. If no records are found, an empty vector is returned.
    fn find_all(conn: &mut PooledConn) -> Result<Vec<Self::Model>> {
        Ok(conn.query(format!("SELECT * FROM {};", Self::table_name()))?)
    }

    /// Retrieves a single entity by its ID.
    ///
    /// This method fetches a single record of the `Model` corresponding to the specified `id`. It is
    /// commonly used to implement 'read' operations in CRUD.
    ///
    /// # Arguments
    ///
    /// * `conn`: A mutable reference to a pooled database connection.
    /// * `id`: The ID of the entity to be fetched.
    ///
    /// # Returns
    ///
    /// A `Result` wrapping the `Model` instance if found. If no record is found with the given ID,
    /// an error is returned.
    fn find_by_id(conn: &mut PooledConn, id: i64) -> Result<Self::Model> {
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
            Err("User not found".into())
        }
    }
}
