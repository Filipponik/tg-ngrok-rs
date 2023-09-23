pub mod ngrok;
pub mod telegram;
pub mod parse_arguments;

use crate::ngrok::request_ngrok;
use crate::telegram::{GoodTelegramResponse, set_bot_webhook};
use crate::parse_arguments::parse_arguments::{TelegramArguments, parse_args};

#[tokio::main]
async fn main() -> Result<(), String> {
    let args: TelegramArguments = parse_args().map_err(parse_err_to_string)?;

    handle(&args.relative_path, &args.token).await
}

async fn handle(relative_url: &str, token: &str) -> Result<(), String> {
    let ngrok_url: String = request_ngrok()
        .await
        .map_err(ngrok_err_to_string)?
        .get_webhook_url(relative_url)
        .map_err(ngrok_err_to_string)?;

    println!(" ✅  Found ngrok URL: {ngrok_url}");

    let result: GoodTelegramResponse = set_bot_webhook(token, &ngrok_url)
        .await
        .map_err(telegram_err_to_string)?;

    println!(" ✅  OK: {}", result.description);

    Ok(())
}

fn ngrok_err_to_string(error_type: ngrok::NgrokError) -> String {
    format!(" ❌  Ngrok URL not found: {error_type:?}")
}

fn parse_err_to_string(_s: String) -> String {
    format!(" ❌  Failed parsing arguments: {_s}")
}

fn telegram_err_to_string(error_type: telegram::TelegramError) -> String {
    format!(" ❌  Failed set telegram webhook: {error_type:?}")
}