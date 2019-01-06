
use websocket::{
	client::{
		ClientBuilder,
		sync::Client
	},
	stream::sync::{TlsStream, TcpStream},
};

pub fn create_client(endpoint: &str) -> Result<Client<TlsStream<TcpStream>>, String> {
	println!("Connecting to: {}", endpoint);
	let client = ClientBuilder::new(&endpoint)
		.unwrap()
		.add_protocol("rust-websocket")
		.connect_secure(None)
		.unwrap();
	println!("Connected!");

	Ok(client)
}
