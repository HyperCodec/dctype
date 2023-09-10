use std::time::Duration;

use reqwest::Client;

const NINE_SECS: Duration = Duration::from_secs(9);

#[derive(Clone)]
pub struct DiscordClient {
    auth: String,
}

impl DiscordClient {
    pub fn new(token: String) -> Self {
        Self {
            auth: token
        }
    }

    pub async fn infinitely_type(&self, channel: String) {
        let rclient = Client::new();

        let url = format!("https://discord.com/api/v9/channels/{}/typing", channel);
        let req = rclient.post(url)
            .header("Authorization", self.auth.clone())
            .header("Content-Length", 0)

            .build()
            .expect("Failed to build request");

        loop {
            let res = rclient.execute(req.try_clone().unwrap()).await;

            match res {
                Ok(r) => {
                    log::info!("Request sent with code {}", r.status());
                },
                Err(e) => {
                    log::error!("{}", e);
                }
            }

            tokio::time::sleep(NINE_SECS).await;
        }
    }
}