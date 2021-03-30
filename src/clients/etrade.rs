use chrono::prelude::*;
use derive_builder::Builder;
use crate::app::{Ticker, SearchType, User};
use crate::stonks_error::RuntimeError;

pub type ClientResult<T> = Result<T, RuntimeError>;

pub struct EtradeTokenInfo {
    pub expires_at: Option<DateTime<Utc>>
}

#[derive(Builder, Clone)]
pub struct Etrade {
}

impl Etrade {
    pub fn new() -> Self {
        Self {}
    }
}

impl Etrade {
    pub async fn ticker(&self, symbol: &str) -> ClientResult<Ticker> {
        todo!();
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
