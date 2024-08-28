use std::collections::HashMap;

use aws_config::BehaviorVersion;
use aws_sdk_dynamodb::{
    error::ProvideErrorMetadata,
    types::{AttributeValue, ReturnValue},
    Client,
};
use lambda_http::{run, service_fn, tracing, Body, Error, Request, RequestExt, Response};

async fn function_handler(request: Request) -> Result<Response<Body>, Error> {
    let params = request.path_parameters();
    let user = params.first("user");

    let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let client = Client::new(&config);

    match user {
        Some(username) => {
            let strike_count = increment_strikes(username, &client).await?;
            Ok(Response::builder()
                .status(200)
                .body(Body::Text(
                    serde_json::json!({"strike_count": strike_count}).to_string(),
                ))
                .expect("Failed to render response"))
        }
        None => Ok(Response::builder()
            .status(400)
            .body(Body::Text("Missing user parameter".to_string()))
            .expect("Failed to render response")),
    }
}

async fn increment_strikes(username: &str, client: &Client) -> Result<u8, Error> {
    let request = client
        .update_item()
        .table_name("Strikes")
        .key("UserId", AttributeValue::S(username.to_string()))
        .update_expression("set Strikes = Strikes + :value")
        .expression_attribute_values(":value", AttributeValue::N("1".to_string()))
        .return_values(ReturnValue::UpdatedNew)
        .send()
        .await
        .map_err(|err| err.into_service_error());

    match request {
        Err(err) => match ProvideErrorMetadata::code(&err) {
            Some("ValidationException") => add_user(username, &client).await,
            _ => return Err(err.into()),
        },
        Ok(response) => {
            let strike_count = extract_strike_count(response.attributes().unwrap());
            Ok(strike_count)
        }
    }
}

fn extract_strike_count(map: &HashMap<String, AttributeValue>) -> u8 {
    map.get("Strikes").unwrap().as_n().unwrap().parse().unwrap()
}

async fn add_user(username: &str, client: &Client) -> Result<u8, Error> {
    tracing::info!("Adding user: {}", username);

    client
        .put_item()
        .table_name("Strikes")
        .item("UserId", AttributeValue::S(username.to_string()))
        .item("Strikes", AttributeValue::N("1".to_string()))
        .send()
        .await?;

    Ok(1)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    run(service_fn(function_handler)).await
}
