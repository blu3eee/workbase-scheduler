use std::sync::Arc;

use mysql::*;
use mysql::prelude::*;

use crate::models::company::company_employee::{
    create_company_employees_table_query,
    CompanyEmployee,
    RequestCreateCompanyEmployee,
    RequestUpdateCompanyEmployee,
};
use crate::models::result::Result;
use crate::prototypes::create_table::DatabaseTable;
use crate::snowflake::{ SnowflakeGenerator, SnowflakeId };

pub struct CompanyEmployeeQueries {}

impl DatabaseTable for CompanyEmployeeQueries {
    fn create_table(&self, conn: &mut PooledConn) -> crate::models::result::Result<()> {
        let query = create_company_employees_table_query();
        let stmt = conn.prep(query)?;
        conn.exec_drop(stmt, ())?;

        Ok(())
    }
}

impl CompanyEmployeeQueries {
    fn table_name() -> String {
        "company_employees".to_string()
    }

    fn insert_statement() -> String {
        format!(
            "INSERT INTO {} (user_id, company_id, punch_id) VALUES (:user_id, :company_id, :punch_id)",
            Self::table_name()
        )
    }

    fn insert_params(create_dto: &RequestCreateCompanyEmployee) -> Result<Params> {
        Ok(
            params! {
                "user_id" => &create_dto.user_id,
                "company_id" => &create_dto.company_id,
                "punch_id" => &create_dto.punch_id,
            }
        )
    }

    fn create_entity_exec(
        conn: &mut PooledConn,
        snowflake_generator: Arc<SnowflakeGenerator>,
        create_dto: &RequestCreateCompanyEmployee
    ) -> Result<i64> {
        let id = snowflake_generator.generate_id();

        // Convert Params to Named if not already
        let params_map = if let Params::Named(map) = Self::insert_params(create_dto)? {
            map
        } else {
            panic!("Expected named parameters");
        };

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
        _create_dto: RequestCreateCompanyEmployee,
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
    pub fn create_entity(
        conn: &mut PooledConn,
        snowflake_generator: Arc<SnowflakeGenerator>,
        create_dto: RequestCreateCompanyEmployee
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
    pub fn create_many(
        conn: &mut PooledConn,
        snowflake_generator: Arc<SnowflakeGenerator>,
        create_dtos: Vec<RequestCreateCompanyEmployee>
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

    pub fn update_entity(
        conn: &mut PooledConn,
        user_id: SnowflakeId,
        company_id: SnowflakeId,
        update_dto: RequestUpdateCompanyEmployee
    ) -> Result<u64> {
        let mut query = format!("UPDATE {} SET ", Self::table_name());
        let mut params: Vec<(String, Value)> = Vec::new();

        if let Some(punch_id) = update_dto.punch_id {
            query.push_str("punch_id = :punch_id, ");
            params.push(("punch_id".to_string(), punch_id.into()));
        }
        if let Some(hired_date) = update_dto.hired_date {
            query.push_str("hired_date = :hired_date, ");
            params.push(("hired_date".to_string(), hired_date.to_string().into()));
        }
        if let Some(notes) = update_dto.notes {
            query.push_str("notes = :notes, ");
            params.push(("notes".to_string(), notes.into()));
        }

        // Remove last comma and space if there are updates
        if !params.is_empty() {
            query.pop();
            query.pop();
        } else {
            return Ok(0);
        }

        query.push_str(&format!(" WHERE user_id = {} AND company_id = {};", user_id, company_id));

        // Convert Vec to Params::Named
        let params = Params::from(params);

        // Use exec_fold to execute the query and process the result
        let query_result = conn.exec_iter(&query, params)?;

        // Return the number of affected rows
        Ok(query_result.affected_rows())
    }

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
    pub fn delete_entity(
        conn: &mut PooledConn,
        user_id: SnowflakeId,
        company_id: SnowflakeId
    ) -> Result<u64> {
        let query = format!(
            "DELETE FROM {}WHERE user_id = {} AND company_id = {};",
            Self::table_name(),
            user_id,
            company_id
        );
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
    pub fn find_all(conn: &mut PooledConn) -> Result<Vec<CompanyEmployee>> {
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
    pub fn find_by_id(conn: &mut PooledConn, id: i64) -> Result<CompanyEmployee> {
        // SQL query to select a row by ID
        let query = format!("SELECT * FROM {} WHERE id = {};", Self::table_name(), id);

        // Execute the query
        let result: Option<CompanyEmployee> = conn.exec_first(query, ())?;

        // Extract the first row from the result (if any)
        if let Some(model) = result {
            // Convert the row into a CompanyEmployee struct
            Ok(model)
        } else {
            // Return an error if no user is found
            Err("User not found".into())
        }
    }
}
