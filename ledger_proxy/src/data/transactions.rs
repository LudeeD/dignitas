use serde::{Serialize, Deserialize};

#[derive(Deserialize, Debug)]
pub struct Transaction{
    pub id: u64,
    pub payload: String,
}

#[derive(Serialize, Debug)]
pub struct TransactionResponse{
    pub status: String
}
