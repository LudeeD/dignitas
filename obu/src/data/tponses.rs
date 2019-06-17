use serde::{Deserialize};

#[derive(Deserialize, Debug)]
pub struct Data {
    pub address: String,
    pub data: String,
}

#[derive(Deserialize, Debug)]
pub struct Paging {
    pub limit: Option<String>,
    pub start: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct RootInterfaceStateResponse {
    pub data: Vec<Data>,
    pub head: String,
    pub link: String,
    pub paging: Paging,
}
