pub mod user;
pub mod organization;
pub mod org_member;
pub mod org_job;
pub mod schedule;
use mysql::PooledConn;

use crate::{ prototypes::create_table::DatabaseTable, models::result::Result };

use self::{
    user::UserQueries,
    organization::OrgQueries,
    org_job::OrgJobQueries,
    org_member::OrgMemberQueries,
    schedule::{
        availability::AvailabilityRequestQueries,
        availability_detail::AvailabilityDetailQueries,
        work_schedule::WorkScheduleQueries,
        shift::ShiftQueries,
        open_shift::OpenShiftQueries,
        shift_cover::ShiftCoverQueries,
        shift_trade::ShiftTradeQueries,
        shift_pickup::ShiftPickupQueries,
    },
};

/// create tables
pub fn create_tables(conn: &mut PooledConn) -> Result<()> {
    let table_queries = vec![
        Box::new(UserQueries {}) as Box<dyn DatabaseTable>,
        Box::new(OrgQueries {}) as Box<dyn DatabaseTable>,
        Box::new(OrgJobQueries {}) as Box<dyn DatabaseTable>,
        Box::new(OrgMemberQueries {}) as Box<dyn DatabaseTable>,
        Box::new(AvailabilityRequestQueries {}) as Box<dyn DatabaseTable>,
        Box::new(AvailabilityDetailQueries {}) as Box<dyn DatabaseTable>,
        Box::new(WorkScheduleQueries {}) as Box<dyn DatabaseTable>,
        Box::new(ShiftQueries {}) as Box<dyn DatabaseTable>,
        Box::new(OpenShiftQueries {}) as Box<dyn DatabaseTable>,
        Box::new(ShiftCoverQueries {}) as Box<dyn DatabaseTable>,
        Box::new(ShiftTradeQueries {}) as Box<dyn DatabaseTable>,
        Box::new(ShiftPickupQueries {}) as Box<dyn DatabaseTable>
    ];

    for table_query in table_queries {
        table_query.create_table(conn)?;
    }

    Ok(())
}
