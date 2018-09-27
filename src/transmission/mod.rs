use reqwest::{
    Client,
    Error as ReqError, header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE, HeaderMap, HeaderValue},
};

mod message;

pub use self::message::{EmailAddress, Message, Options};


/// Transmission result returned by the API
///
#[derive(Debug, Deserialize, PartialEq)]
pub struct TransmissionApiResult {
    pub total_rejected_recipients: usize,
    pub total_accepted_recipients: usize,
    pub id: String,
}

/// Transmission error returned by the API
/// #### Note
/// this is not http error
#[derive(Debug, Deserialize, PartialEq)]
pub struct TransmissionApiError {
    pub description: Option<String>,
    pub code: Option<String>,
    pub message: Option<String>,
}

/// Wrapper struct for TransmissionApiResult and TransmissionApiError
#[derive(Debug, Deserialize, PartialEq, Default)]
pub struct TransmissionResponse {
    pub results: Option<TransmissionApiResult>,
    pub errors: Option<Vec<TransmissionApiError>>,
}

/// Sparkpost Transmission
/// currently only supports sending email message
/// ```rust
/// use sparkpost::transmission::Transmission;
/// let tm = Transmission::new("api_key_form_env".to_string(),
///                            "https://api.eu.sparkpost.com/api/v1/transmissions".into());
/// ```
/// for more info see [https://developers.sparkpost.com/api/transmissions/](https://developers.sparkpost.com/api/transmissions/)
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
    /// Send api request
    pub fn send(&self, message: &Message) -> Result<TransmissionResponse, ReqError> {
        let client = Client::new()
            .post(self.url.as_str())
            .headers(construct_headers(self.api_key.as_str()))
            .json(message);

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
