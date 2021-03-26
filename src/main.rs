mod config;
mod session;
mod stonks_error;
mod store;
mod ui;
mod app;
use config::ClientConfig;
use store::AuthInMemoryStore;
use crate::session::{Credentials, Mode, Session};
use stonks_error::RuntimeError;
use clap::{App as ClapApp, Arg};
use crossterm::{
  // cursor::MoveTo,
  event::{DisableMouseCapture, EnableMouseCapture},
  execute,
  style::Print,
  terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
  // ExecutableCommand,
};
use std::io::{stdout};
use ui::{event::Events, event::Event, key::Key};
use tui::{
  backend::{Backend, CrosstermBackend},
  Terminal,
};
use app::App;

#[tokio::main]
async fn main() -> Result<(), RuntimeError> {
    let matches = ClapApp::new("Stonksss")
        .version("1.0")
        .author("Scott N. <snewcomer24@gmail.com>")
        .about("Trade easy")
        .after_help(
          "Your etrade Client ID and Client Secret are stored in $HOME/.config/stonks-terminal/client.yml",
        )
        .arg(Arg::with_name("config")
            .short("c")
            .long("config")
            .help("Specify configuration file path.")
            .takes_value(true))
        .arg(Arg::with_name("mode")
            .short("m")
            .long("mode")
            .help("Specify either --sandbox or --live. Default is --live.")
            .takes_value(true))
        .get_matches();

    match matches.value_of("mode") {
        Some("sandbox") => run(Mode::Sandbox).await,
        _ => run(Mode::Live).await,
    }
}

async fn run(mode: Mode) -> Result<(), RuntimeError> {
    let mut client_config = ClientConfig::new();
    // ask user for configuration details
    client_config.load_config()?;

    println!("Request token in flight for {:?} mode", mode);

    // let mut session = Session::new(mode, AuthInMemoryStore::new());

    // // 1. make request for request_token
    // // https://apisb.etrade.com/docs/api/authorization/request_token.html
    // let creds = Credentials::new(client_config.consumer_key.to_string(), client_config.consumer_secret.to_string());
    // let request_token_creds = session.request_token(&creds).await;

    // // 2. obtain verification code
    // // lives for 5 minutes
    // // https://apisb.etrade.com/docs/api/authorization/authorize.html
    // if request_token_creds.is_err() {
    //     return Err(RuntimeError { message: "request_token failed".to_string() })
    // }

    // let request_token_creds = request_token_creds.unwrap();
    // let verification_code = session.verification_code(&creds, &request_token_creds)?;
    // session.store.verification_code = verification_code.to_owned();

    // // 3. make request for authorization token
    // // expires at midnight Eastern Time
    // // These should be used and passed in the header of subsequent requests
    // // https://apisb.etrade.com/docs/api/authorization/get_access_token.html
    // let oauth_access_creds = session.access_token(&creds, &request_token_creds, &verification_code).await;
    // dbg!(&oauth_access_creds);
    // let oauth_access_creds = oauth_access_creds.unwrap();

    // // finished oauth process
    // session.store.put(oauth_access_creds.key.to_string(), oauth_access_creds.secret);
    // println!("OAuth saved to in memory store {}", &oauth_access_creds.key);

    let _ = start_ui();

    Ok(())
}

fn start_ui() -> Result<(), RuntimeError> {
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    enable_raw_mode()?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    terminal.draw(|mut f| ui::draw_main(&mut f, App::new()))?;

    let events = Events::new();

    loop {
        match events.next()? {
            Event::Input(key) => {
                if key == Key::Ctrl('c') {
                    break;
                }
            }
            Event::Tick => {}
        }
    }

    close_application()?;

    Ok(())
}

fn close_application() -> Result<(), RuntimeError> {
  disable_raw_mode()?;

  let mut stdout = stdout();
  execute!(stdout, LeaveAlternateScreen, DisableMouseCapture)?;
  Ok(())
}

