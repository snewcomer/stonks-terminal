use crate::app::{ActiveBlock, App, WatchList, RouteId, SearchType, SearchResult, SearchResultType, SelectedTicker, Ticker};
use crate::clients::etrade::{Etrade, EtradeTokenInfo};
use crate::config::ClientConfig;
use crate::session::Session;
use crate::store::Store;
use anyhow::anyhow;
use std::{
    sync::Arc,
};
use tokio::sync::Mutex;

#[derive(Debug)]
pub enum IoEvent {
    RefreshAuthentication,
    GetSearchResults(String),
    GetDowJones,
    GetNasdaq,
    GetSandP,
    GetPortfolio,
    GetAccountsList,
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
            IoEvent::GetAccountsList => {
                self.get_accounts_list().await;
            }
            IoEvent::GetDowJones => {
                self.get_ticker("dji".to_string()).await;
            }
            IoEvent::GetSandP => {
                self.get_ticker("gspc".to_string()).await;
            }
            IoEvent::GetNasdaq => {
                self.get_ticker("ndaq".to_string()).await;
            }
            IoEvent::GetPortfolio => {
                self.get_portfolio().await;
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

    async fn get_accounts_list(&mut self) {
        match self.etrade.accounts_list(&self.session).await {
            Ok(user_accounts) => {
                let mut app = self.app.lock().await;

                app.user_accounts = Some(user_accounts.accounts);
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
                for (i, id) in ids.iter().enumerate() {
                    if let Some(_is_liked) = is_saved_vec.get(i) {
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
        let searched_tickers = self.etrade.search(&self.session, &search_term).await;

        match searched_tickers {
            Ok(tickers) => {
                let mut app = self.app.lock().await;
                // let ticker_ids = tickers
                //     .iter()
                //     .filter_map(|ticker| Some(ticker.symbol.to_owned()))
                //     .collect();

                // // Check if these tickers are saved
                // app.dispatch(IoEvent::CurrentUserSavedTickersContains(ticker_ids));
                app.search_results = SearchResult::tickers(tickers.into_iter().map(|t| t.into()).collect::<Vec<Ticker>>());
                app.search_term = search_term;
            },
            Err(e) => self.handle_error(anyhow!(e)).await
        }
    }

    async fn get_current_user_saved_tickers(&mut self, offset: Option<u32>) {
        match self.etrade.current_user_saved_tickers(self.large_search_limit, offset).await {
            Ok(saved_tickers) => {
                let mut app = self.app.lock().await;

                saved_tickers.iter().for_each(|item| {
                    app.liked_ticker_ids_set.insert(item.symbol.to_string());
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
        match self.etrade.ticker(&self.session, &ticker_id).await {
            Ok(ticker) => {
                let mut app = self.app.lock().await;

                app.selected_ticker = Some(ticker.into());
                app.push_navigation_stack(RouteId::TickerDetail, ActiveBlock::TickerDetail);
            }
            Err(e) => {
                self.handle_error(anyhow!(e)).await;
            }
        }
    }

    async fn get_portfolio(&mut self) {
        match self.etrade.portfolio(&self.session).await {
            Ok(ticker) => {
                let mut app = self.app.lock().await;

                // app.push_navigation_stack(RouteId::Portfolio, ActiveBlock::Portfolio);
            }
            Err(e) => {
                self.handle_error(anyhow!(e)).await;
            }
        }
        // for sidebar portfolio items
        // match self.etrade.portfolio().await {
        //     Ok(portfolio_tickers) => {
                let mut app = self.app.lock().await;

                // app.portfolio_tickers = Some(portfolio_tickers);
                app.selected_ticker_index = Some(0);
            // }
            // Err(e) => {
            //     self.handle_error(anyhow!(e)).await;
            // }
        // }
    }

    async fn refresh_authentication(&mut self) {
        let mut client_config = ClientConfig::new();
        // ask user for configuration details
        if let Ok(_) = client_config.load_config() {
            let _ = self.session.full_access_flow(client_config).await;
        }

    }
}

