pub mod telegram {
    use reqwest;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    pub struct GoodTelegramResponse {
        pub ok: bool,
        pub result: bool,
        pub description: String,
    }

    pub async fn set_bot_webhook(token: &str, url: &str) -> GoodTelegramResponse {
        let url: String = get_telegram_link(token, url);
        println!(" ✅  Telegram link: {url}");
        let json: String = reqwest::get(url).await.unwrap().text().await.unwrap();

        if cfg!(debug_assertions) {
            eprintln!(" ❗  Telegram JSON: {:?}", json)
        }

        serde_json::from_str::<GoodTelegramResponse>(&json).unwrap()
    }

    pub fn get_telegram_link(token: &str, url: &str) -> String {
        format!("https://api.telegram.org/bot{token}/setWebhook?url={url}")
    }
}
