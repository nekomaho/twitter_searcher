use config::Config;
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
mod config;
mod response_searcher;
use serde_json::Value;

#[tokio::main]
async fn get(url: &str, secret: &str) -> Result<String, i32>{
    let req = reqwest::Client::new();
    let res = req
    .get(url)
    .bearer_auth(secret)
    .send()
    .await;
let res_result = match res {
        Ok(result) => result,
        Err(error) => {
            let status_code = match error.status() {
                Some(status) => status.to_string(),
                None => "".to_string()
            };
            println!("error: {}", status_code);
            return Err(1)
        }
    };

    Ok(res_result.text().await.unwrap())
}

fn main() {
    let config = match Config::new() {
        Ok(config) => config,
        Err(_) => {
             std::process::exit(1);
        }
    };

    let client_id_string = env::var( "TWITTER_CLIENT_ID").unwrap().to_string();
    let client_secret_string = env::var( "TWITTER_CLIENT_SECRET").unwrap().to_string();
    let auth_url_string = "http://dummy";
    let token_url_string = "https://api.twitter.com/oauth2/token";

    let client = BasicClient::new(
        ClientId::new(client_id_string.to_string()),
        Some(ClientSecret::new(client_secret_string.to_string())),
        AuthUrl::new(auth_url_string.to_string()).unwrap(),
        Some(TokenUrl::new(token_url_string.to_string()).unwrap())
    );

    let token_result = client
      .exchange_client_credentials()
      .request(http_client);

    let secret = match token_result {
        Ok(result) => result.access_token().secret().to_string(),
        Err(error) => {
             println!("get access token error: {}", error);
             std::process::exit(1);
        }
    };

    let url = format!("https://api.twitter.com/1.1/statuses/user_timeline.json?screen_name={}&tweet_mode=extended", config.screen_name);

    let body = match get(&url, &secret) {
        Ok(result) => result,
        Err(_) => {
            std::process::exit(1);
        }
    };

    let json_body_value: Value = serde_json::from_str(&body).unwrap();
    let json_body = json_body_value.as_array().unwrap();
    for tweets in json_body {
        let title_text = &config.extract_keyword;
        if !response_searcher::ResponseSearcher::new(&tweets["full_text"].to_string(),0).exists(&title_text) {
            continue;
        }

        println!("{}", &tweets["created_at"].to_string());
        let search_texts = &config.extract_lines;
        for search_text in search_texts {
            let result_text = response_searcher::ResponseSearcher::new(&tweets["full_text"].to_string(), 1).search(&search_text);
            println!("{}", &result_text);
        }
    }
}
