
#[macro_export]
macro_rules! mixer_endpoint {
	($endpoint:expr) => ({
		&format!("https://mixer.com/api/v1/{}", $endpoint)
	})
}

#[macro_export]
macro_rules! mixer_request {
	($endpoint:ident, $client:expr, $authorization:expr, $type:ty) => ({
		match $client.get($endpoint).header($authorization).send() {
			Ok(mut result) => {
				match result.status() {
					StatusCode::Unauthorized => Err("unauthorized".to_string()),
					StatusCode::Forbidden => Err("bad oauth request".to_string()),
					StatusCode::Ok => {
						match result.text() {
							Ok(text) => {
								// since we have the data from the request now, we just need to turn it into a JSON object of User.
								match serde_json::from_str::<$type>(&text) {
									Ok(user) => Ok(user),
									Err(e) => Err(e.to_string())
								}
							},
							Err(err) => Err("could not get response text".to_string())
						}
					},
					_ => Err("unknown status".to_string())
				}
			},
			Err(e) => Err(e.to_string())
		}
	})
}
