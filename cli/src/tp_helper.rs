extern crate openssl;
extern crate protobuf;
extern crate sawtooth_sdk;

use std::fs::File;
use std::io::Write;
use rand::prelude::*;

use openssl::sha::sha512;

use protobuf::{Message, RepeatedField};
use sawtooth_sdk::{
    messages::{
        batch::{Batch, BatchHeader, BatchList},
        transaction::{Transaction, TransactionHeader},
    },
    signing::{ PublicKey, Signer},
};

use base64::{encode, decode};

use serde_json::json;

const VALIDATOR_REGISTRY: &str = "dignitas";
const VALIDATOR_REGISTRY_VERSION: &str = "1.0";

pub fn create_batch_list(batch: Batch) -> BatchList {
    // Construct batch list
    let batches = RepeatedField::from_vec(vec![batch]);
    let mut batch_list = BatchList::new();
    batch_list.set_batches(batches);
    batch_list
}

pub fn create_batch(signer: &Signer, transaction: Transaction) -> Batch {
    // Construct BatchHeader
    let mut batch_header = BatchHeader::new();
    // set signer public key
    let public_key = signer
        .get_public_key()
        .expect("Unable to get public key")
        .as_hex();
    let transaction_ids = vec![transaction.clone()]
        .iter()
        .map(|trans| String::from(trans.get_header_signature()))
        .collect();
    batch_header.set_transaction_ids(RepeatedField::from_vec(transaction_ids));
    batch_header.set_signer_public_key(public_key);

    // Construct Batch
    let batch_header_bytes = batch_header
        .write_to_bytes()
        .expect("Error converting batch header to bytes");
    let signature = signer
        .sign(&batch_header_bytes)
        .expect("Error signing the batch header");
    let mut batch = Batch::new();
    batch.set_header_signature(signature);
    batch.set_header(batch_header_bytes);
    batch.set_transactions(RepeatedField::from_vec(vec![transaction]));
    batch
}

pub fn create_transaction_header(
    input_addresses:    &[String],
    output_addresses:   &[String],
    payload:            String,
    public_key:         Box<PublicKey>,
    batcher_public_key: Box<PublicKey>,
) -> TransactionHeader {
    let mut rng = rand::thread_rng();
    let nonce: f64 = rng.gen();
    let nonce_string: String = nonce.to_string();

    let mut transaction_header = TransactionHeader::new();
    transaction_header.set_family_name(VALIDATOR_REGISTRY.to_string());
    transaction_header.set_family_version(VALIDATOR_REGISTRY_VERSION.to_string());
    transaction_header.set_nonce(nonce_string);
    transaction_header.set_payload_sha512(to_hex_string(&sha512(&payload.as_bytes()).to_vec()));
    transaction_header.set_signer_public_key(public_key.as_hex());
    transaction_header.set_batcher_public_key(batcher_public_key.as_hex());
    transaction_header.set_inputs(RepeatedField::from_vec(input_addresses.to_vec()));
    transaction_header.set_outputs(RepeatedField::from_vec(output_addresses.to_vec()));

    transaction_header
}

pub fn create_transaction(
    signer: &Signer,
    transaction_header: TransactionHeader,
    payload: String,
) -> Transaction {
    // Construct a transaction, it has transaction header, signature and payload
    let transaction_header_bytes = transaction_header
        .write_to_bytes()
        .expect("Error converting transaction header to bytes");
    let transaction_header_signature = signer
        .sign(&transaction_header_bytes.to_vec())
        .expect("Error signing the transaction header");
    let mut transaction = Transaction::new();
    transaction.set_header(transaction_header_bytes.to_vec());
    transaction.set_header_signature(transaction_header_signature);
    transaction.set_payload(payload.into_bytes());
    transaction
}

pub fn submit_transaction_to_obu_api(transaction: Transaction) {
    // Create request body, which in this case is batch list
    let raw_bytes = transaction
        .write_to_bytes()
        .expect("Unable to write batch list as bytes");

    let encoded = encode(&raw_bytes);

    let post = json!({
        "id": 0,
        "payload": encoded
    });

    let client = reqwest::Client::new();
    let _res = client
        .post("http://127.0.0.1:8000/api/v1/vote")
        .header("Content-Type", "application/json")
        .body(post.to_string())
        .send();
}

// #TODO
pub fn create_transaction_file(transaction: Transaction, file_name: &str) {
    // Create request body, which in this case is batch list
    let raw_bytes = transaction
        .write_to_bytes()
        .expect("Unable to write batch list as bytes");

    let mut file = File::create(file_name).expect("Error creating file");
    file.write_all(&raw_bytes).expect("Error writing bytes");
}

pub fn to_hex_string(bytes: &Vec<u8>) -> String {
    let strs: Vec<String> = bytes.iter().map(|b| format!("{:02x}", b)).collect();
    strs.join("")
}
