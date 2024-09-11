use ::serde::Serialize;
use aws_config::BehaviorVersion;
use aws_sdk_apigatewaymanagement::config;
use aws_sdk_dynamodb::{primitives::Blob, Client};
use lambda_http::{
    lambda_runtime::{self},
    tracing, LambdaEvent,
};
use lambda_runtime::{service_fn, Error};
use lib::strikes_db::get_strikes;

#[derive(Debug, Serialize)]
struct Response {
    #[serde(rename = "statusCode")]
    status_code: i32,
}

async fn function_handler(
    _event: LambdaEvent<aws_lambda_events::dynamodb::Event>,
) -> Result<Response, Error> {
    let endpoint_url = "https://eyx5jmt9mf.execute-api.eu-central-1.amazonaws.com/v1/";
    let config = aws_config::load_defaults(BehaviorVersion::latest()).await;

    let api_management_config = config::Builder::from(&config)
        .endpoint_url(endpoint_url)
        .build();
    let client = aws_sdk_apigatewaymanagement::Client::from_conf(api_management_config);

    let dynamodb_client = Client::new(&config);
    let connection_ids = Client::new(&config)
        .scan()
        .table_name("Connections".to_string())
        .send()
        .await?;

    for item in connection_ids.items.unwrap() {
        let connection_id = item.get("ConnectionId").unwrap().as_s().unwrap();
        let strikes = get_strikes("Strikes", &dynamodb_client).await?;
        let message = serde_json::json!({
            "strikes": strikes.into_iter().map(|strike| {
                serde_json::json!({
                    "name": strike.user_id,
                    "strike_count": strike.strikes,
                })
            }).collect::<Vec<_>>()
        })
        .to_string();

        send_data(&client, connection_id, message.as_str()).await?;
    }

    Ok(Response { status_code: 200 })
}

async fn send_data(
    client: &aws_sdk_apigatewaymanagement::Client,
    con_id: &str,
    data: &str,
) -> Result<(), aws_sdk_apigatewaymanagement::Error> {
    client
        .post_to_connection()
        .connection_id(con_id)
        .data(Blob::new(data))
        .send()
        .await?;

    Ok(())
}

#[tokio::main]
#[allow(dead_code)]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    lambda_runtime::run(service_fn(function_handler)).await?;
    Ok(())
}
