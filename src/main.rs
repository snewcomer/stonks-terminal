mod config;
mod session;
mod stonks_error;
mod banner;
use banner::BANNER;
use config::ClientConfig;
use crate::session::{Credentials, Mode, Session};
use stonks_error::RuntimeError;
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

    // 1. make request for request_token
    // https://apisb.etrade.com/docs/api/authorization/request_token.html
    let creds = Credentials::new(client_config.consumer_key.to_string(), client_config.consumer_secret.to_string());
    let request_token = session.request_token(&creds).await;

    // 2. obtain verification code
    // https://apisb.etrade.com/docs/api/authorization/authorize.html
    if request_token.is_err() {
        return Err(RuntimeError { message: "request_token failed".to_string() })
    }

    let request_token = request_token.unwrap();
    let verification_code = session.verification_code(&creds, &request_token)?;

    // 3. make request for authorization token
    // https://apisb.etrade.com/docs/api/authorization/get_access_token.html
    let oauth_access_token = session.access_token(&creds, &request_token, &verification_code).await;
    dbg!(oauth_access_token);

    Ok(())
}
