use sawtooth_sdk::messages::processor::TpProcessRequest;
use sawtooth_sdk::processor::handler::ApplyError; use sawtooth_sdk::processor::handler::TransactionContext;
use sawtooth_sdk::processor::handler::TransactionHandler;

use crate::handler::state::get_sw_prefix;
use crate::handler::state::SwState;
use crate::handler::vote::Vote;

use std::collections::BTreeMap;
use serde_cbor::from_slice;

pub struct SwTransactionHandler {
    family_name: String,
    family_versions: Vec<String>,
    namespaces: Vec<String>,
}

//Transactions in dignitas
trait SwTransactions {
    fn create_vote(&self, state: &mut SwState,info : BTreeMap<String,String>)
        -> Result<(), ApplyError>;

    fn vote( &self, state: &mut SwState, customer_pubkey: &str, info: BTreeMap<String,String>,)
        -> Result<(), ApplyError>;

    fn close_vote(&self, state: &mut SwState, info: BTreeMap<String,String>)
        -> Result<(), ApplyError>;

    fn reward(&self, state: &mut SwState, info: BTreeMap<String,String>)
        -> Result<(), ApplyError>;
}

impl SwTransactionHandler {
    pub fn new() -> SwTransactionHandler {
        SwTransactionHandler {
            family_name: String::from("dignitas"),
            family_versions: vec![String::from("1.0")],
            namespaces: vec![String::from(get_sw_prefix().to_string())],
        }
    }
}

impl TransactionHandler for SwTransactionHandler {
    fn family_name(&self) -> String {
        self.family_name.clone()
    }

    fn family_versions(&self) -> Vec<String> {
        self.family_versions.clone()
    }

    fn namespaces(&self) -> Vec<String> {
        self.namespaces.clone()
    }

    fn apply(
        &self,
        request: &TpProcessRequest,
        context: &mut TransactionContext,
        ) -> Result<(), ApplyError> {

        info!("Apply Function Called");
        let header = &request.header;
        let customer_pubkey = match &header.as_ref() {
            Some(s) => &s.signer_public_key,
            None => {
                return Err(ApplyError::InvalidTransaction(String::from(
                            "Invalid header",
                            )));
            }
        };

        let payload: BTreeMap<String, String>
            = from_slice(request.get_payload()).expect("Failed Unpacking Payload");

        let mut state = SwState::new(context);

        match payload.get("action") {
            Some(action) => {
                match action.as_ref() {
                    "create" =>{
                        self.create_vote(&mut state, payload)
                    },
                    "vote" => {
                        let vote_id = payload.get("voteID").unwrap();
                        let vote_value = payload.get("value").unwrap();
                        self.vote(&mut state, customer_pubkey, payload.clone())?;

                        context.add_event(
                            "dignitas/create".to_string(),
                            vec![   ("vote_id".to_string(), vote_id.to_string()),
                            ("voter".to_string(),   customer_pubkey.to_string()),
                            ("value".to_string(),   vote_value.to_string()),
                            ],
                            vec![].as_slice()).expect("Something Went wrong sending the Vote Event");

                        Ok(())
                    },
                    "close" => {
                        self.close_vote(&mut state, payload)
                    },
                    "reward" => {
                        self.reward(&mut state, payload)
                    },
                    _ => {
                        Err(ApplyError::InternalError(String::from("No Action in Payload")))
                    }
                }.expect("Damn, pretty bad error");
                Ok(())
            },
            None =>{
                Err(ApplyError::InternalError(String::from("No Action in Payload")))
            }
        }
    }
}

impl SwTransactions for SwTransactionHandler {

    fn create_vote(&self, state: &mut SwState, info: BTreeMap<String,String>) -> Result<(), ApplyError> {
        // TODO refactor

        info!("Creating New Vote");

        let title   = info.get("title").expect("Bad Payload");
        let details = info.get("info").expect("Bad Payload");
        let lat: f64= info.get("lat").expect("Bad Payload")
            .parse().expect("Bad Payload");
        let lng: f64= info.get("lng").expect("Bad Payload")
            .parse().expect("Bad Payload");
        let dir: f64= info.get("dir").expect("Bad Payload")
            .parse().expect("Bad Payload");
        let time:u64= info.get("timestamp").expect("Bad Payload")
            .parse().expect("Bad Payload");

        let vote = Vote::new(lat,lng,dir,title,details,time);

        state.set_vote(vote)
    }

    fn vote( &self, state: &mut SwState, customer_pubkey: &str, info: BTreeMap<String,String>)
        -> Result<(), ApplyError> {

            info!("Voting");

            let value : i64 = match info.get("value"){
                Some(v) => {
                    v.parse()
                        .map_err(|err| ApplyError::InternalError(format!("{}", err)))?
                },
                None => return Err(ApplyError::InvalidTransaction(String::from("Bad Payload"))),
            };

            let vote_id = match info.get("voteID"){
                Some(v) => v.to_string(),
                None => return Err(ApplyError::InvalidTransaction(String::from("Bad Payload"))),
            };


            let current_balance: i64 = match state.get_balance(customer_pubkey) {
                Ok(Some(v)) => v,
                Ok(None) => {
                    // Means that the account is new
                    // Default Value applies and account is created
                    state.set_balance(customer_pubkey, 50)?;
                    // default value to be returned
                    50
                },
                Err(err) => return Err(err),
            };

            let abs_value = value.abs();

            if abs_value > current_balance {
                return Err(ApplyError::InvalidTransaction(String::from("You Don't have the credits for it",)));
            }else{
                state.set_balance(customer_pubkey, current_balance-abs_value)?;
            };

            let mut vote = state.get_vote(vote_id)?;

            // maybe refactor to had more fields to the payload instead of playing with positive and
            // negatives

            if value.is_positive() {
                vote.agree_more(abs_value as i64)
                    .map_err(|err| ApplyError::InternalError(format!("{}", err)))?;
            }else{
                vote.disagree_more(abs_value as i64)
                    .map_err(|err| ApplyError::InternalError(format!("{}", err)))?;
            };

            state.set_vote( vote )
        }

    fn close_vote(&self, state: &mut SwState, info: BTreeMap<String,String>)
        -> Result<(), ApplyError> {
            // TODO

            info!("Close Vote");

            Ok(())
        }

    fn reward(&self, state: &mut SwState, info: BTreeMap<String,String>)
        -> Result<(), ApplyError> {

            info!("Rewarding People");

            let voter = match info.get("voter"){
                Some(v) => v,
                None => return Err(ApplyError::InvalidTransaction(String::from("Bad Payload"))),
            };

            let value : i64 = match info.get("value"){
                Some(v) => {
                    v.parse()
                        .map_err(|err| ApplyError::InternalError(format!("{}", err)))?
                },
                None => return Err(ApplyError::InvalidTransaction(String::from("Bad Payload"))),
            };

            let curr_balance: i64 = match state.get_balance(voter) {
                Ok(Some(v)) => v,
                _ => return Err(ApplyError::InternalError(String::from("Fetching balance Failed")))
            };

            let new_value = value + curr_balance;

            info!("{}",format!("curr: {} | value: {} | new: {}", curr_balance, value, new_value));

            state.set_balance(voter, new_value)
        }
}
