use serde::Deserialize;
use reqwest::{Client, Error, header};
use reqwest::header::USER_AGENT;

#[derive(Deserialize, Debug)]
struct User {
    login: String,
    id: u32,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let request_url = format!(
        "https://api.github.com/repos/{owner}/{repo}/stargazers",
        owner = "rust-lang-nursery",
        repo = "rust-cookbook"
    );

    println!("Requesting: {}", request_url);

    let client = Client::new();

    let mut headers = header::HeaderMap::new();
    headers.insert(USER_AGENT, header::HeaderValue::from_static("rust web-api-client demo"));

    let response = client
        .get(&request_url)
        .headers(headers)
        .send()
        .await?;

    let users: Vec<User> = response.json().await?;
    println!("{:#?}", users);

    Ok(())
}



