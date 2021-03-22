mod config;
mod session;
mod stonks_error;
mod banner;
use banner::BANNER;
use config::ClientConfig;
use crate::session::{Credentials, Session};
use stonks_error::RuntimeError;

#[tokio::main]
async fn main() -> Result<(), RuntimeError> {
    println!("{}", BANNER);

    let mut client_config = ClientConfig::new();
    // ask user for configuration details
    client_config.load_config()?;

    println!("{}", "Request token in flight");

    let session = Session::new();

    // make request for request_token
    let creds = Credentials::new(client_config.consumer_key.to_string(), client_config.consumer_secret.to_string());
    let _ = session.request_token(&creds).await;

    Ok(())
}
