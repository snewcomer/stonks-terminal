use crate::stonks_error::RuntimeError;
use std::{
    io::{stdin},
    path::{Path, PathBuf}
};
// use chrono::Utc;

const REQUEST_TOKEN_URL_SANDBOX: &str = "https://api.etrade.com/oauth/request_token";
// const REQUEST_TOKEN_URL: &str = "https://apisb.etrade.com/oauth/request_token";

// const DEFAULT_PORT: u16 = 8888;
const FILE_NAME: &str = "client.yml";
const CONFIG_DIR: &str = ".config";
const APP_CONFIG_DIR: &str = "stonks-terminal";
const TOKEN_CACHE_FILE: &str = ".stonks_term_token_cache.json";

pub struct UrlConfig<'a> {
    pub request_token_url: &'a str,
}

impl<'a> Default for UrlConfig<'a> {
    fn default() -> Self {
        Self {
            request_token_url: REQUEST_TOKEN_URL_SANDBOX,
        }
    }
}

struct ConfigPaths {
    config_file_path: PathBuf,
    token_cache_path: PathBuf,
}

#[derive(Debug)]
pub struct ClientConfig {
    pub consumer_key: String,
    pub consumer_secret: String,
    pub verification_code: String,
}

impl ClientConfig {
    pub fn new() -> Self {
        Self {
            consumer_key: "".to_string(),
            consumer_secret: "".to_string(),
            verification_code: "".to_string(),
        }
    }

    pub fn load_config(&mut self) -> Result<(), RuntimeError> {
        let paths = self.get_or_build_paths()?;

        println!(
            "Config will be saved to {}",
            paths.config_file_path.display()
        );

        println!("{}", "Setup instructions");

        println!("1. {}", "Enter consumer key.");
        let mut consumer_key = String::new();
        stdin().read_line(&mut consumer_key)?;
        self.consumer_key = consumer_key.trim().to_string();

        println!("2. {}", "Enter consumer secret.");
        let mut consumer_secret = String::new();
        stdin().read_line(&mut consumer_secret)?;
        self.consumer_secret = consumer_secret.trim().to_string();

        // println!("3. {}", "Manually copy the verification code to your clipboard and paste here: ");
        // let mut verification_code = String::new();
        // stdin().read_line(&mut verification_code)?;
        // self.verification_code = verification_code.trim().to_string();

        // timestamp: Utc::today().with_timezone(&chrono_tz::US::Eastern).naive_local(),

        Ok(())
    }

    fn get_or_build_paths(&self) -> Result<ConfigPaths, RuntimeError> {
        match dirs::home_dir() {
            Some(home) => {
                let path = Path::new(&home);
                let home_config_dir = path.join(CONFIG_DIR);
                let app_config_dir = home_config_dir.join(APP_CONFIG_DIR);

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
}
