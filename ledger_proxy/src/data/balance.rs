use serde::{Serialize, Deserialize};
use base64::decode;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize, Debug)]
pub struct BalanceResponse{
    pub timestamp: u64,
    pub value: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Balance{
    pub value: i64
}

impl Balance{

    pub fn get_balance_response(&self) -> BalanceResponse {
        let start = SystemTime::now();
        let since_the_epoch = start.duration_since(UNIX_EPOCH)
            .expect("Time went backwards");

        return BalanceResponse{
            timestamp: since_the_epoch.as_secs(),
            value : self.value}

    }

    pub fn from_tp_response(data: String) -> Balance{
        let decoded = decode(&data)
            .expect("from_tp_response");
        let s = String::from_utf8(decoded)
            .expect("from_tp_response");
        let ret : i64 = s.parse::<i64>().unwrap();
        Balance{value : ret}
    }
}
