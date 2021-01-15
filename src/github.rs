use crate::env;
use actix_web::{client::Client};
use serde::Deserialize;

#[derive(Deserialize)]
struct UserEmail {
    email: String,
    primary: bool,
    verified: bool,
}

pub async fn get_email(token: &str) -> Result<String, &'static str> {
    let host = env::get_var("GITHUB_API");
    let url = format!("https://{}/user/emails", host);
    let auth_header: String = format!("token {}", token);

    let client = Client::builder()
        .header("Authorization", auth_header)
        .header("Host", host)
        .header("User-Agent", "peach-data")
        .header("Accept", "application/vnd.github.v3+json")
        .finish();

    let request = client.get(&url);

    let mut response = request.send().await.map_err(|e| {
        println!("error: {:?}", e);
        "unable to send request"
    })?;

    let user_email = response.json::<Vec<UserEmail>>()
        .await
        .map_err(|e| {
            println!("error: {:?}", e);
            "unable to parse response"
        })?;

    let email = user_email
        .iter()
        .filter(|e| e.primary && e.verified)
        .next()
        .ok_or("no primary email")?;

    Ok(email.email.to_string())
}
