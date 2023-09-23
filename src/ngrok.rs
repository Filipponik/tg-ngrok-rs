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

impl NgrokApiResponse {
    pub fn get_webhook_url(&self, relative_url: &str) -> Result<String, NgrokError> {
        match self.tunnels.get(0) {
            Some(tunnel) => Ok(tunnel.public_url.to_string().add(relative_url)),
            None => Err(NgrokError::HttpFindTunnelUrlFailed)
        }
    }
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

#[cfg(test)]
mod tests {
    use crate::ngrok::{NgrokApiResponse, NgrokError, NgrokTunnel, NgrokTunnelConfig};

    #[test]
    fn get_webhook_url_negative() {
        let api_response: NgrokApiResponse = NgrokApiResponse {
            tunnels: vec![],
            uri: "123".to_string(),
        };

        assert!(api_response.get_webhook_url("url").is_err());
    }

    #[test]
    fn get_webhook_url_positive() {
        let data_provider = [
            ["/url", "https://example.com/url"],
        ];

        data_provider.iter().for_each(|data| {
            let api_response: NgrokApiResponse = NgrokApiResponse {
                tunnels: vec![NgrokTunnel {
                    id: None,
                    name: "console".to_string(),
                    uri: "/api/tunnels/command_line".to_string(),
                    public_url: "https://example.com".to_string(),
                    proto: "https".to_string(),
                    config: NgrokTunnelConfig {
                        addr: "https://localhost".to_string(),
                        inspect: true,
                    },
                }],
                uri: "/api/tunnels".to_string(),
            };

            let result: Result<String, NgrokError> = api_response.get_webhook_url(data[0]);
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), data[1]);
        });
    }
}
