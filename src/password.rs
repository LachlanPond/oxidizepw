use magic_crypt::{new_magic_crypt, MagicCryptError, MagicCryptTrait};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
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

	pub fn decrypt(&self, decryption_key: &String) -> Result<Password, PasswordError>{
		let mc = new_magic_crypt!(decryption_key, 256);
		Ok(Password {
		    name: mc.decrypt_base64_to_string(&self.name)?,
		    username: mc.decrypt_base64_to_string(&self.username)?,
		    password: mc.decrypt_base64_to_string(&self.password)?,
		})
	}

	pub fn update_encryption_key(&self, current_key: &String, new_key: &String) -> Result<Password, PasswordError> {
		let decrypted_password = self.decrypt(&current_key)?;
		Ok(decrypted_password.encrypt(new_key.to_string()))
	}
}

#[derive(Error, Debug)]
pub enum PasswordError {
    #[error("failed to decrypt password")]
    DecryptionFailure(#[from] MagicCryptError),
}

// Once custom errors have been made and implemented, further tests should be included
// to check that functions fail and return the appropriate error
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encrypt() {
    	let password = Password {
	        name: String::from("testname"),
	        username: String::from("testuser"),
	        password: String::from("testpass"),
    	};
    	let encrypted = password.encrypt("testkey".to_string());
    	let encrypted_manual = Password {
	        name: "hGKSEIywJ6cjGJRAfvFziA==".to_string(),
	        username: "GNdCbYuUh0TogMhvtE1uFQ==".to_string(),
	        password: "idSbpqPWccMx79P/bRH3zw==".to_string(),
	    };
    	assert_eq!(encrypted, encrypted_manual);
    }

    #[test]
    fn decrypt() {
    	let password = Password {
	        name: String::from("testname"),
	        username: String::from("testuser"),
	        password: String::from("testpass"),
    	};
    	let encrypted = password.encrypt("testkey".to_string());
    	assert_eq!(encrypted.decrypt(&"testkey".to_string()).unwrap(), password);
    }

    #[test]
    #[should_panic]
    fn decrypt_wrong_key() {
    	let password = Password {
	        name: String::from("testname"),
	        username: String::from("testuser"),
	        password: String::from("testpass"),
    	};
    	let encrypted = password.encrypt("testkey".to_string());
    	assert_ne!(encrypted.decrypt(&"wrongkey".to_string()).unwrap(), password);
    }

    #[test]
    fn update_encryption_key() {
    	let password = Password {
	        name: String::from("testname"),
	        username: String::from("testuser"),
	        password: String::from("testpass"),
    	};
    	let mut encrypted = password.encrypt("testkey".to_string());
    	encrypted = encrypted.update_encryption_key(&"testkey".to_string(), &"newkey".to_string()).unwrap();
    	assert_eq!(encrypted.decrypt(&"newkey".to_string()).unwrap(), password);
    }

    #[test]
    #[should_panic] // TODO: replace with custom error types
    fn update_encryption_key_attempt_old_key() {
    	let password = Password {
	        name: String::from("testname"),
	        username: String::from("testuser"),
	        password: String::from("testpass"),
    	};
    	let mut encrypted = password.encrypt("testkey".to_string());
    	encrypted = encrypted.update_encryption_key(&"testkey".to_string(), &"newkey".to_string()).unwrap();
    	assert_eq!(encrypted.decrypt(&"testkey".to_string()).unwrap(), password);
    }
}