use std::path::Path;
use std::fs::File;
use log_constants::ERROR_LABEL;

pub fn create_command(db_home: &str, args: &[&str]) {

    if args.len() != 2 {
        println!("{} You messed up!!! Create takes two parameters.", ERROR_LABEL);
    }    
    
    let target: &str = &args[0].to_string().trim().to_uppercase();
    match target {
        "TOPIC" => create_topic(&db_home, args[1]),
        _ => println!("{} I don't know how to create a {}", ERROR_LABEL, args[0])
    }

}


fn create_topic(db_home: &str, topic_id: &str) {
    let topic_path = format!("{}\\{}.tpc", db_home, topic_id);
    let exists = Path::new(&topic_path).exists();
    if exists {
        //Should return a status, not implement a side effect.
        println!("The topic {} already exists.", topic_path);
        return
    }
    match File::create(topic_path) {
        Ok(file) => println!("Topic {} created.", topic_id),
        Err(error) => println!("Error occured creating topic {}", topic_id)
    }
    
}