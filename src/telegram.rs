use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GoodTelegramResponse {
    pub ok: bool,
    pub result: bool,
    pub description: String,
}

#[derive(Debug)]
pub enum TelegramError {
    HttpRequestFailed,
    HttpResponseParseFailed,
}

pub async fn set_bot_webhook(token: &str, url: &str) -> Result<GoodTelegramResponse, TelegramError> {
    let url: String = get_telegram_link(token, url);
    println!(" ✅  Telegram link: {url}");
    let json: String = reqwest::get(url)
        .await
        .map_err(|_| TelegramError::HttpRequestFailed)
        .map(|resp| resp.text())?
        .await
        .map_err(|_| TelegramError::HttpRequestFailed)?;

    if cfg!(debug_assertions) {
        eprintln!(" ❗  Telegram JSON: {json:?}")
    }

    serde_json::from_str::<GoodTelegramResponse>(&json).
        map_err(|_| TelegramError::HttpResponseParseFailed)
}

pub fn get_telegram_link(token: &str, url: &str) -> String {
    format!("https://api.telegram.org/bot{token}/setWebhook?url={url}")
}

#[cfg(test)]
mod tests {
    use crate::telegram::get_telegram_link;

    #[test]
    fn get_telegram_link_positive() {
        let data_provider = [
            ["token", "url", "https://api.telegram.org/bottoken/setWebhook?url=url"],
            ["token1", "url1", "https://api.telegram.org/bottoken1/setWebhook?url=url1"],
            ["", "/", "https://api.telegram.org/bot/setWebhook?url=/"],
        ];

        data_provider.iter().for_each(|data| {
            assert_eq!(get_telegram_link(data[0], data[1]), data[2].to_string());
        });
    }
}
