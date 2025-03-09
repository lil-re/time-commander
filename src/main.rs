mod models;
mod state;
mod database;
mod ui;
mod helpers;

use std::io;
use clap::Parser;
use crate::database::connection;
use crate::database::migrations::run_migrations;
use crate::ui::run_app;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
  /// Init app and database
  #[arg(short, long, default_value_t = false)]
  init: bool,
}

#[tokio::main]
async fn main() -> io::Result<()> {
  let args = Args::parse();

  if args.init {
    connection::establish_connection().expect("DATABASE => Error establishing connection");
    run_migrations().expect("MIGRATION => Error while running migration");
  } else {
    run_app().await.expect("APP => An error occurred");
  }
  Ok(())
}
