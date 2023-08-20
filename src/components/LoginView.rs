use dioxus::prelude::*;

pub fn LoginView(cx: Scope) -> Element {
    rsx! {
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
