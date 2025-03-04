use std::time::{Instant, SystemTime, UNIX_EPOCH};
use ratatui::widgets::{ListState, TableState};
use crate::format_duration;
use crate::models::Record;

#[derive(Default)]
pub struct AppState {
  pub start_time: Option<Instant>,
  pub timer_running: bool,
  pub timer_logs: Vec<String>,
  pub timer_records: Vec<Record>,
  pub logs_state: ListState,
  pub history_state: TableState,
}

impl AppState {
  pub fn start_timer(&mut self) {
    if !self.timer_running {
      self.timer_running = true;
      self.start_time = Some(Instant::now());
      self.timer_logs.push("Timer started.".to_string());
    }
  }

  pub fn stop_timer(&mut self) {
    if self.timer_running {
      match self.start_time {
        Some(start_time) => {
          self.add_log(start_time).expect("APP STATE => Could not add log");
          self.add_record(start_time).expect("APP STATE => Could not add record")
        }
        None => {}
      }
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

  fn add_record (&mut self, start_time: Instant) -> Result<(), &'static str> {
    let created_at = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let duration = start_time.elapsed().as_secs();
    let record = Record::new(created_at, duration);
    self.timer_records.push(record);
    Ok(())
  }
}
