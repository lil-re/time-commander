mod models;
mod state;

use ratatui::{
  backend::CrosstermBackend,
  layout::{Constraint, Direction, Layout},
  style::{Color, Style, Stylize},
  text::{Span},
  widgets::{Block, Borders, Table, Row, List},
  Terminal,
};
use crossterm::{
  event::{self, KeyCode, KeyEvent},
  terminal,
};
use std::{
  io,
  time::{Duration},
};
use tokio::time;
use crate::state::AppState;

fn format_duration(elapsed: Duration) -> String {
  let total_seconds = elapsed.as_secs();
  let hours = total_seconds / 3600;
  let minutes = (total_seconds % 3600) / 60;
  let seconds = total_seconds % 60;
  format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}

async fn run_app() -> io::Result<()> {
  terminal::enable_raw_mode()?;
  let stdout = io::stdout();
  let backend = CrosstermBackend::new(stdout);
  let mut terminal = Terminal::new(backend)?;
  let mut app = AppState::default();

  loop {
    terminal.draw(|f| {
      let area = f.area();
      let chunks = Layout::default()
          .direction(Direction::Horizontal)
          .margin(1)
          .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
          .split(area);

      // Timer Section
      let timer_text = if app.timer_running {
        let elapsed = format_duration(app.start_time.unwrap().elapsed());
        format!(
          "Running: {}.",
          elapsed
        )
      } else {
        "Stopped".to_string()
      };

      let log_text = app
          .timer_logs
          .iter()
          .map(|log| Span::raw(log.clone()))
          .collect::<Vec<_>>();
      let list = List::new(log_text)
          .block(Block::bordered().title("List"))
          .highlight_style(Style::new().reversed())
          .highlight_symbol(">>")
          .repeat_highlight_symbol(true);

      f.render_stateful_widget(list, chunks[0], &mut app.logs_state);
      f.render_widget(ratatui::widgets::Paragraph::new(timer_text), chunks[0]);

      // History
      let table_block = Block::default()
          .title("Table")
          .borders(Borders::ALL)
          .style(Style::default().fg(Color::White));
      let headers = ["Header 1", "Header 2", "Header 3"];
      let rows = vec![
        Row::new(vec!["Row 1", "Data 1", "Data 1"]),
        Row::new(vec!["Row 2", "Data 2", "Data 2"]),
        Row::new(vec!["Row 3", "Data 3", "Data 3"]),
      ];
      let widths = vec![
        Constraint::Min(20),
        Constraint::Min(20),
        Constraint::Min(20),
      ];
      let table = Table::new(rows, widths)
          .header(Row::new(headers).style(Style::default().fg(Color::White)))
          .block(table_block)
          .widths(&[Constraint::Percentage(30), Constraint::Percentage(30), Constraint::Percentage(30)]);
      f.render_widget(table, chunks[1]);
    })?;

    // Handle key press events
    if event::poll(Duration::from_millis(100))? {
      if let event::Event::Key(KeyEvent { code, .. }) = event::read()? {
        match code {
          KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
          KeyCode::Char('s') => {
            app.start_timer();
          },
          KeyCode::Char('d') => {
            app.stop_timer();
          }
          _ => {}
        }
      }
    }

    time::sleep(Duration::from_millis(100)).await;
  }

  terminal::disable_raw_mode()?;
  Ok(())
}

#[tokio::main]
async fn main() -> io::Result<()> {
  run_app().await
}
