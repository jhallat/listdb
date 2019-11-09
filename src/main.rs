extern crate chrono;
extern crate listdb_engine;
extern crate uuid;

use listdb_engine::dbprocess::DBResponse::*;
use listdb_engine::DBEngine;
use properties::Properties;
use std::fs;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use topic::Topics;

mod log_constants;
mod properties;
mod topic;

const DATA_HOME_PROPERTY: &str = "data.home";

const PROPERTY_FILE: &str = "listdb.properties";

fn main() {
    let mut properties = Properties::new();
    let mut context = "".to_string();
    properties.load(PROPERTY_FILE);
    let db_home = properties.get(DATA_HOME_PROPERTY);
    let topics = Topics {
        db_home: db_home.clone(),
    };
    let passed = health_check(&db_home);
    let mut db_engine = DBEngine::new(&db_home);
    if passed {
        println!("Health check passed");
    } else {
        println!("I am not feeling well. I am going to rest now.");
    }
    loop {
        display_prompt(&context);
        let line = read_line();
        let command_line: Vec<&str> = line.split(' ').collect();
        match db_engine.request(&line) {
            Unknown => {
                let command: &str = &command_line[0].to_string().trim().to_uppercase();
                match command {
                    "COMPACT" => compact_item(&topics, &command_line[1..]),
                    _ => println!("{} I just don't understand you", log_constants::ERROR_LABEL),
                }
            }
            Exit => break,
            Data(data) => display_data(data),
            ROk(message) => println!("{}", message),
            Invalid(message) => println!("INVALID: {}", message),
            Error(message) => println!("ERROR: {}", message),
            OpenContext(message) => context = message.clone(),
            CloseContext => println!("Not expecting this response"),
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

fn display_prompt(context: &str) {
    let prompt = if context.len() == 0 {
        "".to_string()
    } else {
        format!("({})", context)
    };
    print!("{}> ", prompt);
    io::stdout().flush().expect("Failed to flush stdout");
}

fn display_data(data: Vec<String>) {
    println!("----------------------------------------------");
    for item in data {
        println!("{}", item);
    }
    println!("----------------------------------------------");
}

fn compact_item(topics: &Topics, args: &[&str]) {
    if args.len() != 2 {
        println!(
            "{} OPEN requires a type (i.e \"TOPIC\") and id",
            log_constants::ERROR_LABEL
        );
        return;
    }
    let target: &str = &args[0].to_string().trim().to_uppercase();
    let target_id: &str = &args[1].to_string().trim().to_string();
    match target {
        "TOPIC" => topics.compact(target_id),
        _ => println!(
            "{} {} Is not a valid type.",
            log_constants::ERROR_LABEL,
            target
        ),
    }
}

fn health_check(db_home: &str) -> bool {
    if !Path::new(&db_home).exists() {
        match fs::create_dir_all(&db_home) {
            Ok(_) => return true,
            Err(_) => return false,
        }
    }
    return true;
}
