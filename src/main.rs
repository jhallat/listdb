use std::io;
use std::io::prelude::*;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use properties::Properties;

mod properties;

const DATA_HOME_PROPERTY : &str = "data.home";

const PROPERTY_FILE : &str = "listdb.properties";

fn main() {

    let mut properties = Properties::new();
    properties.load(PROPERTY_FILE);
    let data_home = properties.get(DATA_HOME_PROPERTY);
    let passed = health_check(&data_home);
    if passed {
        println!("Health check passed");
    } else {
        println!("I am not feeling well. I am going to rest now.");
    }
    loop {
        display_prompt();
        let line = read_line();
        let command_line: Vec<&str> = line.split(' ').collect();
        let command: &str = &command_line[0].to_string().trim().to_uppercase();
        match command {
            "EXIT" => break,
            "CREATE" => create_command(&command_line[1..]),
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

fn create_command(args: &[&str]) {

    if args.len() != 2 {
        println!("You messed up!!! Create takes two parameters.")
    }    
    
    for arg in args {
        println!("{}", arg);
    }
}

fn health_check(db_home: &str) -> bool {
    if !Path::new(&db_home).exists() {
        match fs::create_dir_all(&db_home) {
            Ok(_) => return true,
            Err(_) => return false
        }
    }
    return true
}
