use crate::stonks_error::RuntimeError;
use crate::session::{Credentials, Mode};
use crate::ui::key::Key;
use serde::{Serialize, Deserialize};
use std::{
    fs,
    io::{stdin, Write},
    path::{Path, PathBuf}
};
use tui::style::{Color};
use log::debug;
// use chrono::Utc;

const REQUEST_TOKEN_URL_SANDBOX: &str = "https://api.etrade.com/oauth/request_token";
const SANDBOX_REQUEST_TOKEN_URL: &str = "https://apisb.etrade.com/oauth/request_token";

const ACCESS_TOKEN_URL_SANDBOX: &str = "https://api.etrade.com/oauth/access_token";
const SANDBOX_ACCESS_TOKEN_URL: &str = "https://apisb.etrade.com/oauth/access_token";

const RENEW_TOKEN_URL_SANDBOX: &str = "https://api.etrade.com/oauth/renew_token";
const SANDBOX_RENEW_TOKEN_URL: &str = "https://apisb.etrade.com/oauth/renew_token";

const ACCOUNTS_LIST_URL: &str = "https://api.etrade.com/v1/accounts/list";
const SANDBOX_ACCOUNTS_LIST_URL: &str = "https://apisb.etrade.com/v1/accounts/list";

const QUOTE_URL: &str = "https://api.etrade.com/v1/market/quote";
const SANDBOX_QUOTE_URL: &str = "https://apisb.etrade.com/v1/market/quote";

const SEARCH_URL: &str = "https://api.etrade.com/v1/market/lookup";
const SANDBOX_SEARCH_URL: &str = "https://apisb.etrade.com/v1/market/lookup";

const PORTFOLIO_URL: &str = "https://api.etrade.com/v1/accounts/{}/portfolio";
const SANDBOX_PORTFOLIO_URL: &str = "https://apisb.etrade.com/v1/accounts/{}/portfolio";


// const DEFAULT_PORT: u16 = 8888;
const FILE_NAME: &str = "client.yml";
const CONFIG_DIR: &str = ".config";
const APP_CONFIG_DIR: &str = "stonks-terminal";
const TOKEN_CACHE_FILE: &str = ".stonks_terminal_token_cache.json";

#[derive(Clone)]
pub struct KeyBindings {
  pub back: Key,
  pub next_page: Key,
  pub previous_page: Key,
  pub search: Key,
  pub submit: Key,
  pub basic_view: Key,
}

#[derive(Copy, Clone, Debug)]
pub struct Theme {
  pub active: Color,
  pub banner: Color,
  pub error_border: Color,
  pub error_text: Color,
  pub hint: Color,
  pub hovered: Color,
  pub inactive: Color,
  pub selected: Color,
  pub text: Color,
  pub header: Color,
}

impl Default for Theme {
  fn default() -> Self {
    Theme {
      active: Color::Cyan,
      banner: Color::LightCyan,
      error_border: Color::Red,
      error_text: Color::LightRed,
      hint: Color::Yellow,
      hovered: Color::Magenta,
      inactive: Color::Gray,
      selected: Color::LightCyan,
      text: Color::Reset,
      header: Color::Reset,
    }
  }
}

#[derive(Clone)]
pub struct UserConfig {
    pub path_to_config: Option<PathBuf>,
    pub keys: KeyBindings,
    pub theme: Theme,
}

impl UserConfig {
    pub fn new() -> Self {
        Self {
            path_to_config: None,
            theme: Theme::default(),
            keys: KeyBindings {
                back: Key::Char('q'),
                next_page: Key::Ctrl('d'),
                previous_page: Key::Ctrl('u'),
                search: Key::Char('/'),
                submit: Key::Enter,
                basic_view: Key::Char('B'),
            },

        }
    }
}

#[derive(Debug, Clone)]
pub struct UrlConfig<'a> {
    pub request_token_url: &'a str,
    pub sandbox_request_token_url: &'a str,
    pub access_token_url: &'a str,
    pub sandbox_access_token_url: &'a str,
    pub renew_token_url: &'a str,
    pub sandbox_renew_token_url: &'a str,
    pub quote_url: &'a str,
    pub sandbox_quote_url: &'a str,
    pub search_url: &'a str,
    pub sandbox_search_url: &'a str,
    pub portfolio_url: &'a str,
    pub sandbox_portfolio_url: &'a str,
    pub accounts_list_url: &'a str,
    pub sandbox_accounts_list_url: &'a str,
}

impl<'a> Default for UrlConfig<'a> {
    fn default() -> Self {
        Self {
            request_token_url: REQUEST_TOKEN_URL_SANDBOX,
            sandbox_request_token_url: SANDBOX_REQUEST_TOKEN_URL,
            access_token_url: ACCESS_TOKEN_URL_SANDBOX,
            sandbox_access_token_url: SANDBOX_ACCESS_TOKEN_URL,
            renew_token_url: RENEW_TOKEN_URL_SANDBOX,
            sandbox_renew_token_url: SANDBOX_RENEW_TOKEN_URL,
            quote_url: QUOTE_URL,
            sandbox_quote_url: SANDBOX_QUOTE_URL,
            search_url: SEARCH_URL,
            sandbox_search_url: SANDBOX_SEARCH_URL,
            portfolio_url: PORTFOLIO_URL,
            sandbox_portfolio_url: SANDBOX_PORTFOLIO_URL,
            accounts_list_url: ACCOUNTS_LIST_URL,
            sandbox_accounts_list_url: SANDBOX_ACCOUNTS_LIST_URL,
        }
    }
}

impl<'a> UrlConfig<'a> {
    pub fn authorize_url(&self, key: &String, token: &String) -> String {
        format!(
            "https://us.etrade.com/e/t/etws/authorize?key={}&token={}",
            key,
            token,
        )
    }

    pub fn accounts_list(&self, mode: &Mode) -> String {
        let url = match mode {
            Mode::Sandbox => self.sandbox_accounts_list_url,
            Mode::Live => self.accounts_list_url,
        };

        url.to_string()
    }

    pub fn etrade_ticker_url(&self, symbol: &str, mode: &Mode) -> String {
        let url = match mode {
            Mode::Sandbox => self.sandbox_quote_url,
            Mode::Live => self.quote_url,
        };
        format!(
            "{}/{}",
            url,
            symbol,
        )
    }

    pub fn etrade_search_url(&self, search_input: &str, mode: &Mode) -> String {
        let url = match mode {
            Mode::Sandbox => self.sandbox_search_url,
            Mode::Live => self.search_url,
        };
        format!(
            "{}/{}",
            url,
            search_input,
        )
    }

    pub fn etrade_portfolio_url(&self, account_id_key: &str, mode: &Mode) -> String {
        let url = match mode {
            Mode::Sandbox => format!("https://apisb.etrade.com/v1/accounts/{}/portfolio", account_id_key),
            Mode::Live => format!("https://api.etrade.com/v1/accounts/{}/portfolio", account_id_key),
        };

        url
    }
}

#[derive(Clone, Debug)]
pub struct ConfigPaths {
    pub config_file_path: PathBuf,
    pub token_cache_path: PathBuf,
}

#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClientConfig {
    pub consumer_key: String,
    pub consumer_secret: String,
}

impl ClientConfig {
    pub fn new() -> Self {
        Self {
            consumer_key: "".to_string(),
            consumer_secret: "".to_string(),
        }
    }

    pub fn load_config(&mut self) -> Result<ConfigPaths, RuntimeError> {
        let paths = self.get_or_build_paths()?;
        if paths.config_file_path.exists() {
            debug!("Loading keys from config");

            let config_string = fs::read_to_string(&paths.config_file_path)?;
            let config_yaml: ClientConfig = serde_yaml::from_str(&config_string)?;

            self.consumer_key = config_yaml.consumer_key
                .strip_suffix("\n")
                .unwrap_or(&config_yaml.consumer_key)
                .to_string();
            self.consumer_secret = config_yaml.consumer_secret
                .strip_suffix("\n")
                .unwrap_or(&config_yaml.consumer_secret)
                .to_string();
        } else {
            debug!("Config will be saved to {}", paths.config_file_path.display());

            println!("{}", "Lets get setup!");

            let consumer_key = Self::get_key_from_input("1. Enter consumer_key")?;
            let consumer_secret = Self::get_key_from_input("2. Enter consumer_secret")?;


            let client_config = Self {
                consumer_key,
                consumer_secret,
            };

            let client_yaml = serde_yaml::to_string(&client_config)?;
            let mut new_config = fs::File::create(&paths.config_file_path)?;
            write!(new_config, "{}", client_yaml)?;

            self.consumer_key = client_config.consumer_key.trim().to_string();
            self.consumer_secret = client_config.consumer_secret.trim().to_string();
        }

        Ok(paths)
    }

    fn get_or_build_paths(&self) -> Result<ConfigPaths, RuntimeError> {
        match dirs::home_dir() {
            Some(home) => {
                let path = Path::new(&home);
                let home_config_dir = path.join(CONFIG_DIR);
                let app_config_dir = home_config_dir.join(APP_CONFIG_DIR);

                if !home_config_dir.exists() {
                    fs::create_dir(home_config_dir)?;
                }
                if !app_config_dir.exists() {
                    fs::create_dir(&app_config_dir)?;
                }

                let config_file_path = &app_config_dir.join(FILE_NAME);
                let token_cache_path = &app_config_dir.join(TOKEN_CACHE_FILE);

                let paths = ConfigPaths {
                    config_file_path: config_file_path.to_path_buf(),
                    token_cache_path: token_cache_path.to_path_buf(),
                };

                Ok(paths)
            },
            None => Err(RuntimeError { message: "No $home path for client config".to_string() })
        }
    }

    fn get_key_from_input(label: &str) -> Result<String, RuntimeError> {
        println!("{}", label);

        let mut key = String::new();
        stdin().read_line(&mut key)?;
        Ok(key.trim().to_owned())
    }
}

impl From<Credentials> for ClientConfig {
    fn from(t: Credentials) -> ClientConfig {
        Self {
            consumer_key: t.key,
            consumer_secret: t.secret,
        }
    }
}
