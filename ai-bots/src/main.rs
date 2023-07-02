extern crate dotenv;

use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();

    for (key, value) in env::vars() {
        if key == "OPENAI_API_KEY" {
            println!("{}: {}", key, value);
        }
    }
}
