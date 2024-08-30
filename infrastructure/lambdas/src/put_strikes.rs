use aws_config::BehaviorVersion;
use aws_sdk_dynamodb::{
    error::ProvideErrorMetadata,
    types::{AttributeValue, ReturnValue},
    Client,
};
use lambda_http::{run, service_fn, tracing, Body, Error, Request, RequestExt, Response};
use std::collections::HashMap;

async fn function_handler(request: Request) -> Result<Response<Body>, Error> {
    let params = request.path_parameters();
    let user = params.first("user");

    let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let client = Client::new(&config);

    match user {
        Some(username) => {
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

async fn increment_strikes(username: &str, table_name: &str, client: &Client) -> Result<u8, Error> {
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
        Err(err) => match ProvideErrorMetadata::code(&err) {
            Some("ValidationException") => add_user(username, &table_name, &client).await,
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

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    run(service_fn(function_handler)).await
}

#[cfg(test)]
mod integrationtests {
    use super::*;
    use aws_config::BehaviorVersion;
    use aws_sdk_dynamodb::{
        config::Builder,
        types::{AttributeDefinition, BillingMode, KeySchemaElement, KeyType, ScalarAttributeType},
        Client, Error,
    };
    use uuid::Uuid;

    async fn create_random_table(client: &Client) -> Result<String, Error> {
        let random_table_name = format!("Strikes_{}", Uuid::new_v4());
        let pk = AttributeDefinition::builder()
            .attribute_name("UserId")
            .attribute_type(ScalarAttributeType::S)
            .build()?;

        let ks = KeySchemaElement::builder()
            .attribute_name("UserId")
            .key_type(KeyType::Hash)
            .build()?;

        client
            .create_table()
            .table_name(&random_table_name)
            .key_schema(ks)
            .attribute_definitions(pk)
            .billing_mode(BillingMode::PayPerRequest)
            .send()
            .await?;

        Ok(random_table_name)
    }

    #[tokio::test]
    async fn it_should_add_some_strikes() -> Result<(), Box<dyn std::error::Error>> {
        let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
        let local_config = Builder::from(&config)
            .endpoint_url("http://localhost:8000")
            .build();
        let client = Client::from_conf(local_config);

        let table_name = create_random_table(&client).await.unwrap();
        let _ = increment_strikes("heinz", &table_name, &client)
            .await
            .unwrap();
        let _ = increment_strikes("heinz", &table_name, &client)
            .await
            .unwrap();
        let strikes = increment_strikes("heinz", &table_name, &client)
            .await
            .unwrap();

        assert_eq!(strikes, 3);

        Ok(())
    }
}
