use dioxus::prelude::*;
use ipnet::IpNet;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Props)]
pub struct NetworkRow {
    pub id: uuid::Uuid,
    pub network: IpNet,
    pub available: u32,
    pub used: u32,
    pub free: u32,
    pub vlan: Option<u16>,
    pub description: Option<String>,
    pub father: Option<uuid::Uuid>,
    pub children: Option<i32>,
    pub use_to: String,
}

impl std::cmp::PartialEq for NetworkRow {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Select<T> {
    pub status: u16,
    pub length: usize,
    pub data: Vec<T>,
    pub success: bool,
}
