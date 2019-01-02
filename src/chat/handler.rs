
use packets::{ChatMessageEventPacket, User};

pub trait Handler {
	fn on_connect(&mut self) -> Result<(), String> { Ok(()) }
	fn on_disconnect(&mut self) -> Result<(), String> { Ok(()) }

	fn on_message(&mut self, message: ChatMessageEventPacket) -> Result<(), String>;
	
	fn user_join(&mut self, user: User) -> Result<(), String> { Ok(()) }
	fn user_leave(&mut self, user: User) -> Result<(), String> { Ok(()) }
	fn on_user_timeout(&mut self, message: &str, length: u16) -> Result<(), String> { Ok(()) }
	fn on_user_purged(&mut self, user: &str) -> Result<(), String> { Ok(()) }

	fn on_chat_cleared(&mut self) -> Result<(), String> { Ok(()) }
	fn on_message_deleted(&mut self, message: &str) -> Result<(), String> { Ok(()) }
}
