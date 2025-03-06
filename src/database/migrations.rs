use std::sync::MutexGuard;
use rusqlite::{Connection, Result};
use crate::database::connection::DB_CONNECTION;

pub fn run_migrations() -> Result<()> {
    let conn = DB_CONNECTION.lock().expect("Failed to lock the database connection");

    create_record_table(&conn)?;

    Ok(())
}

/// Create the `record` table
fn create_record_table(conn: &MutexGuard<Connection>) -> Result<()> {
    let record_result = conn.execute("
        CREATE TABLE IF NOT EXISTS record (
            id INTEGER PRIMARY KEY,
            created_at DATE NOT NULL,
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
