use crate::handler::vote::Vote;

use crypto::digest::Digest;
use crypto::sha2::Sha512;

use std::str;

use sawtooth_sdk::processor::handler::ApplyError;
use sawtooth_sdk::processor::handler::TransactionContext;

use geohash_16::{encode, decode, Coordinate};

pub fn get_sw_prefix() -> String {
    "ce9618".to_string()
}

pub fn get_wallets_prefix() -> String {
    get_sw_prefix() + &"00".to_string()
}

pub fn get_votes_prefix() -> String {
    get_sw_prefix() + &"01".to_string()
}

//Dignitas State
pub struct SwState<'a> {
    context: &'a mut TransactionContext,
}

impl<'a> SwState<'a> {
    pub fn new(context: &'a mut TransactionContext) -> SwState {
        SwState { context: context }
    }

    fn calculate_address_wallets( pubkey: &str ) -> String{
        let mut sha = Sha512::new();
        sha.input_str(pubkey);
        get_wallets_prefix() + &sha.result_str()[..62].to_string()
    }

    fn calculate_address_votes( vote_id: String ) -> String{
        let zero_vec : String = vec!['0';50].into_iter().collect();
        let address = get_votes_prefix() + &vote_id + &zero_vec;
        info!("{}", address);
        address
    }

    pub fn get_balance(&mut self, name: &str) -> Result<Option<i64>, ApplyError> {
        let address = SwState::calculate_address_wallets(name);
        info!("Wallet Address: {}", address);
        let d = self.context.get_state_entry(&address)?;
        match d {
            Some(packed) => {
                let value_string = match String::from_utf8(packed) {
                    Ok(v) => v,
                    Err(_) => {
                        return Err(ApplyError::InvalidTransaction(String::from(
                            "Invalid UTF-8 sequence",
                        )));
                    }
                };

                let value: i64 = match value_string.parse() {
                    Ok(v) => v,
                    Err(_) => {
                        return Err(ApplyError::InvalidTransaction(String::from(
                            "Unable to parse UTF-8 String as u32",
                        )));
                    }
                };

                Ok(Some(value))
            }
            None => {
                Ok(None)
            },
        }
    }

    pub fn set_balance(&mut self, name: &str, value: i64) -> Result<(), ApplyError> {
        self.context
            .set_state_entry(SwState::calculate_address_wallets(name),value.to_string().into_bytes())
            .map_err(|err| ApplyError::InternalError(format!("{}", err)))?;
        Ok(())
    }

    pub fn set_vote(&mut self, v: Vote) -> Result<(), ApplyError>{
        self.context
            .set_state_entry(
                SwState::calculate_address_votes(v.id.clone()),
                v.to_cbor_string().into_bytes()
            )
            .map_err(|err| ApplyError::InternalError(format!("{}", err)))?;
        Ok(())

    }

    pub fn get_vote(&mut self, vote_id: String)
        -> Result<Option<Vote>, ApplyError> {

        let address = SwState::calculate_address_votes(vote_id);
        let d = self.context.get_state_entry(&address)?;
        match d {
            Some(packed) => {
                let value_string = match String::from_utf8(packed) {
                    Ok(v) => v,
                    Err(_) => {
                        return Err(ApplyError::InvalidTransaction(String::from(
                            "Invalid UTF-8 sequence",
                        )));
                    }
                };

                let vote: Vote = match Vote::from_cbor_string(value_string) {
                    Some(v) => v,
                    None => {
                        return Err(ApplyError::InvalidTransaction(String::from(
                            "Unable to parse UTF-8 String as u32",
                        )));
                    }
                };

                Ok(Some(vote))
            }
            None => Ok(None),
        }
    }
}
