use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;

pub struct Properties {
    property_map: HashMap<String, String>
}

impl Properties {
    pub fn new() -> Properties {
         Properties {
             property_map: HashMap::new()
        }    
    } 

    pub fn load(&mut self, prop_file_name: &str) {
       self.property_map.clear();
       let file = File::open(prop_file_name)
          .expect("I tried, but I can't open the property file.");
       let buffer = BufReader::new(file);
         
       for property_line in buffer.lines() {
           let contents = property_line.unwrap();
           let property_values: Vec<&str> = contents.split('=').collect();
           println!("{:?}", property_values);
           self.property_map.insert(property_values[0].to_string().trim().to_string(), property_values[1].to_string().trim().to_string());
        }
    }

    pub fn get(&self, key: &str) -> String {
       match self.property_map.get(&key.to_string()) {
           Some(value) => value.to_string(),
           None => "".to_string()
       }
    }

}

#[cfg(test)]
mod tests {

    use super::*;
    use std::fs::File;
    use std::io::{self, BufReader};

    #[test]
    fn load_and_read_properties() {
        let mut properties = Properties::new();
        properties.load("test/resources/test.properties");
        let test_one = properties.get("test.one");
        let test_two = properties.get("test.two");
        assert_eq!(test_one, "testone");
        assert_eq!(test_two, "testtwo");
    }

}
