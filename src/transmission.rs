pub use crate::message::Message;
use reqwest::{
    Client,
    Error, header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE, HeaderMap, HeaderValue},
};
use serde_json::Value;

pub enum ApiRegion {
    US,
    EU,
}

#[derive(Debug)]
pub struct Transmission {
    api_key: String,
    url: String,
}

impl Transmission {
    pub fn new(api_key: String, url: String) -> Self {
        Transmission {
            api_key,
            url,
        }
    }
    pub fn send(&self, message: &Message) -> Result<Value, Error> {
        let client = Client::new()
            .post(self.url.as_str())
            .headers(construct_headers(self.api_key.as_str()))
            .json(&message.json());

        client.send()?
            .json()
    }
}

fn construct_headers(api_key: &str) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(AUTHORIZATION, HeaderValue::from_str(api_key).unwrap());
    headers
}
