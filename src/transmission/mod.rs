//! Module contains sprakpost email sending api
//!
//! ### Example
//!  ```rust
//!  extern crate sparkpost;
//!
//! use sparkpost::transmission::{Transmission, Message, EmailAddress, TransmissionResponse};
//!
//! let tm = Transmission::new("api_key");
//! // to create for EU version use
//! let tm = Transmission::new_eu("api_key");
//! let mut email: Message = Message::new(
//!                              EmailAddress::new("marketing@company.com", "Example Company")
//!                          );
//!
//! email.add_recipient("name@domain.com")
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
    blocking::Client,
    header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    Error,
};
use std::collections::HashMap;

mod message;
mod models;

pub use self::message::*;
pub use self::models::*;

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
/// let tm = Transmission::new("api_key_form_env".to_string());
/// ```
/// for more info see [https://developers.sparkpost.com/api/transmissions/](https://developers.sparkpost.com/api/transmissions/)
#[derive(Debug)]
pub struct Transmission {
    api_key: String,
    url: String,
    client: Client,
}

impl Transmission {
    /// creates new Transmission with api key for global version
    pub fn new<T: Into<String>>(api_key: T) -> Self {
        Transmission {
            api_key: api_key.into(),
            url: "https://api.sparkpost.com/api/v1/transmissions".to_owned(),
            client: Client::new(),
        }
    }

    /// creates new Transmission with api key for EU version
    pub fn new_eu<T: Into<String>>(api_key: T) -> Self {
        Transmission {
            api_key: api_key.into(),
            url: "https://api.eu.sparkpost.com/api/v1/transmissions".to_owned(),
            client: Client::new(),
        }
    }
    /// Send api request
    pub fn send(
        &self,
        message: &Message,
    ) -> Result<TransmissionResponse, ReqError> {
        self.client
            .post(&self.url)
            .headers(self.construct_headers(None))
            .json(message)
            .send()?
            .json()
    }
    /// Retrieve a Scheduled Transmission from API
    pub fn scheduled_by_id(
        &self,
        transmission_id: &str,
    ) -> Result<TransmissionResponse, ReqError> {
        // let url = format!("{}/{}", self.url, transmission_id);
        let url = self.url.clone() + "/" + transmission_id;
        self.client
            .get(&url)
            .headers(self.construct_headers(None))
            .send()?
            .json()
    }

    /// Retrieve all Scheduled Transmissions from API
    ///
    /// Example
    /// ```rust
    /// use sparkpost::transmission::Transmission;
    /// use std::collections::HashMap;
    /// let tm = Transmission::new("api_key");
    ///
    /// let mut headers = HashMap::new();
    /// headers.insert("campaign_id", "your_campaingn_id");
    /// headers.insert("template_id", "your_template_id");
    ///
    /// // filter with headers
    /// let transmissions = tm.scheduled_transmissions(Some(&headers));
    ///
    /// // or for all transmissions
    /// let transmissions = tm.scheduled_transmissions(None);
    ///
    /// ```
    pub fn scheduled_transmissions(
        &self,
        header_map: Option<&HashMap<&'static str, &str>>,
    ) -> Result<TransmissionResponse, ReqError> {
        self.client
            .get(&self.url)
            .headers(self.construct_headers(header_map))
            .send()?
            .json()
    }

    fn construct_headers(
        &self,
        header_map: Option<&HashMap<&'static str, &str>>,
    ) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
        headers
            .insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&self.api_key).unwrap(),
        );

        if let Some(header_map) = header_map {
            for (name, value) in header_map {
                headers.insert(*name, HeaderValue::from_str(value).unwrap());
            }
        }
        headers
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_api_key() -> String {
        use dotenv::dotenv;
        use std::env;
        dotenv().ok();
        env::var("SPARKPOST_API_KEY").expect("SPARKPOST_API_KEY must be set")
    }

    /// actually test the api
    #[ignore]
    #[test]
    fn send_email() {
        let tm = Transmission::new_eu(get_api_key());
        let mut email: Message = Message::new(EmailAddress::new(
            "hello@email.letsorganise.app",
            "noreply",
        ));
        email
            .add_recipient("test@hgill.io")
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
        email.add_attachment(Attachment::from_data(
            "AnImage.png",
            "image/png",
            "iVBORw0KGgoAAAANSUhEUgAAABAAAAAQCAYAAAAf8/9hAAAAAXNSR0IArs4c6QAAAAlwSFlzAAAWJQAAFiUBSVIk8AAAAXxJREFUOBFjvJVg84P5718WBjLAX2bmPyxMf/+xMDH8YyZDPwPDXwYGJkIaOXTNGdiUtHAqI2jA/18/GUQzGsg3gMfKg4FVQo6BiYcPqyF4XcChaczA4+DP8P//f4b/P3+SZgAzvxCDSGYjAyMjI8PvZw+AoYXdLuyiQLtE0uoZWAREwLb+fnKXQTipkngXcJu7MnACQx8G2FX1GHgs3bDGBlYX8HlFM/z9+JbhzewWhmf1CQyfti9j+PfzBwO/ZxTMTDiNmQKBfmZX1GB42V/K8P38YbDCX/dvMDAwMzPwuYbBNcIYmC4AhfjvXwx/376AqQHTf96+ZPj34xuKGIiDaQBQ8PPBTQwCoZkMjJzcYA3MgqIMAr7xDJ/3rAHzkQnGO7FWf5gZ/qLmBSZmBoHgNAZee1+Gf18/MzCyczJ83LyQ4fPetch6Gf4xMP3FbgBMGdAgJqAr/n37zABMTTBROA0ygAWUJUG5Civ4B8xwX78CpbD6FJiHmf4AAFicbTMTr5jAAAAAAElFTkSuQmCC")).subject("Email with attachment");

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
