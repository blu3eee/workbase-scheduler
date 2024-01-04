use mysql::*;
use mysql::prelude::*;

use crate::{
    models::{
        schedule::shift::{
            Shift,
            RequestCreateShift,
            RequestUpdateShift,
            create_shifts_table_query,
        },
        result::Result,
    },
    prototypes::{ basic_queries::BasicQueries, create_table::DatabaseTable },
};

pub struct ShiftQueries {}

impl DatabaseTable for ShiftQueries {
    fn create_table(&self, conn: &mut PooledConn) -> Result<()> {
        let query = create_shifts_table_query();
        conn.query_drop(query)?;
        Ok(())
    }
}

impl BasicQueries for ShiftQueries {
    type Model = Shift;
    type CreateDto = RequestCreateShift;
    type UpdateDto = RequestUpdateShift;

    fn table_name() -> String {
        "shifts".to_string()
    }

    fn insert_statement() -> String {
        format!(
            "INSERT INTO {} (id, user_id, schedule_id, job_id, start_time, end_time, pay_rate, note)
              VALUES (:id, :user_id, :schedule_id, :job_id, :start_time, :end_time, :pay_rate, :note)",
            Self::table_name()
        )
    }

    fn insert_params(create_dto: &Self::CreateDto) -> Result<Params> {
        Ok(
            params! {
                "user_id" => create_dto.user_id,
                "schedule_id" => create_dto.schedule_id,
                "job_id" => create_dto.job_id,
                "start_time" => create_dto.start_time.format("%Y-%m-%d %H:%M:%S").to_string(),
                "end_time" => create_dto.end_time.format("%Y-%m-%d %H:%M:%S").to_string(),
                "pay_rate" => create_dto.pay_rate,
                "note" => &create_dto.note,
            }
        )
    }

    fn update_entity(conn: &mut PooledConn, update_dto: Self::UpdateDto) -> Result<u64> {
        let mut query = "UPDATE shifts SET ".to_string();
        let mut params: Vec<(String, Value)> = Vec::new();

        if let Some(job_id) = update_dto.job_id {
            query.push_str("job_id = :job_id, ");
            params.push(("job_id".to_string(), job_id.into()));
        }
        if let Some(start_time) = update_dto.start_time {
            query.push_str("start_time = :start_time, ");
            params.push((
                "start_time".to_string(),
                start_time.format("%Y-%m-%d %H:%M:%S").to_string().into(),
            ));
        }
        if let Some(end_time) = update_dto.end_time {
            query.push_str("end_time = :end_time, ");
            params.push((
                "end_time".to_string(),
                end_time.format("%Y-%m-%d %H:%M:%S").to_string().into(),
            ));
        }
        if let Some(pay_rate) = update_dto.pay_rate {
            query.push_str("pay_rate = :pay_rate, ");
            params.push(("pay_rate".to_string(), pay_rate.into()));
        }
        if let Some(note) = update_dto.note {
            query.push_str("note = :note, ");
            params.push(("note".to_string(), note.into()));
        }

        // Remove last comma and space if there are updates
        if !params.is_empty() {
            query.pop();
            query.pop();
        } else {
            return Ok(0);
        }

        query.push_str(&format!(" WHERE id = :id;"));
        params.push(("id".to_string(), update_dto.id.into()));

        let params = Params::from(params);
        let query_result = conn.exec_iter(&query, params)?;

        Ok(query_result.affected_rows())
    }
}
