use dioxus::prelude::*;

#[component]
pub fn MountPoint() -> Element {
    rsx! {
        h1 { "Mount Point" }
    }
}
