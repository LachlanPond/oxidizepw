use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Password {
    name: String,
    username: String,
    password: String,
}

impl Password {
	pub fn generate() {
		// TODO
	}
}