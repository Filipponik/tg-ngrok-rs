pub mod ngrok
{
    use reqwest;
    use serde::{Deserialize, Serialize};

    const NGROK_URL: &str = "http://localhost:4040/api/tunnels";

    #[derive(Serialize, Deserialize, Debug)]
    pub struct NgrokTunnelConfig {
        pub addr: String,
        pub inspect: bool,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct NgrokTunnel {
        #[serde(rename = "ID")]
        pub id: String,
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

    pub async fn request_ngrok() -> NgrokApiResponse {
        // chaining .await will yield our query result
        let json: String = reqwest::get(NGROK_URL)
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        if cfg!(debug_assertions) {
            eprintln!("Ngrok JSON: {:?}", json)
        }

        serde_json::from_str::<NgrokApiResponse>(&json).unwrap()
    }
}