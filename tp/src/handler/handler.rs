use sawtooth_sdk::messages::processor::TpProcessRequest;
use sawtooth_sdk::processor::handler::ApplyError; use sawtooth_sdk::processor::handler::TransactionContext;
use sawtooth_sdk::processor::handler::TransactionHandler;

use crate::handler::payload::Action;
use crate::handler::payload::{PayloadBuilder, PayloadCreateVote, PayloadVote, PayloadCloseVote};
use crate::handler::state::get_sw_prefix;
use crate::handler::state::SwState;
use crate::handler::vote::Vote;

pub struct SwTransactionHandler {
    family_name: String,
    family_versions: Vec<String>,
    namespaces: Vec<String>,
}

//Transactions in dignitas
trait SwTransactions {
    fn create_vote(&self, state: &mut SwState,info : PayloadCreateVote) 
        -> Result<(), ApplyError>;

    fn vote(
        &self,
        state: &mut SwState,
        customer_pubkey: &str,
        info: PayloadVote,
    ) -> Result<(), ApplyError>;

    fn close_vote(&self, state: &mut SwState, info: PayloadCloseVote)
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

        let payload_builder = PayloadBuilder::new(request.get_payload());

        let payload_builder = match payload_builder {
            Err(e) => return Err(e),
            Ok(payload) => payload,
        };

        let mut state = SwState::new(context);

        let _payload = match payload_builder.get_action(){
            Action::CreateVote =>{
                self.create_vote(&mut state, payload_builder.create_vote_payload())
            },
            Action::Vote =>{
                self.vote(&mut state, customer_pubkey,
                          payload_builder.vote_payload())
            },
            Action::CloseVote =>{
                self.close_vote(&mut state, payload_builder.close_vote_payload())
            }
        };

        info!("Apply Function Exited");
        Ok(())
    }
}

impl SwTransactions for SwTransactionHandler {

    fn create_vote(&self, state: &mut SwState, info: PayloadCreateVote)
        -> Result<(), ApplyError> {

        info!("Create Vote Called");

        let vote = Vote::new( info.lat, info.lng, info.direction,
                              &info.title, &info.info, info.timestamp);

        state.set_vote(vote).expect("Something Went Wrong");
        Ok(())
    }

    fn vote(
        &self,
        state: &mut SwState,
        customer_pubkey: &str,
        info: PayloadVote
        ) -> Result<(), ApplyError> {

        info!("Vote Called");

        let current_balance: i64 = match state.get_balance(customer_pubkey) {
            Ok(Some(v)) => v,
            Ok(None) => {
                // Means that the account is new
                // Default Value applies and account is created
                state.set_balance(customer_pubkey, 50).expect("something went wrong!");
                // default value to be returned
                50
            },
            Err(err) => return Err(err),
        };

        info!("Current Balance fetched");

        let abs_value = info.value.abs();

        if abs_value > current_balance {
            return Err(ApplyError::InvalidTransaction(String::from(
                        "You Don't have the credits for it",
                        )));
        }else{
            state.set_balance(customer_pubkey, current_balance-abs_value).expect("Something Went Wrong");
        };

        let mut vote = match state.get_vote(info.vote_id) {
            Ok(Some(v)) => v,
            Ok(None) => return Err(ApplyError::InvalidTransaction(String::from(
                        "Deal with this later",
                        ))),
            Err(err) => return Err(err),
        };
        info!("Vote state fetched");

        // missing parameters 
        if info.value.is_positive() {
            vote.agree_more(abs_value as i64).expect("Something Went Wrong");
        }else{
            vote.disagree_more(abs_value as i64).expect("Something Went Wrong");
        };

        state.set_vote( vote ).expect("Something Went Wrong");
        info!("Vote state updated");

        Ok(())
    }

    fn close_vote(&self, state: &mut SwState, info: PayloadCloseVote)
        -> Result<(), ApplyError> {
        Ok(())
    }
}
