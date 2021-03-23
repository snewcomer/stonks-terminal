mod config;
mod session;
mod stonks_error;
mod banner;
use banner::BANNER;
use config::ClientConfig;
use crate::session::{Credentials, Mode, Session};
use stonks_error::RuntimeError;
use std::env;
use clap::{App as ClapApp, Arg};

#[tokio::main]
async fn main() -> Result<(), RuntimeError> {
    let matches = ClapApp::new("Stonksss")
        .version("1.0")
        .author("Scott N. <snewcomer24@gmail.com>")
        .about("Trade easy")
        .before_help(BANNER)
        .after_help(
          "Your spotify Client ID and Client Secret are stored in $HOME/.config/spotify-tui/client.yml",
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

    let session = Session::new(mode);

    // make request for request_token
    let creds = Credentials::new(client_config.consumer_key.to_string(), client_config.consumer_secret.to_string());
    let _ = session.request_token(&creds).await;

    Ok(())
}
