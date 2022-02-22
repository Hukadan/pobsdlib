//! # pobsdlib
//! A library to in interact with the PlayOnBSD database
//! The database can be found at `https://github.com/playonbsd/OpenBSD-Games-Database`
//!
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
mod utils;
// public api
pub mod collections;
pub mod models;
