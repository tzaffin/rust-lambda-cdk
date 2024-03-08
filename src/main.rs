use lambda_http::{aws_lambda_events::query_map::QueryMap, run, service_fn, Body, Request, RequestExt, Response};
use tracing_subscriber::filter::{EnvFilter, LevelFilter};

async fn handler(event: Request) -> Result<Response<Body>, lambda_http::Error> {

    let query_parameters:QueryMap = event.query_string_parameters();
    
    let artifact_name:&str = match query_parameters.first("file") {
        Some(name) => name,
        None => {
            let response:Response<Body> = Response::builder()
                .status(400)
                .body(Body::from("Missing 'file' parameter"))
                .unwrap();
            return Ok(response);
        }
    };


    Ok(Response::builder()
        .status(200)
        .body(Body::from(format!("Searching for artifact {}", artifact_name)))
        .unwrap())
}

#[tokio::main]
async fn main() -> Result<(), lambda_http::Error> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(handler)).await
}
