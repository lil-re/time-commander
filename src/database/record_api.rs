use crate::database::connection::DB_CONNECTION;
use crate::models::{History, Record};

/// Adds a new `Record` to the database.
///
/// # Arguments
/// - `record`: The `Record` to be added, containing `created_at` and `duration`.
///
/// # Returns
/// - `Some(())` if the record was successfully added.
/// - `None` if an error occurred during insertion.
pub fn create_record(record: &Record) -> Option<()> {
    let conn = DB_CONNECTION.lock().expect("Failed to lock the database connection");

    let response = conn.execute(
        "INSERT INTO record (created_at, duration) VALUES (?1, ?2)",
        (&record.created_at, &record.duration),
    )
        .map_err(|e| format!("Failed to add record: {}", e));

    match response {
        Ok(_) => Some(()),
        Err(error) => {
            println!("RECORD API => {}", error);
            None
        }
    }
}

/// Removes a `Record` from the database.
///
/// # Arguments
/// - `record`: The `Record` to be removed, identified by `id`.
///
/// # Returns
/// - `Some(())` if the record was successfully removed.
/// - `None` if an error occurred during the deletion.
pub fn remove_record(record: &Record) -> Option<()> {
    let conn = DB_CONNECTION.lock().expect("Failed to lock the database connection");
    let id = Some(&record.id);

    let response = conn.execute(
        "DELETE FROM record WHERE id = ?1",
        (id,),
    )
        .map_err(|e| format!("Failed to update record: {}", e));

    match response {
        Ok(_) => Some(()),
        Err(error) => {
            println!("RECORD API => {}", error);
            None
        }
    }
}


/// Retrieves all records from the database and aggregates them into `History` objects.
///
/// This function groups records by date and calculates the total duration, start time,
/// and end time for each group. The result is a list of `History` structs.
///
/// # Returns
/// A `Vec<History>` containing aggregated data for each record date, or an empty vector if an error occurs.
pub fn find_all_records() -> Vec<History> {
    let conn = DB_CONNECTION.lock().expect("Failed to lock the database connection");

    let mut stmt = match conn.prepare("\
        SELECT count(record.id) - 1 AS total_pauses,
            sum(record.duration) AS total_duration,
            DATE(record.created_at) AS record_date,
            TIME(MIN(record.created_at)) AS start_time,
            TIME(datetime(MAX(record.created_at), '+' || record.duration || ' seconds')) AS end_time
        FROM record
        GROUP BY record_date;\
    ") {
        Ok(result) => result,
        Err(error) => panic!("{}", error)
    };

    let domains_iter = stmt.query_map([], |row| {
        Ok(History {
            total_pauses: row.get(0)?,
            total_duration: row.get(1)?,
            record_date: row.get(2)?,
            start_time: row.get(3)?,
            end_time: row.get(4)?,
        })
    });

    let domains_result = match domains_iter {
        Ok(result) => result.collect::<Result<Vec<History>, rusqlite::Error>>(),
        Err(_) => Ok(vec![])
    };

    match domains_result {
        Ok(result) => result,
        Err(_) => vec![]
    }
}
