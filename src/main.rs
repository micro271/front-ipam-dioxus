mod components;
mod models;
mod utils;

use components::{Location, Login, MountPoint, Navbar, Network, Networks, Office};
use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Networks {},

    #[route("/:network")]
    Network { network: uuid::Uuid },

    #[route("/login")]
    Login{},

    #[route("/location")]
    Location{},

    #[route("/office")]
    Office{},

    #[route("/mount_point")]
    MountPoint{},

}

fn main() {
    dioxus::launch(App);
}

#[component]
pub fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }

        main {
            class: "container pt-4 mx-auto",
            Router::<Route> {}
        }
    }
}
