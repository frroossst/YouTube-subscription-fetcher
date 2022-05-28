use serde::{Serialize, Deserialize);
#[tokio::main]

struct subs {
    subs = []
}

async fn main() -> Result<(), reqwest::Error>{
    println!("Hello, world!");
    let api_response : String = reqwest::Client::new()
        .get("https://navxe.herokuapp.com/api")
        .send()
        .await?
        .text()
        .await?;

    println!("{:?}",api_response);

    Ok(())
}
