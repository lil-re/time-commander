use crate::database::connection::DB_CONNECTION;
use crate::models::{History, Record};

/// Add a record
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

/// Remove a record
pub fn remove_record(record: &Record) -> Option<()> {
    let conn = DB_CONNECTION.lock().expect("Failed to lock the database connection");

    let response = conn.execute(
        "DELETE FROM record WHERE id = ?1",
        ((&record.id,)),
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

/// Get all records
pub fn find_all_records() -> Vec<History> {
    let conn = DB_CONNECTION.lock().expect("Failed to lock the database connection");

    let mut stmt = match conn.prepare("\
        SELECT count(id) -1 as total_pauses,\
            sum(duration) as total_duration,\
            date(created_at) as record_date,\
            time(MIN(created_at)) as start_time,\
            time(DATE_ADD(MAX(created_at), INTERVAL duration SECOND)) AS end_time\
        FROM records\
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
        Ok(result) => result.collect::<Result<Vec<Record>, rusqlite::Error>>(),
        Err(_) => Ok(vec![])
    };

    match domains_result {
        Ok(result) => result,
        Err(_) => vec![]
    }
}
