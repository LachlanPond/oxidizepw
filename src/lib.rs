//! # Oxidizepw
//!
//! `oxidizepw` is a simple password manager written in Rust
//! to act as a simple intro project for learning Rust

mod password;
mod database;
pub mod config;

use std::error::Error;

use crate::config::Config;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
}