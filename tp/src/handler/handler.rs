use sawtooth_sdk::messages::processor::TpProcessRequest;
use sawtooth_sdk::processor::handler::ApplyError;
use sawtooth_sdk::processor::handler::TransactionContext;
use sawtooth_sdk::processor::handler::TransactionHandler;

use crate::handler::payload::Action;
use crate::handler::payload::SwPayload;
use crate::handler::state::get_sw_prefix;
use crate::handler::state::SwState;
use crate::handler::vote::Vote;

pub struct SwTransactionHandler {
    family_name: String,
    family_versions: Vec<String>,
    namespaces: Vec<String>,
}

//Transactions in simple wallet
trait SwTransactions {
    fn create_vote(&self, state: &mut SwState, vote_id: u32) -> Result<(), ApplyError>;
    fn vote(
        &self,
        state: &mut SwState,
        customer_pubkey: &str,
        vote_id: u32,
        value: u32,
    ) -> Result<(), ApplyError>;
    fn close_vote(&self, state: &mut SwState, vote_id: u32) -> Result<(), ApplyError>;
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

        let payload = SwPayload::new(request.get_payload());
        let payload = match payload {
            Err(e) => return Err(e),
            Ok(payload) => payload,
        };

        let payload = match payload {
            Some(x) => x,
            None => {
                return Err(ApplyError::InvalidTransaction(String::from(
                    "Request must contain a payload",
                )));
            }
        };

        let mut state = SwState::new(context);

        debug!("Payload {} {}", payload.get_vote_id(), payload.get_value());

        match payload.get_action() {
            Action::CreateVote => {
                let vote_id = payload.get_vote_id();
                self.create_vote(&mut state, vote_id)?;
            }

            Action::Vote => {
                let vote_id = payload.get_vote_id();
                let value_to_vote = payload.get_value();
                self.vote(&mut state, customer_pubkey, vote_id, value_to_vote)?;
            }

            Action::CloseVote => {
                let vote_id = payload.get_vote_id();
                self.close_vote(&mut state, vote_id)?;
            }
        }

        info!("Apply Function Exited");
        Ok(())
    }
}

impl SwTransactions for SwTransactionHandler {
    fn create_vote(&self, state: &mut SwState, vote_id: u32) -> Result<(), ApplyError> {
        info!("Create Vote Called");
        let vote = Vote::new(vote_id);
        state.set_vote(vote_id, vote);
        Ok(())
    }

    fn vote(
        &self,
        state: &mut SwState,
        customer_pubkey: &str,
        vote_id: u32,
        value: u32,
    ) -> Result<(), ApplyError> {
        let current_balance: u32 = match state.get_balance(customer_pubkey) {
            Ok(Some(v)) => v,
            Ok(None) => 0,
            Err(err) => return Err(err),
        };

        if value > current_balance {
            return Err(ApplyError::InvalidTransaction(String::from(
                "You Don't have the credits for it",
            )));
        }else{
            state.set_balance(customer_pubkey, current_balance-value);
        }

        // Maybe no need for both errors, refactor!
        let mut vote = match state.get_vote(vote_id) {
            Ok(Some(v)) => v,
            Ok(None) => return Err(ApplyError::InvalidTransaction(String::from(
                "Deal with this later",
            ))),
            Err(err) => return Err(err),
        };

        // missing parameters 
        vote.agree_more(value);


        state.set_vote( vote_id, vote);

        Ok(())
    }

    fn close_vote(&self, state: &mut SwState, vote_id: u32) -> Result<(), ApplyError> {
        Ok(())
    }
}
