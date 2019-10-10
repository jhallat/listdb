  use std::fs;
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
       let contents = fs::read_to_string(prop_file_name)
            .expect("I can't read the properties file :-(");
       let property_lines: Vec<&str> = contents.split("\r\n").collect();
       for property_line in property_lines {
           let property_values: Vec<&str> = contents.split('=').collect();
           self.property_map.insert(property_values[0].to_string(), property_values[1].to_string());
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
