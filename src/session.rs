use crate::utils;
use crate::store::{Store};
use crate::stonks_error::RuntimeError;
use crate::config::{ClientConfig, ConfigPaths, UrlConfig};
use serde::{Deserialize, Serialize};
use chrono::prelude::*;
use http::header::{AUTHORIZATION};
use hyper::{
    client::{connect::dns::GaiResolver, HttpConnector},
    Client, Body, Method, Request, Response
};
use hyper_tls::HttpsConnector;
use log::debug;
use std::{
    fs,
    io::{stdin, Read, Write}
};
// use log::debug;

type HttpClient = Client<HttpsConnector<HttpConnector<GaiResolver>>, hyper::Body>;

// general response struct from oauth apis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Credentials {
    pub key: String,
    pub secret: String,
}

impl Credentials {
    // pub fn new(key: SecUtf8, secret: SecUtf8) -> Credentials {
    pub fn new(key: String, secret: String) -> Credentials {
        Credentials { key, secret }
    }
}

impl Into<oauth::Credentials> for Credentials {
    fn into(self) -> oauth::Credentials {
        oauth::Credentials::new(self.key, self.secret)
    }
}

// https://docs.rs/oauth-credentials/0.3.0/oauth_credentials/struct.Credentials.html
impl<T> From<oauth::Credentials<T>> for Credentials
where
  T: Into<String>,
{
  fn from(input: oauth::Credentials<T>) -> Self {
    Credentials {
      key: input.identifier.into(),
      secret: input.secret.into(),
    }
  }
}

// serde serialization format for writing and retrieving from file
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LocalCredsData {
    pub access_creds: Credentials, // we store this for all requests
    pub request_token_creds: Credentials, // we store this if user quits application and opens up and we just need to renew_access_token
    pub verification_code: String, // we store this if user quits application and opens up and we just need to renew_access_token
    pub expires_at: DateTime<Utc>,
    pub last_request_timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub enum Mode {
    Sandbox,
    Live,
}

#[derive(Debug, Clone)]
pub struct Session<T> {
    pub mode: Mode,
    pub urls: UrlConfig<'static>,
    client: HttpClient,
    pub store: T,
    pub config_paths: ConfigPaths,
}

impl<T> Session<T>
where T: Store
{
    pub fn new(mode: Mode, store: T, config_paths: ConfigPaths) -> Self {
        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, hyper::Body>(https);
        Self {
            mode,
            urls: UrlConfig::default(),
            client,
            store,
            config_paths,
        }
    }

    pub fn get_creds_from_cache(&mut self) -> Option<LocalCredsData> {
        if let Ok(mut file) = fs::File::open(&self.config_paths.token_cache_path) {
            let mut local_data_str = String::new();
            if file.read_to_string(&mut local_data_str).is_ok() {
                if let Ok(local_data) = serde_json::from_str::<LocalCredsData>(&local_data_str) {
                    return Some(local_data);
                }
            }
        }

        None
    }

    pub fn expired_access_token(&mut self, local_data: &LocalCredsData) -> bool {
        return utils::now_eastern() > local_data.expires_at;
    }

    pub fn should_renew_access_token(&mut self) -> bool {
        if let Some(local_data) = self.get_creds_from_cache() {
            let now = utils::now_plus_hours(-2);
            return now > local_data.last_request_timestamp;
        }

        true
    }

    pub fn hydrate_local_store(&mut self, client_config: ClientConfig) {
        // rehydrate local store
        if let Some(local_data) = self.get_creds_from_cache() {
            self.store.put(client_config.consumer_key.to_owned(), local_data.access_creds.clone());
            self.store.put(client_config.consumer_key.to_owned() + &"request_token".to_string(), local_data.request_token_creds.clone());
            self.store.set_verification_code(local_data.verification_code.to_owned());
        }
    }

    pub async fn full_access_flow(&mut self, client_config: ClientConfig) -> Result<(), RuntimeError> {
        let creds = Credentials::new(client_config.consumer_key.to_string(), client_config.consumer_secret.to_string());
        let request_token_creds = self.request_token(&creds).await;

        // 2. obtain verification code
        // lives for 5 minutes
        // https://apisb.etrade.com/docs/api/authorization/authorize.html
        if request_token_creds.is_err() {
            return Err(RuntimeError { message: "request_token failed".to_string() });
        }

        let request_token_creds = request_token_creds.unwrap();

        let verification_code = self.verification_code(&creds, &request_token_creds)?;
        self.store.set_verification_code(verification_code.to_owned());

        // 3. make request for authorization token
        // expires at midnight Eastern Time
        // These should be used and passed in the header of subsequent requests for tickers
        // https://apisb.etrade.com/docs/api/authorization/get_access_token.html
        let uri = match self.mode {
            Mode::Sandbox => self.urls.sandbox_access_token_url,
            Mode::Live => self.urls.access_token_url,
        };
        let oauth_access_creds = self.access_token(uri, &creds, &request_token_creds, &verification_code).await;
        let oauth_access_creds = oauth_access_creds.unwrap();

        // finished oauth process
        self.save_creds_to_file(&request_token_creds, oauth_access_creds)?;

        Ok(())
    }

    // only valid for 5 minutes
    // https://apisb.etrade.com/docs/api/authorization/request_token.html
    pub async fn request_token(&self, consumer: &Credentials) -> Result<Credentials, RuntimeError> {
        let uri = match self.mode {
            Mode::Sandbox => self.urls.sandbox_request_token_url,
            Mode::Live => self.urls.request_token_url,
        };
        let authorization_header = oauth::Builder::<_, _>::new(consumer.clone().into(), oauth::HmacSha1)
            .callback("oob")
            .get(&uri, &());

        let body = self.send_request_for_auth(uri, authorization_header).await;
        let creds: oauth_credentials::Credentials<Box<str>> = serde_urlencoded::from_bytes(&body)?;
        let request_token_creds = creds.into();

        Ok(request_token_creds)
    }

    pub fn verification_code(&self, consumer: &Credentials, request_token: &Credentials) -> Result<String, RuntimeError> {
        let url = self.urls.authorize_url(&consumer.key, &request_token.key);
        let verification_code = self.verify_code(url)?;
        Ok(verification_code)
    }

    // https://apisb.etrade.com/docs/api/authorization/authorize.html
    pub async fn access_token(&self, uri: &str, consumer: &Credentials, request_token_creds: &Credentials, verification_code: &String) -> Result<Credentials, RuntimeError> {
        let authorization_header = oauth::Builder::<_, _>::new(consumer.clone().into(), oauth::HmacSha1)
            .token(Some(request_token_creds.clone().into()))
            .verifier(Some(verification_code.as_ref()))
            .get(&uri, &());

        let body = self.send_request_for_auth(uri, authorization_header).await;
        let creds: oauth_credentials::Credentials<Box<str>> = serde_urlencoded::from_bytes(&body)?;
        let oauth_access_creds = creds.into();

        Ok(oauth_access_creds)
    }

    pub async fn send_request(&self, uri: &str, authorization: String) -> Result<Response<Body>, hyper::Error> {
        let req = Request::builder()
            .method(Method::GET)
            .uri(uri)
            .header(AUTHORIZATION, authorization)
            .body(Body::empty());

        let req = self.client.request(req.unwrap());
        let resp = req.await;
        resp

        // let client = reqwest::Client::builder().build()?;
        // let req = client.get(uri).header(AUTHORIZATION, authorization);
        // let resp = req.send().await?;
    }

    pub async fn send_request_for_auth(&self, uri: &str, authorization: String) -> Vec<u8> {
        let resp = self.send_request(uri, authorization).await.unwrap();

        if resp.status().as_u16() / 100 == 2 {
            let bd = resp.into_body();
            hyper::body::to_bytes(bd).await.unwrap().to_vec()
        } else {
            // soft fail
            println!("error {:?}", resp);
            vec![]
        }
    }

    pub async fn send_post_request(&self, uri: &str, authorization: String, body: String) -> Result<Response<Body>, hyper::Error> {
        // std::fs::write("req-post.txt", format!("{} \n {} \n {}", body.to_string(), &uri, authorization));
        let req = Request::builder()
            .method(Method::POST)
            .uri(uri)
            .header(AUTHORIZATION, authorization)
            .body(Body::from(body));

        let req = self.client.request(req.unwrap());
        let resp = req.await;
        resp
    }

    pub async fn renew_access_token(&mut self, client_config: ClientConfig, local_data: LocalCredsData) -> Result<(), RuntimeError> {
        let creds = Credentials::new(client_config.consumer_key.to_string(), client_config.consumer_secret.to_string());

        let uri = match self.mode {
            Mode::Sandbox => self.urls.sandbox_renew_token_url,
            Mode::Live => self.urls.renew_token_url,
        };
        let oauth_access_creds = self.access_token(uri, &creds, &local_data.request_token_creds, &local_data.verification_code).await;
        if oauth_access_creds.is_ok() {
            let oauth_access_creds = oauth_access_creds.unwrap();

            self.save_creds_to_file(&local_data.request_token_creds, oauth_access_creds)?;
        } else {
            self.full_access_flow(client_config).await?;
        }

        Ok(())
    }

    fn verify_code(&self, url: String) -> Result<String, RuntimeError> {
        let msg = format!("Please visit and accept the license. \n{}\ninput verification code:\n", url,);
        std::io::stderr().write_all(msg.as_bytes())?;

        let mut key = String::new();
        stdin().read_line(&mut key)?;

        let result = key.trim().to_owned();
        Ok(result)
    }

    fn save_creds_to_file(&self, request_token_creds: &Credentials, oauth_access_creds: Credentials) -> Result<(), RuntimeError> {
        // write access creds to file
        let mut file = fs::OpenOptions::new().write(true).create(true).open(&self.config_paths.token_cache_path)?;
        // shrink file
        file.set_len(0)?;
        let data = LocalCredsData {
            request_token_creds: request_token_creds.clone(),
            access_creds: oauth_access_creds.clone(),
            verification_code: self.store.get_verification_code(),
            expires_at: utils::midnight_eastern(1),
            last_request_timestamp: utils::now_eastern(),
        };
        let access_creds = serde_json::to_string::<LocalCredsData>(&data)?;
        file.write_all(access_creds.as_bytes())?;

        debug!("OAuth saved to in memory store: oauth access key {}", &oauth_access_creds.key);

        Ok(())
    }
}
