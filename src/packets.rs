
// TODO: I know there's some cool tricks in Serde to make this A LOT cleaner, I just don't know them.

pub mod common {

	use std::collections::HashMap;

	#[derive(Clone, Debug, Serialize, Deserialize)]
	#[serde(rename_all = "camelCase")]
	pub struct SocialInfo {
		pub twitter: Option<String>,
		pub facebook: Option<String>,
		pub youtube: Option<String>,
		pub player: Option<String>,
		pub discord: Option<String>,
		pub verified: Option<String>
	}

	#[derive(Clone, Debug, Serialize, Deserialize)]
	#[serde(rename_all = "camelCase")]
	pub struct Team {
		pub id: u64,
		pub owner_id: u64,
		pub token: String,
		pub name: String,
		pub description: String,
		pub logo_url: String,
		pub background_url: String,
		pub social: Option<SocialInfo>
	}

	#[derive(Clone, Debug, Serialize, Deserialize)]
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

	#[derive(Clone, Debug, Serialize, Deserialize)]
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

	#[derive(Clone, Debug, Serialize, Deserialize)]
	pub struct UserGroup {
		pub id: u32,
		pub name: UserRole
	}

	#[derive(Clone, Debug, Serialize, Deserialize)]
	pub struct User {
		pub channel: Channel,
		pub groups: Vec<UserGroup>
	}

	#[derive(Clone, Debug, Serialize, Deserialize)]
	pub struct FullUser {
		pub level: u64,
		pub social: HashMap<String, String>,  // TODO: This needs to be verified.
		pub id: u64,
		pub username: String,
		pub verified: bool,
		pub experience: u64,
		pub sparks: u64,
		pub avatar_url: String,
		pub bio: String,
		pub primary_team: Option<Team>,
		pub created_at: String,
		pub updated_at: String,
		pub deleted_at: Option<String>,
		pub channel: Channel
	}
}

pub mod chat {

	use std::{
		vec::Vec,
		collections::HashMap
	};

	use packets::common::Channel;

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
	#[serde(rename_all = "camelCase")]
	pub struct UserJoinData {
		pub originating_channel: u64,
		pub username: String,
		pub roles: Vec<String>,
		pub id: u64
	}

	#[derive(Debug, Deserialize)]
	#[serde(rename_all = "camelCase")]
	pub struct UserJoinPacket {
		#[serde(rename = "type")]
		pub packet_type: PacketType,
		pub event: EventType,
		pub data: UserJoinData 
	}

	#[derive(Clone, Debug, Deserialize)]
	#[serde(rename_all = "camelCase")]
	pub struct UserLeaveData {
		pub originating_channel: u64,
		pub username: String,
		pub id: u64
	}

	#[derive(Clone, Debug, Deserialize)]
	#[serde(rename_all = "camelCase")]
	pub struct UserLeavePacket {
		#[serde(rename = "type")]
		pub packet_type: PacketType,
		pub event: EventType,
		pub data: UserLeaveData
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
	pub struct ModeratorData {
		pub user_name: String,
		pub user_id: u32,
		pub user_roles: Vec<String>,
		pub user_level: u32
	}

	#[derive(Clone, Debug, Deserialize)]
	pub struct PurgeUserData {
		pub moderator: Option<ModeratorData>,
		pub user_id: u32
	}

	#[derive(Clone, Debug, Deserialize)]
	pub struct PurgeUserPacket {
		#[serde(rename = "type")]
		pub packet_type: PacketType,
		pub event: EventType,
		pub data: PurgeUserData
	}

	#[derive(Clone, Debug, Deserialize)]
	pub struct TimeoutUserData {
		pub user_name: String,
		pub user_id: u32,
		pub user_roles: Vec<String>
	}

	#[derive(Clone, Debug, Deserialize)]
	pub struct TimeoutData {
		pub user: TimeoutUserData
	}

	#[derive(Clone, Debug, Deserialize)]
	pub struct TimeoutPacket {
		#[serde(rename = "type")]
		pub packet_type: PacketType,
		pub event: EventType,
		pub data: TimeoutData
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
}

pub mod constellation {

	use packets::common::{Channel, User, FullUser};

	#[derive(Clone, Debug, Deserialize, Serialize)]
	pub enum PacketType {
		Method,
		Event
	}

	#[derive(Clone, Debug, Deserialize, Serialize)]
	#[serde(rename_all = "lowercase")]
	pub enum PacketMethod {
		LiveSubscribe
	}

	#[derive(Clone, Debug, Deserialize, Serialize)]
	pub struct SubscribeParams {
		pub events: Vec<String>
	}

	#[derive(Clone, Debug, Deserialize, Serialize)]
	pub struct SubscribePacket {
		#[serde(rename = "type")]
		pub packet_type: PacketType,
		pub method: PacketMethod,
		pub params: SubscribeParams,
		pub id: u64
	}

	#[derive(Clone, Debug, Deserialize, Serialize)]
	pub struct ChannelHostedPacket {
		pub hoster_id: u64,
		pub hoster: Channel
	}

	#[derive(Clone, Debug, Deserialize, Serialize)]
	pub struct ChannelFollowedPacket {
		pub user: FullUser,
		pub following: bool
	}

	#[derive(Clone, Debug, Deserialize, Serialize)]
	pub struct ChannelSubscribedPacket {
		pub user: FullUser
	}

	#[derive(Clone, Debug, Deserialize, Serialize)]
	pub struct ChannelResubscribedPacket {
		pub user: FullUser,
		pub since: String,
		pub until: String,
		pub total_months: u16
	}
}
