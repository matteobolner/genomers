use futures_util::StreamExt;
use reqwest::get;
use std::io::{self, Write};

#[tokio::main]
pub async fn download(url: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut stdout = io::stdout().lock();
    let resp = get(&url).await?;
    if resp.status() != reqwest::StatusCode::OK {
        return Err(format!("Invalid URL: {}", url).into());
    }
    // let body = resp.bytes().await?;
    let mut stream = resp.bytes_stream();

    while let Some(item) = stream.next().await {
        stdout.write_all(&item.expect("ERROR DOWNLOADING"))?;
    }
    stdout.flush()?;
    Ok(())
}
