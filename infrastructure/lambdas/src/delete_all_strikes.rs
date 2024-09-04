use aws_config::BehaviorVersion;
use aws_sdk_dynamodb::{types::AttributeValue, Client};
use lambda_http::{run, service_fn, tracing, Body, Error, Request, Response};

use crate::get_strikes::get_strikes;

async fn function_handler(_event: Request) -> Result<Response<Body>, Error> {
    let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let client = Client::new(&config);

    delete_all_strikes("Strikes", &client).await?;

    Ok(Response::builder()
        .status(200)
        .body(Body::Text("All strikes deleted".to_string()))
        .expect("Failed to render response"))
}

pub async fn delete_all_strikes(table_name: &str, client: &Client) -> Result<(), Error> {
    let strikes = get_strikes(table_name, client).await?;

    for strike in strikes {
        client
            .delete_item()
            .table_name(table_name)
            .key("UserId", AttributeValue::S(strike.user_id))
            .send()
            .await?;
    }

    Ok(())
}

#[tokio::main]
#[allow(dead_code)]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    run(service_fn(function_handler)).await
}
