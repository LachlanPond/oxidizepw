use std::fs;
use serde::{Deserialize, Serialize};

use crate::{password::Password, config::Command};

#[derive(Serialize, Deserialize)]
pub struct Database {
    pub master_password: String,
    pub passwords: Vec<Password>,
}

impl Database {
    pub fn new(name: String, master_password: String) -> std::io::Result<()>{
        let database = Database {
            master_password,
            passwords: vec![],
        };
        database.save(name + &String::from(".oxd"))
    }

    pub fn load(file_path: &String) -> Result<Database, &'static str> {
        let raw_contents = match fs::read_to_string(file_path) {
            Ok(content) => content,
            Err(_) => return Err("Could not open database file"),
        };

        let database: Database = match serde_json::from_str(&raw_contents) {
            Ok(db) => db,
            Err(_) => return Err("Could not parse database file JSON"),
        };

        Ok(database)
    }

    pub fn save(&self, file_path: String) -> std::io::Result<()> {
        let database_serialized =  serde_json::json!(&self).to_string();
        fs::write(file_path, database_serialized)
    }

    pub fn change_master_password(&self) {
        todo!()
    }

    pub fn list_passwords(&self) -> Result<(), &'static str> {
        let mut password_count = 0;
        for password in &self.passwords {
            println!("{id}. {name} - {user}",
                id = password_count,
                name = password.name,
                user = password.username
            );
            password_count += 1;
        }
        Ok(())
    }

    pub fn new_password(&mut self, file_path: String, cmd: Command) -> Result<(), &'static str> {
        match cmd {
            Command::New { name, user, pass } => {
                let name = if name.is_none() {
                    return Err("No name was supplied for the password, so the password was not made");
                } else {
                    name.unwrap()
                };

                let user = if user.is_none() { String::from("") } else { user.unwrap() };

                let pass = if pass.is_none() { String::from("") } else { pass.unwrap() };

                self.passwords.push(Password {
                    name: name,
                    username: user,
                    password: pass,
                });
            },
            _ => panic!("Expected `Command::New`, got a different Command variant"),
        }

        match self.save(file_path) {
            Ok(_) => Ok(()),
            Err(_) => return Err("Could not save database to file"),
        }
    }

    pub fn edit_password(&mut self, cmd: Command) -> Result<(), &'static str> {
        todo!()
    }

    pub fn del_password(&mut self, file_path: String, cmd: Command) -> Result<(), &'static str> {
        match cmd {
            Command::Delete(id) => {
                if id.is_none() {
                    return Err("No id was supplied for the password, so no password was deleted");
                } else {
                    self.passwords.remove(id.unwrap());
                }
            },
            _ => panic!("Expected `Command::Delete`, got a different Command variant"),
        }
        
        match self.save(file_path) {
            Ok(_) => Ok(()),
            Err(_) => return Err("Could not save database to file"),
        }
    }

    pub fn get_password(&self, cmd: Command) -> Result<(), &'static str> {
        todo!()
    }
}