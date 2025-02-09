use dioxus::prelude::*;
use web_sys::console;

use crate::{
    components::login::TOKEN,
    models::network::{NetworkRow, Select},
    Route,
};

#[component]
pub fn Networks() -> Element {
    let mut networks = use_signal(Vec::new);

    let req = move |_| async move {
        let token = (*TOKEN.read()).clone().unwrap();

        let tmp = reqwest::Client::new()
            .get("http://localhost:3000/api/v1/network")
            .header(reqwest::header::AUTHORIZATION, format!("Bearer {token}"))
            .send()
            .await
            .unwrap()
            .json::<Select<NetworkRow>>()
            .await
            .unwrap();

        networks.set(tmp.data);
    };

    let create = move |_| async move {
        let token = (*TOKEN.read()).clone().unwrap();
        let post = r#"{
            "network": "172.30.0.0/8"
        }"#;
        let tmp = reqwest::Client::new()
            .post("http://localhost:3000/api/v1/network")
            .header(reqwest::header::AUTHORIZATION, format!("Bearer {token}"))
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(post)
            .send()
            .await
            .unwrap();

        console::log_1(&format!("{:?}", tmp).into());
    };

    rsx! {
        section {

            p {
                "Bienvenido a las networks"
            }
            button {
                onclick: req,
                "Get"
            }
            button {
                onclick: create,
                "Create"
            }

            if !networks().is_empty() {
                table {
                    class: "table-auto pt-6 w-full text-center",
                    thead {
                        class:"uppercase bg-gray-100",
                        th {
                            class: "2xl:table-cell hidden px-4 py-2",
                            scope: "col",
                            "id"
                        }
                        th {
                            class: "px-4 py-2",
                            scope: "col",
                            "network"
                        }
                        th {
                            class: "px-4 py-2",
                            scope: "col",
                            "vlan"
                        }
                        th {
                            class: "px-4 py-2",
                            scope: "col",
                            "for"
                        }
                        th {
                            class: "px-4 py-2 lg:table-cell hidden",
                            scope: "col",
                            "description"
                        }
                        th {
                            class: "px-4 py-2 xl:table-cell hidden",
                            scope: "col",
                            "available"
                        }
                        th {
                            class: "px-4 py-2 lg:table-cell hidden",
                            scope: "col",
                            "used"
                        }
                        th {
                            class: "px-4 py-2 ",
                            scope: "col",
                            "free"
                        }
                        th {
                            class: "px-4 py-2 2xl:table-cell hidden",
                            scope: "col",
                            "father"
                        }
                        th {
                            class: "px-4 py-2 ",
                            scope: "col",
                            "children"
                        }
                        th {
                            class: "px-4 py-2 ",
                            scope: "col",
                            "actions"
                        }
                    }
                    tbody {
                        class: "text-center",

                        {
                            networks.iter().map(|x| rsx! {
                                tr {
                                    key: "{x.id}",

                                    th {
                                        class: "2xl:table-cell hidden lg:px-6 lg:py-3 px-4 py-1 border-b border-gray-300",
                                        scope: "row",
                                        "{x.id}",
                                    }
                                    td {
                                        class: "lg:px-6 lg:py-3 px-4 py-1 border-b border-gray-300",
                                        Link {
                                            to: Route::Network { network: x.id },
                                            "{x.network}",
                                        }
                                    }
                                    td {
                                        class: "lg:px-6 lg:py-3 px-4 py-1 border-b border-gray-300",
                                        if let Some(e) = x.vlan {
                                            "{e}"
                                        } else {
                                            "empty"
                                        },
                                    }
                                    td {
                                        class: "lg:px-6 lg:py-3 px-4 py-1 border-b border-gray-300",
                                        "{x.use_to}",
                                    }
                                    td {
                                        class: "lg:px-6 lg:py-3 px-4 py-1 border-b border-gray-300 lg:table-cell hidden",
                                        {
                                            if let Some(e) = x.description.as_ref() {
                                                "{e}"
                                            } else {
                                                "empty"
                                            }
                                        }
                                    }
                                    td {
                                        class: "xl:table-cell hidden lg:px-6 lg:py-4 px-4 py-1 border-b border-gray-300",
                                        "{x.available}",
                                    }
                                    td {
                                        class: "lg:px-6 lg:py-3 px-4 py-1 border-b border-gray-300 lg:table-cell hidden",
                                        "{x.used}",
                                    }
                                    td {
                                        class: "lg:px-6 lg:py-3 px-4 py-1 border-b border-gray-300",
                                        "{x.free}",
                                    }
                                    td {
                                        class: "lg:px-6 lg:py-3 px-4 py-1 border-b border-gray-300 2xl:table-cell hidden",
                                        {
                                            if let Some(e) = x.father {
                                                "{e}"
                                            } else {
                                                "empty"
                                            }
                                        },
                                    }
                                    td {
                                        class: "lg:px-6 lg:py-3 px-4 py-1 border-b border-gray-300",
                                        if let Some(e) = x.children {
                                            "{e}"
                                        } else {
                                            "empty"
                                        },
                                    }

                                    td {
                                        class: "lg:px-6 lg:py-3 px-4 py-1 border-b border-gray-300",
                                        colspan: 2,
                                        div {
                                            class: "flex justify-around",
                                            button { class:"px-1", "delete" }
                                            button { class:"px-1", "clean" }
                                        }
                                    }
                                }
                            })
                        }
                    }
                }
            }

        }
    }
}
