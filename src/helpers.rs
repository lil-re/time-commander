use std::time::Duration;

pub fn format_duration(elapsed: u64) -> String {
  let hours = elapsed / 3600;
  let minutes = (elapsed % 3600) / 60;
  let seconds = elapsed % 60;
  format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}
