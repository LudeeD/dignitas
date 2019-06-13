use std::fmt;
use std::str;

use sawtooth_sdk::processor::handler::ApplyError;

#[derive(Copy, Clone)]
pub enum Action {
    CreateVote,
    Vote,
    CloseVote,
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Action::CreateVote => "Action::CreateVote",
                Action::Vote => "Action::Vote",
                Action::CloseVote => "Action::CloseVote",
            }
        )
    }
}

pub struct PayloadBuilder<'a>{
    action: Action,
    data  : Vec<&'a str>,
}

// Pub values, but later refactor to getters
pub struct PayloadCreateVote{
    pub title: String,
    pub info: String,
    pub lat : f64,
    pub lng : f64,
    pub direction: f64
}

pub struct PayloadVote{
    pub vote_id: String,
    pub value: i64,
}

pub struct PayloadCloseVote{
    pub vote_id: String,
}

impl<'a> PayloadBuilder<'a>{
    pub fn new(payload_data: &[u8]) -> Result<PayloadBuilder, ApplyError> {
        info!("New payload Builder");

        let payload_string = match str::from_utf8(&payload_data) {
            Ok(s) => s,
            Err(_) => {
                return Err(ApplyError::InvalidTransaction(String::from(
                            "Invalid payload serialization",
                            )));
            }
        };

        info!("payload_string: {}", payload_string);

        //Dignitas payload is constructed as comma separated items
        let items: Vec<&str> = payload_string.split(",").collect();

        info!("Items len: {}", items.len());

        if items.len() < 2 {
            return Err(ApplyError::InvalidTransaction(String::from(
                        "Payload must have at least 2 arguments",
                        )));
        }

        let action = items[0];

        let action = match action {
            "CreateVote" => Action::CreateVote,
            "Vote" =>Action::Vote,
            "CloseVote" =>Action::CloseVote,
            _ => { return Err(ApplyError::InvalidTransaction(String::from( "Invalid Action",))) }
        };

        Ok(PayloadBuilder{
            action : action,
            data : items
        })
    }

    pub fn get_action(&self) -> Action {
        self.action
    }

    pub fn create_vote_payload(&self) -> PayloadCreateVote{
        PayloadCreateVote{
            title:      self.data[3].to_string(),
            info:       self.data[4].to_string(),
            lat :       self.data[5].parse::<f64>().expect("Shait"),
            lng :       self.data[6].parse::<f64>().expect("Shait"),
            direction:  self.data[7].parse::<f64>().expect("Shait")
        }
    }
    pub fn vote_payload(&self) -> PayloadVote{
        PayloadVote{
            vote_id:    self.data[1].to_string(),
            value:      self.data[2].parse::<i64>().expect("Shait")
        }
    }
    pub fn close_vote_payload(&self) -> PayloadCloseVote{
        PayloadCloseVote{
            vote_id:    self.data[1].to_string()
        }
    }
}
