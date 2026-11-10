use comments_api::{config::App, routes};
use lambda_http::{run, service_fn, Error};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let app = Arc::new(App::from_env().await?);
    run(service_fn(move |req| routes::handle(app.clone(), req))).await
}
