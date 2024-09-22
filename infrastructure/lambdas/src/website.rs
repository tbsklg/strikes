use aws_config::BehaviorVersion;
use handlebars::Handlebars;
use lambda_http::{run, service_fn, tracing, Error, IntoResponse, Request, Response};
use serde_json::json;

async fn function_handler(_event: Request) -> Result<impl IntoResponse, Error> {
    let rest_api_id = std::env::var("REST_API_ID").unwrap();
    let websocket_api_id = std::env::var("WEBSOCKET_API_ID").unwrap();

    let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let s3_client = aws_sdk_s3::Client::new(&config);

    let s3_response = s3_client
        .get_object()
        .bucket("website-483220362587")
        .key("index.html.hbs")
        .send()
        .await?;

    let data = s3_response.body.collect().await?;
    let template = String::from_utf8(data.into_bytes().to_vec()).unwrap();

    let mut reg = Handlebars::new();
    reg.register_template_string("index", template).unwrap();
    let index = reg
        .render(
            "index",
            &json!(
                {
                    "restApiId": rest_api_id,
                    "websocketApiId": websocket_api_id,
                }
            ),
        )
        .unwrap();

    let resp = Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body(index)
        .map_err(Box::new)?;
    Ok(resp)
}

#[tokio::main]
#[allow(dead_code)]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    run(service_fn(function_handler)).await
}
