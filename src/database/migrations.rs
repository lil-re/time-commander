use std::sync::MutexGuard;
use rusqlite::{Connection, Result};
use crate::database::connection::DB_CONNECTION;

/// Runs all necessary migrations to set up the database schema.
///
/// # Returns
/// - `Ok(())` if all migrations are successful.
/// - `Err(e)` if any of the migrations fail.
pub fn run_migrations() -> Result<()> {
    let conn = DB_CONNECTION.lock().expect("Failed to lock the database connection");

    create_record_table(&conn)?;

    Ok(())
}

/// Creates the `record` table in the database if it does not already exist.
/// This table is used to store individual `Record` entries, including their creation time and duration.
///
/// # Arguments
/// - `conn`: The database connection used to execute the migration.
fn create_record_table(conn: &MutexGuard<Connection>) -> Result<()> {
    let record_result = conn.execute("
        CREATE TABLE IF NOT EXISTS record (
            id INTEGER PRIMARY KEY,
            created_at DATETIME NOT NULL,
            duration INTEGER NOT NULL
        )",
                                       [],
    );

    match record_result {
        Ok(_) => {
            println!("MIGRATION => Successfully created 'record' table.");
        }
        Err(error) => {
            println!("MIGRATION => Could not create 'record' table.");
            println!("MIGRATION => {}", error);
        }
    }

    Ok(())
}
