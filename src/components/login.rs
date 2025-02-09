use crate::utils::login::*;
use dioxus::prelude::*;

pub const TOKEN_KEY: &str = "jwt";

pub static TOKEN: GlobalSignal<Option<String>> = Signal::global(|| {
    let window = web_sys::window()?;
    let st = window.local_storage().ok()??;
    st.get(TOKEN_KEY).ok()?
});

#[component]
pub fn Login() -> Element {
    let mut username = use_signal(String::new);

    let mut password = use_signal(String::new);

    let login_on_click = move |_| async move {
        let user = User {
            username: username(),
            password: password(),
        };
        *TOKEN.write() = Some(login(user).await.unwrap());
    };

    rsx!(
        div {

            input {
                r#type: "text",
                id: "password",
                onchange: move |x| {
                    username.set(x.value());
                },
             },

            input {
                r#type: "text",
                id: "username",
                onchange: move |x| {
                    password.set(x.value());
                }
             },

            button {
                onclick: login_on_click,
                "Send"
            }

        }
    )
}
