use lambda_http::{run, service_fn, tracing, Body, Error, Request, RequestExt, Response};

async fn function_handler(request: Request) -> Result<Response<Body>, Error> {
    let params = request.path_parameters();
    let user = params.first("user");

    tracing::info!("User: {:?}", user);

    match user {
        Some(user) => {
            Ok(Response::builder()
                .status(200)
                .header("Content-Type", "text/plain")
                .body(Body::from(format!("Hello, {}!", user)))
                .unwrap())
        }
        None => {
            Ok(Response::builder()
                .status(400)
                .header("Content-Type", "text/plain")
                .body(Body::from("Missing user parameter"))
                .unwrap())
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    run(service_fn(function_handler)).await
}
