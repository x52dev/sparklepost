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


#[cfg(test)]
mod tests {
    use super::*;

    fn get_api_key() -> String {
        use std::env;
        use dotenv::dotenv;
        dotenv().ok();
        let api_key = env::var("SPARKPOST_API_KEY").expect("SPARKPOST_API_KEY must be set");
        api_key
    }

    /// actually test the api
    #[ignore]
    #[test]
    fn send_email() {
        let tm = Transmission::new(get_api_key(), "https://api.eu.sparkpost.com/api/v1/transmissions".into());
        let mut email: Message =
            Message::new(
                EmailAddress::with_name(
                    "hello@email.letsorganise.app",
                    "noreply")
            );
        email.add_recipient("test@hgill.io".into())
            .set_subject("Testing builder email sandbox")
            .set_html("This is the html body of the email")
            .set_text("This is the text body of the email");

//        println!("{:#?}", &email.json().to_string());
        let result = tm.send(&email);
//        println!("{:#?}", result);
        match result {
            Ok(res) => {
//                println!("{:?}", &res);
                match res.results {
                    Some(result) => {
                        assert_eq!(1, result.total_accepted_recipients);
                        assert_eq!(0, result.total_rejected_recipients);
                    }
                    None => {
                        println!("res: \n {:#?}", &res);
                    }
                }
            }
            Err(error) => {
                println!("error \n {:#?}", error);
            }
        }
    }
}
