use std::fs;
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use thiserror::Error;

use crate::{config::Command, password::{Password, PasswordError}};

#[derive(Serialize, Deserialize)]
pub struct Database {
    pub master_password: Vec<u8>,
    pub passwords: Vec<Password>,
}

impl Database {
    pub fn new(name: String, master_password: String) -> Result<(), DatabaseError>{
        let mut hasher = Sha256::new();
        hasher.update(master_password.as_bytes());
        let master_password_hashed = hasher.finalize();

        let database = Database {
            master_password: master_password_hashed.to_vec(),
            passwords: vec![],
        };

        database.save(name + &String::from(".oxd"))
    }

    pub fn load(file_path: &String) -> Result<Database, DatabaseError> {
        let raw_contents = match fs::read_to_string(file_path) {
            Ok(content) => content,
            Err(_) => return Err(DatabaseError::LoadError("Could not open database file".to_string())),
        };

        let database: Database = match serde_json::from_str(&raw_contents) {
            Ok(db) => db,
            Err(_) => return Err(DatabaseError::LoadError("Could not deserialize database JSON".to_string())),
        };

        Ok(database)
    }

    pub fn save(&self, file_path: String) -> Result<(), DatabaseError> {
        let database_serialized =  serde_json::json!(&self).to_string();
        Ok(fs::write(file_path, database_serialized)?)
    }

    pub fn change_master_password(mut self, file_path: String, old_password: &String, cmd: Command) -> Result<(), DatabaseError> {
        match cmd {
            Command::ChangeMaster(new_password) => {
                let new_password = if new_password.is_none() {
                    return Err(DatabaseError::CommandError("No new password was supplied for the master password".to_string()));
                } else {
                    new_password.unwrap()
                };
                let mut hasher = Sha256::new();
                hasher.update(new_password.as_bytes());
                let master_password_hashed = hasher.finalize().to_vec();

                self.master_password = master_password_hashed;

                self.passwords = self.passwords
                                    .into_iter()
                                    .map(|password| password
                                        .update_encryption_key(old_password, &new_password).unwrap()).collect::<Vec<Password>>();
            },
            _ => panic!("Expected `Command::ChangeMaster, got a different Command variant"),
        }

        self.save(file_path)?;
        Ok(())
    }

    pub fn list_passwords(&self, decryption_key: &String) -> Result<(), PasswordError> {
        let mut password_count = 0;
        for password in &self.passwords {
            let decrypted_password = password.decrypt(decryption_key)?;
            println!("{id}. {name} - {user}",
                id = password_count,
                name = decrypted_password.name,
                user = decrypted_password.username
            );
            password_count += 1;
        }
        Ok(())
    }

    // For any new information, the aim is to immediately encrypt and store it
    pub fn new_password(&mut self, file_path: String, encryption_key: String, cmd: Command) -> Result<(), DatabaseError> {
        match cmd {
            Command::New { name, user, pass } => {
                let name = if name.is_none() {
                    return Err(DatabaseError::CommandError("No name was supplied for the password, so the password was not made".to_string()));
                } else {
                    name.unwrap()
                };

                let username = if user.is_none() { String::from("") } else { user.unwrap() };

                let password = if pass.is_none() { String::from("") } else { pass.unwrap() };

                self.passwords.push(Password {
                    name,
                    username,
                    password,
                }.encrypt(encryption_key));
            },
            _ => panic!("Expected `Command::New`, got a different Command variant"),
        }

        self.save(file_path)?;
        Ok(())
    }

    pub fn edit_password(&mut self, file_path: String, encryption_key: String, cmd: Command) -> Result<(), DatabaseError> {
        match cmd {
            Command::Edit { item, name, user, pass } => {
                if item.is_none() {
                    return Err(DatabaseError::CommandError("No id was supplied for the password, so no password was edited".to_string()));
                } else {
                    let password_id = item.unwrap();
                    if password_id >= self.passwords.len() {
                        return Err(DatabaseError::CommandError(format!("The id supplied does not exist in the database, valid id's are 0-{}",self.passwords.len()-1)));
                    }
                    let encrypted_password = &self.passwords[password_id];
                    let mut decrypted_password = encrypted_password.decrypt(&encryption_key).unwrap();
                    if !name.is_none() { decrypted_password.name = name.unwrap(); }
                    if !user.is_none() { decrypted_password.username = user.unwrap(); }
                    if !pass.is_none() { decrypted_password.password = pass.unwrap(); }
                    self.passwords[item.unwrap()] = decrypted_password.encrypt(encryption_key);
                }
            },
            _ => panic!("Expected `Command::Delete`, got a different Command variant"),
        }
        
        self.save(file_path)?;
        Ok(())
    }

    pub fn del_password(&mut self, file_path: String, cmd: Command) -> Result<(), DatabaseError> {
        match cmd {
            Command::Delete(id) => {
                if id.is_none() {
                    return Err(DatabaseError::CommandError("No id was supplied for the password, so no password was deleted".to_string()));
                } else {
                    let password_id = id.unwrap();
                    if password_id >= self.passwords.len() {
                        return Err(DatabaseError::CommandError(format!("The id supplied does not exist in the database, valid id's are 0-{}",self.passwords.len()-1)));
                    }
                    self.passwords.remove(password_id);
                }
            },
            _ => panic!("Expected `Command::Delete`, got a different Command variant"),
        }
        
        self.save(file_path)?;
        Ok(())
    }

    pub fn get_password(&self, decryption_key: &String, cmd: Command) -> Result<Password, DatabaseError> {
        match cmd {
            Command::Get(id) => {
                if id.is_none() {
                    return Err(DatabaseError::CommandError("No id was supplied for the password, so no password was fetched".to_string()));
                } else {
                    let password_id = id.unwrap();
                    if password_id >= self.passwords.len() {
                        return Err(DatabaseError::CommandError(format!("The id supplied does not exist in the database, valid id's are 0-{}",self.passwords.len()-1)));
                    }
                    Ok(self.passwords[password_id].decrypt(decryption_key)?)
                }
            },
            _ => panic!("Expected `Command::Delete`, got a different Command variant"),
        }
    }

    pub fn verify_master_password(&self, entered_password: &String) -> bool {
        let mut hasher = Sha256::new();
        hasher.update(entered_password.as_bytes());
        let entered_password_hashed = hasher.finalize().to_vec();

        entered_password_hashed == self.master_password
    }

}

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("failed to save database")]
    SaveError(#[from] std::io::Error),
    #[error("`{0}`")]
    LoadError(String),
    #[error("`{0}`")]
    CommandError(String),
    #[error("failed to get the selected password")]
    GetPasswordError(#[from] PasswordError)
}