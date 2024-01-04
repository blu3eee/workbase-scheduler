use std::sync::Arc;

use mysql::*;
use mysql::prelude::*;

use crate::{
    models::{
        schedule::availability::{
            AvailabilityRequest,
            RequestCreateAvailability,
            RequestUpdateAvailability,
            create_availability_requests_table_query,
        },
        result::Result,
    },
    prototypes::{ basic_queries::BasicQueries, create_table::DatabaseTable },
    snowflake::SnowflakeGenerator,
};

use super::availability_detail::AvailabilityDetailQueries;

pub struct AvailabilityRequestQueries;

impl AvailabilityRequestQueries {
    /// Fetches the latest approved availability request for a specific user in an organization.
    pub fn get_current_availability(
        conn: &mut PooledConn,
        user_id: i64,
        org_id: i64
    ) -> Result<Option<AvailabilityRequest>> {
        let query =
            "SELECT * FROM availability_requests 
            WHERE user_id = :user_id 
                AND org_id = :org_id
                AND status = 'APPROVED' 
                AND start_date <= CURDATE()
            ORDER BY start_date DESC 
            LIMIT 1;";

        Ok(
            conn.exec_first(
                query,
                params! {
                "user_id" => user_id,
                "org_id" => org_id,
            }
            )?
        )
    }

    /// Retrieves all availability requests made by a specific user.
    pub fn get_all_availability_requests(
        conn: &mut PooledConn,
        user_id: i64
    ) -> Result<Vec<AvailabilityRequest>> {
        let query = format!(
            "SELECT * FROM availability_requests
            WHERE user_id = {user_id}
            ORDER BY start_date DESC;"
        );
        Ok(conn.query(query)?)
    }
}

impl DatabaseTable for AvailabilityRequestQueries {
    fn create_table(&self, conn: &mut PooledConn) -> Result<()> {
        let query = create_availability_requests_table_query();
        conn.query_drop(query)?;
        Ok(())
    }
}

impl BasicQueries for AvailabilityRequestQueries {
    type Model = AvailabilityRequest;
    type CreateDto = RequestCreateAvailability;
    type UpdateDto = RequestUpdateAvailability;

    fn table_name() -> String {
        "availability_requests".to_string()
    }

    fn insert_statement() -> String {
        format!(
            r"INSERT INTO {} (id, user_id, org_id, start_date) VALUES (:id, :user_id, :org_id, :start_date);",
            Self::table_name()
        )
    }

    fn insert_params(create_dto: &Self::CreateDto) -> Result<Params> {
        Ok(
            params! {
                "user_id" => create_dto.user_id,
                "org_id" => create_dto.org_id,
                "start_date" => create_dto.start_date.to_string(),
            }
        )
    }

    fn create_entity_postprocessor(
        conn: &mut PooledConn,
        _snowflake_generator: Arc<SnowflakeGenerator>,
        create_dto: Self::CreateDto,
        id: i64
    ) -> Result<i64> {
        let request_id = id;
        // create availability's details
        AvailabilityDetailQueries::create_entities(conn, request_id, create_dto.details)?;
        // return the AvailabilityRequest's id
        Ok(request_id)
    }

    fn update_entity(conn: &mut PooledConn, id: i64, update_dto: Self::UpdateDto) -> Result<u64> {
        let mut query = "UPDATE availability_requests SET ".to_string();
        let mut params: Vec<(String, Value)> = Vec::new();

        if let Some(status) = update_dto.status {
            query.push_str("status = :status, ");
            params.push(("status".to_string(), status.to_string().into()));
        } else {
            return Ok(0);
        }

        // Remove last comma and space if there are updates
        if !params.is_empty() {
            query.pop();
            query.pop();
        } else {
            return Ok(0);
        }

        query.push_str(&format!(" WHERE id = :id;"));
        params.push(("id".to_string(), id.into()));

        let params = Params::from(params);
        let query_result = conn.exec_iter(&query, params)?;

        Ok(query_result.affected_rows())
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Add;

    use super::*;
    use crate::{
        tests::{ initialize_test_db, cleanup_test_db },
        queries::{ user::UserQueries, organization::OrgQueries },
        models::{
            organization::RequestCreateOrganization,
            user::RequestCreateUser,
            schedule::{
                request_status::ScheduleRequestStatus,
                availability_detail::{ RequestCreateAvailabilityDetail, DayOfWeek },
            },
        },
    };
    use chrono::{ NaiveDate, Utc, Duration };

    #[test]
    fn test_availability_requests() -> Result<()> {
        // Setup database connection
        let pool = initialize_test_db()?;
        let mut conn = pool.get_conn()?;
        let snowflake_generator = Arc::new(SnowflakeGenerator::new(1));

        // Create a user and an organization for testing
        // Assume these functions return valid IDs
        let user_id = create_test_user(&mut conn, snowflake_generator.clone())?;
        let org_id = create_test_organization(&mut conn, snowflake_generator.clone(), user_id)?;

        let today = Utc::now().date_naive();
        let one_week_later = today.add(Duration::weeks(1));

        // Test creating an availability request
        let create_request = RequestCreateAvailability {
            user_id,
            org_id,
            start_date: one_week_later,
            details: vec![
                RequestCreateAvailabilityDetail {
                    day_of_week: DayOfWeek::MONDAY,
                    is_available: true,
                    whole_day: true,
                    preferred_start_time: None,
                    preferred_end_time: None,
                },
                RequestCreateAvailabilityDetail {
                    day_of_week: DayOfWeek::TUESDAY,
                    is_available: false,
                    whole_day: true,
                    preferred_start_time: None,
                    preferred_end_time: None,
                },
                RequestCreateAvailabilityDetail {
                    day_of_week: DayOfWeek::WEDNESDAY,
                    is_available: true,
                    whole_day: true,
                    preferred_start_time: None,
                    preferred_end_time: None,
                }
            ],
        };

        let request_id = AvailabilityRequestQueries::create_entity(
            &mut conn,
            snowflake_generator.clone(),
            create_request
        )?;
        assert!(request_id > 0);

        // Test updating the availability request
        let update_request = RequestUpdateAvailability {
            status: Some(ScheduleRequestStatus::APPROVED),
        };
        let affected_rows = AvailabilityRequestQueries::update_entity(
            &mut conn,
            request_id,
            update_request
        )?;
        assert_eq!(affected_rows, 1);

        // Once the requested availability is approved, that availability won't be the current availability as the start_day is one week later,
        // so the current availability will be None
        // Test fetching the current availability
        let current_availability = AvailabilityRequestQueries::get_current_availability(
            &mut conn,
            user_id,
            org_id
        )?;
        assert!(current_availability.is_none());

        // This availability request has the start day to be today, so when it is approved and we fetch the current availability, we will get
        // this availability returned
        // Test create another availability request
        let create_new_request = RequestCreateAvailability {
            user_id,
            org_id,
            start_date: today,
            details: vec![
                RequestCreateAvailabilityDetail {
                    day_of_week: DayOfWeek::MONDAY,
                    is_available: true,
                    whole_day: true,
                    preferred_start_time: None,
                    preferred_end_time: None,
                },
                RequestCreateAvailabilityDetail {
                    day_of_week: DayOfWeek::WEDNESDAY,
                    is_available: true,
                    whole_day: true,
                    preferred_start_time: None,
                    preferred_end_time: None,
                }
            ],
        };

        let new_request_id = AvailabilityRequestQueries::create_entity(
            &mut conn,
            snowflake_generator.clone(),
            create_new_request
        )?;
        assert!(new_request_id > 0);

        // Test updating the availability request
        let affected_rows = AvailabilityRequestQueries::update_entity(
            &mut conn,
            new_request_id,
            RequestUpdateAvailability {
                status: Some(ScheduleRequestStatus::APPROVED),
            }
        )?;
        assert_eq!(affected_rows, 1);

        // Test fetching the current availability when there is two availability
        let current_availability = AvailabilityRequestQueries::get_current_availability(
            &mut conn,
            user_id,
            org_id
        )?;
        // we know this Option wrapper contains a value because the (known) requested availability has the start date to be the current day of the test (`today`)
        assert!(current_availability.is_some());
        // unwrap and testing
        let current_availability = current_availability.unwrap();
        assert_eq!(current_availability.id, new_request_id);
        assert_eq!(current_availability.status, ScheduleRequestStatus::APPROVED);

        // Test fetching all availability requests
        let all_requests = AvailabilityRequestQueries::get_all_availability_requests(
            &mut conn,
            user_id
        )?;
        assert!(!all_requests.is_empty());

        // Test deleting the availability request
        let deleted_rows = AvailabilityRequestQueries::delete_entity(&mut conn, request_id)?;
        assert_eq!(deleted_rows, 1);
        // Test if the related AvailabilityDetail rows also releted in cascade effect
        let details = AvailabilityDetailQueries::get_request_details(&mut conn, request_id)?;
        assert!(details.is_empty());

        // Clean up: Drop the test database
        cleanup_test_db(conn)?;

        Ok(())
    }

    fn create_test_user(
        conn: &mut PooledConn,
        snowflake_generator: Arc<SnowflakeGenerator>
    ) -> Result<i64> {
        Ok(
            UserQueries::create_entity(conn, snowflake_generator.clone(), RequestCreateUser {
                email: "testuser@example.com".to_string(),
                password: "password".to_string(),
                first_name: "Test".to_string(),
                last_name: "User".to_string(),
                date_of_birth: NaiveDate::from_ymd_opt(2000, 1, 1).unwrap(),
                phone_number: Some("1234567890".to_string()),
            })?
        )
    }

    fn create_test_organization(
        conn: &mut PooledConn,
        snowflake_generator: Arc<SnowflakeGenerator>,
        user_id: i64
    ) -> Result<i64> {
        Ok(
            OrgQueries::create_entity(conn, snowflake_generator.clone(), RequestCreateOrganization {
                name: "Test Organization".to_string(),
                description: Some("A test organization".to_string()),
                owner_id: user_id, // Assume the user is the owner
                timezone: None,
                icon: None,
            })?
        )
    }
}
