
// TODO: I know there's some cool tricks in Serde to make this A LOT cleaner, I just don't know them.

use std::{
	vec::Vec,
	collections::HashMap
};

#[derive(Clone, Serialize, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PacketType {
	Method,
	Reply,
	Event
}

#[derive(Clone, Serialize, Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum EventType {
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

#[derive(Clone, Serialize, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MethodType {
	Auth,
	Msg,
	Whisper,
	Timeout,
	Purge,
	DeleteMessage,
	ClearMessages,
	History,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BasePacket {
	#[serde(rename = "type")]
	pub packet_type: PacketType
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EventPacket {
	#[serde(rename = "type")]
	pub packet_type: PacketType,
	pub event: EventType
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AuthenticationPacket {
	#[serde(rename = "type")]
	pub packet_type: PacketType,
	pub method: MethodType,
	pub arguments: serde_json::Value,
	pub id: u64
}

#[derive(Clone, Debug, Deserialize)]
pub struct WelcomeEventData {
	pub server: String
}

/// The first packet that is sent by the Mixer chat server.
#[derive(Clone, Debug, Deserialize)]
pub struct WelcomeEventPacket {
	#[serde(rename = "type")]
	pub packet_type: PacketType,
	pub event: EventType,
	pub data: WelcomeEventData
}

#[derive(Clone, Debug, Deserialize)]
pub struct ChatMessageEmoticonCoordsData {
	pub x: u16,
	pub y: u16,
	pub width:  u16,
	pub height: u16
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ChatMessageType {
	Text,
	Tag,
	Emoticon,
	Link
}

#[derive(Clone, Debug, Deserialize)]
pub struct ChatMessageMessage {
	#[serde(rename = "type")]
	pub message_type: ChatMessageType,
	pub data: Option<String>,
	pub text: Option<String>,
	pub url:  Option<String>,
	pub id:   Option<u32>,
	pub source: Option<String>,
	pub coords: Option<ChatMessageEmoticonCoordsData>,
	pub alt: Option<HashMap<String, String>>
}

#[derive(Clone, Debug, Deserialize)]
pub struct ChatMessageMeta {
	pub whisper:  Option<bool>,
	pub me:       Option<bool>,
	pub censored: Option<bool>
}

#[derive(Clone, Debug, Deserialize)]
pub struct ChatMessageMessageData {
	pub message: Vec<ChatMessageMessage>,
	pub meta:    ChatMessageMeta
}

#[derive(Clone, Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
pub struct ChatMessageEventPacket {
	#[serde(rename = "type")]
	packet_type: PacketType,
	pub event: EventType,
	pub data: ChatMessageEventData
}

#[derive(Debug, Deserialize)]
pub struct UserJoinData {
	pub originating_channel: u64,
	pub username: String,
	pub roles: Vec<String>,
	pub id: u64
}

#[derive(Debug, Deserialize)]
pub struct UserJoinPacket {
	#[serde(rename = "type")]
	pub packet_type: PacketType,
	pub event: EventType,
	pub data: UserJoinData 
}

#[derive(Clone, Debug, Deserialize)]
pub struct UserLeaveData {
	pub originating_channel: u64,
	pub username: String,
	pub id: u64
}

#[derive(Clone, Debug, Deserialize)]
pub struct UserLeavePacket {
	#[serde(rename = "type")]
	pub packet_type: PacketType,
	pub event: EventType,
	pub data: UserLeaveData
}

#[derive(Clone, Debug, Deserialize)]
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

#[derive(Clone, Debug, Deserialize)]
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

#[derive(Clone, Debug, Deserialize)]
pub struct UserGroup {
	pub id: u32,
	pub name: UserRole
}

#[derive(Clone, Debug, Deserialize)]
pub struct User {
	pub channel: Channel,
	pub groups: Vec<UserGroup>
}

#[derive(Clone, Debug, Deserialize)]
pub struct ClearerData {
	pub user_name: String,
	pub user_id: u32,
	pub user_roles: Vec<String>,
	pub user_level: u32
}

#[derive(Clone, Debug, Deserialize)]
pub struct ClearMessagesData {
	clearer: ClearerData
}

#[derive(Clone, Debug, Deserialize)]
pub struct ClearMessagesPacket {
	#[serde(rename = "type")]
	pub packet_type: PacketType,
	pub event: EventType,
	pub data: ClearMessagesData
}

#[derive(Clone, Debug, Deserialize)]
pub struct APIChatResponse {
	pub authkey:     String,
	pub endpoints:   Vec<String>,
	pub permissions: Vec<String>
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgumentPacket {
	#[serde(rename = "type")]
	pub packet_type: PacketType,
	pub method: MethodType,
	pub arguments: Vec<String>,
	pub id: u64
}
