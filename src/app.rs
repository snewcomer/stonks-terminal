use crate::clients::etrade_xml_structs::{Account, AlertDetails, Position, TickerSearchData, TickerXML, Alert};
use crate::clients::etrade_json_structs::{Instrument, Order, OrderType, OrderAction, EtradePreviewOrderRequest, PreviewOrderRequest, PreviewOrderResponse, Product};
use crate::config::UserConfig;
use crate::network::IoEvent;
use crate::utils;
use std::sync::mpsc::Sender;
use std::{
    collections::HashSet,
};
use chrono::prelude::*;
use serde::{Serialize, Deserialize};
use tui::layout::Rect;
use std::str::FromStr;

pub const MAJOR_INDICES: [&str; 3] = [
    "Nasdaq",
    "DJIA",
    "S&P",
];

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum DialogContext {
  TickerDetail,
}

#[derive(Debug)]
pub struct Route {
    pub id: RouteId,
    pub active_block: ActiveBlock,
    pub hovered_block: ActiveBlock,
}

const DEFAULT_ROUTE: Route = Route {
    id: RouteId::Home,
    active_block: ActiveBlock::Empty,
    hovered_block: ActiveBlock::WatchList,
};

#[derive(Clone, PartialEq, Debug)]
pub enum RouteId {
    Analysis,
    Error,
    Home,
    RecentlySearched,
    Notifications,
    NotificationDetail,
    Search,
    TickerDetail,
    OrderForm,
    ConfirmOrderForm,
    AccountList,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ActiveBlock {
    Analysis,
    ConfirmOrderForm,
    Dialog(DialogContext),
    Empty,
    Error,
    HelpMenu,
    Home,
    Input,
    Notifications,
    NotificationDetail,
    WatchList,
    Portfolio,
    AccountList,
    OrderForm,
    RecentlySearched,
    SearchResults,
    TickerDetail,
}

#[derive(Clone)]
pub struct WatchList {
    pub selected_index: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Ticker {
    pub symbol: String,
    pub description: String,
    pub date_time: String,
    pub security_type: String,
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

impl Default for Ticker {
    fn default() -> Self {
        Self {
            symbol: "".to_string(),
            description: "".to_string(),
            date_time: "".to_string(),
            security_type: "Equity".to_string(),
            primary_exchange: "".to_string(),
            declared_dividend: "".to_string(),
            dividend: "".to_string(),
            ex_dividend_date: 0,
            bid: "".to_string(),
            ask: "".to_string(),
            open: "".to_string(),
            high52: "".to_string(),
            week52_hi_date: 0,
            low52: "".to_string(),
            week52_low_date: 0,
            eps: "".to_string(),
            pe: "".to_string(),
            beta: "".to_string(),
        }
    }
}

impl From<TickerSearchData> for Ticker {
    fn from(t: TickerSearchData) -> Ticker {
        Ticker {
            symbol: t.symbol,
            description: t.description,
            ..Default::default()
        }
    }
}

impl From<Position> for Ticker {
    fn from(t: Position) -> Ticker {
        Ticker {
            symbol: t.product.symbol,
            description: t.symbol_description,
            ..Default::default()
        }
    }
}

#[derive(Clone, Debug)]
pub struct PreviewOrder {
    pub account_id: String,
    pub quantity: String,
    pub symbol: String,
    pub order_type: OrderType,
    pub order_action: OrderAction,
}

// impl From<PreviewOrderRequest> for PreviewOrder {
//     fn from(t: PreviewOrderRequest) -> PreviewOrder {
//         PreviewOrder {
//             account_id: t.order[0].account_id.to_owned(),
//             quantity: t.order[0].instrument[0].quantity.to_owned(),
//             symbol: t.order[0].instrument[0].product.symbol.to_owned(),
//             order_type: OrderType::from_str(&t.order_type).unwrap(),
//             order_action: OrderAction::from_str(&t.order[0].instrument[0].order_action.to_owned()),
//         }
//     }
// }

impl From<PreviewOrderResponse> for PreviewOrder {
    fn from(t: PreviewOrderResponse) -> PreviewOrder {
        PreviewOrder {
            account_id: "".to_string(),// t.order[0].account_id.to_owned(),
            quantity: t.order[0].instrument[0].quantity.to_owned(),
            symbol: t.order[0].instrument[0].product.symbol.to_owned(),
            order_type: OrderType::from_str(&t.order_type).unwrap(),
            order_action: OrderAction::from_str(&t.order[0].instrument[0].order_action.to_owned()).unwrap(),
        }
    }
}

impl From<PreviewOrder> for PreviewOrderRequest {
    fn from(t: PreviewOrder) -> PreviewOrderRequest {
        PreviewOrderRequest {
            preview_order_request: EtradePreviewOrderRequest {
                order_type: t.order_type.to_string(),
                client_order_id: utils::simple_id(),
                order: vec![Order {
                    // account_id: "".to_string(),
                    // all_or_none: false,
                    price_type: "MARKET".to_string(),
                    order_term: "GOOD_FOR_DAY".to_string(),
                    market_session: "REGULAR".to_string(),
                    // stop_price: "".to_string(),
                    // limit_price: "".to_string(),
                    instrument: vec![
                        Instrument {
                            quantity: t.quantity,
                            quantity_type: "QUANTITY".to_string(),
                            order_action: t.order_action.to_string(),
                            product: Product {
                                symbol: t.symbol,
                                security_type: "EQ".to_string(),
                            },

                            // cancel_quantity: None,
                            // reserve_order: None,
                            // symbol_description: None,
                        }
                    ],
                    ..Default::default()
                }]
            }
        }
    }
}

#[derive(Debug)]
pub enum OrderFormState {
    Initial,
    Quantity,
    Submit,
}

#[derive(Clone, Debug)]
pub struct OptionChain {}

pub enum SearchType {
    Ticker,
    OptionChain, // not Option b/c language keyword
}

#[derive(Debug)]
pub struct SearchResult {
    pub tickers: Option<Vec<Ticker>>,
    pub option_chains: Option<Vec<OptionChain>>,
    pub selected_ticker_index: Option<usize>,
}

impl SearchResult {
    pub fn tickers(tickers: Vec<Ticker>) -> Self {
        Self {
            tickers: Some(tickers),
            option_chains: None,
            selected_ticker_index: Some(0),
        }
    }
}

// #[derive(Clone, Debug, Serialize, Deserialize)]
// pub struct Notification {
//     id: String,
//     create_time: String,
//     subject: String,
//     msg_text: String,
//     read_time: String,
//     delete_time: String,
//     symbol: String,
//     next: String,
//     prev: String,
// }

#[derive(Debug, Clone)]
pub struct SelectedTicker {
    pub ticker: Ticker,
    pub selected_index: usize,
}

impl From<TickerXML> for SelectedTicker {
    fn from(t: TickerXML) -> SelectedTicker {
        let ticker = Ticker {
            symbol: t.quote_data.product.symbol,
            date_time: t.quote_data.date_time,
            security_type: t.quote_data.product.security_type,
            primary_exchange: t.quote_data.info.primary_exchange,
            declared_dividend: t.quote_data.info.declared_dividend,
            dividend: t.quote_data.info.dividend,
            ex_dividend_date: t.quote_data.info.ex_dividend_date,
            bid: t.quote_data.info.bid,
            ask: t.quote_data.info.ask,
            open: t.quote_data.info.open,
            high52: t.quote_data.info.high52,
            week52_hi_date: t.quote_data.info.week52_hi_date,
            low52: t.quote_data.info.low52,
            week52_low_date: t.quote_data.info.week52_low_date,
            pe: t.quote_data.info.pe,
            eps: t.quote_data.info.eps,
            beta: t.quote_data.info.beta,
            ..Default::default()
        };

        SelectedTicker {
            ticker,
            selected_index: 0,
        }
    }
}

#[derive(Clone)]
pub struct SelectedOptionChain {
    pub option_chain: OptionChain,
    pub selected_index: usize,
}

// Watch Lists
#[derive(Clone)]
pub struct Library {
    pub selected_index: usize,
    pub saved_tickers: Vec<Ticker>,
    pub saved_option_chains: Vec<OptionChain>,
}

pub struct User {}

pub struct App {
    pub user_config: UserConfig,
    pub major_indices: WatchList,
    navigation_stack: Vec<Route>,
    pub api_error: String,
    // Inputs:
    // input is the string for input;
    // input_idx is the index of the cursor in terms of character;
    // input_cursor_position is the sum of the width of characters preceding the cursor.
    // Reason for this complication is due to non-ASCII characters, they may
    // take more than 1 bytes to store and more than 1 character width to display.
    pub input: Vec<char>,
    pub input_idx: usize,
    pub input_cursor_position: u16,
    pub liked_ticker_ids_set: HashSet<String>,
    pub saved_ticker_ids_set: HashSet<String>,

    pub active_ticker_index: Option<usize>,
    pub selected_watch_list_index: Option<usize>,

    pub selected_notification: Option<AlertDetails>,
    pub selected_notification_index: usize,

    pub selected_ticker: Option<SelectedTicker>,

    pub user_accounts: Option<Vec<Account>>,
    pub selected_account_index: Option<usize>,
    pub active_account_index: Option<usize>,

    pub notifications: Option<Vec<Alert>>,
    pub total_notifications: Option<u32>,

    pub library: Library,
    pub portfolio_tickers: Option<Vec<Ticker>>,

    pub preview_order_form: Option<PreviewOrder>,
    pub preview_order_ticker: Option<String>,
    pub order_form_state: OrderFormState,

    pub large_search_limit: u32,
    pub search_results: SearchResult,
    pub recently_searched: Vec<SearchResult>,
    pub search_term: String,
    pub size: Rect,
    pub small_search_limit: u32,
    pub user: Option<User>,
    pub help_docs_size: u32,
    pub help_menu_page: u32,
    pub help_menu_max_lines: u32,
    pub help_menu_offset: u32,
    pub is_loading: bool,
    io_tx: Option<Sender<IoEvent>>,
    pub etrade_token_expiry: DateTime<Utc>,
    pub dialog: Option<String>,
    pub confirm: bool,

}

impl Default for App {
    fn default() -> Self {
        App {
            user_config: UserConfig::new(),
            recently_searched: Default::default(),
            size: Rect::default(),
            major_indices: WatchList {
                selected_index: 0,
            },

            library: Library {
                selected_index: 0,
                saved_tickers: vec![],
                saved_option_chains: vec![],
            },

            portfolio_tickers: None,

            preview_order_form: None,
            preview_order_ticker: None,
            order_form_state: OrderFormState::Initial,

            selected_notification: None,
            selected_notification_index: 0,

            selected_ticker: None,

            active_ticker_index: None,
            selected_watch_list_index: None,

            user_accounts: None,
            active_account_index: None,
            selected_account_index: None,

            notifications: None,
            total_notifications: None,

            navigation_stack: vec![DEFAULT_ROUTE],
            large_search_limit: 20,
            small_search_limit: 4,
            api_error: String::new(),
            input: vec![],
            input_idx: 0,
            input_cursor_position: 0,
            liked_ticker_ids_set: HashSet::new(),
            saved_ticker_ids_set: HashSet::new(),
            search_term: "".to_string(),
            search_results: SearchResult {
                selected_ticker_index: None,
                tickers: None,
                option_chains: None,
            },
            user: None,
            help_docs_size: 0,
            help_menu_page: 0,
            help_menu_max_lines: 0,
            help_menu_offset: 0,
            is_loading: false,
            io_tx: None,
            etrade_token_expiry: utils::now_eastern(),
            dialog: None,
            confirm: false,
        }
    }
}


impl App {
    pub fn new(
        io_tx: Sender<IoEvent>,
        user_config: UserConfig,
        etrade_token_expiry: DateTime<Utc>,
    ) -> Self {
        Self {
            io_tx: Some(io_tx),
            user_config,
            etrade_token_expiry,
            ..App::default()
        }
    }

    pub fn new_preview_order(&mut self, order_type: OrderType, order_action: OrderAction) {
        if let Some(active_account_index) = self.active_account_index {
            let account_id = &self.user_accounts.as_ref().unwrap()[active_account_index].account_id;
            self.preview_order_form = Some(PreviewOrder {
                account_id: account_id.to_string(),
                order_type,
                order_action,
                symbol: "".to_string(),
                quantity: "".to_string(),
            })
        }
    }

    pub fn cancel_preview_order(&mut self) {
        // TODO: confirm modal
        self.preview_order_form = None;
    }

    pub fn add_next_order_field(&mut self, key: &str, value: String) {
        match key {
            "symbol" => {
                if let Some(ref mut order_form) = self.preview_order_form {
                    order_form.symbol = value;
                }
            }
            "quantity" => {
                if let Some(ref mut order_form) = self.preview_order_form {
                    order_form.quantity = value;
                }
            }
            _ => {},
        }
    }

    // Send a network event to the network thread
    pub fn dispatch(&mut self, action: IoEvent) {
        // `is_loading` will be set to false again after the async action has finished in network.rs
        self.is_loading = true;
        if let Some(io_tx) = &self.io_tx {
            if let Err(e) = io_tx.send(action) {
                self.is_loading = false;
                println!("Error from dispatch {}", e);
                // TODO: handle error
            };
        }
    }

    // The navigation_stack actually only controls the large block to the right of `library` and
    // `playlists`
    pub fn push_navigation_stack(&mut self, next_route_id: RouteId, next_active_block: ActiveBlock) {
        self.navigation_stack.push(Route {
            id: next_route_id,
            active_block: next_active_block,
            hovered_block: next_active_block,
        });
    }


    pub fn get_current_route(&self) -> &Route {
        // if for some reason there is no route return the default
        self.navigation_stack.last().unwrap_or(&DEFAULT_ROUTE)
    }

    fn get_current_route_mut(&mut self) -> &mut Route {
        self.navigation_stack.last_mut().unwrap()
    }

    pub fn set_current_route_state(
        &mut self,
        active_block: Option<ActiveBlock>,
        hovered_block: Option<ActiveBlock>,
    ) {
        let mut current_route = self.get_current_route_mut();
        if let Some(active_block) = active_block {
            current_route.active_block = active_block;
        }
        if let Some(hovered_block) = hovered_block {
            current_route.hovered_block = hovered_block;
        }
    }

    pub fn pop_navigation_stack(&mut self) -> Option<Route> {
        if self.navigation_stack.len() == 1 {
          None
        } else {
          self.navigation_stack.pop()
        }
    }

    pub fn handle_error(&mut self, e: anyhow::Error) {
        self.push_navigation_stack(RouteId::Error, ActiveBlock::Error);
        self.api_error = e.to_string();
    }
}
