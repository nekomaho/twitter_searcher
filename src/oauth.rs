use oauth2::{basic::BasicClient};
use oauth2::reqwest::http_client;
use oauth2::{
    AuthUrl,
    ClientId,
    ClientSecret,
    TokenResponse,
    TokenUrl
};
use std::env;

pub struct Oauth {
    pub secret: String
}

impl Oauth {
    pub fn get_token() -> Result<Self, i32> {
        let client_id_string = env::var( "TWITTER_CLIENT_ID").unwrap().to_string();
        let client_secret_string = env::var( "TWITTER_CLIENT_SECRET").unwrap().to_string();
        let auth_url_string = "http://dummy";
        let token_url_string = "https://api.twitter.com/oauth2/token";

        let secret = match BasicClient::new(
            ClientId::new(client_id_string.to_string()),
            Some(ClientSecret::new(client_secret_string.to_string())),
            AuthUrl::new(auth_url_string.to_string()).unwrap(),
            Some(TokenUrl::new(token_url_string.to_string()).unwrap())
            )
            .exchange_client_credentials()
            .request(http_client) {
                Ok(result) => result.access_token().secret().to_string(),
                Err(error) => {
                    println!("get access token error: {}", error);
                    return Err(1);
                }
            };

        Ok(Oauth {secret: secret})
    }
    
}