use sawtooth_sdk::messages::processor::TpProcessRequest;
use sawtooth_sdk::processor::handler::ApplyError;
use sawtooth_sdk::processor::handler::TransactionContext;
use sawtooth_sdk::processor::handler::TransactionHandler;

pub fn get_xo_prefix() -> String {
    String::from("pula")
}

pub struct XoTransactionHandler {
    family_name: String,
    family_versions: Vec<String>,
    namespaces: Vec<String>,
}

impl XoTransactionHandler {
    pub fn new() -> XoTransactionHandler {
        XoTransactionHandler {
            family_name: String::from("xo"),
            family_versions: vec![String::from("1.0")],
            namespaces: vec![String::from(get_xo_prefix().to_string())],
        }
    }
}

impl TransactionHandler for XoTransactionHandler {
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
        // --snip--
        Ok(())
    }
}

