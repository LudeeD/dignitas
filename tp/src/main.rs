extern crate sawtooth_sdk;

mod handler;

use sawtooth_sdk::processor::TransactionProcessor;

use handler::handler::SwTransactionHandler;

fn main() {
    let endpoint = "tcp://localhost:4004";

    let handler = SwTransactionHandler::new();
    let mut processor = TransactionProcessor::new(endpoint);

    processor.add_handler(&handler);
    processor.start();
}
