use chrono::prelude::*;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::Write;
use std::path::Path;

const ACTION_DELETE: &str = "D";

#[derive(Clone)]
struct Record {
  id: String,
  action: String,
  content: String,
}

struct Topic {
  path: String,
  line_map: HashMap<usize, String>,
  record_map: HashMap<String, Record>,
}

impl Topic {
  pub fn new(topic_id: &str, topic_path: &str) -> Topic {
    let mut topic = Topic {
      path: topic_path.to_string(),
      line_map: HashMap::new(),
      record_map: HashMap::new(),
    };
    let records = Topic::get_records(topic_path);
    for record in records {
      if record.action == ACTION_DELETE {
        if topic.record_map.contains_key(&record.id) {
          topic.record_map.remove(&record.id);
        }
      } else {
        topic.record_map.insert(record.id.clone(), record.clone());
      }
    }
    let mut index = 1;
    for value in topic.record_map.values() {
      topic.line_map.insert(index, value.id.clone());
      index += 1;
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
        content: line[37..].to_string(),
      };
      records.push(record);
    }
    return records;
  }

  fn append_data(&self, record: &Record) {
    let output = format!("{}{}{}\n", record.id, record.action, record.content);
    let mut file = OpenOptions::new().append(true).open(&self.path).unwrap();
    file.write_all(output.as_bytes()).expect("Add failed");
  }

  fn refresh(&mut self) {
    self.line_map.clear();
    self.record_map.clear();

    let records = Topic::get_records(&self.path);
    for record in records {
      if record.action == ACTION_DELETE {
        if self.record_map.contains_key(&record.id) {
          self.record_map.remove(&record.id);
        }
      } else {
        self.record_map.insert(record.id.clone(), record.clone());
      }
    }
    let mut index = 1;
    for value in self.record_map.values() {
      self.line_map.insert(index, value.id.clone());
      index += 1;
    }
  }

  fn compact(&mut self) {
    let time_stamp: DateTime<Local> = Local::now();
    let move_path = format!("{}.bkp_{}", self.path, time_stamp.format("%Y%m%d_%H%M%S%f"));
    fs::rename(&self.path, &move_path);
    File::create(&self.path);
    for record in self.record_map.values() {
      self.append_data(&record);
    }
    self.refresh();
  }
}

/// Manages topics in the database
pub struct Topics {
  /// Location of the database
  pub db_home: String,
}

impl Topics {
  fn topic_path(&self, topic_id: &str) -> String {
    format!("{}\\{}.tpc", self.db_home, topic_id)
  }

  fn topic_exists(&self, topic_id: &str) -> bool {
    let topic_path = self.topic_path(topic_id);
    Path::new(&topic_path).exists()
  }

  pub fn compact(&self, topic_id: &str) {
    if !self.topic_exists(&topic_id) {
      println!("{} does not exist.", topic_id);
      return;
    }
    let topic_path = self.topic_path(topic_id);
    let mut topic = Topic::new(topic_id, &topic_path);
    topic.compact();
  }
}
