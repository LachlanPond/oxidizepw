//! # Oxidizepw
//!
//! `oxidizepw` is a simple password manager written in Rust
//! to act as a simple intro project for learning Rust

mod password;
mod database;
pub mod config;

use std::error::Error;
use std::io;

use crate::config::Config;
use crate::config::Command;
use crate::database::Database;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {

    match config.command {

        config::Command::List => Database::load(&config.database_name)?.list_passwords()?,

        config::Command::New { name, user, pass } => {
            Database::load(&config.database_name)?
                .new_password(config.database_name, Command::New { name, user, pass })?
        },

        config::Command::Edit { item, name, user, pass } => {
            Database::load(&config.database_name)?
                .edit_password(config.database_name, Command::Edit { item, name, user, pass })?
        },

        config::Command::Delete(id) => Database::load(&config.database_name)?
                .del_password(config.database_name, Command::Delete(id))?,

        config::Command::Get(id) => {
            let database = Database::load(&config.database_name)?;
            let password = database.get_password(Command::Get(id))?;
            println!("Name: {name}\nUser:{user}\nPass: {pass}",
                name=password.name,
                user=password.username,
                pass=password.password
            )
        }

        config::Command::None => {
            println!("Please enter the master password for the new database");
            let mut master_password = String::new();
            io::stdin()
                .read_line(&mut master_password)
                .expect("Failed to read line");
            Database::new(config.database_name, master_password)?;
        },
        
    };

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
}