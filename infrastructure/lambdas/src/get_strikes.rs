use aws_config::BehaviorVersion;
use aws_sdk_dynamodb::Client;
use lambda_http::{run, service_fn, tracing, Body, Error, Request, Response};
use lib::strikes_db::{get_strikes, sort_strikes_desc};

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let client = Client::new(&config);

    let strikes = &get_strikes("Strikes", &client).await?;
    let body = &strikes
        .iter()
        .map(|strike| {
            serde_json::json!({
                "name": strike.user_id,
                "strike_count": strike.strikes,
            })
        })
        .collect::<Vec<_>>();

    let accept = event.headers().get("accept").unwrap().to_str().unwrap();

    let strikes_desc = sort_strikes_desc(strikes);
    match accept {
        "text/html" => {
            let li = &strikes_desc
                .iter()
                .map(|strike| format!("<li>{}: {}</li>", strike.user_id, strike.strikes))
                .collect::<Vec<String>>()
                .join("");
            let ul = format!("<ul hx-swap-oob=\"innerHTML:#content\">{}</ul>", li);

            Ok(Response::builder()
                .status(200)
                .header("Content-Type", "text/html")
                .header("Access-Control-Allow-Origin", "*")
                .body(Body::Text(ul))
                .expect("Failed to render response"))
        }
        _ => Ok(Response::builder()
            .status(200)
            .header("Content-Type", "application/json")
            .header("Access-Control-Allow-Origin", "*")
            .body(Body::Text(serde_json::json!(body).to_string()))
            .expect("Failed to render response")),
    }
}

#[tokio::main]
#[allow(dead_code)]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    run(service_fn(function_handler)).await
}
