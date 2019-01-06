
#[macro_use]
mod macros;

pub mod chat;
pub mod constellation;
pub mod api;
pub mod common;
pub mod packets;

extern crate serde;

#[macro_use]
extern crate serde_derive;
extern crate reqwest;
extern crate serde_json;
extern crate websocket;