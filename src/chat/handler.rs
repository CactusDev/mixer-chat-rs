
use packets::{ChatMessageEventPacket, User};

pub enum HandlerResult {
	Nothing,
	Error(String),
	Message(String)
}

/// Handle events from the Mixer chat socket.
pub trait Handler {
	/// Fired when we have successfully connected to the websocket.
	fn on_connect(&mut self) -> HandlerResult { HandlerResult::Nothing }
	/// Fired when the websocket is closed
	fn on_disconnect(&mut self) -> HandlerResult { HandlerResult::Nothing }
	/// Fired when the websocket is successfully reconnected after a disconnection.
	fn on_reconnect(&mut self) -> HandlerResult { HandlerResult::Nothing }

	/// Fired when a message is sent in the chat we are connected to.
	fn on_message(&mut self, _message: ChatMessageEventPacket) -> HandlerResult;
	
	/// Fired when a user joins the channel.
	fn user_join(&mut self, _user: User) -> HandlerResult { HandlerResult::Nothing }
	/// Fired when a user leaves the channel.
	fn user_leave(&mut self, _user: User) -> HandlerResult { HandlerResult::Nothing }
	/// Fired when a user is timed out in chat.
	fn on_user_timeout(&mut self, _user: &str, _length: u16) -> HandlerResult { HandlerResult::Nothing }
	/// Fired when a user is purged in chat.
	fn on_user_purged(&mut self, _user: &str) -> HandlerResult { HandlerResult::Nothing }

	/// Fired when chat has been cleared.
	fn on_chat_cleared(&mut self) -> HandlerResult { HandlerResult::Nothing }
	/// Fired when any message in chat has been deleted.
	fn on_message_deleted(&mut self, _message: &str) -> HandlerResult { HandlerResult::Nothing }
}
