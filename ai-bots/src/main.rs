// extern crate dotenv;

use dotenv::dotenv;
use std::env;
use reqwest;
use tokio;
use crate::openai::client::Client;

mod openai;

#[tokio::main]
async fn main() {
    dotenv().ok();
    // _debug().await;
    let openai_token = env::var("OPENAI_API_KEY").unwrap();
    let client = Client::new(openai_token);
    let res = client.chat().await;
    println!("res = {:?}", res.unwrap());
}

async fn _get_body_simple() -> Result<String, reqwest::Error> {
    let body = reqwest::get("https://dxd1234.heidianer.com/.ping")
        .await?
        .text()
        .await?;

    println!("body = {:?}", body);

    Ok(body)
}

async fn _debug() {
    for (key, value) in env::vars() {
        if key == "OPENAI_API_KEY" {
            println!("{}: {}", key, value);
        }
    }
    let res = _get_body_simple().await;
    println!("res = {:?}", res.unwrap());
}
