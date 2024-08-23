use async_trait::async_trait;
use reqwest;
use std::collections::HashMap;

use super::client::StrikeClient;
use crate::tarnished::Tarnished;

pub struct RemoteClient {
    pub api_key: String,
    pub base_url: String,
}

#[async_trait]
impl StrikeClient for RemoteClient {
    fn add_strike(&self, _name: &str) -> HashMap<String, i8> {
        HashMap::new()
    }

    fn get_tarnished(&self) -> Vec<Tarnished> {
        vec![]
    }

    fn clear_strikes(&self) {}

    async fn check_health(&self) -> Result<(), String> {
        let client = HttpClient {
            base_url: self.base_url.clone(),
            api_key: self.api_key.clone(),
        };

        client.get_health().await
    }
}

struct HttpClient {
    base_url: String,
    api_key: String,
}

impl HttpClient {
    async fn get_health(&self) -> Result<(), String> {
        println!("Checking health for remote client");

        println!("Ping URL: {}/health", &self.base_url);
        let client = reqwest::Client::new();
        let response = client
            .get(format!("{}/health", &self.base_url))
            .header("x-api-key", &self.api_key)
            .send()
            .await
            .expect("Failed to execute request");
        println!("Response: {:?}", response.status());

        match response.status() {
            reqwest::StatusCode::OK => Ok(()),
            err => Err(err.to_string()),
        }
    }
}
