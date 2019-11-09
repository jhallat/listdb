extern crate listdb_engine;

use listdb_engine::dbprocess::DBResponse::*;
use listdb_engine::DBEngine;
use properties::Properties;
use std::fs;
use std::io;
use std::io::prelude::*;
use std::path::Path;

mod properties;

const DATA_HOME_PROPERTY: &str = "data.home";

const PROPERTY_FILE: &str = "listdb.properties";

fn main() {
    let mut properties = Properties::new();
    let mut context = "".to_string();
    properties.load(PROPERTY_FILE);
    let db_home = properties.get(DATA_HOME_PROPERTY);
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
        match db_engine.request(&line) {
            Unknown => println!("INVALID: Unknown request"),
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

fn health_check(db_home: &str) -> bool {
    if !Path::new(&db_home).exists() {
        match fs::create_dir_all(&db_home) {
            Ok(_) => return true,
            Err(_) => return false,
        }
    }
    return true;
}
