
use std::vec::Vec;
use api::MixerAPI;
use chat::handler::Handler;
use packets::*;

use websocket::client::sync::Client;
use websocket::stream::sync::{TlsStream, TcpStream};

use serde_json::json;

use websocket::{
	client::ClientBuilder,
	Message, OwnedMessage
};

/// Connect to Mixer's chat and listen for messages
pub struct MixerChat {
	channel:   String,
	packet_id: u64,
	
	endpoints:     Vec<String>,
	last_endpoint: u8,
	permissions:   Vec<String>,

	pub api: MixerAPI,
	handler: Box<Handler>,
	client: Option<Client<TlsStream<TcpStream>>>,

	me: Option<User>
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
			client: None,
			me: None
		}
	}

	fn send_packet(&mut self, message: OwnedMessage) -> Result<(), String> {
		self.packet_id += 1;
		match self.client {
			Some(ref mut client) => client.send_message(&message)
				.map_err(|_| "could not send message".to_string())?,
			None => return Err("not connected".to_string())
		}
		Ok(())
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

	/// join the current connection to a given channel.
	pub fn join(&mut self, channel: &Channel, authkey: &str) -> Result<(), String> {
		let me = &self.me.clone().unwrap();
		let packet = AuthenticationPacket {
			id: self.packet_id,
			packet_type: PacketType::Method,
			method: MethodType::Auth,
			arguments: json!([ channel.id, me.channel.user_id, authkey ])
		};

		let message = OwnedMessage::Text(serde_json::to_string(&packet).unwrap());
		self.send_packet(message)?;

		Ok(())
	}

	/// Connect to chat of the Mixer channel provided.
	pub fn connect(&mut self) -> Result<(), String> {
		self.me = Some(self.api.get_self()?);

		// Get the information about the chat we're going to connect to
		let chat = self.api.get_chat(&self.channel)?;
		let channel_data = self.api.get_channel(&self.channel)?;
		
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

		println!("Joining initial channel: {}", &self.channel);
		self.join(&channel_data, &chat.authkey);

		Ok(())
	}

	/// Begin handing chat packets for the connected channel
	pub fn handle_chat(&mut self) -> Result<(), String> {
		match self.client {
			Some(ref mut client) => {
				loop {
					match client.recv_message() {
						Ok(packet) => {
							match packet {
								OwnedMessage::Text(text) => {
									match serde_json::from_str::<ChatMessageEventPacket>(&text) {
										Ok(packet) => self.handler.on_message(packet).unwrap(),
										Err(e) => println!("{:?}", e)
									};
								},
								OwnedMessage::Ping(packet) => client.send_message(&OwnedMessage::Ping(packet)).unwrap(),
								OwnedMessage::Close(_) => break,
								_ => println!("Unhandled packet type!")
							};
							Ok(())
						},
						Err(err) => {
							println!("Encountered an error getting a packet: {:?}", err);
							Err(err)
						}
					};
				}
				Ok(())
			},
			None => Err("no client".to_string())
		};
		Ok(())
	}

	/// Send a message to the connected channel
	pub fn send_message(&mut self, message: &str, target: Option<String>) -> Result<(), String> {
		let (arguments, method) = match target {
			Some(target) => (vec! [ target, message.to_string() ], MethodType::Whisper),
			None => (vec! [ message.to_string() ], MethodType::Msg)
		};

		let packet = ArgumentPacket {
			packet_type: PacketType::Method,
			method: method,
			arguments,
			id: self.packet_id
		};

		let packet = OwnedMessage::Text(serde_json::to_string(&packet).unwrap());
		self.send_packet(packet)
	}

	// Timeout a given user from chat for the provided time.
	pub fn timeout_user(&mut self, user: &str, time: u16) -> Result<(), String> {
		let arguments = vec! [ user.to_string(), time.to_string() ];

		let packet = ArgumentPacket {
			packet_type: PacketType::Method,
			method: MethodType::Timeout,
			arguments,
			id: self.packet_id
		};
		let packet = OwnedMessage::Text(serde_json::to_string(&packet).unwrap());
		self.send_packet(packet)
	}

	pub fn purge_user(&mut self, user: &str) -> Result<(), String> {
		let arguments = vec! [ user.to_string() ];

		let packet = ArgumentPacket {
			packet_type: PacketType::Method,
			method: MethodType::Purge,
			arguments,
			id: self.packet_id
		};
		let packet = OwnedMessage::Text(serde_json::to_string(&packet).unwrap());
		self.send_packet(packet)
	}
}
