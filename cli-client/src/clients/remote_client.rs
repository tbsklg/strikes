use reqwest;
use std::collections::HashMap;

use super::client::StrikeClient;
use crate::tarnished::Tarnished;

pub struct RemoteClient {
    pub api_key: String,
    pub base_url: String,
}

impl StrikeClient for RemoteClient {
    fn add_strike(&self, _name: &str) -> HashMap<String, i8> {
        HashMap::new()
    }

    fn get_tarnished(&self) -> Vec<Tarnished> {
        vec![]
    }

    fn clear_strikes(&self) {}

    fn check_health(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}

pub async fn check_health(base_url: String, api_key: String) {
    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/v1/health", base_url))
        .header("x-api-key", api_key)
        .send()
        .await
        .expect("Failed to execute request");

    println!(
        "Try to reach remote location: {:?}",
        response.text().await.unwrap()
    );
}
