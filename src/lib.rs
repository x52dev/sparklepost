//! # sparkpost
//!
//! Rust bindings for sparkpost email api v1
//! ## WARNING!
//! ### Work in Progress, Expect breaking changes
//! ### Example
//!  ```rust
//!  extern crate sparkpost;
//!
//! use sparkpost::transmission::{Transmission, Message, EmailAddress, TransmissionResponse};
//! 
//! let tm = Transmission::new("api_key".to_string(), "https://api.eu.sparkpost.com/api/v1".to_string());
//! let mut email: Message = Message::new(
//!                              EmailAddress::with_name("marketing@example.sink.sparkpostmail.com", "Example Company")
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
//!}
//!
//! ```

extern crate reqwest;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;
extern crate serde;

#[cfg(test)]
extern crate base64;
#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;
#[cfg(test)]
extern crate dotenv;

pub mod transmission;
