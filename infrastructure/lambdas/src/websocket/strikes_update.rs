use ::serde::Serialize;
use aws_config::BehaviorVersion;
use aws_sdk_apigatewaymanagement::{config, Client};
use aws_sdk_dynamodb::{primitives::Blob, Client as DynamoDbClient};
use lambda_http::{
    lambda_runtime::{self},
    tracing, LambdaEvent,
};
use lambda_runtime::{service_fn, Error};
use lib::strikes_db::{get_strikes, sort_strikes_desc, StrikeEntity};

#[derive(Debug, Serialize)]
struct Response {
    #[serde(rename = "statusCode")]
    status_code: i32,
}

async fn function_handler(
    _event: LambdaEvent<aws_lambda_events::dynamodb::Event>,
) -> Result<Response, Error> {
    let websocket_api_id = std::env::var("WEBSOCKET_API_ID").unwrap();

    let endpoint_url = format!("https://{}.execute-api.eu-central-1.amazonaws.com/v1/", websocket_api_id);
    let config = aws_config::load_defaults(BehaviorVersion::latest()).await;

    let dynamodb_client = DynamoDbClient::new(&config);
    let connection_ids = connection_ids(&dynamodb_client).await?;

    let strikes = sort_strikes_desc(&get_strikes("Strikes", &dynamodb_client).await?);
    let message = ul_from_strikes(strikes);

    let api_management_config = config::Builder::from(&config)
        .endpoint_url(endpoint_url)
        .build();
    let client = Client::from_conf(api_management_config);
    for connection_id in connection_ids {
        send_data(&client, &connection_id, message.as_str()).await?;
    }

    Ok(Response { status_code: 200 })
}

fn ul_from_strikes(strikes: Vec<StrikeEntity>) -> String {
    let li = strikes
        .iter()
        .map(|strike| format!("<li>{}: {}</li>", strike.user_id, strike.strikes))
        .collect::<Vec<String>>();
    format!(
        "<ul hx-swap-oob=\"innerHTML:#content\">{}</ul>",
        li.join("")
    )
}

async fn connection_ids(
    client: &aws_sdk_dynamodb::Client,
) -> Result<Vec<String>, aws_sdk_dynamodb::Error> {
    let result = client
        .scan()
        .table_name("Connections".to_string())
        .send()
        .await?;

    let connection_ids = result
        .items
        .unwrap()
        .iter()
        .map(|item| {
            item.get("ConnectionId")
                .unwrap()
                .as_s()
                .unwrap()
                .to_string()
        })
        .collect();

    Ok(connection_ids)
}

async fn send_data(
    client: &Client,
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
