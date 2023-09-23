pub mod ngrok
{
    use std::ops::Add;
    use reqwest;
    use serde::{Deserialize, Serialize};

    const NGROK_URL: &str = "http://localhost:4040/api/tunnels";

    #[derive(Debug)]
    pub enum NgrokError {
        HttpRequestFailed,
        HttpResponseGetTextFailed,
        HttpResponseParseFailed,
        HttpFindTunnelUrlFailed
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct NgrokTunnelConfig {
        pub addr: String,
        pub inspect: bool,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct NgrokTunnel {
        #[serde(default)]
        #[serde(rename = "ID")]
        pub id: Option<String>,
        pub name: String,
        pub uri: String,
        pub public_url: String,
        pub proto: String,
        pub config: NgrokTunnelConfig,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct NgrokApiResponse {
        pub tunnels: Vec<NgrokTunnel>,
        pub uri: String,
    }

    pub async fn request_ngrok() -> Result<NgrokApiResponse, NgrokError> {
        let json: String = reqwest::get(NGROK_URL)
            .await
            .map_err(|_| NgrokError::HttpRequestFailed)
            .map(|resp| resp.text())?
            .await
            .map_err(|_| NgrokError::HttpResponseGetTextFailed)?;

        if cfg!(debug_assertions) {
            eprintln!(" ❗  Ngrok JSON: {json:?}")
        }

        serde_json::from_str::<NgrokApiResponse>(&json)
            .map_err(|_| NgrokError::HttpResponseParseFailed)
    }

    pub fn get_webhook_url(ngrok_info: &NgrokApiResponse, relative_url: &str) -> Result<String, NgrokError>
    {
        match ngrok_info.tunnels.get(0) {
            Some(tunnel) => { Ok(tunnel.public_url.to_string().add(relative_url)) }
            None => { Err(NgrokError::HttpFindTunnelUrlFailed) }
        }
    }
}