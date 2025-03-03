use std::io::{stdin, stdout, Write};

#[derive(Debug, PartialEq)]
pub enum Command {
    Log,
    View,
    Update, 
    Delete,
    Help,
    Exit,
    Unknown(String),
}

pub fn parse_command(input: &str) -> Command {
    match input.trim().to_lowercase().as_str() {
        "log" => Command::Log, 
        "view" => Command::View,
        "update" => Command::Update,
        "delete" => Command::Delete,
        "help" => Command::Help,
        "exit" => Command::Exit,
        other => Command::Unknown(other.to_string()),
    }
}

pub fn print_help() {
    println!("$ ************** Available Commands **************");
    println!("$ Type 'man' command for a detailed description of the command");
    println!("1. log"); 
    println!("2. view");
    println!("4. update");
    println!("5. delete");
    println!("6. exit"); 
}

pub fn log_workout() {
    println!("$ Please input exercise_name <reps> <sets> <percieved_effort> <description>");
    
}

pub fn view_workout() {
    println!("$ Please enter date range or date");
}

pub fn update_workout() {
    println!("$ Please enter workout id");
}

pub fn delete_workout() {
    println!("$ Please enter workout id");
}

pub fn run_cli() {
    println!("$ Welcome to Workout tracker!");
    
    let mut input = String::new();

    loop {
        println!("$ Type HELP for a list of commands");
        print!("$ ");
        stdout().flush().unwrap();

        input.clear();
        stdin().read_line(&mut input).expect("Error reading input");

        let command = parse_command(&input);

        match command {
            Command::Log => log_workout(),
            Command::View => view_workout(),
            Command::Update => update_workout(),
            Command::Delete => delete_workout(),
            Command::Help => print_help(),
            Command::Exit => {
                println!("$ Exiting workout_tracker...");
                break;
            }
            Command::Unknown(cmd) => println!("$ Unknown Command: {}", cmd),
        }
    }
}
