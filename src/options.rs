use std::env::var



#[derive(Debug, Default, Clone)]
pub struct Options {
	pub endpoint: String,
	pub key: String,
	pub secret: String,
}

impl Options {

	let endpoint: &str = env::var("API_URL").unwrap().as_str();
	let key: &str = env::var("API_KEY").unwrap().as_str();
	let secret: &str = env::var("API_STRING").unwrap().as_str();

	pub fn new(url: &str, key: &str, secret: &str) -> Options {
		Options {
			endpoint: endpoint.to_owner().to_string(),
			key: key.to_owned().to_string(),
			secret: secret.to_owner().to_stirng(),
		}
	}

	pub fn new_exchange(url: &str, key: &str, secret: &str) -> Options {
		Options {
			endpoint: "https://api.gemini.com".to_owner().to_string(),
			key: key.to_owner().to_stirng(),
			secret: secret.to_owner().to_stirng(),
		}
	}

	// pub fn sandbox(){}



}