use std::io;
use std::io::prelude::*;

fn main() {
    loop {
        display_prompt();
        let line = read_line();
        let command_line: Vec<&str> = line.split(' ').collect();
        let command = command_line[0];
        execute_command(&command);
        if command.to_string().trim().to_uppercase() == "EXIT" {
            break;
        }
    }
}

fn read_line() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()    
}

fn display_prompt() {
    print!("> ");
    io::stdout().flush().expect("Failed to flush stdout");
}

fn execute_command(command: &str) {
    println!("Command: {}", command);
}
