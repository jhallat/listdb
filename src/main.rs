use std::io;
use std::io::prelude::*;

fn main() {
    loop {
        display_prompt();
        let line = read_line();
        let command_line: Vec<&str> = line.split(' ').collect();
        let command: &str = &command_line[0].to_string().trim().to_uppercase();
        match command {
            "EXIT" => break,
            "CREATE" => create_command(command_line[1..].to_vec()),
            _ => println!("I just don't understand you")
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

fn create_command(args: Vec<&str>) {

    if args.len() != 2 {
        println!("You messed up!!! Create takes two parameters.")
    }    
    
    for arg in &args {
        println!("{}", arg);
    }
}