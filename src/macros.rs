
#[macro_export]
macro_rules! mixer_endpoint {
	($endpoint:expr) => ({
		&format!("https://mixer.com/api/v1/{}", $endpoint)
	})
}

#[macro_export]
macro_rules! mixer_request {
	($endpoint:ident, $client:expr, $token:expr, $type:ty) => ({
		match $client.get($endpoint).bearer_auth($token).send() {
			Ok(mut result) => {
				match result.status() {
					StatusCode::UNAUTHORIZED => Err("unauthorized".to_string()),
					StatusCode::FORBIDDEN => Err("bad oauth request".to_string()),
					StatusCode::OK => {
						match result.text() {
							Ok(text) => {
								match serde_json::from_str::<$type>(&text) {
									Ok(user) => Ok(user),
									Err(e) => Err(e.to_string())
								}
							},
							Err(err) => Err(format!("could not get response text {}", err.to_string()))
						}
					},
					_ => Err("unknown status".to_string())
				}
			},
			Err(e) => Err(e.to_string())
		}
	})
}
