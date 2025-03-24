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
use tokio::time::sleep;
use crate::export::{export_history};
use crate::helpers::format_duration;
use crate::state::AppState;

/// Main app UI function that drives the entire terminal user interface.
/// This function runs the UI in a loop, updates the UI, listens for user input,
/// and reacts to keypress events (like starting/stopping the timer).
pub(crate) async fn run_app() -> io::Result<()> {
  terminal::enable_raw_mode()?;
  let stdout = io::stdout();
  let backend = CrosstermBackend::new(stdout);
  let mut terminal = Terminal::new(backend)?;

  // Initialize the app's state, which includes timer and history data
  let mut app = AppState::default();
  app.set_history();

  loop {
    // Render UI layout
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
          // Quit the app by stopping the timer and exiting
          KeyCode::Char('q') | KeyCode::Esc => {
            app.stop_timer();
            return Ok(())
          },
          // Start time
          KeyCode::Char('s') => {
            app.start_timer();
          },
          // Pause/stop timer
          KeyCode::Char('p') => {
            app.stop_timer();
          },
          // Export history to csv file
          KeyCode::Char('e') => {
            export_history(&app.history).expect("Error while exporting Records");
          }
          _ => {}
        }
      }
    }

    sleep(Duration::from_millis(100)).await;
  }
}

/// Defines the layout chunks for the terminal UI.
/// This function splits the terminal window into sections for rendering various parts of the app.
///
/// # Arguments
/// - `area`: The total available space for rendering in the terminal window.
///
/// # Returns
/// - A vector of `Rect` representing the different layout sections.
fn get_chunks(area: Rect) -> Rc<[Rect]> {
  Layout::default()
      .direction(Direction::Horizontal)
      .margin(1)
      .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
      .split(area)
}

/// Renders the history table displaying records of timer activity.
///
/// # Arguments
/// - `app`: The app state that contains the history to render.
/// - `frame`: The terminal frame used to render the widget.
/// - `chunks`: The layout chunks where this history table should be rendered.
fn render_history(app: &mut AppState, frame: &mut Frame, chunks: Rc<[Rect]>) {
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
  frame.render_widget(table, chunks[1]);
}

/// Shows the current timer status (running or stopped), logs, and commands available to the user.
///
/// # Arguments
/// - `app`: The app state that contains the timer and logs to display.
/// - `f`: The terminal frame used to render the widgets.
/// - `chunks`: The layout chunks where this section should be rendered.
fn render_timer(app: &mut AppState, frame: &mut Frame, chunks: &Rc<[Rect]>) {
  // Create inner layout with 3 parts (timer, logs, command list)
  let inner_layout = Layout::default()
      .direction(Direction::Vertical)
      .constraints(vec![
        Constraint::Min(4),
        Constraint::Fill(50),
        Constraint::Min(6),
      ])
      .split(chunks[0]);

  // Timer block
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
  frame.render_widget(timer_list, inner_layout[0]);

  // Logs block
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
  frame.render_stateful_widget(logs_list, inner_layout[1], &mut app.logs_state);

  // Commands block
  let commands_text = vec![
    Span::raw("<s> Start timer".to_string()),
    Span::raw("<p> Pause timer".to_string()),
    Span::raw("<e> Export records".to_string()),
    Span::raw("<q> Quit".to_string()),
  ];
  let commands_list = List::new(commands_text)
      .block(Block::bordered().title("Commands"))
      .highlight_style(Style::new())
      .highlight_symbol(">>")
      .repeat_highlight_symbol(true);
  frame.render_widget(commands_list, inner_layout[2]);
}
