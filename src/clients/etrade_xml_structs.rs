use crate::app::{SelectedTicker, Ticker};
use serde::{Deserialize};

#[derive(Debug, Deserialize, PartialEq)]
pub struct All {
    pub bid: String,
    pub ask: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Product {
    pub symbol: String,
    pub securityType: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct QuoteData {
    pub All: All,
    pub Product: Product,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct TickerXML {
    pub QuoteData: QuoteData,
}

