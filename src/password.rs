use magic_crypt::{new_magic_crypt, MagicCryptError, MagicCryptTrait};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Password {
    pub name: String,
    pub username: String,
    pub password: String,
}

impl Password {
	pub fn encrypt(&self, encryption_key: String) -> Self {
		let mc = new_magic_crypt!(encryption_key, 256);
		Password {
			name: mc.encrypt_str_to_base64(&self.name),
			username: mc.encrypt_str_to_base64(&self.username),
			password: mc.encrypt_str_to_base64(&self.password)
		}
	}

	pub fn decrypt(&self, decryption_key: &String) -> Result<Password, MagicCryptError>{
		let mc = new_magic_crypt!(decryption_key, 256);
		Ok(Password {
		    name: mc.decrypt_base64_to_string(&self.name)?,
		    username: mc.decrypt_base64_to_string(&self.username)?,
		    password: mc.decrypt_base64_to_string(&self.password)?,
		})
	}

	pub fn update_encryption_key(&self, current_key: &String, new_key: &String) -> Result<Password, MagicCryptError> {
		let decrypted_password = self.decrypt(&current_key)?;
		Ok(decrypted_password.encrypt(new_key.to_string()))
	}
}