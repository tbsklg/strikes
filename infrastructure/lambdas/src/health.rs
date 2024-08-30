use lambda_http::{run, service_fn, tracing, Body, Error, Request, Response};

async fn function_handler(_request: Request) -> Result<Response<Body>, Error> {
    Ok(Response::builder()
        .status(200)
        .header("Content-Type", "text/plain")
        .body(Body::from("OK"))
        .unwrap())
}

#[tokio::main]
#[allow(dead_code)]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    run(service_fn(function_handler)).await
}
