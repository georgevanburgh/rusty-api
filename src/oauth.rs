use std::{
    env,
    sync::{Arc, Mutex},
};

use oauth2::{
    basic::BasicClient, AsyncCodeTokenRequest, AuthUrl, AuthorizationCode, ClientId, ClientSecret,
    TokenResponse, TokenUrl,
};

use oauth2::reqwest::async_http_client;

use log::{info, trace};

#[derive(Clone)]
pub struct OauthAccessor {
    client: Arc<Mutex<BasicClient>>,
}

fn get_var(name: &str) -> String {
    env::var_os(name).unwrap().to_str().unwrap().to_string()
}

const auth_url: &'static str = "https://github.com/login/oauth/authorize";
const token_url: &'static str = "https://github.com/login/oauth/access_token";

impl OauthAccessor {
    pub fn new() -> OauthAccessor {
        let client_id = ClientId::new(get_var("OAUTH2_CLIENT_ID"));
        let client_secret = ClientSecret::new(get_var("OAUTH2_CLIENT_SECRET"));

        println!("client_id: {:?}", get_var("OAUTH2_CLIENT_ID"));
        println!("client_secret: {:?}", get_var("OAUTH2_CLIENT_SECRET"));

        let client = BasicClient::new(
            client_id,
            Some(client_secret),
            AuthUrl::new(auth_url.to_string()).unwrap(),
            Some(TokenUrl::new(token_url.to_string()).unwrap()),
        );

        println!("created oauth accessor");

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
