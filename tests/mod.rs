
#[macro_use]
extern crate mixer_chat;

#[test]
fn test_endpoint_macro() {
	assert_eq!(mixer_endpoint!("channels/Stanley"), "https://mixer.com/api/v1/channels/Stanley");
}
