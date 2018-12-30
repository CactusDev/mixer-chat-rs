
#[macro_use]
mod macros;

pub mod packets;
pub mod api;

#[macro_use]
extern crate serde;

#[macro_use]
extern crate serde_derive;
extern crate reqwest;
extern crate serde_json;