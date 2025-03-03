use std::io::{stdin, stdout, Write};

#[derive(Debug, PartialEq)]
pub enum Command {
    Help,
    Exit,
    Unknown(String),
}

pub fn parse_command(input: &str) -> Command {
    match input.trim().to_lowercase().as_str() {
        "help" => Command::Help,
        "exit" => Command::Exit,
        other => Command::Unknown(other.to_string()),
    }
}

pub fn print_help() {
    println!("$ Inside Help Function");
}

pub fn run_cli() {
    println!("$ Welcome to Workout tracker!");
    println!("$ Type HELP for a list of commands");
    
    let mut input = String::new();

    loop {
        print!("$ ");
        stdout().flush().unwrap();

        input.clear();
        stdin().read_line(&mut input).expect("Error reading input");

        let command = parse_command(&input);

        match command {
            Command::Help => print_help(),
            Command::Exit => {
                println!("$ Exiting workout_tracker...");
                break;
            }
            Command::Unknown(cmd) => println!("$ Unknown Command: {}", cmd),
        }
    }
}
