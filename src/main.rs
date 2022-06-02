use serde_json;
use std::fs;

fn main() {
    
    // Reading JSON
    
    let data = fs::read_to_string("./subscriptions.json").expect("Unable to read file");

    let json : serde_json::Value = serde_json::from_str(&data).expect("JSON does not have correct format");

    println!("{}",json);

}
