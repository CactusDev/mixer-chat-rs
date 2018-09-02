
#[macro_export]
macro_rules! mixer_endpoint {
	($endpoint:expr) => ({
		&format!("https://mixer.com/api/v1/{}", $endpoint)
	})
}
