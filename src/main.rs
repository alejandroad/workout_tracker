mod db_utils;
mod models;
use crate::db_utils::{connect_to_db, add_dummy_data};
use rusqlite::{Result};

fn main() -> Result<()> {
    println!("Hello, world! Welcome to Workout Tracker! Author: Alejandro Arias");

    let _conn = connect_to_db()?;
    println!("Database connection successful!");

    println!("Initializing databse with dummy values...");
    add_dummy_data(&_conn)?;

    Ok(())
}