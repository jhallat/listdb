use std::path::Path;
use std::fs::File;
use std::fs;
use std::io;
use std::io::prelude::*;
use std::fs::OpenOptions;
use std::io::Write;
use uuid::Uuid;
use std::collections::HashMap;

#[derive(Clone)]
struct Record {
  id: String,
  action: String, 
  content: String
}


struct Topic {
  path: String,
  id: String,
  line_map: HashMap<usize, Record> 
}

impl Topic {

  pub fn new(topic_id: &str, topic_path: &str) -> Topic {
    let mut topic = Topic {
      path: topic_path.to_string(),
      id: topic_id.to_string(),
      line_map: HashMap::new()
    };
    let records = Topic::get_records(topic_path);
    for (index, record) in records.iter().enumerate() {
      topic.line_map.insert(index + 1, record.clone());
    }
    return topic;
  }

  fn get_records(path: &str) -> Vec<Record> {
    let mut file = OpenOptions::new().read(true).open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let mut lines: Vec<&str> = contents.split('\n').collect();
    lines.pop();
    let mut records: Vec<Record> = Vec::new();
    for line in lines {
      let record = Record {
        id: line[0..36].to_string(),
        action: line[36..37].to_string(),
        content: line[37..].to_string()
      };
      records.push(record);
    }
    return records;
  }

  fn display_prompt(topic_id: &str) {
      print!("({})> ", topic_id);
      io::stdout().flush().expect("Failed to flush stdout");
  }

  fn read_line() -> String {
      let mut input = String::new();
      io::stdin()
          .read_line(&mut input)
          .expect("Failed to read line");
      input.trim().to_string()    
  }

  fn append_data(&self, record: &Record) {
    let output = format!("{}{}{}\n", record.id, record.action, record.content);
    let mut file = OpenOptions::new().append(true).open(&self.path).unwrap();
    file.write_all(output.as_bytes()).expect("Add failed");
  }

  fn add(&self, args: &[&str]) {
    if args.len() == 0 {
      println!("Nothing to add");
      return
    } 
    let output = args.join(" ");
    let id = Uuid::new_v4();
    let record = Record {
      id: id.to_string(),
      action: "A".to_string(),
      content: output
    };
    self.append_data(&record);
  }

  fn delete(&self, args: &[&str]) {
    if args.len() == 0 {
      println!("Nothing to delete. Line number required");
      return
    }
    let index = args[0].to_string().parse::<usize>().unwrap();
    let record = &self.line_map.get(&index);
    if record.is_some() {
      let selected_record = record.unwrap();
      let deleted_record = Record {
        id: selected_record.id.clone(),
        action: "D".to_string(),
        content: "-".to_string()
      };
      self.append_data(&deleted_record);
    } else {
      println!("No item found at position {}", index);
    }
    
  }

  fn list(&self) {
    let mut file = OpenOptions::new().read(true).open(&self.path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let mut lines: Vec<&str> = contents.split('\n').collect();
    lines.pop();
    println!("----------------------------------------------");
    for (i, line) in lines.iter().enumerate() {
      let output: &str = &line[37..];
      println!("{}: {}", i + 1, output);
    }
    println!("----------------------------------------------");
  }

  fn open(&self) {
    loop {
      Topic::display_prompt(&self.id);
      let line = Topic::read_line();
      let command_line: Vec<&str> = line.split(' ').collect();
      let command: &str = &command_line[0].to_string().trim().to_uppercase();
      match command {
        "CLOSE" => break,
        "ADD" => self.add(&command_line[1..]),
        "DELETE" => self.delete(&command_line[1..]),
        "LIST" => self.list(),
        _ => println!("Not a valid command")
      }
    }

  }

}

/// Manages topics in the database
pub struct Topics {
  /// Location of the database
  pub db_home: String
}

impl Topics {

  /// Creates a topic in the database. The name of the topic must be
  /// unique. If the topic already exists, a new topic will not be created.
  /// 
  /// # Arguments
  /// 
  /// * `topic_id` - The name of the new topic. 
  pub fn create(&self, topic_id: &str) {

      if self.topic_exists(&topic_id) {
          //TODO Should return a status, not implement a side effect.
          println!("The topic {} already exists.", topic_id);
          return
      }
      match File::create(self.topic_path(topic_id)) {
          //TODO Should return a status, not implement a side effect.
          Ok(_) => println!("Topic {} created.", topic_id),
          Err(_) => println!("Error occured creating topic {}", topic_id)
      }
  }

  pub fn list(&self) {

      let files = fs::read_dir(self.db_home.clone()).unwrap();
      for file in files {
         let path = file.unwrap().path();
         let topic_name = path.file_stem().unwrap().to_str().unwrap(); 
         let topic_type = path.extension().unwrap().to_str().unwrap();
         if topic_type == "tpc" {
            println!("{}", topic_name);
        }
      }
  }

  fn topic_path(&self, topic_id: &str) -> String {
    format!("{}\\{}.tpc", self.db_home, topic_id)
  }

  fn topic_exists(&self, topic_id: &str) -> bool {
    let topic_path = self.topic_path(topic_id);
    Path::new(&topic_path).exists()
  }

  pub fn open(&self, topic_id: &str) {
    if !self.topic_exists(&topic_id) {
      println!("{} does not exist.", topic_id);
      return
    }
    let topic_path = self.topic_path(topic_id);
    let topic = Topic::new(topic_id, &topic_path);
    topic.open();
  }
}