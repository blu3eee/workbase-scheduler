use mysql::*;
use mysql::prelude::*;

use crate::{
    models::{
        schedule::work_schedule::{
            WorkSchedule,
            RequestCreateWorkSchedule,
            RequestUpdateWorkSchedule,
            create_work_schedules_table_query,
        },
        result::Result,
    },
    prototypes::{ basic_queries::BasicQueries, create_table::DatabaseTable },
};

pub struct WorkScheduleQueries {}

impl WorkScheduleQueries {
    pub fn get_org_schedules(conn: &mut PooledConn, org_id: i64) -> Result<Vec<WorkSchedule>> {
        Ok(conn.query(format!("SELECT * FROM work_schedules WHERE org_id = {};", org_id))?)
    }
}

impl DatabaseTable for WorkScheduleQueries {
    fn create_table(&self, conn: &mut PooledConn) -> Result<()> {
        let query = create_work_schedules_table_query();
        conn.query_drop(query)?;
        Ok(())
    }
}

impl BasicQueries for WorkScheduleQueries {
    type Model = WorkSchedule;
    type CreateDto = RequestCreateWorkSchedule;
    type UpdateDto = RequestUpdateWorkSchedule;

    fn table_name() -> String {
        "work_schedules".to_string()
    }

    fn insert_statement() -> String {
        format!(
            "INSERT INTO {} (id, org_id, start_date, end_date) VALUES (:id, :org_id, :start_date, :end_date)",
            Self::table_name()
        )
    }

    fn insert_params(create_dto: &Self::CreateDto) -> Result<Params> {
        Ok(
            params! {
                "org_id" => create_dto.org_id,
                "start_date" => create_dto.start_date.to_string(),
                "end_date" => create_dto.end_date.to_string(),
            }
        )
    }

    fn update_entity(conn: &mut PooledConn, update_dto: Self::UpdateDto) -> Result<u64> {
        let mut query = "UPDATE work_schedules SET ".to_string();
        let mut params: Vec<(String, Value)> = Vec::new();

        if let Some(start_date) = update_dto.start_date {
            query.push_str("start_date = :start_date, ");
            params.push(("start_date".to_string(), start_date.to_string().into()));
        }
        if let Some(end_date) = update_dto.end_date {
            query.push_str("end_date = :end_date, ");
            params.push(("end_date".to_string(), end_date.to_string().into()));
        }
        if let Some(publish) = update_dto.publish {
            query.push_str("published = :publish, ");
            params.push(("publish".to_string(), publish.into()));
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
