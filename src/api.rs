
use std::io::copy;

use reqwest::{
	header::{Authorization, Bearer},
	StatusCode,
	Client
};

use packets::{User, APIChatResponse, Channel};
use serde_json;

/// Handles all interactions between us and Mixer's API.
pub struct MixerAPI {
	authorization: Authorization<Bearer>,
	client: Client
}

impl MixerAPI {

	pub fn new(token: &str) -> Self {
		MixerAPI {
			authorization: Authorization(Bearer { token: token.to_string() }),
			client: Client::new()
		}
	}

	pub fn reauth(&mut self, auth: &str) {
		self.authorization = Authorization(Bearer { token: auth.to_string() });
	}

	/// Get the current user based off the `token` that was provided.
	pub fn get_self(&self) -> Result<User, String> {
		let endpoint = mixer_endpoint!("users/current");
		mixer_request!(endpoint, self.client, self.authorization.clone(), User)
	}

	pub fn get_channel(&self, channel: &str) -> Result<Channel, String> {
		let endpoint = mixer_endpoint!(&format!("channels/{}", channel));
		mixer_request!(endpoint, self.client, self.authorization.clone(), Channel)
	}

	pub fn get_chat(&self, channel: &str) -> Result<APIChatResponse, String> {
		let channel_data = self.get_channel(channel)?;
		let endpoint = mixer_endpoint!(&format!("chats/{}", channel_data.id));
		mixer_request!(endpoint, self.client, self.authorization.clone(), APIChatResponse)
	}
}
