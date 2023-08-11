#![allow(non_snake_case)]

mod bluesky;

use anyhow::Result;
use bluesky::StratosphereApp;

use dioxus::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    dioxus_desktop::launch(App);
    Ok(())
}

async fn login_to_bluesky(username: String, password: String) -> Result<StratosphereApp> {
    // let username = std::env::var("BLUESKY_HANDLE").context("BLUESKY_HANDLE not set")?;
    // let password = std::env::var("BLUESKY_PASSWORD").context("BLUESKY_PASSWORD not set")?;
    let client = StratosphereApp::login(username.clone(), password).await?;

    println!("Logged in as {}", username);

    let profile = client.get_profile(username).await?;

    println!("{:?}", profile);

    Ok(client)
}

fn App(cx: Scope) -> Element {
    let username_input = use_state(cx, || "".to_string());
    let password_input = use_state(cx, || "".to_string());
    let client = use_state(cx, || Option::<StratosphereApp>::None);

    let handle_login = move |_| {
        cx.spawn({
            let username_input = username_input.to_owned();
            let password_input = password_input.to_owned();
            let client = client.to_owned();

            async move {
                let resp =
                    login_to_bluesky(username_input.get().clone(), password_input.get().clone())
                        .await;

                match resp {
                    Ok(_client) => {
                        println!("Logged in!");
                        client.set(Some(_client));
                        ()
                    }

                    Err(_err) => {
                        log::error!("Failed to log in: {:?}", _err);
                        ()
                    }
                }
            }
        });
    };

    cx.render(rsx! {
        div {
            h1 {"Hey guys!"}

            if let Some(client) = client.get() {
                rsx!(
                    h1 { "Logged in!" }
                )
            } else {
                rsx!(
                    form {
                        onsubmit: handle_login,
                        input {
                            value: "{username_input}",
                            oninput: move |e| username_input.set(e.value.clone()),
                        }
                        input {
                            value: "{password_input}",
                            oninput: move |e| password_input.set(e.value.clone()),
                        }
                        input {
                            r#type: "submit",
                        }
                    }
                )
            }
        }
    })
}

fn LoggedInComponent(cx: Scope) -> Element {
    render! {
        "Logged in!"
    }
}
