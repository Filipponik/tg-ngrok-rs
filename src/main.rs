pub mod ngrok;
pub mod telegram;
pub mod parse_arguments;

use std::ops::Add;
use crate::ngrok::ngrok::{NgrokApiResponse, request_ngrok};
use crate::telegram::telegram::{GoodTelegramResponse, set_webhook_telegram};
use crate::parse_arguments::parse_arguments::{TelegramArguments, parse_args};

#[tokio::main]
async fn main() -> Result<(), String> {
    let args: TelegramArguments = match parse_args() {
        Ok(parsed_args) => parsed_args,
        Err(_s) => {
            return Err(format!(" ❌  Failed parsing arguments: {}", _s))
        }
    };

    handle(&args.relative_path, &args.token).await;

    Ok(())
}

async fn handle(relative_url: &str, token: &str) -> () {
    let ngrok_info: NgrokApiResponse = request_ngrok().await;
    let ngrok_url: &str = &ngrok_info
        .tunnels[0]
        .public_url
        .to_string()
        .add(relative_url);
    println!(" ✅  Found ngrok url: {ngrok_url}");

    let result: GoodTelegramResponse = set_webhook_telegram(token, ngrok_url).await;
    println!(" ✅  OK: {}", result.description)
}