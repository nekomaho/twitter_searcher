#[tokio::main]
pub async fn get(url: &str, secret: &str) -> Result<String, i32>{
    let res_result = match reqwest::Client::new()
    .get(url)
    .bearer_auth(secret)
    .send()
    .await {
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
