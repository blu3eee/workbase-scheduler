use crate::{
    models::{
        schedule::availability_detail::{
            AvailabilityDetail,
            RequestCreateAvailabilityDetail,
            create_availability_details_table_query,
        },
        result::Result,
    },
    prototypes::create_table::DatabaseTable,
};
use mysql::*;
use mysql::prelude::*;

pub struct AvailabilityDetailQueries;

impl DatabaseTable for AvailabilityDetailQueries {
    fn create_table(&self, conn: &mut PooledConn) -> Result<()> {
        let query = create_availability_details_table_query();
        conn.query_drop(query)?;
        Ok(())
    }
}

impl AvailabilityDetailQueries {
    fn table_name() -> String {
        "availability_details".to_string()
    }

    fn insert_statement() -> String {
        format!(
            "INSERT INTO {} (request_id, day_of_week, is_available, whole_day, preferred_start_time, preferred_end_time) VALUES (:request_id, :day_of_week, :is_available, :whole_day, :preferred_start_time, :preferred_end_time)",
            Self::table_name()
        )
    }

    fn insert_params(
        request_id: i64,
        create_dto: &RequestCreateAvailabilityDetail
    ) -> Result<Params> {
        Ok(
            params! {
                "request_id" => request_id,
                "day_of_week" => create_dto.day_of_week.to_string(),
                "is_available" => create_dto.is_available,
                "whole_day" => create_dto.whole_day,
                "preferred_start_time" => create_dto.preferred_start_time.map(|t| t.to_string()),
                "preferred_end_time" => create_dto.preferred_end_time.map(|t| t.to_string()),
            }
        )
    }

    pub fn create_entity(
        conn: &mut PooledConn,
        request_id: i64,
        create_dto: RequestCreateAvailabilityDetail
    ) -> Result<()> {
        conn.exec_drop(Self::insert_statement(), Self::insert_params(request_id, &create_dto)?)?;
        Ok(())
    }

    pub fn create_entities(
        conn: &mut PooledConn,
        request_id: i64,
        create_dtos: Vec<RequestCreateAvailabilityDetail>
    ) -> Result<()> {
        let params_iter = create_dtos
            .iter()
            .map(|create_dto| Ok(Self::insert_params(request_id, create_dto)?))
            .collect::<Result<Vec<_>>>()?; // Collect into Result<Vec<Params>, _>

        conn.exec_batch(
            Self::insert_statement(),
            params_iter.into_iter() // Now params_iter is Iterator<Item = Params>
        )?;
        Ok(())
    }

    pub fn get_request_details(
        conn: &mut PooledConn,
        request_id: i64
    ) -> Result<Vec<AvailabilityDetail>> {
        Ok(
            conn.query(
                format!("SELECT * FROM {} WHERE request_id = {};", Self::table_name(), request_id)
            )?
        )
    }
}
