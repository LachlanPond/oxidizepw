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

const FIVE_MINUTES: i32 = 300;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {

    match config.command {
        config::Command::None => {
            println!("Please enter the master password for the new database");
            let mut master_password = String::new();
            io::stdin()
                .read_line(&mut master_password)
                .expect("Failed to read line");
            Database::new(config.database_name, master_password)?;
            return Ok(())
        },
        _ => ()
    }

    // TODO: Update the database to encrypt the passwords and master password.
    // Everything is currently in plain text!
    let mut database = Database::load(&config.database_name)?;

    // Check the entered password against the stored master password
    let entered_password = rpassword::prompt_password("Please enter the database master password\n").unwrap();
    if !database.verify_master_password(entered_password) {
        println!("The password you entered was incorrect");
        return Ok(())
    }

    match config.command {

        config::Command::List => database.list_passwords()?,

        config::Command::New { name, user, pass } => {
            database.new_password(config.database_name, entered_password, Command::New { name, user, pass })?
        },

        config::Command::Edit { item, name, user, pass } => {
            database.edit_password(config.database_name, Command::Edit { item, name, user, pass })?
        },

        config::Command::Delete(id) => database.del_password(config.database_name, Command::Delete(id))?,

        config::Command::Get(id) => {
            let password = database.get_password(Command::Get(id))?;
            println!("Name: {name}\nUser:{user}\nPass: {pass}",
                name=password.name,
                user=password.username,
                pass=password.password
            )
        },

        _ => ()
    };

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
}