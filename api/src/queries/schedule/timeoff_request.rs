use mysql::{ *, prelude::Queryable };

use crate::{
    prototypes::{ create_table::DatabaseTable, basic_queries::BasicQueries },
    models::{
        schedule::timeoff_request::{
            create_time_off_requests_table_query,
            TimeOffRequest,
            RequestCreateTimeOff,
            RequestUpdateTimeOff,
        },
        result::Result,
    },
};

pub struct TimeOffRequestQueries;
impl DatabaseTable for TimeOffRequestQueries {
    fn create_table(&self, conn: &mut PooledConn) -> Result<()> {
        let query = create_time_off_requests_table_query();
        conn.query_drop(query)?;
        Ok(())
    }
}

impl BasicQueries for TimeOffRequestQueries {
    type Model = TimeOffRequest;
    type CreateDto = RequestCreateTimeOff;
    type UpdateDto = RequestUpdateTimeOff;

    fn table_name() -> String {
        "time_off_requests".to_string()
    }

    fn insert_statement() -> String {
        format!(
            "INSERT INTO {} (id, user_id, company_id, start_time, end_time, reason) VALUES (:id, :user_id, :company_id, :start_time, :end_time, :reason);",
            Self::table_name()
        )
    }

    fn insert_params(create_dto: &Self::CreateDto) -> Result<Params> {
        Ok(
            params! {
                "user_id" => create_dto.user_id,
                "company_id" => create_dto.company_id,
                "start_time" => create_dto.start_time.to_string(),
                "end_time" => create_dto.end_time.to_string(),
                "reason" => &create_dto.reason,
            }
        )
    }

    fn update_entity(
        conn: &mut mysql::PooledConn,
        id: i64,
        update_dto: Self::UpdateDto
    ) -> crate::models::result::Result<u64> {
        let mut query = format!("UPDATE {} SET ", Self::table_name());
        let mut params: Vec<(String, Value)> = Vec::new();

        if let Some(status) = update_dto.status {
            query.push_str("status = :status, ");
            params.push(("status".to_string(), status.to_string().into()));
        }
        if let Some(admin_id) = update_dto.admin_id {
            query.push_str("admin_id = :admin_id, ");
            params.push(("admin_id".to_string(), admin_id.into()));
        }
        if let Some(reason) = update_dto.reason {
            query.push_str("reason = :reason, ");
            params.push(("reason".to_string(), reason.into()));
        }

        // Remove last comma and space if there are updates
        if !params.is_empty() {
            query.pop();
            query.pop();
        } else {
            return Ok(0);
        }

        query.push_str(" WHERE id = :id;");
        params.push(("id".to_string(), id.into()));

        let params = Params::from(params);
        let query_result = conn.exec_iter(&query, params)?;
        Ok(query_result.affected_rows())
    }
}
