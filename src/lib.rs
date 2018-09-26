//! # sparkpost
//!
//! Rust bindings for sparkpost email api v1

#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;
#[cfg(test)]
extern crate dotenv;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate reqwest;

pub use self::message::{EmailAddress, Message, Options};
pub use self::transmission::{Transmission, TransmissionResponse, TransmissionApiResult, TransmissionApiError};

mod message;
mod transmission;

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
