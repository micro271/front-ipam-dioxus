use dioxus::prelude::*;

use crate::{
    components::login::TOKEN,
    models::network::{NetworkRow, Select},
};

#[component]
pub fn Network(network: uuid::Uuid) -> Element {
    let req = use_resource(move || async move {
        let tk = (*TOKEN.read()).clone().unwrap();
        let mut req = reqwest::Client::new()
            .get(format!("http://localhost:3000/api/v1/network?id={network}"))
            .header(reqwest::header::AUTHORIZATION, format!("Bearer {tk}"))
            .send()
            .await
            .unwrap()
            .json::<Select<NetworkRow>>()
            .await
            .unwrap();
        req.data.remove(0)
    });

    rsx! {
        if let Some(req) = (*(req.read())).clone() {

            section {
                class:"",

                div {
                    class:"border-b border-gray-300 pt-4 pb-1",
                    p {
                        "id: "
                        span {
                            " {req.id}"
                        }
                    }
                }
                div {
                    class:"border-b border-gray-300 pt-4 pb-1",
                    p {
                        "network: "
                        span {"{req.network}"}
                    }
                }
                div {
                    class:"border-b border-gray-300 pt-4 pb-1",
                    p {
                        "vlan: "
                        span {
                            if let Some(e) = req.vlan {
                                "{e}"
                            } else {
                                "empty"
                            }
                        }
                    }
                }
                div {
                    class:"border-b border-gray-300 pt-4 pb-1",
                    p {
                        "description: "
                        span {
                            if let Some(e) = req.description {
                                "{e}"
                            } else {
                                "empty"
                            }
                        }
                    }

                }
                div {
                    class:"border-b border-gray-300 pt-4 pb-1",
                    p {
                        "father: "
                        span {
                            if let Some(e) = req.father {
                                "{e}"
                            } else {
                                "empty"
                            }
                        }
                    }

                }
                div {
                    class:"border-b border-gray-300 pt-4 pb-1",
                    p {
                        "children: "
                        span {
                            if let Some(e) = req.children {
                                "{e}"
                            } else {
                                "empty"
                            }
                        }
                    }

                }
                div {
                    class:"border-b border-gray-300 pt-4 pb-1",
                    p {
                        "From: "
                        span {
                            "{req.use_to}"
                        }
                    }
                }
                div {
                    class:"border-b border-gray-300 pt-4 pb-1",
                    p {
                        "used: "
                        span {"{req.used}"}
                    }

                }
                div {
                    class:"border-b border-gray-300 pt-4 pb-1",
                    p {
                        "free: "
                        span {"{req.free}"}
                    }

                }
                div {
                    class:"border-b border-gray-300 pt-4 pb-1",
                    p {
                        "available: "
                        span {"{req.available}"}
                    }

                }
            }
            h1 { "hay" }
        } else {
            h1 { "none" }
        }
    }
}
