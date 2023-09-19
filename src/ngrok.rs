pub mod ngrok
{
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
        let json: String = match reqwest::get(NGROK_URL).await {
            Ok(resp) => { resp.text().await.unwrap() }
            Err(_) => { return Err(NgrokError::HttpRequestFailed); }
        };

        if cfg!(debug_assertions) {
            eprintln!(" ❗  Ngrok JSON: {:?}", json)
        }

        Ok(serde_json::from_str::<NgrokApiResponse>(&json).unwrap())
    }
}