use std::time::Duration;

use dotenv::dotenv;
use reqwest::header::HeaderMap;
use reqwest::header::HeaderValue;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let key = std::env::var("KEY").unwrap();
    let key_id = std::env::var("KEY_ID").unwrap();
    let token = std::env::var("GITHUB_TOKEN").unwrap();
    let owner = std::env::var("OWNER").unwrap();
    let repo_name = std::env::var("REPO_NAME").unwrap();
    let secret_name = std::env::var("SECRET_NAME").unwrap();
    let value = "plain-text-secret";

    let message_bytes = value.as_bytes();
    let key_bytes = base64::decode(key).unwrap();

    let mut encrypted_bytes =
        vec![
            0;
            message_bytes.len() + libsodium_sys::crypto_box_SEALBYTES as usize
        ];
    let ret = unsafe {
        libsodium_sys::crypto_box_seal(
            encrypted_bytes.as_mut_ptr(),
            message_bytes.as_ptr(),
            message_bytes.len() as u64,
            key_bytes.as_ptr() as *const u8,
        )
    };
    assert_eq!(0, ret);

    let encrypted = base64::encode(encrypted_bytes);
    println!("{}", &encrypted);

    let request_url = format!(
        "https://api.github.com/repos/{org}/{repo}/actions/secrets/{secret_name}",
        org = owner,  // organization
        repo = repo_name, // repo name
        secret_name = secret_name
    );

    let mut default_headers = HeaderMap::with_capacity(2);
    default_headers.insert(
        "Accept",
        HeaderValue::from_static("application/vnd.github.v3+json"),
    );
    default_headers.insert(
        "Authorization",
        HeaderValue::from_str(&format!("token {}", token)).unwrap(),
    );

    let body = json!({ "encrypted_value": encrypted, "key_id": key_id});

    let client = reqwest::Client::builder()
        .user_agent("kps-github")
        .default_headers(default_headers)
        .connect_timeout(Duration::from_secs(4))
        .timeout(Duration::from_secs(20))
        .build()
        .unwrap();

    let response = client.put(&request_url).json(&body).send().await?;

    let headers = response.headers();
    let status = response.status();

    println!("{:?}", headers);
    println!("{:?}", status);

    Ok(())
}
