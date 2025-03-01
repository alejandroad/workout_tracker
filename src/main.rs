use rusqlite::{Connection, Result};

fn connect_to_db() -> Result<Connection> {
    let conn = Connection::open("workout_tracker.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS workout_entriess (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            exercise_name TEXT NOT NULL,
            sets INTEGER NOT NULL,
            reps INTEGER NOT NULL,
            perceived_effort INTEGER NOT NULL,
            date TEXT NOT NULL UNIQUE, 
            descriptions TEXT
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

    Ok(())
}