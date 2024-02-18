use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Password {
    pub name: String,
    pub username: String,
    pub password: String,
}

impl Password {
	pub fn generate() {
		// TODO
	}
}