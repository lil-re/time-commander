use std::time::{Instant, SystemTime, UNIX_EPOCH};
use ratatui::widgets::{ListState, TableState};
use crate::database::record_api::{create_record, find_all_records};
use crate::helpers::format_duration;
use crate::models::{History, Record};

#[derive(Default)]
pub struct AppState {
  pub start_time: Option<Instant>,
  pub start_date: Option<u64>,
  pub timer_running: bool,
  pub timer_logs: Vec<String>,
  pub history: Vec<History>,
  pub logs_state: ListState,
  pub history_state: TableState,
}

impl AppState {
  pub fn start_timer(&mut self) {
    if !self.timer_running {
      self.timer_running = true;
      self.start_time = Some(Instant::now());
      self.start_date = Some(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs());
      self.timer_logs.push("Timer started.".to_string());
    }
  }

  pub fn stop_timer(&mut self) {
    if self.timer_running {
      let start_time = self.start_time.unwrap();
      let start_date = self.start_date.unwrap();

      self.add_log(start_time).expect("APP STATE => Could not add log");
      self.add_record(start_time, start_date).expect("APP STATE => Could not add record")
    }
  }

  fn add_log (&mut self, start_time: Instant) -> Result<(), &'static str> {
    let elapsed = format_duration(start_time.elapsed());
    self.timer_logs.push(format!(
      "Timer stopped. Duration: {}.",
      elapsed
    ));
    self.timer_running = false;
    self.start_time = None;
    Ok(())
  }

  fn add_record (&mut self, start_time: Instant, start_date: u64) -> Result<(), &'static str> {
    let duration = start_time.elapsed().as_secs();
    let record = Record::new(start_date, duration);
    create_record(&record);
    Ok(())
  }

  pub fn get_history(&mut self) -> Vec<History> {
    find_all_records()
  }
}
