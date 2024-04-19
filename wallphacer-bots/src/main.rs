use reqwest::{self, header::AUTHORIZATION};
use tokio;

const BASE_URL: &str = "https://api.spacetraders.io/v2/";

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();

    let rest = client
        .get(format!("{}{}", BASE_URL, "my/agent/"))
        .header(AUTHORIZATION, "Bearer <TOKEN>")
        .send()
        .await
        .expect("failed to get response")
        .text()
        .await
        .expect("failed to get payload");

    println!("{:?}", rest);
    Ok(())
}
