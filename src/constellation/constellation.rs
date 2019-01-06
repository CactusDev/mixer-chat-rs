
use std::vec::Vec;

use websocket::{
	client::{
		ClientBuilder,
		sync::Client
	},
	stream::sync::{TlsStream, TcpStream},
	OwnedMessage
};

use common::create_client;

const CONSTELLATION_URL: &'static str = "wss://constellation.mixer.com";

pub struct Constellation {
	channel: u32
}

impl Constellation {

	pub fn connect(channel: u32) -> Result<Self, String> {
		let client = create_client(CONSTELLATION_URL)?;

		Ok(Constellation {
			channel
		})
	}

	/// format the interfaces that we want to subscribe to.
	fn interfaces(&self, channel: u32) -> Vec<String> {
		vec! [
			"channel:{channel}:update",
			"channel:{channel}:hosted",
			"channel:{channel}:status",
			"channel:{channel}:followed",
			"channel:{channel}:subscribed",
			"channel:{channel}:resubscribed"
		].into_iter()
			.map(|interface| interface.replace("{channel}", &channel.to_string()))
			.collect()
	}

	/// send the initial subscribe packets to Constellation to begin getting our packets.
	fn start(&mut self) -> Result<(), String> {
		// TODO
		Ok(())
	}
}
