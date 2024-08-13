use reqwest;

pub async fn check_health(base_url: String) {
    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/v1/health", base_url))
        .send()
        .await
        .expect("Failed to execute request");
        

    println!("body = {:?}", response.text().await.unwrap());
}
