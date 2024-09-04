use aws_config::BehaviorVersion;
use aws_sdk_dynamodb::Client;
use lambda_http::{run, service_fn, tracing, Body, Error, Request, Response};
use lib::strikes_db::get_strikes;

async fn function_handler(_event: Request) -> Result<Response<Body>, Error> {
    let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
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

#[tokio::main]
#[allow(dead_code)]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    run(service_fn(function_handler)).await
}
