use std::borrow::Borrow;
use lambda_http::{Body, Request};
use lambda_runtime::{service_fn, LambdaEvent, Error};
use serde_json::{json, Value};
use serde::{Deserialize, Serialize};
use tracing::log::info;

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init();

    lambda_http::run(service_fn(func)).await
}

#[derive(Deserialize)]
struct TestType {
    firstName: String
}

#[derive(Debug, Serialize)]
struct TestError {
    msg: String,
}

impl std::error::Error for TestError {
    // this implementation required `Debug` and `Display` traits
}

impl std::fmt::Display for TestError {
    /// Display the error struct as a JSON string
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let err_as_json = json!(self).to_string();
        write!(f, "{}", err_as_json)
    }
}

async fn func(event: Request) -> Result<Value, Error> {
    println!(
        "body: {}",
        serde_json::to_string(event.body())
            .unwrap_or("cannot read request".to_string())
    );

    let body: Result<TestType, serde_json::Error> = match event.body() {
        Body::Text(body) => serde_json::from_str(body),
        _ => Ok(TestType{ firstName: "world".to_string()})
    };

    let name = match body {
        Ok(res) => res.firstName,
        Err(_) => "world".to_string()
    };

    Ok(json!({ "message": format!("Hello, {}!", name) }))
}