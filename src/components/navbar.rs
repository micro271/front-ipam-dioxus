use crate::Route;
use dioxus::prelude::*;

const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

#[component]
pub fn Navbar() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        div {
            id: "navbar",
            Link {
                to: Route::Networks {},
                "Home"
            }
            Link {
                to: Route::Login {},
                "Login"
            }
        }

        Outlet::<Route> {}
    }
}
