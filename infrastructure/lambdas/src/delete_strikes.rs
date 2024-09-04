use aws_config::BehaviorVersion;
use aws_sdk_dynamodb::Client;
use lambda_http::{run, service_fn, tracing, Body, Error, Request, Response};
use lib::strikes_db::delete_all_strikes;

async fn function_handler(_event: Request) -> Result<Response<Body>, Error> {
    let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let client = Client::new(&config);

    delete_all_strikes("Strikes", &client).await?;

    Ok(Response::builder()
        .status(200)
        .body(Body::Text("All strikes deleted".to_string()))
        .expect("Failed to render response"))
}

#[tokio::main]
#[allow(dead_code)]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    run(service_fn(function_handler)).await
}
