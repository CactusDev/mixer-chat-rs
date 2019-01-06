
pub enum ConstellationResult {
	Nothing,
	Error(String),
	Message(String)
}

/// Handle events from the Mixer chat socket.
pub trait ConstellationHandler {
	/// Fired when we have successfully connected to the websocket.
	fn on_connect(&mut self) -> ConstellationResult { ConstellationResult::Nothing }
	/// Fired when the websocket is closed
	fn on_disconnect(&mut self) -> ConstellationResult { ConstellationResult::Nothing }
	/// Fired when the websocket is successfully reconnected after a disconnection.
	fn on_reconnect(&mut self) -> ConstellationResult { ConstellationResult::Nothing }
}
