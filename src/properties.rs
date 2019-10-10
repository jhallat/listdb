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

    pub fn load(&mut self) {
       self.property_map.clear();
       let contents = fs::read_to_string("listdb.properties")
            .expect("I can read the properties file :-(");
       let property_lines: Vec<&str> = contents.split('\n').collect();
        for property_line in property_lines {
            let property_values: Vec<&str> = contents.split('=').collect();
            self.property_map.insert(property_values[0].to_string(), property_values[1].to_string());
        }
    }

    pub fn get(&self, key: String) -> String {
       match self.property_map.get(&key) {
           Some(value) => value.to_string(),
           None => "".to_string()
       }
    }

}
