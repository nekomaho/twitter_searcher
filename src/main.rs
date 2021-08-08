use config::Config;
mod config;
mod http;
mod response_searcher;
mod oauth;
use serde_json::Value;

fn main() {
    let config = match Config::new() {
        Ok(config) => config,
        Err(_) => {
             std::process::exit(1);
        }
    };

    let oauth = match oauth::Oauth::get_token() {
        Ok(result) => result,
        Err(_) => {
            std::process::exit(1);
        }
    };

    let url = format!("https://api.twitter.com/1.1/statuses/user_timeline.json?screen_name={}&tweet_mode=extended", config.screen_name);

    let body = match http::get(&url, &oauth.secret) {
        Ok(result) => result,
        Err(_) => {
            std::process::exit(1);
        }
    };

    let json_body_value: Value = serde_json::from_str(&body).unwrap();
    let json_body = json_body_value.as_array().unwrap();
    let mut count = 0;
    let limit: i32 = config.output_lines.parse().unwrap();
    for tweets in json_body {
        if count >= limit {
            break;
        }

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
        count += 1;
    }
}
