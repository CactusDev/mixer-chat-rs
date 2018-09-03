
use std::{
	vec::Vec
};

use api::MixerAPI;

use ws;

/// Handles the connection to the websocket.
struct Connector;

impl ws::Handler for Connector {

	fn on_open(&mut self, _: ws::Handshake) -> ws::Result<()> {
		println!("[Mixer Chat] Connected to Mixer!");
		Ok(())
	}

	fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
		println!("{}", msg);
		Ok(())
	}
}

/// Connect to Mixer's chat and listen for messages
pub struct MixerChat {
	channel:   String,
	packet_id: u64,
	
	endpoints:     Vec<String>,
	last_endpoint: u8,
	permissions:   Vec<String>,

	api: MixerAPI,
	handler: Option<Connector>
}

impl MixerChat {

	pub fn new(token: &str, channel: &str) -> Self {
		MixerChat {
			channel: channel.to_string(),
			packet_id: 0,
			endpoints: vec! [],
			last_endpoint: 0,
			permissions: vec! [],
			api: MixerAPI::new(token),
			handler: None
		}
	}

	fn get_next_endpoint(&mut self) -> Option<&str> {
		return Some(
			if self.last_endpoint == ((self.endpoints.len() - 1) as u8) {
				self.last_endpoint = 0;
				&self.endpoints[self.last_endpoint as usize]
			} else {
				self.last_endpoint += 1;
				&self.endpoints[self.last_endpoint as usize]
			}
		)
	}

	/// Connect to chat of the Mixer channel provided.
	pub fn connect(&mut self) -> Result<(), String> {
		// Get the information about the chat we're going to connect to
		let chat = self.api.get_chat(&self.channel)?;
		self.endpoints = chat.endpoints;
		self.permissions = chat.permissions;

		// Attempt to get an endpoint to connect to
		let endpoint = self.get_next_endpoint();
		if endpoint.is_none() {
			// We don't have any endpoints, so we can't connect!
			return Err("could not get an endpoint".to_string());
		}
		println!("Connecting to: {}", endpoint.unwrap());

		ws::connect("wss://chat10-dal.mixer.com:443", |out| {
			Connector {
			}
		}).unwrap();
		Ok(())
	}
}
