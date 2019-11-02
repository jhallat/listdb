use std::path::Path;
use std::fs::File;
use std::fs;

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
      let topic_path = format!("{}\\{}.tpc", self.db_home, topic_id);
      let exists = Path::new(&topic_path).exists();
      if exists {
          //TODO Should return a status, not implement a side effect.
          println!("The topic {} already exists.", topic_path);
          return
      }
      match File::create(topic_path) {
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

  fn topic_exists(&self, topic_id: &str) {

  }

  pub fn goto_topic(topic_id: &str) {

  }
}