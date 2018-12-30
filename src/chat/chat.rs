
use std::vec::Vec;
use api::MixerAPI;
use chat::handler::Handler;

use websocket::client::sync::Client;
use websocket::stream::sync::{TlsStream, TcpStream};

use websocket::{
	client::ClientBuilder,
	Message, OwnedMessage
};

/// Connect to Mixer's chat and listen for messages
pub struct MixerChat {
	pub channel:   String,
	packet_id: u64,
	
	endpoints:     Vec<String>,
	last_endpoint: u8,
	permissions:   Vec<String>,

	pub api: MixerAPI,
	handler: Box<Handler>,
	client: Option<Client<TlsStream<TcpStream>>>
}

impl MixerChat {

	pub fn new(token: &str, channel: &str, handler: Box<Handler>) -> Self {
		MixerChat {
			channel: channel.to_string(),
			packet_id: 0,
			endpoints: vec! [],
			last_endpoint: 0,
			permissions: vec! [],
			api: MixerAPI::new(token),
			handler,
			client: None
		}
	}

	fn get_next_endpoint(&mut self) -> &str {
		return if self.last_endpoint == ((self.endpoints.len() - 1) as u8) {
			self.last_endpoint = 0;
			&self.endpoints[self.last_endpoint as usize]
		} else {
			self.last_endpoint += 1;
			&self.endpoints[self.last_endpoint as usize]
		}
	}

	/// Connect to chat of the Mixer channel provided.
	pub fn connect(&mut self) -> Result<(), String> {
		// Get the information about the chat we're going to connect to
		let chat = self.api.get_chat(&self.channel)?;
		
		self.endpoints = chat.endpoints;
		self.permissions = chat.permissions;
		
		let endpoint = self.get_next_endpoint().to_string();

		println!("Connecting to: {}", endpoint);
		self.client = Some(ClientBuilder::new(&endpoint)
			.unwrap()
			.add_protocol("rust-websocket")
			.connect_secure(None)
			.unwrap());
		println!("Connected to Mixer!");

		Ok(())
	}

	pub fn handle_chat(&mut self) -> Result<(), String> {
		match self.client {
			Some(ref mut client) => {
				loop {
					match client.recv_message() {
						Ok(packet) => {
							match packet {
								OwnedMessage::Text(text) => self.handler.on_message(text).unwrap(),
								OwnedMessage::Ping(packet) => client.send_message(&OwnedMessage::Ping(packet)).unwrap(),
								OwnedMessage::Close(_) => break,
								_ => println!("Unhandled packet type!")
							};
						},
						Err(err) => println!("Encountered an error getting a packet: {:?}", err)
					};
				}
				Ok(())
			},
			None => Err("no client".to_string())
		};
		Ok(())
	}
}
