use std::io;
use std::rc::Rc;
use std::time::Duration;
use crossterm::{event, terminal};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::prelude::{Color, Span, Style};
use ratatui::{Frame, Terminal};
use ratatui::widgets::{Block, Borders, List, Row, Table};
use crate::helpers::format_duration;
use crate::state::AppState;

pub async fn run_app() -> io::Result<()> {
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

      render_timer(&mut app, f, &chunks);
      render_history(&mut app, f, chunks);
    })?;

    handle_inputs(&mut app);
    tokio::time::sleep(Duration::from_millis(100)).await;
  }

  terminal::disable_raw_mode()?;
  Ok(())
}

/// Handle key press events
fn handle_inputs(app: &mut AppState) {
  match event::poll(Duration::from_millis(100)) {
    Ok(_) => {
      if let Ok(event::Event::Key(KeyEvent { code, .. })) = event::read() {
        match code {
          KeyCode::Char('q') | KeyCode::Esc => (),
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
    Err(_) => {}
  }
}

/// Records history table
fn render_history(app: &mut AppState, f: &mut Frame, chunks: Rc<[Rect]>) {
  let table_block = Block::default()
      .title("Table")
      .borders(Borders::ALL)
      .style(Style::default().fg(Color::White));
  let headers = ["Date", "Start", "End", "Total", "Pauses"];
  let history = app.get_history();
  let rows = history.iter().map(|h| Row::new(vec![
    h.record_date.clone(),
    h.start_time.clone(),
    h.end_time.clone(),
    h.total_duration.to_string(),
    h.total_pauses.to_string(),
  ]));
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
}

/// Timer container
fn render_timer(app: &mut AppState, f: &mut Frame, chunks: &Rc<[Rect]>) {
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
      .highlight_style(Style::new())
      .highlight_symbol(">>")
      .repeat_highlight_symbol(true);

  f.render_stateful_widget(list, chunks[0], &mut app.logs_state);
  f.render_widget(ratatui::widgets::Paragraph::new(timer_text), chunks[0]);
}
