
// TODO: I know there's some cool tricks in Serde to make this A LOT cleaner, I just don't know them.

use std::{
	vec::Vec,
	collections::HashMap
};

/// Response for the endpoint /chats/{channelID}
#[derive(Deserialize)]
pub struct ChatEndpointResponse {
	authkey:     String,
	endpoints:   Vec<String>,
	permissions: Vec<String>
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum WebsocketPacketType {
	Method,
	Reply,
	Event
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "event", rename_all = "PascalCase")]
pub enum WebsocketEventType {
	WelcomeEvent,
	ChatMessage,
	UserJoin,
	UserLeave,
	PollStart,
	PollEnd,
	DeleteMessage,
	PurgeMessage,
	ClearMessages,
	UserUpdate,
	UserTimeout
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "method", rename_all = "PascalCase")]
pub enum WebsocketMethodType {
	Auth,
	Msg,
	Whisper,
	Timeout,
	Purge,
	DeleteMessage,
	ClearMessages,
	History,
}

#[derive(Deserialize)]
pub struct WelcomeEventData {
	server: String
}

/// The first packet that is sent by the Mixer chat server.
#[derive(Deserialize)]
pub struct WelcomeEventPacket {
	#[serde(rename = "type")]
	packet_type: WebsocketPacketType,
	event: WebsocketEventType,
	data: WelcomeEventData
}

#[derive(Deserialize)]
pub struct ChatMessageEmoticonCoordsData {
	x: u16,
	y: u16,
	width:  u16,
	height: u16
}

#[derive(Deserialize)]
#[serde(tag = "source", rename_all = "camelCase")]
pub enum ChatMessageEmoticonSource {
	External,
	Builtin
}

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ChatMessageType {
	Text,
	Tag,
	Emoticon,
	Link
}

#[derive(Deserialize)]
pub struct ChatMessageMessage {
	#[serde(rename = "type")]
	message_type: ChatMessageType,
	data: Option<String>,
	text: Option<String>,
	url:  Option<String>,
	id:   Option<u32>,
	source: Option<ChatMessageEmoticonSource>,
	coords: Option<ChatMessageEmoticonCoordsData>
}

#[derive(Deserialize)]
pub struct ChatMessageMeta {
	whisper:  Option<bool>,
	me:       Option<bool>,
	censored: Option<bool>
}

#[derive(Deserialize)]
pub struct ChatMessageMessageData {
	message: Vec<ChatMessageMessage>,
	meta:    ChatMessageMeta
}

#[derive(Deserialize)]
pub struct ChatMessageEventData {
	channel: u32,
	id: String,
	user_name: String,
	user_id: u32,
	user_roles: Vec<String>,
	user_level: u64,
	user_avatar: String,
	message: ChatMessageMessageData,
	target: Option<String>
}

#[derive(Deserialize)]
pub struct ChatMessageEventPacket {
	#[serde(rename = "type")]
	packet_type: WebsocketPacketType,
	event: WebsocketEventType,
	data: ChatMessageEventData
}

#[derive(Deserialize)]
pub struct UserJoinData {
	originating_channel: u64,
	username: String,
	roles: Vec<String>,
	id: u64
}

#[derive(Deserialize)]
pub struct UserJoinPacket {
	#[serde(rename = "type")]
	packet_type: WebsocketPacketType,
	event: WebsocketEventType,
	data: UserJoinData 
}

#[derive(Deserialize)]
pub struct UserLeaveData {
	originating_channel: u64,
	username: String,
	id: u64
}

#[derive(Deserialize)]
pub struct UserLeavePacket {
	#[serde(rename = "type")]
	packet_type: WebsocketPacketType,
	event: WebsocketEventType,
	data: UserLeaveData
}
