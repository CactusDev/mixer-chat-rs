
use packets::{ChatMessageEventPacket, UserJoinPacket, UserLeavePacket, PurgeUserPacket, TimeoutPacket};

pub enum ChatResult {
	Nothing,
	Error(String),
	Message(String)
}

/// Handle events from the Mixer chat socket.
pub trait ChatHandler {
	/// Fired when we have successfully connected to the websocket.
	fn on_connect(&mut self) -> ChatResult { ChatResult::Nothing }
	/// Fired when the websocket is closed
	fn on_disconnect(&mut self) -> ChatResult { ChatResult::Nothing }
	/// Fired when the websocket is successfully reconnected after a disconnection.
	fn on_reconnect(&mut self) -> ChatResult { ChatResult::Nothing }

	/// Fired when a message is sent in the chat we are connected to.
	fn on_message(&mut self, _message: ChatMessageEventPacket) -> ChatResult;
	
	/// Fired when a user joins the channel.
	fn on_user_join(&mut self, _packet: UserJoinPacket) -> ChatResult { ChatResult::Nothing }
	/// Fired when a user leaves the channel.
	fn on_user_leave(&mut self, _packet: UserLeavePacket) -> ChatResult { ChatResult::Nothing }
	/// Fired when a user is timed out in chat.
	fn on_user_timeout(&mut self, _packet: TimeoutPacket) -> ChatResult { ChatResult::Nothing }
	/// Fired when a user is purged in chat.
	fn on_user_purged(&mut self, _packet: PurgeUserPacket) -> ChatResult { ChatResult::Nothing }

	/// Fired when chat has been cleared.
	fn on_chat_cleared(&mut self) -> ChatResult { ChatResult::Nothing }
}
