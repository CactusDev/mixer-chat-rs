
// TODO: I know there's some cool tricks in Serde to make this A LOT cleaner, I just don't know them.

use std::{
	vec::Vec,
	collections::HashMap
};

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
#[serde(rename_all = "camelCase")]
pub struct ChatMessageEventData {
	pub channel: u32,
	pub id: String,
	pub user_name: String,
	pub user_id: u32,
	pub user_roles: Vec<String>,
	pub user_level: u64,
	pub user_avatar: String,
	pub message: ChatMessageMessageData,
	pub target: Option<String>
}

#[derive(Deserialize)]
pub struct ChatMessageEventPacket {
	#[serde(rename = "type")]
	packet_type: WebsocketPacketType,
	pub event: WebsocketEventType,
	pub data: ChatMessageEventData
}

#[derive(Deserialize)]
pub struct UserJoinData {
	pub originating_channel: u64,
	pub username: String,
	pub roles: Vec<String>,
	pub id: u64
}

#[derive(Deserialize)]
pub struct UserJoinPacket {
	#[serde(rename = "type")]
	pub packet_type: WebsocketPacketType,
	pub event: WebsocketEventType,
	pub data: UserJoinData 
}

#[derive(Deserialize)]
pub struct UserLeaveData {
	pub originating_channel: u64,
	pub username: String,
	pub id: u64
}

#[derive(Deserialize)]
pub struct UserLeavePacket {
	#[serde(rename = "type")]
	pub packet_type: WebsocketPacketType,
	pub event: WebsocketEventType,
	pub data: UserLeaveData
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Channel {
	pub id: u32,
	pub user_id: u32,
	pub token: String,
	pub online: bool,
	pub featured: bool,
	pub feature_level: i32,
	pub partnered: bool,
	pub transcoding_profile_id: Option<u32>,
	pub suspended: bool,
	pub name: String,
	pub audience: String,
	pub viewers_total: u64,
	pub viewers_current: u64,
	pub num_followers: u64,
	pub description: Option<String>,
	pub type_id: Option<u32>,
	pub interactive: bool,
	pub interactive_game_id: Option<u32>,
	pub ftl: i8,
	pub has_vod: bool,
	pub language_id: Option<String>,
	pub cover_id: Option<u32>,
	pub thumbnail_id: Option<u32>,
	pub badge_id: Option<u32>,
	pub banner_url: Option<String>,
	pub hostee_id: Option<u32>,
	pub has_transcodes: bool,
	pub vods_enabled: bool,
	pub costream_id: Option<String>
}

#[derive(Deserialize)]
pub enum UserRole {
	User,
	Banned,
	Pro,
	VerifiedPartner,
	Partner,
	Subscriber,
	ChannelEditor,
	Mod,
	GlobalMod,
	Staff,
	Founder,
	Owner
}

#[derive(Deserialize)]
pub struct UserGroup {
	pub id: u32,
	pub name: UserRole
}

#[derive(Deserialize)]
pub struct User {
	pub channel: Channel,
	pub groups: Vec<UserGroup>
}

#[derive(Deserialize)]
pub struct APIChatResponse {
	pub authkey:     String,
	pub endpoints:   Vec<String>,
	pub permissions: Vec<String>
}
