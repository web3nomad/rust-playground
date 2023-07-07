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
    // let openai_token = env::var("OPENAI_API_KEY").unwrap();
    let openai_token = env::var("AZURE_OPENAI_API_KEY").unwrap();

    let client = Client::new(&openai_token);

    let _prompt_suffix = "你应该尽可能表达你的观点，对我的观点进行细化和补充。不要说任何的客气话，你的回复里不要出现“非常感谢”或者“非常同意“之类的短语，也不要和类似的短语，你应该直接说出你的观点。不要结束对话。请用中文回复我。";

    let mut messages1 = vec![
        Message {
            role: "system".to_string(),
            content: format!("你是 Bob，我是 Alice，我们正在进行一场关于 web3 和 AI 的辩论，你是一个去中心化的集大成者，追求自由的互联网和数据的所有权，你认为 web3 将主宰世界。你是一个技术大牛，说话尖酸刻薄。"),
            // content: format!("你是 Bob，我是 Alice，我们正在进行一场关于 web3 和 AI 未来的对话，你是一个去中心化的集大成者，追求自由的互联网和数据的所有权。{}", prompt_suffix),
        },
        Message {
            role: "user".to_string(),
            content: "你好，我是 Alice！".to_string(),
        },
    ];

    let mut messages2 = vec![
        Message {
            role: "system".to_string(),
            content: format!("你是 Alice，我是 Bob，我们正在进行一场关于 web3 和 AI 的辩论，你是一个人工智能的疯狂追求着，认为 AI 将主宰世界。你是一个儒雅的学者，说话很有耐心。"),
            // content: format!("你是 Alice，我是 Bob，我们正在进行一场关于 web3 和 AI 未来的对话，你是一个人工智能的疯狂追求着，认为机器将主宰世界。{}", prompt_suffix),
        },
    ];

    let mut i = 0;

    loop {
        println!("Round {}", i);
        let messages = if i % 2 == 0 {
            &mut messages1
        } else {
            &mut messages2
        };
        let res = client.chat(&messages).await;
        let ret_msg: &str = &res.unwrap();
        if i % 2 == 0 {
            messages1.push(Message {
                role: "assistant".to_string(),
                content: format!("{}", ret_msg),
            });
            messages2.push(Message {
                role: "user".to_string(),
                content: format!("{}", ret_msg),
                // content: format!("{}\n\n下面说说你的想法。{}", ret_msg, prompt_suffix),
            });
        } else {
            messages2.push(Message {
                role: "assistant".to_string(),
                content: format!("{}", ret_msg),
            });
            messages1.push(Message {
                role: "user".to_string(),
                content: format!("{}", ret_msg),
                // content: format!("{}\n\n下面说说你的想法。{}", ret_msg, prompt_suffix),
            });
        }
        i += 1;
        if i > 32 {
            break;
        }
    }
}



/* */

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
        if key == "OPENAI_API_KEY" || key == "AZURE_OPENAI_API_KEY" {
            println!("{}: {}", key, value);
        }
    }
    let res = _get_body_simple().await;
    println!("res = {:?}", res.unwrap());
}
