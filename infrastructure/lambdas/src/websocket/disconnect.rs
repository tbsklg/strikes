use ::serde::Serialize;
use aws_config::BehaviorVersion;
use aws_sdk_dynamodb::{types::AttributeValue, Client};
use lambda_http::{
    aws_lambda_events::apigw::ApiGatewayWebsocketProxyRequest,
    lambda_runtime::{self},
    tracing,
};
use lambda_runtime::{service_fn, Error, LambdaEvent};

#[derive(Debug, Serialize)]
struct Response {
    #[serde(rename = "statusCode")]
    status_code: i32,
}

async fn function_handler(
    event: LambdaEvent<ApiGatewayWebsocketProxyRequest>,
) -> Result<Response, Error> {
    let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let client = Client::new(&config);

    client
        .delete_item()
        .table_name("Connections".to_string())
        .key(
            "ConnectionId",
            AttributeValue::S(event.payload.request_context.connection_id.unwrap()),
        )
        .send()
        .await?;

    Ok(Response { status_code: 200 })
}

#[tokio::main]
#[allow(dead_code)]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    lambda_runtime::run(service_fn(function_handler)).await?;
    Ok(())
}
