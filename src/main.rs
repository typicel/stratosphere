#![allow(non_snake_case, unused)]

mod bluesky;
mod components;

use crate::bluesky::{ClientResponse, Command, CreateRecordCommand, CreateRecordPostArgs};
use anyhow::Result;
use atrium_api::app::bsky::feed::defs::FeedViewPost;
use bluesky::StratosphereApp;

use dioxus::prelude::*;
use dioxus_hot_reload::*;

use components::TimelineView::TimelineView;

#[tokio::main]
async fn main() -> Result<()> {
    // hot_reload_init!();

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

    let timeline = use_state(cx, || Option::<Vec<FeedViewPost>>::None);

    let post_input = use_state(cx, || "".to_string());

    let load_timeline = move |_| {
        cx.spawn({
            let client = client.to_owned();
            let timeline = timeline.to_owned();

            async move {
                if let Some(_client) = client.get() {
                    let resp = _client.handle_command(Command::GetTimeline).await;

                    match resp {
                        Ok(output) => match output {
                            ClientResponse::Timeline(output) => {
                                timeline.set(Some(output.feed));
                                ()
                            }
                            _ => {
                                log::error!("Failed to load timeline");
                                ()
                            }
                        },

                        Err(_err) => {
                            log::error!("Failed to load timeline: {:?}", _err);
                            ()
                        }
                    }
                }
            }
        })
    };

    let login_if_env = use_future(cx, (), |_| {
        let username = std::env::var("BLUESKY_HANDLE");
        let password = std::env::var("BLUESKY_PASSWORD");
        let client = client.to_owned();

        async move {
            if let Ok(usr) = username {
                if let Ok(pwd) = password {
                    println!("user: {:?}, pass: {:?}", usr, pwd);
                    let client_ = login_to_bluesky(usr, pwd).await.unwrap();
                    client.set(Some(client_));
                } else {
                    println!("Failed to get password from env");
                }
            } else {
                println!("Failed to get username from env");
            }
        }
    });

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
                rsx! {
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

                    button {
                        onclick: load_timeline,
                        "Load timeline"
                    }

                    if let Some(_timeline) = timeline.get() {
                        rsx! (
                            TimelineView {
                                timeline: _timeline.clone(),
                            }
                        )
                    }

                }
            } else {

                rsx!{
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
                }
            }
        }
    })
}
