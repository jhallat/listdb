use std::path::Path;
use std::fs::File;
use std::fs;
use std::io;
use std::io::prelude::*;
use std::fs::OpenOptions;
use std::io::Write;
use uuid::Uuid;

struct Topic {
  path: String,
  id: String
}

impl Topic {

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

  fn add(&self, args: &[&str]) {
    if args.len() == 0 {
      println!("Nothing to add");
      return
    } 
    let output = args.join(" ");
    let id = Uuid::new_v4();
    let output = format!("{}{}\n", id, output);
    let mut file = OpenOptions::new().append(true).open(&self.path).unwrap();
    file.write_all(output.as_bytes()).expect("Add failed");
  }

  fn list(&self) {
    let mut file = OpenOptions::new().read(true).open(&self.path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let mut lines: Vec<&str> = contents.split('\n').collect();
    lines.pop();
    println!("----------------------------------------------");
    for (i, line) in lines.iter().enumerate() {
      println!("{}: {}", i + 1, line);
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
      let topic_path = self.topic_path(topic_id);
      match File::create(self.topic_path(topic_id)) {
          //TODO Should return a status, not implement a side effect.
          Ok(file) => println!("Topic {} created.", topic_id),
          Err(error) => println!("Error occured creating topic {}", topic_id)
      }
  }

  pub fn list(&self) {

      let files = fs::read_dir(self.db_home.clone()).unwrap();
      for file in files {
         let path = file.unwrap().path();
         let topic_name = path.file_stem().unwrap().to_str().unwrap(); 
         let topic_type = path.extension().unwrap().to_str().unwrap();
         if (topic_type == "tpc") {
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
    let topic = Topic {
      path: topic_path.to_string(),
      id: topic_id.to_string()
    };
    topic.open();
  }
}