use serde::{Serialize};
use serde_json;
use std::fs;

struct Subscriptions {
    channels_list: Vec<String>
}

fn main() {

    channel_subscriptions Vec<String> = vec!::New();
    
    // Reading JSON
    
    let data = fs::read_to_string("./subscriptions.json").expect("Unable to read file");

    let json : serde_json::Value = serde_json::from_str(&data).expect("JSON does not have correct format");

    println!("{}",json);

}
