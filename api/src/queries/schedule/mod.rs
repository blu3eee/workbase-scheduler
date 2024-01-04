pub mod availability;
pub mod availability_detail;
pub mod timeoff_request;
pub mod work_schedule;
pub mod shift;
pub mod shift_cover;
pub mod shift_pickup;
pub mod open_shift;
pub mod shift_trade;

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use chrono::NaiveDate;

    use crate::{
        models::{
            result::Result,
            user::RequestCreateUser,
            organization::RequestCreateOrganization,
            schedule::{
                work_schedule::RequestCreateWorkSchedule,
                shift::{ RequestCreateShift, Shift },
                shift_trade::{ ShiftTrade, RequestCreateShiftTrade },
                ShiftRequestStatus,
                RequestUpdateShiftRequest,
                shift_cover::RequestCreateShiftCover,
            },
            org_job::RequestCreateOrgJob,
            org_member::RequestCreateOrgMember,
        },
        tests::{ initialize_test_db, cleanup_test_db },
        queries::{
            user::UserQueries,
            organization::OrgQueries,
            schedule::{
                work_schedule::WorkScheduleQueries,
                shift::ShiftQueries,
                shift_trade::ShiftTradeQueries,
                shift_cover::ShiftCoverQueries,
            },
            org_job::OrgJobQueries,
            org_member::OrgMemberQueries,
        },
        prototypes::basic_queries::BasicQueries,
        utilities::parse_chrono::parse_naive_date_time_from_str,
        snowflake::SnowflakeGenerator,
    };

    #[test]
    fn test_work_schedule_queries() -> Result<()> {
        // Setup database connection
        let pool = initialize_test_db()?;
        let mut conn = pool.get_conn()?;
        let snowflake_generator = Arc::new(SnowflakeGenerator::new(1));

        // Create a user
        let user = RequestCreateUser {
            email: "owner@example.com".to_string(),
            password: "password".to_string(),
            first_name: "Owner".to_string(),
            last_name: "User".to_string(),
            date_of_birth: NaiveDate::from_ymd_opt(1980, 1, 1).unwrap(),
            phone_number: Some("9094443333".to_string()),
        };

        let owner_user_id: i64 = UserQueries::create_entity(
            &mut conn,
            snowflake_generator.clone(),
            user
        )?;

        // Create an organization
        let org = RequestCreateOrganization {
            name: "Test Organization".to_string(),
            description: Some("A test organization".to_string()),
            owner_id: owner_user_id,
            timezone: None,
            icon: None,
        };

        let org_id: i64 = OrgQueries::create_entity(&mut conn, snowflake_generator.clone(), org)?;

        // Create a work schedule
        let schedule = RequestCreateWorkSchedule {
            org_id,
            start_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            end_date: NaiveDate::from_ymd_opt(2024, 1, 7).unwrap(),
        };
        let schedule_id = WorkScheduleQueries::create_entity(
            &mut conn,
            snowflake_generator.clone(),
            schedule
        )?;

        // Retrieve the created schedule
        let retrieved_schedule = WorkScheduleQueries::find_by_id(&mut conn, schedule_id)?;
        assert_eq!(retrieved_schedule.org_id, org_id);
        assert_eq!(retrieved_schedule.start_date, NaiveDate::from_ymd_opt(2024, 1, 1).unwrap());
        assert_eq!(retrieved_schedule.end_date, NaiveDate::from_ymd_opt(2024, 1, 7).unwrap());

        let job_id = OrgJobQueries::create_entity(
            &mut conn,
            snowflake_generator.clone(),
            RequestCreateOrgJob {
                org_id,
                name: "Cashier".to_string(),
                description: None,
                base_pay_rate: 50.0,
                color: None,
            }
        )?;

        let employee1_user_id = UserQueries::create_entity(
            &mut conn,
            snowflake_generator.clone(),
            RequestCreateUser {
                email: "john_doe@example.com".to_string(),
                password: "john_doe".to_string(),
                first_name: "John".to_string(),
                last_name: "Doe".to_string(),
                date_of_birth: NaiveDate::from_ymd_opt(2000, 11, 2).unwrap(),
                phone_number: Some("1234567890".to_string()),
            }
        )?;

        let _ = OrgMemberQueries::create_entity(&mut conn, RequestCreateOrgMember {
            org_id,
            user_id: employee1_user_id,
            job_id,
        })?;

        let employee2_user_id = UserQueries::create_entity(
            &mut conn,
            snowflake_generator.clone(),
            RequestCreateUser {
                email: "lwilliamson@example.com".to_string(),
                password: "lwilliamson".to_string(),
                first_name: "Lori".to_string(),
                last_name: "Williamson".to_string(),
                date_of_birth: NaiveDate::from_ymd_opt(2000, 11, 2).unwrap(),
                phone_number: Some("9095141197".to_string()),
            }
        )?;

        let _ = OrgMemberQueries::create_entity(&mut conn, RequestCreateOrgMember {
            org_id,
            user_id: employee2_user_id,
            job_id,
        })?;

        let employee3_user_id = UserQueries::create_entity(
            &mut conn,
            snowflake_generator.clone(),
            RequestCreateUser {
                email: "msosa@example.com".to_string(),
                password: "msosa111".to_string(),
                first_name: "Michele".to_string(),
                last_name: "Sosa".to_string(),
                date_of_birth: NaiveDate::from_ymd_opt(2000, 11, 2).unwrap(),
                phone_number: Some("5122755798".to_string()),
            }
        )?;

        let _ = OrgMemberQueries::create_entity(&mut conn, RequestCreateOrgMember {
            org_id,
            user_id: employee3_user_id,
            job_id,
        })?;

        // start create and send shift requests
        let shift_create_requests: Vec<RequestCreateShift> = vec![
            RequestCreateShift {
                user_id: employee1_user_id,
                schedule_id,
                job_id: job_id,
                start_time: parse_naive_date_time_from_str("2024-01-01 09:00:00")?,
                end_time: parse_naive_date_time_from_str("2024-01-01 16:00:00")?,
                pay_rate: Some(20.0),
                note: Some("Morning Shift".to_string()),
            },
            RequestCreateShift {
                user_id: employee2_user_id,
                schedule_id,
                job_id: job_id,
                start_time: parse_naive_date_time_from_str("2024-01-01 11:00:00")?,
                end_time: parse_naive_date_time_from_str("2024-01-01 17:00:00")?,
                pay_rate: Some(22.0),
                note: None,
            }
        ];

        // Create the shifts
        let _ = ShiftQueries::create_many(
            &mut conn,
            snowflake_generator.clone(),
            shift_create_requests
        )?;

        let all_shifts: Vec<Shift> = ShiftQueries::find_all(&mut conn)?;
        for shift in &all_shifts {
            println!("{shift:?}");
        }

        // test shift trade operations
        let shift_trade_request_id = ShiftTradeQueries::create_entity(
            &mut conn,
            snowflake_generator.clone(),
            RequestCreateShiftTrade {
                shift1_id: all_shifts[0].id,
                shift2_id: all_shifts[1].id,
            }
        )?;
        let shift_trade_request: ShiftTrade = ShiftTradeQueries::find_by_id(
            &mut conn,
            shift_trade_request_id
        )?;
        assert_eq!(shift_trade_request.status, ShiftRequestStatus::PENDING);

        let _ = ShiftTradeQueries::update_entity(
            &mut conn,
            shift_trade_request_id,
            RequestUpdateShiftRequest {
                status: Some(ShiftRequestStatus::DECLINED),
                admin_id: Some(owner_user_id),
                note: Some("test declining shift trade request".to_string()),
            }
        );

        let updated_shift_trade: ShiftTrade = ShiftTradeQueries::find_by_id(
            &mut conn,
            shift_trade_request_id
        )?;

        assert_eq!(updated_shift_trade.status, ShiftRequestStatus::DECLINED);
        assert_eq!(updated_shift_trade.admin_id, Some(owner_user_id));
        assert_eq!(
            updated_shift_trade.note,
            Some("test declining shift trade request".to_string())
        );

        // test shift cover operations
        let _shift_cover_request_id = ShiftCoverQueries::create_entity(
            &mut conn,
            snowflake_generator.clone(),
            RequestCreateShiftCover {
                shift_id: all_shifts[0].id,
                cover_user_id: employee3_user_id,
            }
        )?;

        // Clean up: Drop the database
        cleanup_test_db(conn)?;

        Ok(())
    }
}
