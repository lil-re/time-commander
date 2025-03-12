use serde::{Deserialize, Serialize};

/// Record
#[derive(Serialize, Deserialize, Debug)]
pub struct Record {
  pub(crate) id: Option<i32>,
  pub(crate) created_at: String,
  pub(crate) duration: u64
}

impl Record {
  pub fn new(created_at: String, duration: u64) -> Self {
    Record {
      id: None,
      created_at,
      duration
    }
  }
}


/// History
/// Represents a group of Records for one date
#[derive(Serialize, Deserialize, Debug)]
pub struct History {
  pub(crate) total_pauses: i32,
  pub(crate) total_duration: u64,
  pub(crate) record_date: String,
  pub(crate) start_time: String,
  pub(crate) end_time: String
}

