use serde::{Deserialize, Serialize};
use std::time::Instant;
use sawtooth_sdk::processor::handler::ApplyError;

use geohash_16::{encode, Coordinate};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Location {
  pub lat: f64,
  pub lng: f64,
  pub direction: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Status {
  #[serde(rename = "type")]
  pub _type: String,
  #[serde(rename = "true")]
  pub _true: i64,
  #[serde(rename = "false")]
  pub _false: i64,
  pub verdict: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vote {
    pub id: String,
    pub timestamp: u64,
    pub location: Location,
    pub title: String,
    pub info: String,
    pub status: Status,
}

impl Vote {
    pub fn new(lat: f64, lng:f64, dir:f64, title:&str, info:&str) -> Vote {
        Vote {
            id: Vote::generate_id(lat, lng),
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

    fn generate_id(lat: f64, lng:f64) -> String {
        let c = Coordinate {x: lng, y: lat};
        let encoded : String = encode(c, 12usize)
            .expect("Generating ID");
        info!("Size {}", encoded.len());
        encoded
    }

    pub fn to_cbor(&self) -> Option<Vec<u8>> {
        let ret: Vec<u8> = serde_cbor::to_vec(&self)
            .expect("to_cbor_string");
        Some(ret)
    }

    pub fn from_cbor(vote_bytes: Vec<u8>) -> Option<Vote>{
        let vote: Vote = serde_cbor::from_slice(&vote_bytes)
            .expect("from_cbor_string");
        Some(vote)
    }

    pub fn agree_more(&mut self, value: i64) -> Result<(), ApplyError> {
        info!("Function agree_more : {}", value);
        self.status._true = self.status._true + value;
        Ok(())
    }

    pub fn disagree_more(&mut self, value: i64) -> Result<(), ApplyError> {
        info!("Function disagree_more : {}", value);
        self.status._false = self.status._false + value;
        Ok(())
    }
}
