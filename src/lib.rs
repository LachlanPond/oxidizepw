//! # Oxidizepw
//!
//! `oxidizepw` is a simple password manager written in Rust
//! to act as a simple intro project for learning Rust

mod password;
mod database;
pub mod config;

use std::error::Error;

use crate::config::Config;
use crate::config::Command;
use crate::database::Database;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {

    match config.command {
        config::Command::None => {
            let master_password = rpassword::prompt_password("Please enter the master password for the new database\n").unwrap();
            Database::new(config.database_name, master_password)?;
            return Ok(())
        },
        _ => ()
    }

    // Database is encrypted at this point
    let mut database = Database::load(&config.database_name)?;

    // Check the entered password against the stored master password
    let entered_password = rpassword::prompt_password("Please enter the database master password\n").unwrap();
    if !database.verify_master_password(&entered_password) {
        println!("The password you entered was incorrect");
        return Ok(())
    }

    match config.command {

        config::Command::List => database.list_passwords(&entered_password)?,

        config::Command::New { name, user, pass } => {
            database.new_password(config.database_name, entered_password, Command::New { name, user, pass })?
        },

        config::Command::Edit { item, name, user, pass } => {
            database.edit_password(config.database_name, entered_password, Command::Edit { item, name, user, pass })?
        },

        config::Command::Delete(id) => database.del_password(config.database_name, Command::Delete(id))?,

        config::Command::Get(id) => {
            let password = database.get_password(&entered_password, Command::Get(id))?;
            println!("Name: {name}\nUser: {user}\nPass: {pass}",
                name=password.name,
                user=password.username,
                pass=password.password
            );
        },

        config::Command::ChangeMaster(new_password) => {
            database.change_master_password(config.database_name, &entered_password, Command::ChangeMaster(new_password))?;
            println!("Database master password has been updated");
        },

        _ => ()
    };

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
}