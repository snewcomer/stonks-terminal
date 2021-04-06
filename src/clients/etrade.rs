use chrono::prelude::*;
use derive_builder::Builder;
use crate::app::{Ticker, SearchType, User};
use crate::config::ClientConfig;
use crate::stonks_error::RuntimeError;
use crate::session::{Credentials, Session};
use crate::store::Store;

pub type ClientResult<T> = Result<T, RuntimeError>;

pub struct EtradeTokenInfo {
    pub expires_at: Option<DateTime<Utc>>
}

#[derive(Builder, Clone)]
pub struct Etrade {
    client_creds: Credentials,
}

impl Etrade {
    pub fn new(client_config: ClientConfig) -> Self {
        let client_creds = Credentials::new(client_config.consumer_key.to_string(), client_config.consumer_secret.to_string());
        Self {
            client_creds,
        }
    }
}

impl Etrade {
    fn build_authorization_header<T: Store>(&self, uri: &str, session: &Session<T>) -> String {
        let authorization_header = oauth::Builder::<_, _>::new(self.client_creds.clone().into(), oauth::HmacSha1)
            .token(Some(self.get_access_creds(&session)))
            .get(uri, &());

        authorization_header
    }

    fn get_access_creds<T: Store>(&self, session: &Session<T>) -> oauth::Credentials {
        let access_creds: Option<&Credentials> = session.store.get(self.client_creds.key.to_string());
        access_creds.unwrap().clone().into()
    }

    pub async fn ticker<T: Store>(&self, session: &Session<T>, symbol: &str) -> ClientResult<Ticker> {
        let uri = session.urls.etrade_ticker_url(symbol);
        let authorization_header = self.build_authorization_header(&uri, &session);

        let body = session.send_request(&uri, authorization_header).await;
        let ticker: Ticker = serde_urlencoded::from_bytes(&body)?;

        Ok(ticker)
    }

    pub async fn portfolio(&self) -> ClientResult<Vec<Ticker>> {
        todo!();
    }

    pub async fn search(&self, search_term: &String, search_type: SearchType, search_limit: u32) -> ClientResult<Vec<Ticker>> {
        todo!();
    }

    pub async fn current_user(&self) -> ClientResult<User> {
        todo!();
    }

    pub async fn current_user_saved_tickers(&self, search_limit: u32, offset: Option<u32>) -> ClientResult<Vec<Ticker>> {
        todo!();
    }

    pub async fn current_user_saved_tickers_add(&self, ticker_symbols: &[String]) -> ClientResult<()> {
        todo!();
    }

    pub async fn current_user_saved_tickers_delete(&self, ticker_symbols: &[String]) -> ClientResult<()> {
        todo!();
    }

    pub async fn current_user_saved_tickers_contains(&self, ticker_symbols: &Vec<String>) -> ClientResult<Vec<String>> {
        todo!();
    }
}

#[derive(Clone)]
pub struct EtradeOAuth {
}

impl EtradeOAuth {
    pub fn new() -> Self {
        Self {}
    }
}


pub fn get_token() {

}
