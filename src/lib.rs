//! # Oxidizepw
//!
//! `oxidizepw` is a simple password manager written in Rust
//! to act as a simple intro project for learning Rust

use std::error::Error;
use std::{fs, i32};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Database {
    pub name: String,
    pub master_password: String,
    pub passwords: Vec<Password>,
}

impl Database {
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
}

#[derive(Serialize, Deserialize)]
pub struct Password {
    name: String,
    username: String,
    password: String,
}

pub enum Command {
    List,
    New {name: Option<String>, user: Option<String>, pass: Option<String>},
    Edit {item: Option<i32>, name: Option<String>, user: Option<String>, pass: Option<String>},
    Delete(Option<i32>),
    Get(Option<i32>),
}

pub struct Config {
    pub database_name: String,
    pub command: Command,
}

impl Config {
    pub fn build(
        mut args: impl Iterator<Item = String>,
    ) -> Result<Config, &'static str> {
        args.next();

        let database_name = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a database file name"),
        };

        // Sort command inputs into a Command enum
        let command = match args.next() {
            Some(arg) => match arg.as_str() {
                "list" => Command::List,
                "new" => Command::New { name: args.next(), user: args.next(), pass: args.next() },
                "edit" => {
                    let item = args.next().unwrap().parse::<i32>();

                    let mut edit_args = HashMap::new();

                    loop {
                        let arg = args.next();
                        if arg.is_none() {
                            break;
                        }
                        match arg.unwrap().as_str() {
                            "-n" => edit_args.insert("-n", args.next()),
                            "-u" => edit_args.insert("-u", args.next()),
                            "-p" => edit_args.insert("-p", args.next()),
                            _ => None,
                        };
                    }

                    Command::Edit { 
                        item: item.ok(), 
                        name: edit_args.get("-n").unwrap().clone(), 
                        user: edit_args.get("-u").unwrap().clone(), 
                        pass: edit_args.get("-p").unwrap().clone() 
                    }
                },
                "delete" => {
                    let item = args.next().unwrap().parse::<i32>();

                    Command::Delete(item.ok())
                },
                "get" => {
                    let item = args.next().unwrap().parse::<i32>();

                    Command::Get(item.ok())
                }
                _ => return Err("Command option does not exist"),
            },
            None => return Err("Didn't get a command"),
        };

        Ok(Config {
            database_name,
            command,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
}