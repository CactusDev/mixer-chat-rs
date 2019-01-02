
use packets::{ChatMessageEventPacket, User};

/// Handle events from the Mixer chat socket.
pub trait Handler {
	/// Fired when we have successfully connected to the websocket.
	fn on_connect(&mut self) -> Result<(), String> { Ok(()) }
	/// Fired when the websocket is closed
	fn on_disconnect(&mut self) -> Result<(), String> { Ok(()) }
	/// Fired when the websocket is successfully reconnected after a disconnection.
	fn on_reconnect(&mut self) -> Result<(), String> { Ok(()) }

	/// Fired when a message is sent in the chat we are connected to.
	fn on_message(&mut self, message: ChatMessageEventPacket) -> Result<(), String>;
	
	/// Fired when a user joins the channel.
	fn user_join(&mut self, user: User) -> Result<(), String> { Ok(()) }
	/// Fired when a user leaves the channel.
	fn user_leave(&mut self, user: User) -> Result<(), String> { Ok(()) }
	/// Fired when a user is timed out in chat.
	fn on_user_timeout(&mut self, user: &str, length: u16) -> Result<(), String> { Ok(()) }
	/// Fired when a user is purged in chat.
	fn on_user_purged(&mut self, user: &str) -> Result<(), String> { Ok(()) }

	/// Fired when chat has been cleared.
	fn on_chat_cleared(&mut self) -> Result<(), String> { Ok(()) }
	/// Fired when any message in chat has been deleted.
	fn on_message_deleted(&mut self, message: &str) -> Result<(), String> { Ok(()) }
}
