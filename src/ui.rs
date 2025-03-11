use std::io;
use std::rc::Rc;
use std::time::Duration;
use crossterm::{event, terminal};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::prelude::{Color, Span, Style};
use ratatui::{Frame, Terminal};
use ratatui::style::Stylize;
use ratatui::widgets::{Block, Borders, List, Padding, Paragraph, Row, Table};
use tokio::time::sleep;
use crate::helpers::format_duration;
use crate::state::AppState;

pub(crate) async fn run_app() -> io::Result<()> {
  terminal::enable_raw_mode()?;
  let stdout = io::stdout();
  let backend = CrosstermBackend::new(stdout);
  let mut terminal = Terminal::new(backend)?;
  let mut app = AppState::default();
  app.set_history();

  loop {
    terminal.draw(|f| {
      let area = f.area();
      let chunks: Rc<[Rect]> = get_chunks(area);
      render_timer(&mut app, f, &chunks);
      render_history(&mut app, f, chunks);
    })?;

    // Handle key press events
    if event::poll(Duration::from_millis(100))? {
      if let event::Event::Key(KeyEvent { code, .. }) = event::read()? {
        match code {
          KeyCode::Char('q') | KeyCode::Esc => {
            app.stop_timer();
            return Ok(())
          },
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

    sleep(Duration::from_millis(100)).await;
  }

  terminal::disable_raw_mode()?;
  Ok(())
}

fn get_chunks(area: Rect) -> Rc<[Rect]> {
  Layout::default()
      .direction(Direction::Horizontal)
      .margin(1)
      .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
      .split(area)
}

/// Records history table
fn render_history(app: &mut AppState, f: &mut Frame, chunks: Rc<[Rect]>) {
  let table_block = Block::default()
      .title("Table")
      .borders(Borders::ALL)
      .style(Style::default().fg(Color::White));
  let headers = ["Date", "Start", "End", "Total", "Pauses"];
  let rows = app.history.iter().map(|h| Row::new(vec![
    h.record_date.clone(),
    h.start_time.clone(),
    h.end_time.clone(),
    format_duration(h.total_duration),
    h.total_pauses.to_string(),
  ]));
  let widths = vec![
    Constraint::Min(20),
    Constraint::Min(20),
    Constraint::Min(20),
    Constraint::Min(20),
    Constraint::Min(20),
  ];
  let table = Table::new(rows, widths)
      .header(Row::new(headers).style(Style::default().fg(Color::White)))
      .block(table_block)
      .widths(&[Constraint::Percentage(30), Constraint::Percentage(30), Constraint::Percentage(30), Constraint::Percentage(30), Constraint::Percentage(30)]);
  f.render_widget(table, chunks[1]);
}

/// Timer container
fn render_timer(app: &mut AppState, f: &mut Frame, chunks: &Rc<[Rect]>) {
  let inner_layout = Layout::default()
      .direction(Direction::Vertical)
      .constraints(vec![
        Constraint::Min(4),
        Constraint::Fill(50),
        Constraint::Min(6),
      ])
      .split(chunks[0]);

  let timer_text = if app.timer_running {
    let elapsed = format_duration(app.start_time.unwrap().elapsed().as_secs());
    vec![
      Span::raw("Running".to_string()),
      Span::raw(format!("{}", elapsed))
    ]
  } else {
    vec![
      Span::raw("Stopped".to_string())
    ]
  };
  let timer_list = List::new(timer_text)
      .block(Block::bordered().title("Timer"))
      .highlight_style(Style::new())
      .highlight_symbol(">>")
      .repeat_highlight_symbol(true);
  f.render_widget(timer_list, inner_layout[0]);

  let logs_text = app
      .timer_logs
      .iter()
      .map(|log| Span::raw(log.clone()))
      .collect::<Vec<_>>();
  let logs_list = List::new(logs_text)
      .block(Block::bordered().title("Logs"))
      .highlight_style(Style::new())
      .highlight_symbol(">>")
      .repeat_highlight_symbol(true);
  f.render_stateful_widget(logs_list, inner_layout[1], &mut app.logs_state);

  let commands_text = vec![
    Span::raw("<s> Start timer".to_string()),
    Span::raw("<d> Stop timer".to_string()),
    Span::raw("<q> Quit".to_string()),
  ];
  let commands_list = List::new(commands_text)
      .block(Block::bordered().title("Commands"))
      .highlight_style(Style::new())
      .highlight_symbol(">>")
      .repeat_highlight_symbol(true);
  f.render_widget(commands_list, inner_layout[2]);
}
