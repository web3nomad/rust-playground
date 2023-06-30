extern crate dotenv;

use reqwest;
use dotenv::dotenv;
use std::env;
use tokio;

async fn get_body() -> Result<String, reqwest::Error> {
    // call openai api using gpt-3.5-turbo model, with native request crate
    // let url = "https://api.openai.com/v1/engines/davinci-codex/completions";

    let body = reqwest::get("https://dxd1234.heidianer.com/.ping")
        .await?
        .text()
        .await?;

    println!("body = {:?}", body);

    Ok(body)
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    for (key, value) in env::vars() {
        if key == "OPENAI_API_KEY" {
            println!("{}: {}", key, value);
        }
    }

    get_body().await.unwrap();
}
