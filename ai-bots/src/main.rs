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

    let mut a = 1;

    let mut c = || {
        a += 1;
        println!("a: {}", a);
        // println!("a");
    };

    c();

    // call_fn_once(c); // 实现了 FnOnce trait
    // call_fn(c); // 实现了Fn trait，FnMut trait,FnOnce trait,后面两种trait都是通过继承实现的
    // call_fn_mut(c); // 实现了FnMut trait,FnOnce trait
}
