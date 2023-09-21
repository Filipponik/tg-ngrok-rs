pub mod ngrok;
pub mod telegram;
pub mod parse_arguments;

use crate::ngrok::ngrok::{NgrokApiResponse, request_ngrok, get_webhook_url};
use crate::telegram::telegram::{GoodTelegramResponse, set_bot_webhook};
use crate::parse_arguments::parse_arguments::{TelegramArguments, parse_args};

#[tokio::main]
async fn main() -> Result<(), String> {
    let args: TelegramArguments = match parse_args() {
        Ok(parsed_args) => parsed_args,
        Err(_s) => {
            return Err(format!(" ❌  Failed parsing arguments: {}", _s))
        }
    };

    handle(&args.relative_path, &args.token).await
}

async fn handle(relative_url: &str, token: &str) -> Result<(), String> {
    let ngrok_info: NgrokApiResponse = match request_ngrok().await {
        Ok(parsed_args) => parsed_args,
        Err(error_type) => {
            return Err(format!(" ❌  Ngrok URL not found: {:?}", error_type))
        }
    };

    let ngrok_url: &str = &get_webhook_url(&ngrok_info, relative_url);
    println!(" ✅  Found ngrok URL: {ngrok_url}");

    let result: GoodTelegramResponse = set_bot_webhook(token, ngrok_url).await;
    println!(" ✅  OK: {}", result.description);

    Ok(())
}