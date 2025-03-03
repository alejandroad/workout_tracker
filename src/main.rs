mod db_utils;
mod models;
use crate::db_utils::{connect_to_db, add_dummy_data};
use rusqlite::{Result};

fn run_cli() {
    use std::io::{stdin, stdout, Write};
    println!("$ Welcome to Workout tracker!");
    println!("$ Type HELP for a list of commands");
    let mut input = String::new();
    let _=stdout().flush();
    loop {
        input.clear();
        stdin().read_line(&mut input).expect("Error reading input");

        if input.trim() == "HELP" {
            println!("User entered: {}", input);
            break;
        }

        println!("Invalid command, try again!");
    }
}

fn main() -> Result<()> {
    println!("Hello, world! Welcome to Workout Tracker! Author: Alejandro Arias");

    let _conn = connect_to_db()?;
    println!("Database connection successful!");

    println!("Initializing databse with dummy values...");
   
    run_cli();

    Ok(())
}