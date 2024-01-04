use async_trait::async_trait;
use axum::{
    Extension,
    Json,
    extract::Path,
    Router,
    routing::{ get, post, patch, delete },
    http::StatusCode,
    response::IntoResponse,
};
use crate::{
    app::{ AppState, AppResult, ApiResponse },
    utilities::app_error::AppError,
    models::{ ResponseDataList, ResponseDataJson },
};

use super::{ create_table::DatabaseTable, basic_queries::BasicQueries };

type PrimaryKey = i64;

/// The `UniqueIdRouter` trait provides a set of standard routing functionalities for entities
/// that can be identified by a unique ID, particularly useful in RESTful APIs.
///
/// This trait is intended to be implemented for various entities in the application,
/// providing a standardized interface for typical CRUD operations via HTTP endpoints.
#[async_trait]
pub trait UniqueIdRouter: 'static {
    /// The type that implements `DatabaseTable` and `BasicQueries`.
    /// It specifies the model and its associated CRUD operations.
    type Queries: DatabaseTable + BasicQueries;

    /// Returns a string representing the base path for the routes associated with the entity.
    ///
    /// # Returns
    /// A `String` representing the base path for the entity's routes.
    fn path() -> String;

    /// Asynchronously retrieves an entity by its ID.
    ///
    /// # Arguments
    /// * `state` - Application state containing database pool and other configurations.
    /// * `id` - The primary key ID of the entity to be fetched.
    ///
    /// # Returns
    /// An `ApiResponse` that wraps a JSON response containing the entity or an error message.
    async fn get_by_id(
        Extension(state): Extension<AppState>,
        Path(id): Path<PrimaryKey>
    ) -> ApiResponse {
        let mut conn = state.db_pool.get_conn()?;
        match Self::Queries::find_by_id(&mut conn, id) {
            Ok(model) => {
                let json = Json(ResponseDataJson { data: model });
                let mut response = json.into_response();
                *response.status_mut() = StatusCode::OK;
                Ok(response)
            }
            Err(_) => {
                return Err(AppError::not_found("not found"));
            }
        }
    }

    /// Asynchronously retrieves all entities.
    ///
    /// # Arguments
    /// * `state` - Application state containing database pool and other configurations.
    ///
    /// # Returns
    /// An `AppResult` that wraps a JSON response containing a list of entities.
    async fn get_all(Extension(
        state,
    ): Extension<AppState>) -> AppResult<Json<ResponseDataList<<Self::Queries as BasicQueries>::Model>>> {
        let mut conn = state.db_pool.get_conn()?;
        let models = Self::Queries::find_all(&mut conn).map_err(|_|
            AppError::internal_server_error("Failed to fetch records")
        )?;

        Ok(Json(ResponseDataList { data: models }))
    }

    /// Asynchronously creates a new entity.
    ///
    /// # Arguments
    /// * `state` - Application state containing database pool and other configurations.
    /// * `create_dto` - The data transfer object used for creating the new entity.
    ///
    /// # Returns
    /// An `ApiResponse` that wraps a JSON response containing the ID of the newly created entity or an error message.
    async fn create_entity(
        Extension(state): Extension<AppState>,
        Json(create_dto): Json<<Self::Queries as BasicQueries>::CreateDto>
    ) -> ApiResponse {
        let mut conn = state.db_pool.get_conn()?;
        match
            Self::Queries::create_entity(&mut conn, state.snowflake_generator.clone(), create_dto)
        {
            Ok(id) => {
                let json = Json(ResponseDataJson { data: id });
                let mut response = json.into_response();
                *response.status_mut() = StatusCode::CREATED;
                Ok(response)
            }
            Err(e) => {
                println!("error {e}");
                Err(AppError::from(e))
            }
        }
    }

    /// Asynchronously updates an existing entity.
    ///
    /// # Arguments
    /// * `state` - Application state containing database pool and other configurations.
    /// * `id` - The primary key ID of the entity to be updated.
    /// * `update_dto` - The data transfer object containing the updated data.
    ///
    /// # Returns
    /// An `ApiResponse` that wraps a JSON response with the number of affected rows or an error message.
    async fn update_entity(
        Extension(state): Extension<AppState>,
        Path(id): Path<PrimaryKey>,
        Json(update_dto): Json<<Self::Queries as BasicQueries>::UpdateDto>
    ) -> ApiResponse {
        let mut conn = state.db_pool.get_conn()?;
        match Self::Queries::update_entity(&mut conn, id, update_dto) {
            Ok(affected_rows) => {
                let json = Json(ResponseDataJson { data: affected_rows });
                let mut response = json.into_response();
                *response.status_mut() = StatusCode::OK;
                Ok(response)
            }
            Err(e) => { Err(AppError::from(e)) }
        }
    }

    /// Asynchronously deletes an entity.
    ///
    /// # Arguments
    /// * `state` - Application state containing database pool and other configurations.
    /// * `id` - The primary key ID of the entity to be deleted.
    ///
    /// # Returns
    /// An `ApiResponse` that wraps a JSON response with the number of affected rows or an error message.
    async fn delete_entity(
        Extension(state): Extension<AppState>,
        Path(id): Path<PrimaryKey>
    ) -> ApiResponse {
        let mut conn = state.db_pool.get_conn()?;
        match Self::Queries::delete_entity(&mut conn, id) {
            Ok(affected_rows) => {
                let json = Json(ResponseDataJson { data: affected_rows });
                let mut response = json.into_response();
                *response.status_mut() = StatusCode::OK;
                Ok(response)
            }
            Err(e) => { Err(AppError::from(e)) }
        }
    }

    /// Asynchronously creates the default routes for the entity.
    ///
    /// This method sets up standard RESTful routes, such as `GET`, `POST`, `PATCH`, and `DELETE`.
    ///
    /// # Returns
    /// A `Router` configured with the default routes for the entity.
    async fn default_routes() -> Router {
        Router::new()
            .route("/", get(Self::get_all))
            .route("/:id", get(Self::get_by_id))
            .route("/", post(Self::create_entity))
            .route("/:id", patch(Self::update_entity))
            .route("/:id", delete(Self::delete_entity))
    }

    /// Asynchronously creates additional custom routes for the entity.
    ///
    /// This method can be overridden in implementations to provide additional or custom routes.
    ///
    /// # Returns
    /// A `Router` configured with additional custom routes for the entity.
    /// The default implementation returns an empty router.
    async fn more_routes() -> Router {
        Router::new()
    }

    /// Asynchronously creates a combined router with both default and custom routes.
    ///
    /// This method merges the default routes provided by `default_routes` with any custom routes
    /// defined in `more_routes`. It nests these routes under a base path defined by `path()`.
    ///
    /// # Returns
    /// A `Router` that combines default and custom routes under a common base path.
    async fn router() -> Router {
        let default_routes = Self::default_routes().await;
        let custom_routes = Self::more_routes().await;

        Router::new().nest(&format!("/{}", &Self::path()), default_routes.merge(custom_routes))
    }
}
