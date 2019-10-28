use std::io;
use std::io::prelude::*;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use properties::Properties;

mod properties;
mod object_creation;
mod log_constants;

const DATA_HOME_PROPERTY : &str = "data.home";

const PROPERTY_FILE : &str = "listdb.properties";

fn main() {

    let mut properties = Properties::new();
    properties.load(PROPERTY_FILE);
    let db_home = properties.get(DATA_HOME_PROPERTY);
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
            "CREATE" => object_creation::create_command(&db_home, &command_line[1..]),
            "LIST" => list(&db_home, &command_line[1..]),
            "STATUS" => display_status(&properties),
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

fn list(db_home: &str, args: &[&str]) {
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
        "TOPIC" | "TOPICS" => list_topics(&db_home),
        _ => println!("{} NOOOOO!!!!! That is not an option.", log_constants::ERROR_LABEL)
    }
}

fn list_topics(db_home: &str) {

    let files = fs::read_dir(&db_home).unwrap();
    for file in files {
       let path = file.unwrap().path();
       let topic_name = path.file_stem().unwrap().to_str().unwrap(); 
       let topic_type = path.extension().unwrap().to_str().unwrap();
       if (topic_type == "tpc") {
          println!("{}", topic_name);
       }
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
