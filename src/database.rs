use std::fs;
use serde::{Deserialize, Serialize};

use crate::{password::Password, config::Command};

#[derive(Serialize, Deserialize)]
pub struct Database {
    pub name: String,
    pub master_password: String,
    pub passwords: Vec<Password>,
}

impl Database {
    pub fn new(name: String, master_password: String) -> std::io::Result<()>{
        let database = Database {
            name:name,
            master_password,
            passwords: vec![],
        };
        database.save(database.name.clone() + &String::from(".oxd"))
    }

    pub fn load(file_path: String) -> Result<Database, &'static str> {
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

    }

    pub fn list_passwords(&self) {

    }

    pub fn new_password(&self, password: Command) {

    }

    pub fn edit_password(&self, cmd: Command) {

    }

    pub fn del_password(&self, password_id: Command) {

    }

    pub fn get_password(&self, password_id: Command) {

    }
}