extern crate uuid;

use std::io;
use std::io::prelude::*;
use std::path::Path;
use properties::Properties;
use topic::Topics;
use std::fs;

mod properties;
mod log_constants;
mod topic;

const DATA_HOME_PROPERTY : &str = "data.home";

const PROPERTY_FILE : &str = "listdb.properties";

fn main() {

    let mut properties = Properties::new();
    properties.load(PROPERTY_FILE);
    let db_home = properties.get(DATA_HOME_PROPERTY);
    let topics = Topics {
        db_home: db_home.clone()
    };
    let passed = health_check(&db_home);
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
            "CREATE" => create_command(&topics, &command_line[1..]),
            "LIST" => list(&topics, &command_line[1..]),
            "STATUS" => display_status(&properties),
            "OPEN" => open_item(&topics, &command_line[1..]),
            _ => println!("{} I just don't understand you", log_constants::ERROR_LABEL)
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

fn display_status(properties: &Properties) {
    let contents = properties.contents();
    println!("");
    println!("Properties");
    println!("----------------------------------------------");
    println!("{}", contents);
}

fn list(topics: &Topics, args: &[&str]) {
    if args.len() == 0 {
        println!("{} I need to know what you want a list of.", log_constants::ERROR_LABEL);
        return
    }
    if args.len() > 1 {
        println!("{} Only one thing at a time please.", log_constants::ERROR_LABEL);
        return
    }
    let target: &str = &args[0].to_string().trim().to_uppercase();
    match target {
        "TOPIC" | "TOPICS" => topics.list(),
        _ => println!("{} NOOOOO!!!!! That is not an option.", log_constants::ERROR_LABEL)
    }
}


fn open_item(topics: &Topics, args: &[&str]) {
    
    if args.len() != 2 {
        println!("{} You need to tell me where to go", log_constants::ERROR_LABEL);
        return
    }
    let target: &str = &args[0].to_string().trim().to_uppercase();
    let target_id: &str = &args[1].to_string().trim().to_string();
    match target {
        "TOPIC" => topics.open(target_id),
        _ => println!("{} NOOOOO!!!!! That is not an option.", log_constants::ERROR_LABEL)
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

fn create_command(topics: &Topics, args: &[&str]) {

    if args.len() != 2 {
        println!("{} You messed up!!! Create takes two parameters.", log_constants::ERROR_LABEL);
        return
    }    
    
    let target: &str = &args[0].to_string().trim().to_uppercase();
    match target {
        "TOPIC" => topics.create(args[1]),
        _ => println!("{} I don't know how to create a {}", log_constants::ERROR_LABEL, args[0])
    }

}