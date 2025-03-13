use std::time::Duration;

/// Transforms number of seconds to colon-separated time string (hh:mm:ss)
///
/// # Example
///
/// ```
/// format_duration(3666) // Returns "01:01:06"
/// ```
pub fn format_duration(elapsed: u64) -> String {
  let hours = elapsed / 3600;
  let minutes = (elapsed % 3600) / 60;
  let seconds = elapsed % 60;
  format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}
