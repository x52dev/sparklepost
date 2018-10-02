//! Module contains sprakpost email sending api
//!
//! ### Example
//!  ```rust
//!  extern crate sparkpost;
//!
//! use sparkpost::transmission::{Transmission, Message, EmailAddress, TransmissionResponse};
//!
//! let tm = Transmission::new("api_key".to_string(), "https://api.eu.sparkpost.com/api/v1".to_string());
//! let mut email: Message = Message::new(
//!                              EmailAddress::new("marketing@example.sink.sparkpostmail.com", "Example Company")
//!                          );
//!
//! email.add_recipient("name@domain.com".into())
//!      .subject("My Awesome email 😎")
//!      .html("<h1>html body of the email</h1>")
//!      .text("text body of the email");
//!
//! let result = tm.send(&email);
//!
//! match result {
//!    Ok(res) => {
//!         println!("{:?}", &res);
//!         match res {
//!             TransmissionResponse::ApiResponse(api_res) => {
//!              //   assert_eq!(1, api_res.total_accepted_recipients);
//!              //   assert_eq!(0, api_res.total_rejected_recipients);
//!             }
//!             TransmissionResponse::ApiError(errors) => {
//!                 println!("res: \n {:#?}", &errors);
//!             }
//!         }
//!     }
//!     Err(error) => {
//!         println!("error \n {:#?}", error);
//!     }
//! }
//!
//! ```
use reqwest::{
    header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    Client, Error,
};

mod message;
mod recipients;

pub use self::message::*;
pub use self::recipients::*;

/// Reqwest Error
pub type ReqError = Error;

/// Transmission result returned by the API
///
#[derive(Debug, Deserialize)]
pub struct ApiResponse {
    pub total_rejected_recipients: usize,
    pub total_accepted_recipients: usize,
    pub id: String,
}

/// Transmission error returned by the API
/// #### Note
/// this is not http error
#[derive(Debug, Deserialize)]
pub struct ApiError {
    pub description: Option<String>,
    pub code: Option<String>,
    pub message: Option<String>,
}

/// Wrapper Enum for ApiResponse and ApiError
#[derive(Debug, Deserialize)]
pub enum TransmissionResponse {
    #[serde(rename = "results")]
    ApiResponse(ApiResponse),
    #[serde(rename = "errors")]
    ApiError(Vec<ApiError>),
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
        Transmission { api_key, url }
    }
    /// Send api request
    pub fn send(&self, message: &Message) -> Result<TransmissionResponse, ReqError> {
        let client = Client::new()
            .post(self.url.as_str())
            .headers(construct_headers(self.api_key.as_ref()))
            .json(message);

        client.send()?.json()
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
        use dotenv::dotenv;
        use std::env;
        dotenv().ok();
        let api_key = env::var("SPARKPOST_API_KEY").expect("SPARKPOST_API_KEY must be set");
        api_key
    }

    /// actually test the api
    #[ignore]
    #[test]
    fn send_email() {
        let tm = Transmission::new(
            get_api_key(),
            "https://api.eu.sparkpost.com/api/v1/transmissions".into(),
        );
        let mut email: Message =
            Message::new(EmailAddress::new("hello@email.letsorganise.app", "noreply"));
        email
            .add_recipient("test@hgill.io".into())
            .subject("Testing builder email sandbox")
            .html("This is the html body of the email")
            .text("This is the text body of the email");

        //        println!("{:#?}", &email.json().to_string());
        let result = tm.send(&email);
        //        println!("{:#?}", result);
        match result {
            Ok(res) => {
                println!("{:?}", &res);
                match res {
                    TransmissionResponse::ApiResponse(api_res) => {
                        assert_eq!(1, api_res.total_accepted_recipients);
                        assert_eq!(0, api_res.total_rejected_recipients);
                    }
                    TransmissionResponse::ApiError(errors) => {
                        println!("res: \n {:#?}", &errors);
                    }
                }
            }
            Err(error) => {
                println!("error \n {:#?}", error);
            }
        }
        // attach file to email
        email.add_attachment(Attachment {
            file_type: "image/png".into(),
            name: "AnImage.png".into(),
            data: "iVBORw0KGgoAAAANSUhEUgAAABAAAAAQCAYAAAAf8/9hAAAAAXNSR0IArs4c6QAAAAlwSFlzAAAWJQAAFiUBSVIk8AAAAXxJREFUOBFjvJVg84P5718WBjLAX2bmPyxMf/+xMDH8YyZDPwPDXwYGJkIaOXTNGdiUtHAqI2jA/18/GUQzGsg3gMfKg4FVQo6BiYcPqyF4XcChaczA4+DP8P//f4b/P3+SZgAzvxCDSGYjAyMjI8PvZw+AoYXdLuyiQLtE0uoZWAREwLb+fnKXQTipkngXcJu7MnACQx8G2FX1GHgs3bDGBlYX8HlFM/z9+JbhzewWhmf1CQyfti9j+PfzBwO/ZxTMTDiNmQKBfmZX1GB42V/K8P38YbDCX/dvMDAwMzPwuYbBNcIYmC4AhfjvXwx/376AqQHTf96+ZPj34xuKGIiDaQBQ8PPBTQwCoZkMjJzcYA3MgqIMAr7xDJ/3rAHzkQnGO7FWf5gZ/qLmBSZmBoHgNAZee1+Gf18/MzCyczJ83LyQ4fPetch6Gf4xMP3FbgBMGdAgJqAr/n37zABMTTBROA0ygAWUJUG5Civ4B8xwX78CpbD6FJiHmf4AAFicbTMTr5jAAAAAAElFTkSuQmCC".into(),
        }).subject("Email with attachment");

        let result = tm.send(&email);
        //        println!("{:#?}", result);
        match result {
            Ok(res) => {
                println!("{:?}", &res);
                match res {
                    TransmissionResponse::ApiResponse(api_res) => {
                        assert_eq!(1, api_res.total_accepted_recipients);
                        assert_eq!(0, api_res.total_rejected_recipients);
                    }
                    TransmissionResponse::ApiError(errors) => {
                        println!("res: \n {:#?}", &errors);
                    }
                }
            }
            Err(error) => {
                println!("error \n {:#?}", error);
            }
        }
    }
}
