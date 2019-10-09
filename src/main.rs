use std::io;
use std::io::prelude::*;

fn main() {
    display_prompt();
    let name = read_line();
    execute_command(name);

}

fn read_line() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()    
}

fn display_prompt() {
    print!(">");
    io::stdout().flush().expect("Failed to flush stdout");
}

fn execute_command(command: String) {
    println!("Command: {}", command);
}
