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
        loop {
            display_prompt(&context);
            let line = read_line();
            match db_engine.request(&line) {
                Unknown => invalid("Unknown request"),
                Exit => break,
                Data(data) => data_table(&data),
                ROk(message) => ok(&message),
                Invalid(message) => invalid(&message),
                Error(message) => error(&message),
                OpenContext(message) => context = message.clone(),
                CloseContext => println!("Not expecting this response"),
            }
        }
    } else {
        error("Unable to access data folder");
    }
}

fn repeat(item: &str, count: usize) -> String {
    let mut repeated = String::new();
    for _ in 0..count {
        repeated.push_str(item);
    }
    repeated
}

fn right_pad(item: &str, count: usize) -> String {
    let mut padded_string = String::new();
    padded_string.push_str(item);
    if padded_string.len() < count {
        for _ in padded_string.len()..count {
            padded_string.push_str(" ");
        }
    }
    padded_string
}

fn error(message: &str) {
    println!("ERROR: {}", message);
}

fn invalid(message: &str) {
    println!("INVALID: {}", message);
}

fn ok(message: &str) {
    println!("OK: {}", message);
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

fn data_table(data: &Vec<String>) {
    let mut longest = 0;
    for item in data {
        if item.len() > longest {
            longest = item.len();
        }
    }
    let top_border = repeat("\u{2500}", longest + 2);
    println!("\u{250C}{}\u{2510}", top_border);
    for item in data {
        let padded = right_pad(&item, longest);
        println!("\u{2502} {} \u{2502}", padded);
    }
    println!("\u{2514}{}\u{2518}", top_border);
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
