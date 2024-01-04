use mysql::*;
use mysql::prelude::*;

use crate::{
    models::{
        schedule::{
            shift_pickup::{
                ShiftPickup,
                RequestCreateShiftPickup,
                create_shift_pickups_table_query,
            },
            RequestUpdateShiftRequest,
        },
        result::Result,
    },
    prototypes::{ basic_queries::BasicQueries, create_table::DatabaseTable },
};

pub struct ShiftPickupQueries;

impl DatabaseTable for ShiftPickupQueries {
    fn create_table(&self, conn: &mut PooledConn) -> Result<()> {
        let query = create_shift_pickups_table_query();
        conn.query_drop(query)?;
        Ok(())
    }
}

impl BasicQueries for ShiftPickupQueries {
    type Model = ShiftPickup;
    type CreateDto = RequestCreateShiftPickup;
    type UpdateDto = RequestUpdateShiftRequest;

    fn table_name() -> String {
        "shift_pickups".to_string()
    }

    fn insert_statement() -> String {
        format!(
            r"INSERT INTO {} (id, openshift_id, user_id) VALUES (:id, :openshift_id, :user_id)",
            Self::table_name()
        )
    }

    fn insert_params(create_dto: &Self::CreateDto) -> Result<Params> {
        Ok(
            params! {
                "openshift_id" => create_dto.openshift_id,
                "user_id" => create_dto.user_id,
            }
        )
    }

    fn update_entity(conn: &mut PooledConn, id: i64, update_dto: Self::UpdateDto) -> Result<u64> {
        let mut query = "UPDATE shift_pickups SET ".to_string();
        let mut params: Vec<(String, Value)> = Vec::new();

        if let Some(status) = update_dto.status {
            query.push_str("status = :status, ");
            params.push(("status".to_string(), status.to_string().into()));
        }
        if let Some(admin_id) = update_dto.admin_id {
            query.push_str("admin_id = :admin_id, ");
            params.push(("admin_id".to_string(), admin_id.into()));
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
        params.push(("id".to_string(), id.into()));

        let params = Params::from(params);
        let query_result = conn.exec_iter(&query, params)?;
        Ok(query_result.affected_rows())
    }
}
