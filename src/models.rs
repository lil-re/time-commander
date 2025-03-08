use serde::{Deserialize, Serialize};

/// Record
/// Represents a Top Level Domain (.com, .net, .org...)
#[derive(Serialize, Deserialize, Debug)]
pub struct Record {
  pub(crate) id: Option<i32>,
  pub(crate) created_at: u64,
  pub(crate) duration: u64
}

impl Record {
  pub fn new(created_at: u64, duration: u64) -> Self {
    Record {
      id: None,
      created_at,
      duration
    }
  }
}


/// Extension
/// Represents a Top Level Domain (.com, .net, .org...)
#[derive(Serialize, Deserialize, Debug)]
pub struct History {
  pub(crate) total_pauses: i32,
  pub(crate) total_duration: i32,
  pub(crate) record_date: String,
  pub(crate) start_time: String,
  pub(crate) end_time: String
}

