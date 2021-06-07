use crate::app::{ActiveBlock, App, RouteId, SearchResult, Ticker};
use crate::clients::etrade::{Etrade};
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
    GetAccountBalance,
    GetTicker(String),
    GetNotifications,
    GetNotification(String),
    SubmitPreviewRequest,
    GetCurrentSavedTickers(Option<u32>),
    CurrentUserSavedTickerDelete(String),
    CurrentUserSavedTickerAdd(String),
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
            IoEvent::GetAccountsList => {
                self.get_accounts_list().await;
            }
            IoEvent::GetAccountBalance => {
                self.get_accounts_balance().await;
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
            IoEvent::SubmitPreviewRequest => {
                self.preview_order_request().await;
            }
            IoEvent::GetCurrentSavedTickers(offset) => {
                self.get_current_user_saved_tickers(offset).await;
            }
            IoEvent::GetTicker(ticker_id) => {
                self.get_ticker(ticker_id).await;
            }
            IoEvent::GetNotifications => {
                self.get_notifications().await;
            }
            IoEvent::GetNotification(notification_id) => {
                self.get_notification(notification_id).await;
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

    async fn get_accounts_list(&mut self) {
        match self.etrade.accounts_list(&self.session).await {
            Ok(user_accounts) => {
                let mut app = self.app.lock().await;

                let account_len = user_accounts.accounts.accounts.len();
                app.user_accounts = Some(user_accounts.accounts.accounts);
                if account_len > 0 {
                    app.active_account_index = Some(0);
                }

                app.dispatch(IoEvent::GetAccountBalance);
            }
            Err(e) => {
                self.handle_error(anyhow!(e)).await;
            }
        }
    }

    async fn get_accounts_balance(&mut self) {
        let mut app = self.app.lock().await;
        if let Some(ref mut accounts) = app.user_accounts {
            for item in accounts.iter_mut() {
                match self.etrade.account_balance(&item.account_id_key, &self.session).await {
                    Ok(account_balance) => {
                        item.account_balance = Some(account_balance);
                    }
                    Err(e) => {
                        // self.handle_error(anyhow!(e)).await;
                    }
                }
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

    async fn get_notifications(&mut self) {
        match self.etrade.alerts(&self.session).await {
            Ok(alerts) => {
                let mut app = self.app.lock().await;

                app.notifications = Some(alerts.alerts);
                app.total_notifications = Some(alerts.total_alerts);
                app.push_navigation_stack(RouteId::Notifications, ActiveBlock::Notifications);
            }
            Err(e) => {
                self.handle_error(anyhow!(e)).await;
            }
        }
    }

    async fn get_notification(&mut self, notification_id: String) {
        match self.etrade.alert(&notification_id, &self.session).await {
            Ok(alert) => {
                let mut app = self.app.lock().await;

                app.selected_notification = Some(alert);
                app.push_navigation_stack(RouteId::NotificationDetail, ActiveBlock::NotificationDetail);
            }
            Err(e) => {
                self.handle_error(anyhow!(e)).await;
            }
        }
    }

    async fn get_portfolio(&mut self) {
        let mut app = self.app.lock().await;
        if let Some(accounts) = &app.user_accounts {
            match self.etrade.portfolio(&accounts.first().unwrap().account_id_key, &self.session).await {
                Ok(portfolio) => {
                    app.portfolio_tickers = Some(portfolio.account_portfolio.positions.into_iter().map(|t| t.into()).collect::<Vec<Ticker>>());
                }
                Err(e) => {
                    self.handle_error(anyhow!(e)).await;
                }
            }
        }
    }

    async fn preview_order_request(&mut self) {
        let mut app = self.app.lock().await;
        if let Some(active_account_index) = app.active_account_index {
            let account_id_key = &app.user_accounts.as_ref().unwrap()[active_account_index].account_id_key;
            if let Some(preview_order_form) = &app.preview_order_form {
                match self.etrade.preview_order_request(account_id_key, &self.session, preview_order_form.clone().into()).await {
                    Ok(preview_order_response) => {
                        app.preview_order_form = Some(preview_order_response.into());
                        app.push_navigation_stack(RouteId::ConfirmOrderForm, ActiveBlock::ConfirmOrderForm);
                    }
                    Err(e) => {
                        app.push_navigation_stack(RouteId::Error, ActiveBlock::Error);
                    }
                }
            }
        }
    }

    async fn refresh_authentication(&mut self) {
        let mut client_config = ClientConfig::new();
        // ask user for configuration details
        if let Ok(_) = client_config.load_config(&self.session.mode) {
            let _ = self.session.full_access_flow(client_config).await;
        }

    }
}

