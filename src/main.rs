#![allow(non_snake_case)]

mod bluesky;

use anyhow::{Context, Result};
use bluesky::StratosphereApp;

use dioxus::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    dioxus_desktop::launch(App);
    Ok(())
}

async fn login_to_bluesky() -> Result<StratosphereApp> {
    let username = std::env::var("BLUESKY_HANDLE").context("BLUESKY_HANDLE not set")?;
    let password = std::env::var("BLUESKY_PASSWORD").context("BLUESKY_PASSWORD not set")?;
    let client = StratosphereApp::login(username.clone(), password).await?;

    println!("Logged in as {}", username);

    let profile = client.get_profile(username).await?;

    println!("{:?}", profile);

    Ok(client)
}

fn App(cx: Scope) -> Element {
    let client = use_future(cx, (), |_| login_to_bluesky());

    match client.value() {
        Some(Ok(client)) => render! {
            LoggedInComponent {},
        },

        Some(Err(err)) => {
            render! {
                "Failed to log in: {err}"
            }
        }

        None => {
            render! {
                "Logging in..."
            }
        }
    }
}

#[derive(Props)]
struct LoggedInComponentProps {
    client: StratosphereApp,
}

fn LoggedInComponent(cx: Scope) -> Element {
    render! {
        "Logged in!"
    }
}
