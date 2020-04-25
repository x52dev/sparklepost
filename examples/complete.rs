#[macro_use]
extern crate serde_derive;
extern crate chrono;
extern crate dotenv;
extern crate serde;
extern crate sparkpost;

use chrono::prelude::*;
use sparkpost::transmission::{
    Attachment, EmailAddress, Message, Options, Recipient, Transmission,
    TransmissionResponse,
};

#[derive(Debug, Serialize)]
struct Data {
    name: String,
}

#[allow(unused)]
fn get_api_key() -> String {
    use dotenv::dotenv;
    use std::env;
    dotenv().ok();
    env::var("SPARKPOST_API_KEY").expect("SPARKPOST_API_KEY must be set")
}

fn main() {
    let tm = Transmission::new_eu("get_api_key()".to_owned());

    // new email message with sender name and email
    let mut email = Message::new(EmailAddress::new(
        "marketing@example.sink.sparkpostmail.com",
        "Example Company",
    ));

    let options = Options {
        open_tracking: true,
        click_tracking: true,
        transactional: false,
        sandbox: false,
        inline_css: false,
        start_time: Some(Utc.ymd(2019, 1, 1).and_hms(0, 0, 0)),
    };

    // recipient with substitute data for the template
    let recipient = Recipient::with_substitution(
        EmailAddress::new("bob@company.com", "Bob"),
        Data { name: "Bob".into() },
    );

    let attachment = Attachment::from_data(
        "AnImage.png",
        "image/png",
        "iVBORw0KGgoAAAANSUhEUgAAABAAAAAQCAYAAAAf8/9hAAAAAXNSR0IArs4c6QAAAAlwSFlzAAAWJQAAFiUBSVIk8AAAAXxJREFUOBFjvJVg84P5718WBjLAX2bmPyxMf/+xMDH8YyZDPwPDXwYGJkIaOXTNGdiUtHAqI2jA/18/GUQzGsg3gMfKg4FVQo6BiYcPqyF4XcChaczA4+DP8P//f4b/P3+SZgAzvxCDSGYjAyMjI8PvZw+AoYXdLuyiQLtE0uoZWAREwLb+fnKXQTipkngXcJu7MnACQx8G2FX1GHgs3bDGBlYX8HlFM/z9+JbhzewWhmf1CQyfti9j+PfzBwO/ZxTMTDiNmQKBfmZX1GB42V/K8P38YbDCX/dvMDAwMzPwuYbBNcIYmC4AhfjvXwx/376AqQHTf96+ZPj34xuKGIiDaQBQ8PPBTQwCoZkMjJzcYA3MgqIMAr7xDJ/3rAHzkQnGO7FWf5gZ/qLmBSZmBoHgNAZee1+Gf18/MzCyczJ83LyQ4fPetch6Gf4xMP3FbgBMGdAgJqAr/n37zABMTTBROA0ygAWUJUG5Civ4B8xwX78CpbD6FJiHmf4AAFicbTMTr5jAAAAAAElFTkSuQmCC");

    // complete the email message with details
    email
        .add_recipient(recipient)
        .add_attachment(attachment)
        .options(options)
        .campaign_id("marketing_blitz")
        .subject("My Awesome email 😎")
        .html("<h1>hello {name}</h1>")
        .text("hello {name}");

    let result = tm.send(&email);

    match result {
        Ok(res) => {
            println!("{:?}", &res);
            match res {
                TransmissionResponse::ApiResponse(api_res) => {
                    println!("API Response: \n {:#?}", api_res);
                    //   assert_eq!(1, api_res.total_accepted_recipients);
                    //   assert_eq!(0, api_res.total_rejected_recipients);
                }
                TransmissionResponse::ApiError(errors) => {
                    println!("Response Errors: \n {:#?}", &errors);
                }
            }
        }
        Err(error) => {
            println!("error \n {:#?}", error);
        }
    }
}
