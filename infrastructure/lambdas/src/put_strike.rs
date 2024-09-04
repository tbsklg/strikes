use aws_config::BehaviorVersion;
use aws_sdk_dynamodb::Client;
use lambda_http::{run, service_fn, tracing, Body, Error, Request, RequestExt, Response};
use lib::strikes_db::increment_strikes;

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
