
use std::io::copy;

use reqwest::{
	header::{Authorization, Bearer},
	StatusCode,
	Client
};

use packets::{User};
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

	/// Get the current user based off the `token` that was provided.
	pub fn get_self(&self) -> Result<User, String> {
		let endpoint = mixer_endpoint!("users/current");
		
		match self.client.get(endpoint).header(self.authorization.clone()).send() {
			Ok(mut result) => {
				match result.status() {
					StatusCode::Unauthorized => Err("unauthorized".to_string()),
					StatusCode::Forbidden => Err("bad oauth request".to_string()),
					StatusCode::Ok => {
						match result.text() {
							Ok(text) => {
								// since we have the data from the request now, we just need to turn it into a JSON object of User.
								match serde_json::from_str::<User>(&text) {
									Ok(user) => Ok(user),
									Err(e) => Err(e.to_string())
								}
							},
							Err(err) => Err("could not copy data".to_string())
						}
					},
					_ => Err("unknown status".to_string())
				}
			},
			Err(e) => Err(e.to_string())
		}
	}
}
