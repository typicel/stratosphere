mod bluesky;

use anyhow::{Context, Result};
use bluesky::StratosphereApp;

#[tokio::main]
async fn main() -> Result<()> {
    let username = std::env::var("BLUESKY_HANDLE").context("BLUESKY_HANDLE not set")?;
    let password = std::env::var("BLUESKY_PASSWORD").context("BLUESKY_PASSWORD not set")?;

    let client = StratosphereApp::login(username.clone(), password).await?;

    client.get_profile(username).await?;

    Ok(())
}
