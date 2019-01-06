
use reqwest::{StatusCode, Client};

use packets::{User, APIChatResponse, Channel};
use serde_json;

/// Handles all interactions between us and Mixer's API.
pub struct MixerAPI {
	token: String,
	client: Client
}

impl MixerAPI {

	pub fn new(token: &str) -> Self {
		MixerAPI {
			token: token.to_string(),
			client: Client::new()
		}
	}

	pub fn reauth(&mut self, auth: &str) {
		self.token = auth.to_string();
	}

	/// Get the current user based off the `token` that was provided.
	pub fn get_self(&self) -> Result<User, String> {
		let endpoint = mixer_endpoint!("users/current");
		mixer_request!(endpoint, self.client, &self.token, User)
	}

	pub fn get_channel(&self, channel: &str) -> Result<Channel, String> {
		let endpoint = mixer_endpoint!(&format!("channels/{}", channel));
		mixer_request!(endpoint, self.client, &self.token, Channel)
	}

	pub fn get_chat(&self, channel: &str) -> Result<APIChatResponse, String> {
		let channel_data = self.get_channel(channel)?;
		let endpoint = mixer_endpoint!(&format!("chats/{}", channel_data.id));
		mixer_request!(endpoint, self.client, &self.token, APIChatResponse)
	}
}
