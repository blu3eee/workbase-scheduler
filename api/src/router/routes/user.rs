use async_trait::async_trait;
use axum::{ Extension, Json, response::IntoResponse, Router, routing::post };
use hyper::StatusCode;

use crate::{
    queries::user::{ UserQueries, LoginForm, PasswordChangeForm },
    prototypes::uniqueid_routers::UniqueIdRouter,
    app::{ ApiResponse, AppState },
    utilities::app_error::AppError,
    models::ResponseDataJson,
};

pub struct UserRouter;

#[async_trait]
impl UniqueIdRouter for UserRouter {
    type Queries = UserQueries;

    fn path() -> String {
        "users".to_string()
    }

    async fn more_routes() -> Router {
        Router::new()
            .route("/check-password", post(Self::verify_password))
            .route("/change-password", post(Self::change_password))
    }
}

impl UserRouter {
    pub async fn verify_password(
        Extension(state): Extension<AppState>,
        Json(form): Json<LoginForm>
    ) -> ApiResponse {
        let mut conn = state.db_pool.get_conn()?;
        match UserQueries::verify_password(&mut conn, form) {
            Ok(is_valid) => {
                let json = Json(ResponseDataJson { data: is_valid });
                let mut response = json.into_response();
                *response.status_mut() = if is_valid {
                    StatusCode::OK
                } else {
                    StatusCode::UNAUTHORIZED
                };
                Ok(response)
            }
            Err(e) => Err(AppError::from(e)),
        }
    }

    pub async fn change_password(
        Extension(state): Extension<AppState>,
        Json(form): Json<PasswordChangeForm>
    ) -> ApiResponse {
        let mut conn = state.db_pool.get_conn()?;
        match UserQueries::change_password(&mut conn, form) {
            Ok(_) => {
                let json = Json(ResponseDataJson { data: "Password changed successfully" });
                let mut response = json.into_response();
                *response.status_mut() = StatusCode::OK;
                Ok(response)
            }
            Err(e) => {
                return Err(AppError::bad_request(e.to_string()));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;
    use axum::{ body::Body, http::{ Request, StatusCode }, Extension, Router };
    use chrono::NaiveDate;
    use serde_json::json;
    use tower::ServiceExt; // for `oneshot` method

    use crate::{
        router::{ tests::initialize_test_app_state, utils::extract_response_body },
        models::{ user::{ RequestCreateUser, User, RequestUpdateUser }, result::Result },
    };

    // Initialize test environment
    async fn initialize_test_router() -> Router {
        let state = initialize_test_app_state().await.expect("failed to initialize test app state");
        UserRouter::router().await.layer(Extension(state))
    }

    async fn get_user_by_id(router: Router, id: i64) -> User {
        let get_response = router
            .clone()
            .oneshot(
                Request::builder().uri(format!("/users/{}", id)).body(Body::empty()).unwrap()
            ).await
            .unwrap();
        assert_eq!(get_response.status(), StatusCode::OK);
        extract_response_body(get_response).await.expect("error extracting user from body")
    }

    #[tokio::test]
    async fn test_user_routes() -> Result<()> {
        let router = initialize_test_router().await;

        // Test Create User
        let create_user_dto = RequestCreateUser {
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
            first_name: "Test".to_string(),
            last_name: "User".to_string(),
            date_of_birth: NaiveDate::from_str("1990-01-01").unwrap(),
            phone_number: Some("1234567890".to_string()),
        };

        println!("{create_user_dto:?}");

        let create_response = router
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/users")
                    .header("Content-Type", "application/json")
                    .body(Body::from(json!(create_user_dto).to_string()))
                    .unwrap()
            ).await
            .unwrap();

        assert_eq!(create_response.status(), StatusCode::CREATED);
        let created_user_id: i64 =
            extract_response_body(create_response).await.expect("error extracting body");

        let created_user: User = get_user_by_id(router.clone(), created_user_id).await;
        assert_eq!(created_user.email, create_user_dto.email);
        assert_eq!(created_user.first_name, create_user_dto.first_name);
        assert_eq!(created_user.last_name, create_user_dto.last_name);
        assert_eq!(created_user.date_of_birth, create_user_dto.date_of_birth);
        assert_eq!(created_user.phone_number, create_user_dto.phone_number);

        // Test Get All Users
        let get_all_response = router
            .clone()
            .oneshot(Request::builder().uri("/users").body(Body::empty()).unwrap()).await
            .unwrap();
        assert_eq!(get_all_response.status(), StatusCode::OK);
        // Assert that the data length is 1
        let all_users: Vec<User> = extract_response_body(get_all_response).await.expect(
            "error extracting response body"
        );
        assert_eq!(all_users.len(), 1);

        // Test Update User
        let update_user_dto = RequestUpdateUser {
            phone_number: Some("9093334444".to_string()),
            ..Default::default()
        };

        let update_response = router
            .clone()
            .oneshot(
                Request::builder()
                    .method("PATCH")
                    .uri(format!("/users/{}", created_user_id))
                    .header("Content-Type", "application/json")
                    .body(Body::from(json!(update_user_dto).to_string()))
                    .unwrap()
            ).await
            .unwrap();
        assert_eq!(update_response.status(), StatusCode::OK);

        let user: User = get_user_by_id(router.clone(), created_user_id).await;
        assert_eq!(user.phone_number, update_user_dto.phone_number);

        let check_password_response = router
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/users/check-password")
                    .header("Content-Type", "application/json")
                    .body(
                        Body::from(
                            json!(LoginForm {
                                email: "test@example.com".to_string(),
                                password: "password123".to_string(),
                            }).to_string()
                        )
                    )
                    .unwrap()
            ).await
            .unwrap();
        assert_eq!(check_password_response.status(), StatusCode::OK);

        let check_password_response = router
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/users/check-password")
                    .header("Content-Type", "application/json")
                    .body(
                        Body::from(
                            json!(LoginForm {
                                email: "test@example.com".to_string(),
                                password: "newasdasd".to_string(),
                            }).to_string()
                        )
                    )
                    .unwrap()
            ).await
            .unwrap();
        assert_eq!(check_password_response.status(), StatusCode::UNAUTHORIZED);

        let change_password_response = router
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/users/change-password")
                    .header("Content-Type", "application/json")
                    .body(
                        Body::from(
                            json!(PasswordChangeForm {
                                id: created_user_id,
                                old_password: "password123".to_string(),
                                new_password: "newpassword123".to_string(),
                            }).to_string()
                        )
                    )
                    .unwrap()
            ).await
            .unwrap();
        assert_eq!(change_password_response.status(), StatusCode::OK);

        let check_password_response = router
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/users/check-password")
                    .header("Content-Type", "application/json")
                    .body(
                        Body::from(
                            json!(LoginForm {
                                email: "test@example.com".to_string(),
                                password: "newpassword123".to_string(),
                            }).to_string()
                        )
                    )
                    .unwrap()
            ).await
            .unwrap();

        assert_eq!(check_password_response.status(), StatusCode::OK);

        Ok(())
    }
}
