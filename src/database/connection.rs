use std::env;
use std::path::PathBuf;
use std::sync::Mutex;
use rusqlite::{Connection, Result};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref DB_CONNECTION: Mutex<Connection> = Mutex::new(
        // match Connection::open(get_database_path()) {
        //     Ok(connection) => connection,
        //     Err(_) => panic!("Failed to establish connection with database")
        // }
        match Connection::open_in_memory() {
            Ok(connection) => connection,
            Err(_) => panic!("Failed to establish connection with database")
        }
    );
}

pub fn establish_connection() -> Result<()> {
    let _conn = DB_CONNECTION.lock().expect("Failed to lock the database connection");
    // Additional initialization code for the connection can go here if needed.
    Ok(())
}

fn get_database_path() -> PathBuf {
    let exe_path = env::current_exe().expect("Failed to get executable path");
    let exe_dir = exe_path.parent().expect("Failed to get parent directory").to_path_buf();
    exe_dir.join("time_commander_db.db")
}
