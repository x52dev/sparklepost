// #![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![cfg_attr(test, deny(warnings))]
#![cfg_attr(docs_rs_workaround, feature(extern_prelude))]
//! # sparkpost
//!
//! rust api for [sparkpost](https://sparkpost.com) email service

#[macro_use]
extern crate serde_derive;
extern crate chrono;
extern crate reqwest;
extern crate serde;
extern crate serde_json;

#[cfg(test)]
extern crate base64;
#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;
#[cfg(test)]
extern crate dotenv;

pub mod transmission;
