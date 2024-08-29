use async_trait::async_trait;
use reqwest;

use super::client::StrikeClient;
use crate::tarnished::Tarnished;

pub struct RemoteClient {
    pub api_key: String,
    pub base_url: String,
}

struct HttpClient {
    base_url: String,
    api_key: String,
}

#[derive(serde::Deserialize)]
struct StrikeResponse {
    strike_count: i8,
}

#[async_trait]
impl StrikeClient for RemoteClient {
    async fn add_strike(&self, username: &str) -> Result<i8, String> {
        let client = HttpClient {
            base_url: self.base_url.clone(),
            api_key: self.api_key.clone(),
        };

        client.put_strike(username).await
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

    async fn put_strike(&self, username: &str) -> Result<i8, String> {
        let client = reqwest::Client::new();
        let response = client
            .put(format!("{}/strikes/{}", &self.base_url, username))
            .header("x-api-key", &self.api_key)
            .send()
            .await
            .expect("Failed to execute request");

        match response.status() {
            reqwest::StatusCode::OK => {
                let body = response.text().await.expect("Failed to read response body");
                Ok(serde_json::from_str::<StrikeResponse>(&body)
                    .expect("Failed to parse response")
                    .strike_count)
            }
            err => Err(err.to_string()),
        }
    }
}

#[cfg(test)]
mod unittests {
    use wiremock::{matchers::any, Mock, MockServer, ResponseTemplate};

    use crate::clients::remote_client::HttpClient;

    #[tokio::test]
    async fn it_should_add_a_strike() -> Result<(), Box<dyn std::error::Error>> {
        let mock_server = MockServer::start().await;
        Mock::given(any())
            .respond_with(
                ResponseTemplate::new(200).set_body_json(serde_json::json!({"strike_count": 3})),
            )
            .expect(1)
            .mount(&mock_server)
            .await;

        let client = HttpClient {
            api_key: "abc".to_string(),
            base_url: mock_server.uri(),
        };

        let strike_count = client.put_strike("guenther").await?;

        assert_eq!(3, strike_count);

        Ok(())
    }
}
