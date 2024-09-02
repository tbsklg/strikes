use aws_config::BehaviorVersion;
use aws_sdk_dynamodb::{
    error::ProvideErrorMetadata,
    types::{AttributeValue, ReturnValue},
    Client,
};
use lambda_http::{run, service_fn, tracing, Body, Error, Request, RequestExt, Response};
use std::collections::HashMap;

pub async fn function_handler(request: Request) -> Result<Response<Body>, Error> {
    let params = request.path_parameters();
    let user = params.first("user");

    let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let client = Client::new(&config);

    match user {
        Some(username) => {
            if username.is_empty() || username.len() > 20 {
                return Ok(Response::builder()
                    .status(400)
                    .body(Body::Text("Invalid username".to_string()))
                    .expect("Failed to render response"));
            }

            let strike_count = increment_strikes(username, "Strikes", &client).await?;
            Ok(Response::builder()
                .status(200)
                .body(Body::Text(
                    serde_json::json!({"name": username, "strike_count": strike_count}).to_string(),
                ))
                .expect("Failed to render response"))
        }
        None => Ok(Response::builder()
            .status(400)
            .body(Body::Text("Missing user parameter".to_string()))
            .expect("Failed to render response")),
    }
}

#[tokio::main]
#[allow(dead_code)]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    run(service_fn(function_handler)).await
}

pub async fn increment_strikes(
    username: &str,
    table_name: &str,
    client: &Client,
) -> Result<u8, Error> {
    let request = client
        .update_item()
        .table_name(table_name.to_string())
        .key("UserId", AttributeValue::S(username.to_string()))
        .update_expression("set Strikes = Strikes + :value")
        .expression_attribute_values(":value", AttributeValue::N("1".to_string()))
        .return_values(ReturnValue::UpdatedNew)
        .send()
        .await
        .map_err(|err| err.into_service_error());

    match request {
        Ok(response) => {
            let strike_count = extract_strike_count(response.attributes().unwrap());
            Ok(strike_count)
        }
        Err(err) => match ProvideErrorMetadata::code(&err) {
            Some("ValidationException") => add_user(username, table_name, client).await,
            _ => Err(err.into()),
        },
    }
}

fn extract_strike_count(map: &HashMap<String, AttributeValue>) -> u8 {
    map.get("Strikes").unwrap().as_n().unwrap().parse().unwrap()
}

async fn add_user(username: &str, table_name: &str, client: &Client) -> Result<u8, Error> {
    client
        .put_item()
        .table_name(table_name.to_string())
        .item("UserId", AttributeValue::S(username.to_string()))
        .item("Strikes", AttributeValue::N("1".to_string()))
        .send()
        .await?;

    Ok(1)
}
