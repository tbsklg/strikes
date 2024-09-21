use lambda_http::{run, service_fn, tracing, Body, Error, Request, Response};

async fn function_handler(_event: Request) -> Result<Response<Body>, Error> {
    Ok(Response::builder().status(200).body(Body::Text("ok".to_string())).expect("Failed to render response"))
}

#[tokio::main]
#[allow(dead_code)]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    run(service_fn(function_handler)).await
}
