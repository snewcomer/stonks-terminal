use super::etrade_xml_structs;
use super::etrade_json_structs;
use chrono::prelude::*;
use derive_builder::Builder;
use crate::app::{Ticker, User};
use crate::config::ClientConfig;
use crate::stonks_error::RuntimeError;
use crate::session::{Credentials, Session};
use crate::store::Store;
use serde_json::json;

pub type ClientResult<T> = Result<T, RuntimeError>;

// pub struct EtradeTokenInfo {
//     pub expires_at: Option<DateTime<Utc>>
// }

#[derive(oauth::Request)]
struct AC {
    instType: String,
    realTimeNAV: String,
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

    pub async fn ticker<T: Store>(&self, session: &Session<T>, symbol: &str) -> ClientResult<etrade_xml_structs::TickerXML> {
        let uri = session.urls.etrade_ticker_url(symbol, &session.mode);
        let authorization_header = self.build_authorization_header(&uri, &session);

        let resp = session.send_request(&uri, authorization_header).await.unwrap();
        if resp.status().as_u16() / 100 == 2 {
            let bd = resp.into_body();
            let bytes = hyper::body::to_bytes(bd).await?;
            let ticker: etrade_xml_structs::TickerXML = serde_xml_rs::from_reader(&bytes[..])?;
            Ok(ticker)
        } else if resp.status().as_u16() == 401 {
            // retry auth and rety
            // if let Some(cached_creds) = session.get_creds_from_cache() {
            //     session.renew_access_token(self.client_creds.clone().into(), cached_creds).await?;
            // }
            return Err(RuntimeError { message: "request failed".to_string() });
        } else {
            return Err(RuntimeError { message: "request failed".to_string() });
        }
    }

    pub async fn portfolio<T: Store>(&self, account_id_key: &str, session: &Session<T>) -> ClientResult<etrade_xml_structs::PortfolioXML> {
        let uri = session.urls.etrade_portfolio_url(account_id_key, &session.mode);
        let authorization_header = self.build_authorization_header(&uri, &session);

        let resp = session.send_request(&uri, authorization_header).await.unwrap();
        if resp.status().as_u16() / 100 == 2 {
            let bd = resp.into_body();
            let bytes = hyper::body::to_bytes(bd).await?;
            let results: etrade_xml_structs::PortfolioXML = serde_xml_rs::from_reader(&bytes[..])?;
            Ok(results)
        } else if resp.status().as_u16() == 401 {
            return Err(RuntimeError { message: "request failed".to_string() });
        } else {
            return Err(RuntimeError { message: "request failed".to_string() });
        }
    }

    pub async fn search<T: Store>(&self, session: &Session<T>, search_term: &String) -> ClientResult<Vec<etrade_xml_structs::TickerSearchData>> {
        let uri = session.urls.etrade_search_url(search_term, &session.mode);
        let authorization_header = self.build_authorization_header(&uri, &session);

        let resp = session.send_request(&uri, authorization_header).await.unwrap();
        if resp.status().as_u16() / 100 == 2 {
            let bd = resp.into_body();
            let bytes = hyper::body::to_bytes(bd).await?;
            let results: etrade_xml_structs::SearchXML = serde_xml_rs::from_reader(&bytes[..])?;
            Ok(results.items)
        } else if resp.status().as_u16() == 401 {
            return Err(RuntimeError { message: "request failed".to_string() });
        } else {
            return Err(RuntimeError { message: "request failed".to_string() });
        }
    }

    pub async fn accounts_list<T: Store>(&mut self, session: &Session<T>) -> Result<etrade_xml_structs::AccountsListXML, RuntimeError> {
        let uri = session.urls.accounts_list(&session.mode);
        let authorization_header = self.build_authorization_header(&uri, &session);

        let resp = session.send_request(&uri, authorization_header).await?;
        let bd = resp.into_body();
        let bytes = hyper::body::to_bytes(bd).await?;
        // std::fs::write("res-account.txt", &std::str::from_utf8(&bytes).unwrap());
        let results: etrade_xml_structs::AccountsListXML = serde_xml_rs::from_reader(&bytes[..])?;

        Ok(results)
    }

    pub async fn account_balance<T: Store>(&mut self, account_id_key: &str, session: &Session<T>) -> Result<etrade_xml_structs::AccountBalance, RuntimeError> {
        let req = AC {
            instType: "BROKERAGE".to_string(),
            realTimeNAV: "true".to_string()
        };
        let uri = session.urls.account_balance(account_id_key, &session.mode);
        let authorization_header = oauth::Builder::<_, _>::new(self.client_creds.clone().into(), oauth::HmacSha1)
            .token(Some(self.get_access_creds(&session)))
            .get(&session.urls.account_balance_base(&account_id_key, &session.mode), &req);

        let resp = session.send_request(&uri, authorization_header).await?;
        let bd = resp.into_body();
        let bytes = hyper::body::to_bytes(bd).await?;
        let results: etrade_xml_structs::AccountBalance = serde_xml_rs::from_reader(&bytes[..])?;

        Ok(results)
    }

    pub async fn preview_order_request<T: Store>(&self, account_id_key: &str, session: &Session<T>, preview_order_request: etrade_json_structs::PreviewOrderRequest) -> ClientResult<etrade_json_structs::PreviewOrderResponse> {
        let uri = session.urls.etrade_order_preview_url(account_id_key, &session.mode);
        // OAuth specification explicitly states that only form-encoded data should be included,
        // not JSON body
        let authorization_header = oauth::Builder::<_, _>::new(self.client_creds.clone().into(), oauth::HmacSha1)
            .token(Some(self.get_access_creds(&session)))
            .post(&uri, &());

        let body = json!(preview_order_request);
        let resp = session.send_post_request(&uri, authorization_header, body.to_string()).await?;
        let status = resp.status();
        let bd = resp.into_body();
        let bytes = hyper::body::to_bytes(bd).await?;
        // std::fs::write("res.txt", &std::str::from_utf8(&bytes).unwrap());
        if status.as_u16() / 100 == 2 {
            // let bd = resp.into_body();
            // let bytes = hyper::body::to_bytes(bd).await?;
            let results: etrade_json_structs::PreviewOrderResponse = serde_json::from_reader(&bytes[..])?;
            Ok(results)
        } else {
            return Err(RuntimeError { message: "Request for Preview Order failed".to_string() });
        }
    }

    pub async fn place_order_request<T: Store>(&self, account_id_key: &str, session: &Session<T>) -> ClientResult<etrade_json_structs::PlaceOrderResponse> {
        let uri = session.urls.etrade_order_preview_url(account_id_key, &session.mode);

        let authorization_header = self.build_authorization_header(&uri, &session);
        let resp = session.send_request(&uri, authorization_header).await?;
        let bd = resp.into_body();
        let bytes = hyper::body::to_bytes(bd).await?;
        let results: etrade_json_structs::PlaceOrderResponse = serde_xml_rs::from_reader(&bytes[..])?;

        Ok(results)
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
}

// #[derive(Clone)]
// pub struct EtradeOAuth {
// }

// impl EtradeOAuth {
//     pub fn new() -> Self {
//         Self {}
//     }
// }


// pub fn get_token() {

// }
