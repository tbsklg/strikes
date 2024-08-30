use aws_sdk_dynamodb::{operation::scan::ScanOutput, Client};
use lambda_http::{run, service_fn, tracing, Body, Error, Request, Response};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct StrikeEntity {
    pub user_id: String,
    pub strikes: u8,
}

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);

    let strikes = get_strikes("Strikes", &client).await?;
    let body = strikes
        .into_iter()
        .map(|strike| {
            serde_json::json!({
                "name": strike.user_id,
                "strike_count": strike.strikes,
            })
        })
        .collect::<Vec<_>>();

    Ok(Response::builder()
        .status(200)
        .header("Content-Type", "application/json")
        .body(Body::Text(serde_json::json!(body).to_string()))
        .expect("Failed to render response"))
}

pub async fn get_strikes(table_name: &str, client: &Client) -> Result<Vec<StrikeEntity>, Error> {
    let request: ScanOutput = client.scan().table_name(table_name).send().await?;

    request
        .items()
        .iter()
        .map(|item| {
            let user_id = item.get("UserId").unwrap().as_s().unwrap().to_string();
            let strikes = item
                .get("Strikes")
                .unwrap()
                .as_n()
                .unwrap()
                .parse()
                .unwrap();

            Ok(StrikeEntity { user_id, strikes })
        })
        .collect()
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    run(service_fn(function_handler)).await
}
