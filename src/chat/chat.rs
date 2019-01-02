
use api::MixerAPI;
use chat::handler::{Handler, HandlerResult};
use packets::*;

use websocket::client::sync::Client;
use websocket::stream::sync::{TlsStream, TcpStream};

use serde_json::json;

use websocket::{client::ClientBuilder, OwnedMessage};

fn handle_handler_result(result: HandlerResult, chat: &mut MixerChat) -> Result<(), String> {
	match result {
		HandlerResult::Nothing => {},
		HandlerResult::Error(e) => println!("An internal handler error has occurred: {}", e),
		HandlerResult::Message(message) => chat.send_packet(OwnedMessage::Text(message.to_string()))?
	}
	Ok(())
}

/// Connect to Mixer's chat and listen for messages
pub struct MixerChat {
	channel:   String,
	packet_id: u64,
	
	channel_data:  Channel,
	chat:          APIChatResponse,

	pub api: MixerAPI,
	handler: Box<Handler>,
	client: Client<TlsStream<TcpStream>>,

	me: User
}

fn connect(endpoint: &str) -> Result<Client<TlsStream<TcpStream>>, String> {
	println!("Connecting to: {}", endpoint);
	let client = ClientBuilder::new(&endpoint)
		.unwrap()
		.add_protocol("rust-websocket")
		.connect_secure(None)
		.unwrap();
	println!("Connected to Mixer!");

	Ok(client)
}

impl MixerChat {

	pub fn connect(token: &str, channel: &str, handler: Box<Handler>) -> Result<Self, String> {
		let api = MixerAPI::new(token);
		let me = api.get_self()?;
		let chat = api.get_chat(channel)?;
		let channel_data = api.get_channel(channel)?;

		let client = connect(&chat.endpoints[0].clone())?;

		Ok(MixerChat {
			channel: channel.to_string(),
			packet_id: 0,
			api: MixerAPI::new(token),
			handler,
			client,
			me,
			channel_data,
			chat
		})
	}

	pub fn send_packet(&mut self, message: OwnedMessage) -> Result<(), String> {
		self.packet_id += 1;
		self.client.send_message(&message).map_err(|_| "could not send message".to_string())?;
		Ok(())
	}

	/// join the current connection to a given channel.
	pub fn join(&mut self) -> Result<(), String> {
		// CLEANUP: Do we _really_ need to clone all of those? Is there a better way
		//          to satisfy the borrow checker?
		let channel_id = self.channel_data.id.clone();
		let user_id = self.me.channel.user_id.clone();
		let authkey = self.chat.authkey.clone();

		let packet = AuthenticationPacket {
			id: self.packet_id,
			packet_type: PacketType::Method,
			method: MethodType::Auth,
			arguments: json!([ channel_id, user_id, authkey ])
		};

		let message = OwnedMessage::Text(serde_json::to_string(&packet).unwrap());
		self.send_packet(message)?;

		Ok(())
	}

	/// Begin handing chat packets for the connected channel
	pub fn handle_chat(mut self) -> Result<(), String> {
		println!("Joining channel: {}", &self.channel);
		self.join()?;
		println!("Connected to: {}", &self.channel);

		// Now that we're connected to the channel, we want to fire the `on_connect` event.
		let result = self.handler.on_connect();
		handle_handler_result(result, &mut self)?;

		while let Ok(packet) = self.client.recv_message() {
			match packet {
				OwnedMessage::Ping(packet) => self.client.send_message(&OwnedMessage::Ping(packet)).unwrap(),
				OwnedMessage::Text(text) => {
					let packet: ChatMessageEventPacket = serde_json::from_str(&text).map_err(|e| e.to_string())?;
					let result = self.handler.on_message(packet);
					handle_handler_result(result, &mut self)?;
				},
				OwnedMessage::Close(_) => return Ok(()),
				_ => println!("Unhandled packet type!")
			};
		}
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
		self.send_packet(packet)?;

		let result = self.handler.on_user_timeout(user, time);
		handle_handler_result(result, self)?;
		Ok(())
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
		self.send_packet(packet)?;

		let result = self.handler.on_user_purged(user);
		handle_handler_result(result, self)?;
		Ok(())
	}

	pub fn delete_message(&mut self, message: &str) -> Result<(), String> {
		let arguments = vec! [ message.to_string() ];

		let packet = ArgumentPacket {
			packet_type: PacketType::Method,
			method: MethodType::DeleteMessage,
			arguments,
			id: self.packet_id
		};

		let packet = OwnedMessage::Text(serde_json::to_string(&packet).unwrap());
		self.send_packet(packet)?;

		let result = self.handler.on_message_deleted(message);
		handle_handler_result(result, self)?;
		Ok(())
	}

	pub fn clear_chat(&mut self) -> Result<(), String> {
		let packet = ArgumentPacket {
			packet_type: PacketType::Method,
			method: MethodType::ClearMessages,
			arguments: vec! [],
			id: self.packet_id
		};
		let packet = OwnedMessage::Text(serde_json::to_string(&packet).unwrap());
		self.send_packet(packet)?;

		let result = self.handler.on_chat_cleared();
		handle_handler_result(result, self)?;
		Ok(())
	}

	pub fn get_history(&mut self, amount: u8) -> Result<(), String> {
		if amount > 100 {
			return Err("cannot get more than 100 messages in history".to_string());
		}

		let arguments = vec! [ amount.to_string() ];
		let packet = ArgumentPacket {
			packet_type: PacketType::Method,
			method: MethodType::History,
			arguments,
			id: self.packet_id
		};
		let packet = OwnedMessage::Text(serde_json::to_string(&packet).unwrap());
		self.send_packet(packet)
	}
}
