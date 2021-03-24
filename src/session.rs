use crate::stonks_error::RuntimeError;
use crate::config::UrlConfig;
// use secstr::SecUtf8;
// use serde::ser::Serialize;
use http::header::{AUTHORIZATION};
use hyper::{
    client::{connect::dns::GaiResolver, HttpConnector},
    Client, Body, Method, Request
};
use hyper_tls::HttpsConnector;
use std::io::{stdin, Write};

type HttpClient = Client<HttpsConnector<HttpConnector<GaiResolver>>, hyper::Body>;

// general response struct from oauth apis
#[derive(Debug, Clone)]
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

#[derive(Debug)]
pub enum Mode {
    Sandbox,
    Live,
}

pub struct Session {
    mode: Mode,
    urls: UrlConfig<'static>,
    client: HttpClient,
}

impl Session {
    pub fn new(mode: Mode) -> Self {
        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, hyper::Body>(https);
        Self {
            mode,
            urls: UrlConfig::default(),
            client,
        }
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

        let body = self.send_request(uri, authorization_header).await;
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
    pub async fn access_token(&self, consumer: &Credentials, request_token: &Credentials, verification_code: &String) -> Result<Credentials, RuntimeError> {
        let uri = match self.mode {
            Mode::Sandbox => self.urls.sandbox_access_token_url,
            Mode::Live => self.urls.access_token_url,
        };
        let authorization_header = oauth::Builder::<_, _>::new(consumer.clone().into(), oauth::HmacSha1)
            .token(Some(request_token.clone().into()))
            .verifier(Some(verification_code.as_ref()))
            .get(&uri, &());

        let body = self.send_request(uri, authorization_header).await;
        let creds: oauth_credentials::Credentials<Box<str>> = serde_urlencoded::from_bytes(&body)?;
        let oauth_token_creds = creds.into();

        Ok(oauth_token_creds)
    }

    async fn send_request(&self, uri: &str, authorization: String) -> Vec<u8> {
        let req = Request::builder()
            .method(Method::GET)
            .uri(uri)
            .header(AUTHORIZATION, authorization)
            .body(Body::empty());

        let req = self.client.request(req.unwrap());
        let resp = req.await.unwrap();

        if resp.status().as_u16() / 100 == 2 {
            hyper::body::to_bytes(resp.into_body()).await.unwrap().to_vec()
        } else {
            vec![]
        }

        // let client = reqwest::Client::builder().build()?;
        // let req = client.get(uri).header(AUTHORIZATION, authorization);
        // let resp = req.send().await?;
        // dbg!(resp);
    }

    fn verify_code(&self, url: String) -> Result<String, RuntimeError> {
        let msg = format!("Please visit and accept the license. {}\ninput verification code:\n", url,);
        std::io::stderr().write_all(msg.as_bytes())?;

        let mut key = String::new();
        stdin().read_line(&mut key)?;

        let result = key.trim().to_owned();
        eprintln!("{}", result);
        Ok(result)
    }
}
