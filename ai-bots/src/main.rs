// extern crate dotenv;

use dotenv::dotenv;
use std::env;
use reqwest;
use tokio;
use crate::openai::{client::Client, message::Message};

mod openai;

#[tokio::main]
async fn main() {
    dotenv().ok();
    // _debug().await;
    let openai_token = env::var("OPENAI_API_KEY").unwrap();

    let client = Client::new(&openai_token);

    let mut messages1 = vec![
        Message {
            role: "system".to_string(),
            content: "You are Bob, a smart boy, and fell in love with Alice. 后面请用中文回复我。".to_string(),
        },
        Message {
            role: "user".to_string(),
            content: "Hello, I'm Alice!".to_string(),
        },
    ];

    let mut messages2 = vec![
        Message {
            role: "system".to_string(),
            content: "You are Alice, a smart girl, and fell in love with Bob. 后面请用中文回复我。".to_string(),
        },
    ];

    let mut i = 0;

    loop {
        let messages = if i % 2 == 0 {
            &mut messages1
        } else {
            &mut messages2
        };
        let res = client.chat(&messages).await;
        let ret_msg: &str = &res.unwrap().clone();
        println!("{:?}", ret_msg);
        if i % 2 == 0 {
            messages1.push(Message {
                role: "assistant".to_string(),
                content: ret_msg.to_string(),
            });
            messages2.push(Message {
                role: "user".to_string(),
                content: ret_msg.to_string(),
            });
        } else {
            messages2.push(Message {
                role: "assistant".to_string(),
                content: ret_msg.to_string(),
            });
            messages1.push(Message {
                role: "user".to_string(),
                content: ret_msg.to_string(),
            });
        }
        i += 1;
        if i > 10 {
            break;
        }
    }
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
