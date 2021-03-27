use crate::app::{ActiveBlock, App, MajorIndices, RouteId, SearchType, SearchResult, SearchResultType, SelectedTicker};
use crate::clients::etrade::{Etrade, EtradeTokenInfo};
use crate::config::ClientConfig;
use crate::session::Session;
use crate::store::Store;
use crate::utils;
use anyhow::anyhow;
use chrono::prelude::*;
// use retrade::{
//   client::Etrade,
//   model::{
//     album::SimplifiedAlbum,
//     artist::FullArtist,
//     offset::for_position,
//     page::Page,
//     playlist::{Playlistticker, SimplifiedPlaylist},
//     recommend::Recommendations,
//     search::SearchResult,
//     show::SimplifiedShow,
//     ticker::Ticker,
//     PlayingItem,
//   },
//   oauth2::{EtradeClientCredentials, EtradeOAuth, TokenInfo},
//   senum::{AdditionalType, Country, RepeatState, SearchType},
//   util::get_token,
// };
use serde_json::{map::Map, Value};
use std::{
    sync::Arc,
    time::{Duration, Instant, SystemTime},
};
use tokio::sync::Mutex;
use tokio::try_join;

#[derive(Debug)]
pub enum IoEvent {
    RefreshAuthentication,
    GetSearchResults(String),
    GetDowJones,
    GetNasdaq,
    GetSandP,
    GetUser,
    GetCurrentSavedTickers(Option<u32>),
    CurrentUserSavedTickersContains(Vec<String>),
    CurrentUserSavedTickerDelete(String),
    CurrentUserSavedTickerAdd(String),
    UpdateSearchLimits(u32, u32),
    GetTicker(String),
}

#[derive(Clone)]
pub struct Network<'a, T> {
    pub etrade: Etrade,
    pub session: Session<T>,
    large_search_limit: u32,
    small_search_limit: u32,
    pub client_config: ClientConfig,
    pub app: &'a Arc<Mutex<App>>,
}

impl<'a, T> Network<'a, T>
where T: Store {
    pub fn new(
        etrade: Etrade,
        session: Session<T>,
        client_config: ClientConfig,
        app: &'a Arc<Mutex<App>>,
    ) -> Self {
        Network {
            etrade,
            session,
            large_search_limit: 20,
            small_search_limit: 4,
            client_config,
            app,
        }
    }

    pub async fn handle_network_event(&mut self, io_event: IoEvent) {
        match io_event {
            IoEvent::RefreshAuthentication => {
                self.refresh_authentication().await;
            }
            IoEvent::GetUser => {
                self.get_user().await;
            }
            IoEvent::GetDowJones => {
                self.get_ticker("dji".to_string()).await;
            }
            IoEvent::GetSandP => {
                self.get_ticker("inx".to_string()).await;
            }
            IoEvent::GetNasdaq => {
                self.get_ticker("ndaq".to_string()).await;
            }
            IoEvent::GetSearchResults(search_term) => {
                self.get_search_results(search_term).await;
            }
            IoEvent::GetCurrentSavedTickers(offset) => {
                self.get_current_user_saved_tickers(offset).await;
            }
            IoEvent::UpdateSearchLimits(large_search_limit, small_search_limit) => {
                self.large_search_limit = large_search_limit;
                self.small_search_limit = small_search_limit;
            }
            IoEvent::GetTicker(ticker_id) => {
                self.get_ticker(ticker_id).await;
            }
            IoEvent::CurrentUserSavedTickersContains(ticker_ids) => {
                self.current_user_saved_tickers_contains(ticker_ids).await;
            }
            IoEvent::CurrentUserSavedTickerDelete(show_id) => {
                self.current_user_saved_ticker_delete(show_id).await;
            }
            IoEvent::CurrentUserSavedTickerAdd(show_id) => {
                self.current_user_saved_ticker_add(show_id).await;
            }
        };

        let mut app = self.app.lock().await;
        app.is_loading = false;
    }

    async fn handle_error(&mut self, e: anyhow::Error) {
        let mut app = self.app.lock().await;
        app.handle_error(e);
    }

    async fn get_user(&mut self) {
        match self.etrade.current_user().await {
            Ok(user) => {
                let mut app = self.app.lock().await;
                app.user = Some(user);
            }
            Err(e) => {
                self.handle_error(anyhow!(e)).await;
            }
        }
    }

    async fn current_user_saved_tickers_contains(&mut self, ids: Vec<String>) {
        match self.etrade.current_user_saved_tickers_contains(&ids).await {
            Ok(is_saved_vec) => {
                let mut app = self.app.lock().await;
                for id in ids.iter() {
                    if let Some(is_liked) = is_saved_vec.get(id) {
                        app.liked_ticker_ids_set.insert(id.to_string());
                    } else {
                        // The song is not liked, so check if it should be removed
                        if app.liked_ticker_ids_set.contains(id) {
                            app.liked_ticker_ids_set.remove(id);
                        }
                    };
                }
            }
            Err(e) => {
                self.handle_error(anyhow!(e)).await;
            }
        }
    }

    async fn get_search_results(&mut self, search_term: String) {
        let searched_tickers = self.etrade.search(
            &search_term,
            SearchType::Ticker,
            self.small_search_limit,
        ).await;

        match searched_tickers {
            Ok(tickers) => {
                let mut app = self.app.lock().await;
                let ticker_ids = tickers
                    .iter()
                    .filter_map(|ticker| Some(ticker.symbol.to_owned()))
                    .collect();

                // Check if these tickers are saved
                app.dispatch(IoEvent::CurrentUserSavedTickersContains(ticker_ids));
                app.search_results = SearchResult::tickers(tickers);
            },
            Err(e) => self.handle_error(anyhow!(e)).await
        }
    }

    async fn get_current_user_saved_tickers(&mut self, offset: Option<u32>) {
        match self.etrade.current_user_saved_tickers(self.large_search_limit, offset).await {
            Ok(saved_tickers) => {
                let mut app = self.app.lock().await;

                saved_tickers.iter().for_each(|item| {
                    if let ticker_id = &item.symbol {
                        app.liked_ticker_ids_set.insert(ticker_id.to_string());
                    }
                });

                app.library.saved_tickers = saved_tickers;
            }
            Err(e) => {
                self.handle_error(anyhow!(e)).await;
            }
        }
    }

    async fn toggle_save_ticker(&mut self, ticker_id: String) {
        match self
            .etrade
            .current_user_saved_tickers_contains(&vec![ticker_id.clone()])
            .await
            {
                Ok(saved) => {
                    if saved.contains(&ticker_id) {
                        match self.etrade.current_user_saved_tickers_delete(&[ticker_id.clone()]).await {
                            Ok(()) => {
                                let mut app = self.app.lock().await;
                                app.liked_ticker_ids_set.remove(&ticker_id);
                            }
                            Err(e) => {
                                self.handle_error(anyhow!(e)).await;
                            }
                        }
                    } else {
                        match self.etrade.current_user_saved_tickers_add(&[ticker_id.clone()]).await {
                            Ok(()) => {
                                // TODO: This should ideally use the same logic as `self.current_user_saved_tickers_contains`
                                let mut app = self.app.lock().await;
                                app.liked_ticker_ids_set.insert(ticker_id);
                            }
                            Err(e) => {
                                self.handle_error(anyhow!(e)).await;
                            }
                        }
                    }
                }
                Err(e) => {
                    self.handle_error(anyhow!(e)).await;
                }
            };
    }

    pub async fn current_user_saved_ticker_delete(&mut self, ticker_id: String) {
        match self
            .etrade
            .current_user_saved_tickers_delete(&[ticker_id.to_owned()])
            .await
            {
                Ok(_) => {
                    self.get_current_user_saved_tickers(None).await;
                    let mut app = self.app.lock().await;
                    app.saved_ticker_ids_set.remove(&ticker_id.to_owned());
                }
                Err(e) => {
                    self.handle_error(anyhow!(e)).await;
                }
            };
    }

    async fn current_user_saved_ticker_add(&mut self, ticker_id: String) {
        match self
            .etrade
            .current_user_saved_tickers_add(&[ticker_id.to_owned()]).await
            {
                Ok(_) => {
                    let mut app = self.app.lock().await;
                    app.saved_ticker_ids_set.insert(ticker_id.to_owned());
                }
                Err(e) => self.handle_error(anyhow!(e)).await,
            }
    }

    async fn get_ticker(&mut self, ticker_id: String) {
        match self.etrade.ticker(&ticker_id).await {
            Ok(ticker) => {
                let selected_ticker = SelectedTicker {
                    ticker,
                    selected_index: 0,
                };

                let mut app = self.app.lock().await;

                app.selected_ticker = Some(selected_ticker);
                app.push_navigation_stack(RouteId::Ticker, ActiveBlock::Ticker);
            }
            Err(e) => {
                self.handle_error(anyhow!(e)).await;
            }
        }
    }

    async fn refresh_authentication(&mut self) {
        let mut client_config = ClientConfig::new();
        // ask user for configuration details
        client_config.load_config();

        let _ = self.session.full_access_flow(client_config).await;
    }
}

