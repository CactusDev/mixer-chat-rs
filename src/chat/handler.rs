
pub trait Handler {
	fn on_message(&mut self, message: String) -> Result<(), String>;
}
