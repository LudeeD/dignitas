extern crate reqwest;
extern crate serde_json;

use std::io::Read;

use serde_json::{Result, Value};

const api_url : &str = "http://localhost:8008";

pub fn send( data: Vec<u8> ){
    let client = reqwest::Client::new();
    let url = api_url.to_string() + "/batches";
    let res = client
        .post(&url)
        .header("Content-Type", "application/octet-stream")
        .body(data)
        .send();
}

pub fn get_state( state_address: &str ){
    let client = reqwest::Client::new();
    let url = api_url.to_string() + "/state/"+ state_address;
    let mut res = client
        .get(&url)
        .send()
        .expect("Something went wrong with the request");

    let mut body = String::new();
    res.read_to_string(&mut body).expect("Failed to read respons");

    let v : Value = serde_json::from_str(&body).expect("Upsi");
    println!("{}", v);
}
