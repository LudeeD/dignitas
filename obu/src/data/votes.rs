extern crate base64;

use serde::{Serialize, Deserialize};
use std::time::Instant;

use self::base64::decode;

#[derive(Serialize, Deserialize, Debug)]
struct Location {
  lat: f64,
  lng: f64,
  direction: f64,
}

#[derive(Serialize, Deserialize, Debug)]
struct Status {
  #[serde(rename = "type")]
  _type: String,
  #[serde(rename = "true")]
  _true: i64,
  #[serde(rename = "false")]
  _false: i64,
  verdict: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Vote {
  id: String,
  timestamp: u64,
  location: Location,
  title: String,
  info: String,
  status: Status,
}
impl Vote{

    pub fn new(id: String, lat:f64,lng:f64,dir:f64,title:&str,info:&str) -> Vote {
        Vote{
            id : id,
            timestamp : Instant::now().elapsed().as_secs(),
            location: Location { lat: lat, lng:lng, direction: dir},
            title: title.to_string(),
            info: info.to_string(),
            status: Status{ _type: "OPEN".to_string(),
                            _true:0,
                            _false:0,
                            verdict: "".to_string() }
        }
    }

    pub fn from_tp_response(data : String) -> Vote {

        let decoded = decode(&data)
            .expect("from_tp_response 1");
        let vote : Vote = serde_cbor::from_slice(&decoded)
            .expect("from_tp_response 2");

        vote
    }

}

#[derive(Serialize, Deserialize, Debug)]
pub struct VoteResponse {
    status: String,
    votes: Vec<Vote>
}
impl VoteResponse{

    pub fn new( votes: Vec<Vote>, status: &str) -> VoteResponse{
        VoteResponse{
            status : status.to_string(),
            votes: votes
        }

    }
}
