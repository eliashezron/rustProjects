use serde::Deserialize;
use reqwest::{Client, Error};
use reqwest::header::USER_AGENT;

#[derive(Deserialize, Debug)]
struct User {
    login: String,
    id: u32,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let owner = "rust-lang-nursery";
    let repo = "rust-cookbook";

    let request_url = format!("https://api.github.com/repos/{}/{}/stargazers", owner, repo);

    // It's good practice to define the user agent string in a more descriptive way
    let user_agent_value = "My GitHub API Client/1.0";

    let client = Client::new();
    let response = client
        .get(&request_url)
        .header(USER_AGENT, user_agent_value)
        .send()
        .await?;

    // Ensure the response status is success
    if response.status().is_success() {
        let users: Vec<User> = response.json().await?;
        println!("{:?}", users);
    } 

    Ok(())
}
