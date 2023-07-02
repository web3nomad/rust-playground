// extern crate dotenv;

use dotenv::dotenv;
use std::env;
use reqwest;
use serde_json::json;
use tokio;

async fn _get_body_simple() -> Result<String, reqwest::Error> {
    let body = reqwest::get("https://dxd1234.heidianer.com/.ping")
        .await?
        .text()
        .await?;

    println!("body = {:?}", body);

    Ok(body)
}

async fn request_openai(openai_token: &str) -> Result<String, reqwest::Error> {
    // call openai's chat api using gpt-3.5-turbo model, with reqwest crate
    let body = reqwest::Client::new()
        .post("https://api.openai.com/v1/chat/completions")
        .bearer_auth(openai_token)
        .json(&json!({
            "model": "gpt-3.5-turbo",
            "messages": [
                {"role": "system", "content": "You are a helpful assistant."},
                {"role": "user", "content": "Hello!"}
            ]
        }))
        .send()
        .await?
        .text()
        .await?;

    // println!("body = {:?}\n", body);

    let json_data: serde_json::Value = serde_json::from_str(&body).unwrap();
    // println!("json_data = {:?}\n", json_data);
    // println!("json_data = {:?}\n", json_data["choices"][0]["message"]["content"]);
    let message_value = &(json_data["choices"][0]["message"]["content"]);
    let response = message_value.as_str().unwrap().to_string();
    // let response = json_data["choices"][0]["message"]["content"].as_str().unwrap().to_string();

    Ok(response)
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    for (key, value) in env::vars() {
        if key == "OPENAI_API_KEY" {
            println!("{}: {}", key, value);
        }
    }

    // let res = _get_body_simple().await;
    // println!("res = {:?}", res.unwrap());

    let openai_token = env::var("OPENAI_API_KEY").unwrap();
    let res = request_openai(&openai_token).await;
    println!("res = {:?}", res.unwrap());
}
