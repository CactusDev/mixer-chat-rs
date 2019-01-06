
use api::MixerAPI;
use chat::handler::{ChatHandler, ChatResult};
use packets::chat::*;

use websocket::{
	client::sync::Client,
	stream::sync::{TlsStream, TcpStream},
	OwnedMessage
};

use common::{create_client, MixerError};
use serde_json::json;

fn handle_handler_result(result: ChatResult, chat: &mut MixerChat) -> Result<(), MixerError> {
	match result {
		ChatResult::Nothing => {},
		ChatResult::Error(e) => println!("An internal handler error has occurred: {}", e),
		ChatResult::Message(message) => chat.send_message(&message, None)?
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
	handler: Box<ChatHandler>,
	client: Client<TlsStream<TcpStream>>,

	me: User
}

impl MixerChat {

	pub fn connect(token: &str, channel: &str, handler: Box<ChatHandler>) -> Result<Self, MixerError> {
		let api = MixerAPI::new(token);
		let me = api.get_self().map_err(|e| MixerError::Request(e))?;
		let chat = api.get_chat(channel).map_err(|e| MixerError::Request(e))?;
		let channel_data = api.get_channel(channel).map_err(|e| MixerError::Request(e))?;

		let client = create_client(&chat.endpoints[0].clone())?;

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

	pub fn send_packet(&mut self, message: OwnedMessage) -> Result<(), MixerError> {
		self.packet_id += 1;
		self.client.send_message(&message).map_err(|e| MixerError::Websocket(e))?;
		Ok(())
	}

	/// join the current connection to a given channel.
	pub fn join(&mut self) -> Result<(), MixerError> {
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
	pub fn handle_chat(mut self) -> Result<(), MixerError> {
		println!("Joining channel: {}", &self.channel);
		self.join()?;
		println!("Connected to: {}", &self.channel);

		// Now that we're connected to the channel, we want to fire the `on_connect` event.
		let result = self.handler.on_connect();
		handle_handler_result(result, &mut self)?;

		while let Ok(packet) = self.client.recv_message() {
			match packet {
				OwnedMessage::Ping(packet) => self.client.send_message(&OwnedMessage::Ping(packet)).unwrap(),
				OwnedMessage::Text(text) => match serde_json::from_str::<BasePacket>(&text) {
					Ok(packet) => {
						match packet.packet_type {
							PacketType::Event => {
								let event_packet = serde_json::from_str(&text);
								if let Err(_) = event_packet {
									continue;
								}
								let event_packet: EventPacket = event_packet.unwrap();
								let result = match event_packet.event {
									EventType::ClearMessages => {
										self.handler.on_chat_cleared()
									},
									EventType::UserJoin => {
										let user_packet = serde_json::from_str::<UserJoinPacket>(&text).unwrap();
										self.handler.on_user_join(user_packet)
									},
									EventType::UserLeave => {
										let user_packet = serde_json::from_str::<UserLeavePacket>(&text).unwrap();
										self.handler.on_user_leave(user_packet)
									},
									EventType::PurgeMessage => {
										let purge_packet = serde_json::from_str::<PurgeUserPacket>(&text).unwrap();
										self.handler.on_user_purged(purge_packet)
									},
									EventType::UserTimeout => {
										let timeout_packet = serde_json::from_str::<TimeoutPacket>(&text).unwrap();
										self.handler.on_user_timeout(timeout_packet)
									},
									EventType::ChatMessage => {
										let chat_packet = serde_json::from_str::<ChatMessageEventPacket>(&text).unwrap();
										self.handler.on_message(chat_packet)
									}
									_ => ChatResult::Nothing
								};
								handle_handler_result(result, &mut self)?;
							},
							PacketType::Method => unreachable!(),
							PacketType::Reply => {}
						}
					},
					Err(_) => println!("Could not parse text into JSON: {}", text)
				},
				OwnedMessage::Close(_) => return Ok(()),
				_ => println!("Unhandled packet type!")
			};
		}
		Ok(())
	}

	/// Send a message to the connected channel
	pub fn send_message(&mut self, message: &str, target: Option<String>) -> Result<(), MixerError> {
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
		self.send_packet(packet)?;

		Ok(())
	}

	// Timeout a given user from chat for the provided time.
	pub fn timeout_user(&mut self, user: &str, time: u16) -> Result<(), MixerError> {
		let arguments = vec! [ user.to_string(), time.to_string() ];

		let packet = ArgumentPacket {
			packet_type: PacketType::Method,
			method: MethodType::Timeout,
			arguments,
			id: self.packet_id
		};
		let packet = OwnedMessage::Text(serde_json::to_string(&packet).unwrap());
		self.send_packet(packet)?;

		Ok(())
	}

	pub fn purge_user(&mut self, user: &str) -> Result<(), MixerError> {
		let arguments = vec! [ user.to_string() ];

		let packet = ArgumentPacket {
			packet_type: PacketType::Method,
			method: MethodType::Purge,
			arguments,
			id: self.packet_id
		};
		let packet = OwnedMessage::Text(serde_json::to_string(&packet).unwrap());
		self.send_packet(packet)?;

		Ok(())
	}

	pub fn delete_message(&mut self, message: &str) -> Result<(), MixerError> {
		let arguments = vec! [ message.to_string() ];

		let packet = ArgumentPacket {
			packet_type: PacketType::Method,
			method: MethodType::DeleteMessage,
			arguments,
			id: self.packet_id
		};

		let packet = OwnedMessage::Text(serde_json::to_string(&packet).unwrap());
		self.send_packet(packet)?;

		Ok(())
	}

	pub fn clear_chat(&mut self) -> Result<(), MixerError> {
		let packet = ArgumentPacket {
			packet_type: PacketType::Method,
			method: MethodType::ClearMessages,
			arguments: vec! [],
			id: self.packet_id
		};
		let packet = OwnedMessage::Text(serde_json::to_string(&packet).unwrap());
		self.send_packet(packet)?;

		Ok(())
	}

	pub fn get_history(&mut self, amount: u8) -> Result<(), MixerError> {
		if amount > 100 {
			return Err(MixerError::Other("cannot get more than 100 messages in history".to_string()));
		}

		let arguments = vec! [ amount.to_string() ];
		let packet = ArgumentPacket {
			packet_type: PacketType::Method,
			method: MethodType::History,
			arguments,
			id: self.packet_id
		};
		let packet = OwnedMessage::Text(serde_json::to_string(&packet).unwrap());
		self.send_packet(packet)?;

		Ok(())
	}
}
