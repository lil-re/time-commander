use std::time::Instant;
use chrono::Local;
use ratatui::widgets::{ListState};
use crate::database::record_api::{create_record, find_all_records};
use crate::helpers::format_duration;
use crate::models::{History, Record};

#[derive(Default)]
pub struct AppState {
  pub start_time: Option<Instant>,
  pub start_date: Option<String>,
  pub timer_running: bool,
  pub timer_logs: Vec<String>,
  pub history: Vec<History>,
  pub logs_state: ListState,
}

impl AppState {
  /// Starts the timer and records the start time and date.
  /// It also logs a message to alert the user that the timer has been started.
  pub fn start_timer(&mut self) {
    if !self.timer_running {
      self.timer_running = true;
      self.start_time = Some(Instant::now());
      self.start_date = Some(Local::now().format("%Y-%m-%d %H:%M:%S").to_string());
      self.timer_logs.push("Timer started.".to_string());
    }
  }

  /// Stops the timer, records the elapsed time, and adds a new record to the database.
  /// It also updates the history to reflect the new record.
  pub fn stop_timer(&mut self) {
    if self.timer_running {
      let start_time = self.start_time.unwrap();
      let start_date = self.start_date.clone().unwrap();

      self.add_log(start_time).expect("APP STATE => Could not add log");
      self.add_record(start_time, start_date).expect("APP STATE => Could not add record");
      self.set_history();
    }
  }

  /// Adds a log message to indicate when the timer was stopped and the elapsed time.
  ///
  /// # Arguments
  /// - `start_time`: The time when the timer was started, used to calculate elapsed time.
  ///
  /// # Returns
  /// - `Ok(())` if the log was successfully added.
  /// - `Err(&'static str)` if there was an issue adding the log.
  fn add_log (&mut self, start_time: Instant) -> Result<(), &'static str> {
    let elapsed = format_duration(start_time.elapsed().as_secs());
    self.timer_logs.push(format!(
      "Timer stopped. Duration: {}.",
      elapsed
    ));
    self.timer_running = false;
    self.start_time = None;
    Ok(())
  }

  /// Adds a new `Record` to the database based on the elapsed time.
  ///
  /// # Arguments
  /// - `start_time`: The time when the timer started, used to calculate elapsed time.
  /// - `start_date`: The formatted date and time when the timer was started.
  ///
  /// # Returns
  /// - `Ok(())` if the record was successfully added.
  /// - `Err(&'static str)` if there was an issue adding the record.
  fn add_record (&mut self, start_time: Instant, start_date: String) -> Result<(), &'static str> {
    let duration = start_time.elapsed().as_secs();
    let record = Record::new(duration, start_date);
    create_record(&record);
    Ok(())
  }

  /// Retrieves the entire record history from the database and updates the app state.
  pub fn set_history(&mut self) {
    self.history = find_all_records();
  }
}
