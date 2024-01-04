use mysql::PooledConn;

use crate::models::result::Result;

pub trait DatabaseTable {
    /// Creates the table in the database.
    ///
    /// This method should handle the creation of the database table if it does not already exist.
    fn create_table(&self, conn: &mut PooledConn) -> Result<()>;
}
