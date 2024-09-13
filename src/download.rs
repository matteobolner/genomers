use reqwest::get;
use std::io::{self, Write};

#[tokio::main]
pub async fn download(url: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut stdout = io::stdout().lock();
    let body = get(&url).await?.text().await?;
    stdout.write_all(body.as_bytes())?;
    stdout.flush()?;
    Ok(())
}
