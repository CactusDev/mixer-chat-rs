
use websocket::{
	client::{
		ClientBuilder,
		sync::Client
	},
	stream::sync::{TlsStream, TcpStream},
	WebSocketError
};

#[derive(Debug)]
pub enum MixerError {
	Websocket(WebSocketError),
	Request(RequestError),
	Other(String)
}

#[derive(Debug, Clone)]
pub enum RequestError {
	Unauthorized,
	BadOauth,
	BadResponseText(String),
	BadJson,
	UnknownStatusCode,
	Other(String)
}

pub fn create_client(endpoint: &str) -> Result<Client<TlsStream<TcpStream>>, MixerError> {
	println!("Connecting to: {}", endpoint);
	let client = ClientBuilder::new(&endpoint)
		.unwrap()
		.add_protocol("rust-websocket")
		.connect_secure(None)
		.map_err(|e| MixerError::Websocket(e))?;
	println!("Connected!");

	Ok(client)
}
