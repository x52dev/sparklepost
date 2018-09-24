#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;
#[macro_use]
extern crate serde_derive;

pub use self::message::{Message, MessageBuilder, Options};
pub use self::transmission::Transmission;

mod message;
mod transmission;

#[cfg(test)]
mod tests {
    use serde_json::Value;
    use super::*;


    fn get_api_key() -> String {
        use std::env;
        use dotenv::dotenv;
        dotenv().ok();
        let api_key = env::var("SPARKPOST_API_KEY").expect("SPARKPOST_API_KEY must be set");
        api_key
    }

    #[test]
    fn create_message() {
        let email: Value = MessageBuilder::new("test@test.com", "name").finish().json();
        assert_eq!("test@test.com", email["content"]["from"]["email"].as_str().unwrap());
        assert_eq!("name", email["content"]["from"]["name"].as_str().unwrap());
        assert_eq!("test@test.com", email["content"]["from"]["email"].as_str().unwrap());
        assert!(email["options"]["sandbox"].as_bool().unwrap());
        assert!(!email["options"]["click_tracking"].as_bool().unwrap());
        assert!(!email["options"]["open_tracking"].as_bool().unwrap());
        assert!(!email["options"]["transactional"].as_bool().unwrap());
    }

    #[ignore]
    #[test]
    fn crate_transmission() {
//        let tm = Transmission::new(api_key.as_str());
//        let email: Message =
//            MessageBuilder::new("no-reply@email.letsorganise.app", "Let's Organise App")
//                .add_recipient("tech@hgill.io", Some("Techey"))
//                .add_subject("Testing builder email")
//                .add_html("This is the html body of the email")
//                .add_text("This is the text body of the email")
//                .finish();
////        tm.send(&email);
//        let email: Message =
//            MessageBuilder::new("no-reply@email.letsorganise.app", "Let's Organise App")
//                .add_recipient("tech@hgill.io", Some("Techey"))
//                .add_subject("Testing Second email")
//                .add_html("This is the html body of the email")
//                .add_text("This is the text body of the email")
//                .finish();
////        tm.send(&email);
    }
}
