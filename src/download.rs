use reqwest::get;
use std::io::{self, Write};

#[tokio::main]
pub async fn download(url: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut stdout = io::stdout().lock();
    let resp = get(&url).await?;
    if resp.status() != reqwest::StatusCode::OK {
        return Err(format!("Invalid URL: {}", url).into());
    }
    let body = resp.bytes().await?;
    stdout.write_all(&body)?;
    stdout.flush()?;
    Ok(())
}
