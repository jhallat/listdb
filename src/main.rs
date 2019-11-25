extern crate listdb_engine;

extern crate env_logger;
extern crate log;

use listdb_engine::dbprocess::DBResponse::*;
use listdb_engine::DBEngine;
use log::debug;
use properties::Properties;
use std::collections::HashMap;
use std::fs;
use std::io;
use std::io::prelude::*;
use std::path::Path;

mod properties;

const DATA_HOME_PROPERTY: &str = "data.home";

const PROPERTY_FILE: &str = "listdb.properties";

fn main() {
    env_logger::init();
    let mut line_map: HashMap<usize, String> = HashMap::new();
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
            let command_line = intercept(&line, &line_map);
            //TODO handle deletes with line map
            debug!("{}", command_line);
            match db_engine.request(&command_line) {
                Unknown(message) => invalid(&format!("Unknown request {}", message)),
                Exit => break,
                Data(data) => data_table(&mut line_map, &data),
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

fn intercept(item: &str, line_map: &HashMap<usize, String>) -> String {
    let test_string = item.trim_start();
    if test_string.to_ascii_uppercase().starts_with("DELETE")
        || test_string.to_ascii_uppercase().starts_with("UPDATE")
    {
        let mut tokens: Vec<&str> = item.split(" ").collect();
        if tokens.len() < 2 {
            return item.to_string(); //This string is invalid anyway
        }
        let command = tokens.remove(0);
        let line_no = tokens.remove(0);
        if !line_no.parse::<usize>().is_ok() {
            return item.to_string(); //This string is invalid anyway
        }
        let line_id = line_no.parse::<usize>().unwrap();
        let id = line_map.get(&line_id).unwrap();

        return format!("{} {} {}", command, id, tokens.join(" "));
    }
    item.to_string()
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

fn left_pad(item: &str, count: usize) -> String {
    let mut padded_string = String::new();
    if padded_string.len() < count - item.len() {
        for _ in padded_string.len()..count {
            padded_string.push_str(" ");
        }
    }
    padded_string.push_str(item);
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
        "(\\)".to_string()
    } else {
        format!("({})", context)
    };
    print!("{}> ", prompt);
    io::stdout().flush().expect("Failed to flush stdout");
}

fn data_table(line_map: &mut HashMap<usize, String>, data: &Vec<(String, String)>) {
    let mut longest_value = 0;
    for (_, value) in data {
        if value.len() > longest_value {
            longest_value = value.len();
        }
    }
    let line_no_text = data.len().to_string();
    let line_no_size = line_no_text.len();
    let top_border_data = repeat("\u{2500}", longest_value + 2);
    let top_border_line = repeat("\u{2500}", line_no_size + 2);
    println!(
        "\u{250C}{}\u{252C}{}\u{2510}",
        top_border_line, top_border_data
    );
    for (line, (key, value)) in data.iter().enumerate() {
        let padded_line = left_pad(&(line + 1).to_string(), line_no_size);
        let padded_value = right_pad(value, longest_value);
        println!(
            "\u{2502} {} \u{2502} {} \u{2502}",
            padded_line, padded_value
        );
        line_map.insert(line + 1, key.to_string());
    }
    println!(
        "\u{2514}{}\u{2534}{}\u{2518}",
        top_border_line, top_border_data
    );
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
