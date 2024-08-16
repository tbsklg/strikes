use reqwest;

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
