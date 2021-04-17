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
    fn get_oauth_info() -> Result<(String, String), i32> {
        let client_id_string = match env::var( "TWITTER_CLIENT_ID") {
            Ok(result) => result.to_string(),
            Err(_) => {
                println!("TWITTER_CLIENT_ID is not defined");
                return Err(1);
            }
        };
        let client_secret_string = match env::var( "TWITTER_CLIENT_SECRET") {
            Ok(result) => result.to_string(),
            Err(_) => {
                println!("TWITTER_CLIENT_SECRET is not defined");
                return Err(1);
            }
        };

        Ok((client_id_string, client_secret_string))
    }

    pub fn get_token() -> Result<Self, i32> {
        let auth_info = match Oauth::get_oauth_info() {
            Ok(result) => result,
            Err(_) => {
                return Err(1);
            }
        };
        let auth_url_string = "http://dummy";
        let token_url_string = "https://api.twitter.com/oauth2/token";

        let secret = match BasicClient::new(
            ClientId::new(auth_info.0),
            Some(ClientSecret::new(auth_info.1)),
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