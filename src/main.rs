mod config;
mod session;
mod stonks_error;
mod banner;
mod store;
use banner::BANNER;
use config::ClientConfig;
use store::AuthInMemoryStore;
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

    let mut session = Session::new(mode, AuthInMemoryStore::new());

    // 1. make request for request_token
    // https://apisb.etrade.com/docs/api/authorization/request_token.html
    let creds = Credentials::new(client_config.consumer_key.to_string(), client_config.consumer_secret.to_string());
    let request_token_creds = session.request_token(&creds).await;

    // 2. obtain verification code
    // lives for 5 minutes
    // https://apisb.etrade.com/docs/api/authorization/authorize.html
    if request_token_creds.is_err() {
        return Err(RuntimeError { message: "request_token failed".to_string() })
    }

    let request_token_creds = request_token_creds.unwrap();
    let verification_code = session.verification_code(&creds, &request_token_creds)?;
    session.store.verification_code = verification_code.to_owned();

    // 3. make request for authorization token
    // expires at midnight Eastern Time
    // These should be used and passed in the header of subsequent requests
    // https://apisb.etrade.com/docs/api/authorization/get_access_token.html
    let oauth_access_creds = session.access_token(&creds, &request_token_creds, &verification_code).await;
    dbg!(&oauth_access_creds);
    let oauth_access_creds = oauth_access_creds.unwrap();

    // finished oauth process
    session.store.put(oauth_access_creds.key.to_string(), oauth_access_creds.secret);
    println!("OAuth saved to in memory store {}", &oauth_access_creds.key);

    start_ui();

    Ok(())
}

fn start_ui() -> Result<(), RuntimeError> {
    dbg!("start ui");
    Ok(())
}
