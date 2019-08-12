extern crate reqwest;
extern crate serde_json;

use std::io::Read;

use serde_json::{Result, Value};

use crate::data::tponses::RootInterfaceStateResponse;

const api_url : &str = "http://172.20.0.3:8008";

pub fn send( data: Vec<u8> ){
    let client = reqwest::Client::new();
    let url = api_url.to_string() + "/batches";
    let res = client
        .post(&url)
        .header("Content-Type", "application/octet-stream")
        .body(data)
        .send();
}

pub fn get_state_address( state_address: &str ) 
    -> RootInterfaceStateResponse
{
    let client = reqwest::Client::new();

    let url = format!("{}/state?address={}",
                      api_url, state_address);

    let mut res = client
        .get(&url)
        .send()
        .expect("Something went wrong with the request");

    let mut body = String::new();
    res.read_to_string(&mut body).expect("get_state_address");

    let v : RootInterfaceStateResponse =
        serde_json::from_str(&body).expect("get_state_address");
    println!("{:?}", v);
    v
}
