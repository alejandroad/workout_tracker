mod db_utils;
mod models;
mod cli;
use crate::db_utils::{connect_to_db};
use crate::cli::{run_cli};
use rusqlite::{Result};

fn main() -> Result<()> {
    println!("Hello, world! Welcome to Workout Tracker! Author: Alejandro Arias");

    let _conn = connect_to_db()?;
    println!("Database connection successful!");

    println!("Initializing databse with dummy values...");
   
    run_cli();

    Ok(())
}