use crate::app::{SelectedTicker, Ticker};
use serde::{Deserialize};

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Info {
    pub primary_exchange: String,
    pub declared_dividend: String,
    pub dividend: String,
    pub ex_dividend_date: i64,
    pub bid: String,
    pub ask: String,
    pub open: String,
    pub high52: String,
    pub week52_hi_date: i64,
    pub low52: String,
    pub week52_low_date: i64,
    pub eps: String,
    pub pe: String,
    pub beta: String,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Product {
    pub symbol: String,
    pub security_type: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct QuoteData {
    #[serde(rename = "All")]
    pub info: Info,
    #[serde(rename = "Product")]
    pub product: Product,

    #[serde(rename = "dateTime")]
    pub date_time: String,
    #[serde(rename = "dateTimeUTC")]
    pub date_time_utc: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct TickerXML {
    #[serde(rename = "QuoteData")]
    pub quote_data: QuoteData,
}


#[derive(Debug, Deserialize, PartialEq)]
pub struct TickerSearchData {
    pub symbol: String,
    pub description: String,
    // #[serde(rename = "type")]
    // pub security_type: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct SearchXML {
    #[serde(rename = "Data")]
    pub items: Vec<TickerSearchData>,
}


#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub account_id: String,
    pub account_id_key: String,
    pub account_mode: String,
    pub account_desc: String,
    pub account_name: String,
    pub account_type: String,
    pub account_status: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Accounts {
    #[serde(rename = "Account")]
    pub accounts: Vec<Account>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct AccountsListXML {
    #[serde(rename = "Accounts")]
    pub accounts: Accounts,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AccountProduct {
    pub symbol: String,
    pub security_type: String,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    pub position_id: String,
    #[serde(rename = "Product")]
    pub product: AccountProduct,
    pub symbol_description: String,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AccountPortfolio {
    pub account_id: String,
    #[serde(rename = "Position")]
    pub positions: Vec<Position>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct PortfolioXML {
    #[serde(rename = "AccountPortfolio")]
    pub account_portfolio: AccountPortfolio,
}

