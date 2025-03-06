use crate::database::connection::DB_CONNECTION;
use crate::models::Record;

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
            println!("WISHLIST API => {}", error);
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
            println!("WISHLIST API => {}", error);
            None
        }
    }
}

/// Get all records
pub fn find_all_records() -> Vec<Record> {
    let conn = DB_CONNECTION.lock().expect("Failed to lock the database connection");

    let mut stmt = match conn.prepare("SELECT * FROM record") {
        Ok(result) => result,
        Err(error) => panic!("{}", error)
    };

    let domains_iter = stmt.query_map([], |row| {
        Ok(Record {
            id: row.get(1)?,
            created_at: row.get(2)?,
            duration: row.get(3)?
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
