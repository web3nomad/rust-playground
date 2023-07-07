use serde_json::{json};
use futures_util::StreamExt;
use serde::Deserialize;
use reqwest;
use super::message::Message;
use std::io::Write;

pub struct Client {
    openai_token: String,
}

#[derive(Debug, Deserialize)]
struct ChatChunkDelta {
    content: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ChatChunkChoice {
    delta: ChatChunkDelta,
    // index: usize,
    // finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ChatCompletionChunk {
    // id: String,
    // object: String,
    // created: usize,
    // model: String,
    choices: Vec<ChatChunkChoice>,
}

impl Client {
    pub fn new(openai_token: &str) -> Client {
        Client {
            openai_token: openai_token.to_string(),
        }
    }

    /// <https://github.com/a-poor/openai-stream-rust-demo/blob/main/src/main.rs>
    async fn send_request(&self, messages: Vec<serde_json::Value>) -> Result<String, reqwest::Error> {
        let model_name = "TezignOpenAI-GPT-35";
        let version = "2023-03-15-preview";
        let endpoint = "tezignopenai";
        let url = format!(
            "https://{}.openai.azure.com/openai/deployments/{}/chat/completions?api-version={}",
            endpoint, model_name, version);
        let payload = json!({
            "messages": messages,
            "stream": true,
            "temperature": 1,
        });
        let res = reqwest::Client::new()
            .post(url)
            .header("Content-Type", "application/json")
            .header("api-key", &self.openai_token)
            .json(&payload)
            .send()
            .await?;
        println!("status = {}", res.status());
        let mut final_content: String = "".to_string();
        let mut stream = res.bytes_stream();
        while let Some(item) = stream.next().await {
            let item = item?;
            let s = match std::str::from_utf8(&item) {
                Ok(v) => v,
                // Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
                Err(_) => continue,
            };
            for p in s.split("\n\n") {
                match p.strip_prefix("data: ") {
                    Some(p) => {
                        // Check if the stream is done...
                        if p == "[DONE]" {
                            break;
                        }
                        if let Ok(d) = serde_json::from_str::<ChatCompletionChunk>(p) {
                            // Is there data?
                            let c = d.choices.get(0).expect("No choice returned");
                            if let Some(content) = &c.delta.content {
                                print!("{}", content);
                                final_content += content;
                            }
                        }
                        if let Err(error) = std::io::stdout().flush() {
                            panic!("{}", error);
                        }
                    }
                    None => {}
                }
            }
        }
        println!("\n");
        Ok(final_content)
    }

    pub async fn chat(&self, history_messages: &Vec<Message>) -> Result<String, reqwest::Error> {
        let messages: Vec<serde_json::Value> = history_messages.iter().map(|message| {
            // println!("{}: {}", message.role, message.content);
            json!({
                "role": message.role,
                "content": message.content
            })
        }).collect();
        let content = self.send_request(messages).await?;
        Ok(content)
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
