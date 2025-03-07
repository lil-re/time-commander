use std::time::Duration;

pub fn format_duration(elapsed: Duration) -> String {
  let total_seconds = elapsed.as_secs();
  let hours = total_seconds / 3600;
  let minutes = (total_seconds % 3600) / 60;
  let seconds = total_seconds % 60;
  format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}
