//! # sparkpost
//!
//! Rust bindings for sparkpost email api v1
//! ## WARNING!
//! ### Work in Progress, Expect breaking changes

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
