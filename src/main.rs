mod models;
use models::workout_model::WorkoutEntry;
use rusqlite::{Connection, Result};

fn add_dummy_data(conn: &Connection) -> Result<()> {
    let workout_entry = WorkoutEntry {
        exercise_name: "Bench Press".to_string(),
        weight: 135,
        sets: 3,
        reps: 10,
        perceived_effort: 8,
        date: "2025-02-28".to_string(),
        description: "".to_string()
    };

    conn.execute(
        "INSERT INTO workout_entries (exercise_name, weight, sets, reps, perceived_effort, date, description) 
              VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        (   
            workout_entry.exercise_name, 
            workout_entry.weight, 
            workout_entry.sets, 
            workout_entry.reps, 
            workout_entry.perceived_effort, 
            workout_entry.date,
            workout_entry.description
        )
    )?;

    println!("Successfully added dummy data to database!");
    Ok(())
}

fn connect_to_db() -> Result<Connection> {
    let conn = Connection::open("workout_tracker.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS workout_entries (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            exercise_name TEXT NOT NULL,
            weight INTEGER NOT NULL,
            sets INTEGER NOT NULL,
            reps INTEGER NOT NULL,
            perceived_effort INTEGER NOT NULL,
            date TEXT NOT NULL UNIQUE, 
            description TEXT
        )",
        [],
    )?;

    println!("Database table creation complete.");
    Ok(conn)
}

fn main() -> Result<()> {
    println!("Hello, world! Welcome to Workout Tracker! Author: Alejandro Arias");

    let _conn = connect_to_db()?;
    println!("Database connection successful!");

    println!("Initializing databse with dummy values...");
    add_dummy_data(&_conn)?;

    Ok(())
}