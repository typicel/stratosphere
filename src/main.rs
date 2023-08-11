#![allow(non_snake_case)]

mod bluesky;

use crate::bluesky::{Command, CreateRecordCommand, CreateRecordPostArgs};
use anyhow::Result;
use bluesky::StratosphereApp;

use dioxus::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    dioxus_desktop::launch(App);
    Ok(())
}

async fn login_to_bluesky(username: String, password: String) -> Result<StratosphereApp> {
    let client = StratosphereApp::login(username.clone(), password).await?;
    println!("Logged in as {}", username);

    Ok(client)
}

fn App(cx: Scope) -> Element {
    let username_input = use_state(cx, || "".to_string());
    let password_input = use_state(cx, || "".to_string());
    let client = use_state(cx, || Option::<StratosphereApp>::None);

    let post_input = use_state(cx, || "".to_string());

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

    let submit_post = move |_| {
        cx.spawn({
            let client = client.to_owned();
            let text = post_input.to_owned();

            async move {
                if let Some(_client) = client.get() {
                    let command =
                        Command::CreateRecord(CreateRecordCommand::Post(CreateRecordPostArgs {
                            text: text.get().clone(),
                        }));

                    let resp = _client.handle_command(command).await;

                    match resp {
                        Ok(_resp) => {
                            println!("Posted!");
                            ()
                        }

                        Err(_err) => {
                            log::error!("Failed to post: {:?}", _err);
                            ()
                        }
                    }
                }
            }
        })
    };

    #[allow(unused_variables)]
    cx.render(rsx! {
        div {
            if let Some(client) = client.get() {
                rsx!(
                    form {
                        onsubmit: submit_post,
                        input {
                            value: "{post_input}",
                            oninput: move |e| post_input.set(e.value.clone()),
                        }
                        input {
                            r#type: "submit",
                        }
                    }
                )
            } else {
                rsx!(
                    h1{ "Login to Bluesky" }
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
