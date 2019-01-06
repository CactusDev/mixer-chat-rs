
#[macro_export]
macro_rules! mixer_endpoint {
	($endpoint:expr) => ({
		&format!("https://mixer.com/api/v1/{}", $endpoint)
	})
}

#[macro_export]
macro_rules! mixer_request {
	($endpoint:ident, $client:expr, $token:expr, $type:ty) => ({
		use $crate::common::RequestError;

		match $client.get($endpoint).bearer_auth($token).send() {
			Ok(mut result) => {
				match result.status() {
					StatusCode::UNAUTHORIZED => Err(RequestError::Unauthorized),
					StatusCode::FORBIDDEN => Err(RequestError::BadOauth),
					StatusCode::OK => {
						match result.text() {
							Ok(text) => {
								let user = serde_json::from_str::<$type>(&text).map_err(|_| RequestError::BadJson)?;
								Ok(user)
							},
							Err(err) => Err(RequestError::BadResponseText(err.to_string()))
						}
					},
					_ => Err(RequestError::UnknownStatusCode)
				}
			},
			Err(e) => Err(RequestError::Other(e.to_string()))
		}
	})
}
