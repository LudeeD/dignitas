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

pub struct SwPayload {
    action: Action,
    vote_id: u32,
    value: i32,
}

impl SwPayload {
    pub fn new(payload_data: &[u8]) -> Result<Option<SwPayload>, ApplyError> {
        info!("New payload ");
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

        let (action, vote_id) = (items[0], items[1]);

        if action.is_empty() {
            return Err(ApplyError::InvalidTransaction(String::from(
                "Action is required",
            )));
        }

        let action = match action {
            "CreateVote" => Action::CreateVote,
            "Vote" => Action::Vote,
            "CloseVote" => Action::CloseVote,
            _ => {
                return Err(ApplyError::InvalidTransaction(String::from(
                    "Invalid Action",
                )));
            }
        };

        let vote_id: u32 = match vote_id.parse() {
            Ok(num) => num,
            Err(_) => {
                return Err(ApplyError::InvalidTransaction(String::from(
                    "Missing integer value",
                )));
            }
        };

        let mut value: i32 = 0;

        if items.len() == 3 {
            if items[2].is_empty() {
                return Err(ApplyError::InvalidTransaction(String::from(
                    "Value cannot be empty ",
                )));
            }

            value = match items[2].parse() {
                Ok(num) => num,
                Err(_) => {
                    return Err(ApplyError::InvalidTransaction(String::from(
                        "Missing integer value",
                    )));
                }
            };
        }

        let payload = SwPayload {
            action: action,
            vote_id: vote_id,
            value: value,
        };

        Ok(Some(payload))
    }

    pub fn get_action(&self) -> Action {
        self.action
    }

    pub fn get_value(&self) -> i32 {
        self.value
    }

    pub fn get_vote_id(&self) -> u32 {
        self.vote_id
    }
}
