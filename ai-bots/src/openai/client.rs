use serde_json::json;
use reqwest;
use super::message::Message;

pub struct Client {
    openai_token: String,
}

impl Client {
    pub fn new(openai_token: &str) -> Client {
        Client {
            openai_token: openai_token.to_string(),
        }
    }
    pub async fn chat(&self, history_messages: &Vec<Message>) -> Result<String, reqwest::Error> {
        // println!("{:?}", history_messages);
        let messages: Vec<serde_json::Value> = history_messages.iter().map(|message| {
            json!({
                "role": message.role,
                "content": message.content
            })
        }).collect();
        let body = reqwest::Client::new()
            .post("https://api.openai.com/v1/chat/completions")
            .bearer_auth(&self.openai_token)
            .json(&json!({
                "model": "gpt-3.5-turbo",
                "messages": messages
            }))
            .send()
            .await?
            .text()
            .await?;
        // println!("body = {:?}\n", body);
        let json_data: serde_json::Value = serde_json::from_str(&body).unwrap();
        let message_value = &(json_data["choices"][0]["message"]["content"]);
        let response = message_value.as_str().unwrap().to_string();
        Ok(response)
    }
}

/* --- */

async fn _request_openai(openai_token: &str) -> Result<String, reqwest::Error> {
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
