use serde::{Deserialize, Serialize};

/// Represents a single record entry.
#[derive(Serialize, Deserialize, Debug)]
pub struct Record {
  pub(crate) id: Option<i32>,   // Optional unique identifier for the record
  pub(crate) duration: u64,     // Duration of the record in seconds
  pub(crate) created_at: String // Timestamp when the record was created
}

impl Record {
  /// Constructs a new `Record` with a given duration and creation timestamp.
  ///
  /// # Parameters:
  /// - `duration`: The duration associated with the record.
  /// - `created_at`: The timestamp when the record was created.
  ///
  /// # Returns:
  /// A new `Record` instance with `id` set to `None`.
  pub fn new(duration: u64, created_at: String) -> Self {
    Record {
      id: None,
      duration,
      created_at
    }
  }
}

/// Represents a group of `Record`s associated with a specific date.
#[derive(Serialize, Deserialize, Debug)]
pub struct History {
  pub(crate) total_pauses: i32,   // Total number of pauses for one date
  pub(crate) total_duration: u64, // Total accumulated duration for one date
  pub(crate) record_date: String, // The date to which this history belongs (in "YYYY-MM-DD" format)
  pub(crate) start_time: String,  // The time when the first record was created (in hh:mm:ss format)
  pub(crate) end_time: String     // The time when the last record was stopped (in hh:mm:ss format)
}
