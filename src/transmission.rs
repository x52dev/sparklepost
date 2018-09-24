pub use crate::message::Message;
use reqwest::{
    Client,
    Error, header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE, HeaderMap, HeaderValue},
};
use serde_json::Value;

#[derive(Debug)]
pub struct Transmission {
    api_key: String,
    url: &'static str,
}

impl Transmission {
    pub fn new(api_key: &str) -> Self {
        Transmission {
            api_key: api_key.into(),
            url: "https://api.eu.sparkpost.com/api/v1/transmissions/",
        }
    }
    pub fn send(&self, message: &Message) -> Result<Value, Error> {
        let client = Client::new()
            .post(self.url)
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
