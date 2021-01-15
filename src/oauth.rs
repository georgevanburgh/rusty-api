use std::sync::{Arc, Mutex};

use crate::env;
use oauth2::{
    basic::BasicClient, AsyncCodeTokenRequest, AuthUrl, AuthorizationCode, ClientId, ClientSecret,
    TokenResponse, TokenUrl,
};

use oauth2::reqwest::async_http_client;

#[derive(Clone)]
pub struct OauthAccessor {
    client: Arc<Mutex<BasicClient>>,
}

const auth_url: &'static str = "https://github.com/login/oauth/authorize";
const token_url: &'static str = "https://github.com/login/oauth/access_token";

impl OauthAccessor {
    pub fn new() -> OauthAccessor {
        let client_id = ClientId::new(env::get_var("OAUTH2_CLIENT_ID"));
        let client_secret = ClientSecret::new(env::get_var("OAUTH2_CLIENT_SECRET"));

        let client = BasicClient::new(
            client_id,
            Some(client_secret),
            AuthUrl::new(auth_url.to_string()).unwrap(),
            Some(TokenUrl::new(token_url.to_string()).unwrap()),
        );

        OauthAccessor {
            client: Arc::new(Mutex::new(client)),
        }
    }

    pub async fn get_token(&self, code: &str) -> Result<(String, Option<String>), &'static str> {
        println!("code: {}", code);

        let code = AuthorizationCode::new(code.to_string());

        let response = self
            .client
            .lock()
            .unwrap()
            .exchange_code(code)
            .request_async(async_http_client)
            .await;

        println!("token response: {:#?}", response);
        let result = response.map_err(|_| "auth token request failed")?;
        println!("token response: {:#?}", result);

        let access_token = result.access_token().secret().clone();
        let refresh_token = result
            .refresh_token()
            .and_then(|rt| Some(rt.secret().clone()));

        Ok((access_token, refresh_token))
    }
}
