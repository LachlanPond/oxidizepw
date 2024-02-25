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
        config::Command::Help => {
            print_help();
            return Ok(())
        }
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
            
        },

        _ => ()
    };

    Ok(())
}

fn print_help() {
    let help_string = "Usage: <COMMAND|DATABASE_NAME> <INPUTS>...
Simple commandline password manager with basic SHA2 master password hashing and key-based base64 encryption.
    
    oxidizepw new <db_namename>
        Make a new database file. New files will be given the .oxd extension.
        db_namename: Name for the new database
    
    oxidizepw help|-h
        Print this help output :)
    
    oxidizepw <db_file> list
        List all passwords in the database. The index given to each password how passwords
        are identified the edit/delete/get commands.
        db_file: Database file
    
    oxidizepw <db_file> new <name> <username> <password>
        Adds a new password to the database.
        db_file: Database file
        name: Name of the new password
        username: Username associated with the password
        password: The new password to save

    oxidizepw <db_file> edit <id> [-n <name>] [-u <username>] [-p <password>]
        Edits a specific password, you have the option to edit any or all properties of the
        password.
        db_file: Database file
        id: The id of the password to edit, use the list command to find the password id
        name: The new name to give the password
        username: The new username to set for the password
        password: The new password

    oxidizepw <db_file> delete <id>
        Delete a specific password from the database.
        db_file: Database file
        id: The id of the password to delete, use the list command to find the password id

    oxidizepw <db_file> get <id>
        Prints the information associated with a specific password.
        db_file: Database file
        id: The id of the password to print, use the list command to find the password id

    oxidizepw <db_file> updatepass <new_master_pass>
        Update the master password of the database
        db_file: Database file
        new_master_pass: The new master password for the database";
    println!("{}", help_string);
}