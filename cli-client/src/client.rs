use reqwest;

pub async fn get_example() {
    let client = reqwest::Client::new();
    let response = client
        .get("https://rn5aez7sm9.execute-api.eu-central-1.amazonaws.com/v1/health")
        .send()
        .await
        .expect("Failed to execute request");
        

    println!("body = {:?}", response.text().await.unwrap());
}
