
use packets::ChatMessageEventPacket;

pub trait Handler {
	fn on_message(&mut self, message: ChatMessageEventPacket) -> Result<(), String>;
}
