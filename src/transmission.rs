pub use message::Message;
use reqwest::{
    Client,
    Error, header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE, HeaderMap, HeaderValue},
};
use serde_json::Value;


/// sparkpost Transmission
/// currently only supports sending email message
/// ```rust
/// use sparkpost::Transmission;
/// let tm = Transmission::new("api_key_form_env".into(),
///                            "https://api.eu.sparkpost.com/api/v1".into());
/// ```
#[derive(Debug)]
pub struct Transmission {
    api_key: String,
    url: String,
}

impl Transmission {
    /// creates new Transmission with api key and Api url
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
