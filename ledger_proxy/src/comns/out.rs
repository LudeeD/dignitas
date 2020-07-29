extern crate reqwest;
extern crate serde_json;

use std::io::Read;

use crate::data::tponses::RootInterfaceStateResponse;

const API_URL : &str = "http://localhost:8008";

pub fn send( data: Vec<u8> ){
    let client = reqwest::Client::new();
    let url = API_URL.to_string() + "/batches";
    let _res = client
        .post(&url)
        .header("Content-Type", "application/octet-stream")
        .body(data)
        .send();
}

pub fn get_state_address( state_address: &str ) 
    -> Result<RootInterfaceStateResponse, &str>
{
    let client = reqwest::Client::new();

    let url = format!("{}/state?address={}",
                      API_URL, state_address);

    let res = client .get(&url) .send();

    let mut body = String::new();
    match res {
        Ok(mut n) => {
            n.read_to_string(&mut body).expect("Failed Reading Response Body From Sawtooth");
            let v : RootInterfaceStateResponse =
                serde_json::from_str(&body).expect("get_state_address");
            Ok(v)
        }
        Err(_t) => Err("Failed Comunication")
    }
}
