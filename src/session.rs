use crate::stonks_error::RuntimeError;
use crate::config::UrlConfig;
use reqwest::header::{AUTHORIZATION};
// use secstr::SecUtf8;
// use serde::ser::Serialize;
// use hyper::{Body, Client, Method, Request, Uri};
// use hyper_tls::HttpsConnector;

#[derive(Debug, Clone)]
pub struct Credentials {
    pub key: String,
    pub secret: String,
    // key: SecUtf8,
    // secret: SecUtf8,
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

pub struct Session {
    urls: UrlConfig<'static>
}

impl Session {
    pub fn new() -> Self {
        Self {
            urls: UrlConfig::default(),
        }
    }

    pub async fn request_token(&self, consumer: &Credentials) -> Result<(), RuntimeError> {
        // let uri = http::Uri::from_static(self.urls.request_token_url);
        let uri = self.urls.request_token_url;
        let authorization_header = oauth::Builder::<_, _>::new(consumer.clone().into(), oauth::HmacSha1)
            .callback("oob")
            .get(&uri, &());

        let res = send_request(uri, authorization_header).await?;

        Ok(())
    }
}

async fn send_request(uri: &str, authorization: String) -> Result<(), RuntimeError> {
    // let req = Request::builder()
    //     .method(Method::GET)
    //     .uri(uri)
    //     .header("authorization", authorization)
    //     .body(Body::empty());

    // let https = HttpsConnector::new();
    // let client = Client::builder().build::<_, hyper::Body>(https);
    // let req = client.request(req.unwrap());
    // let resp = req.await?;
    // dbg!(resp);

    let client = reqwest::Client::builder().build()?;
    let req = client.get(uri).header(AUTHORIZATION, authorization);
    let resp = req.send().await?;
    dbg!(resp);

    Ok(())
}
