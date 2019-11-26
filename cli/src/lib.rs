use std::time::{SystemTime, UNIX_EPOCH};

use base64::decode;

use crypto::digest::Digest;
use crypto::sha2::Sha512;

use openssl::sha::sha512;

use protobuf::{Message, RepeatedField};

use rand::prelude::*;

use sawtooth_sdk::messages::transaction::{Transaction, TransactionHeader};
use sawtooth_sdk::signing::secp256k1::{Secp256k1PrivateKey, Secp256k1PublicKey};
use sawtooth_sdk::signing::{create_context, PublicKey,PrivateKey,Signer };

use std::fs::File;
use std::io::Read;
use std::io::Write;

use std::collections::BTreeMap;
use serde_cbor::to_vec;


const VALIDATOR_REGISTRY: &str = "dignitas";
const VALIDATOR_REGISTRY_VERSION: &str = "1.0";

// UNWRAP Section
pub fn unwrap_votes(vote: &str){
    println!("Vote ID | Agree | Disagree");
    let decoded = decode(vote).expect("Upsi");
    println!("{}", String::from_utf8(decoded).expect("Upsi a dobrar"));
}

pub fn unwrap_balance(vote:  &str){
    println!("Value");
    let decoded = decode(vote).expect("Upsi");
    println!("{}", String::from_utf8(decoded).expect("Upsi a dobrar"));
}

// GENKEY Section
pub fn generate_key() -> Box<PrivateKey> {
    println!("Creating and Storing a Key");

    let context = create_context("secp256k1").expect("Failed creating context");

    let private_key = context
        .new_random_private_key()
        .expect("Failed creating private key");
    private_key
}

pub fn key_to_file(private_key: &PrivateKey, file_name: String) {
    let mut file_priv = File::create(file_name.clone()).expect("Failed creating file");

    let file_name_pub = format!("pub.{}", &file_name);

    let mut file_pub = File::create(file_name_pub).expect("Failed creating file");

    let context = create_context("secp256k1")
        .expect("Unsupported algorithm");

    let signer = Signer::new(context.as_ref(), private_key);

    let pubkey = signer.get_public_key().expect("Fuck");

    file_priv.write_all(private_key.as_hex().as_bytes())
        .expect("Failed writing to file");

    file_pub.write_all(pubkey.as_hex().as_bytes())
        .expect("Failed writing to file");
}

pub fn pub_key_from_hex(hex_string: &str) -> Box<PublicKey> {
    let key =
        Secp256k1PublicKey::from_hex(hex_string)
        .expect("Unable to Generate Pub Key from Hex");

    Box::new(key)
}

pub fn priv_key_from_file(file_name: &str) -> Box<PrivateKey> {

    let mut key_hex_data = String::new();

    let mut file = File::open(file_name).expect("Failed opening file");

    file.read_to_string(&mut key_hex_data)
        .expect("Unable to read string");

    let private_key =
        Secp256k1PrivateKey::from_hex(&key_hex_data).expect("Unable to generate private key");

    Box::new(private_key)
}

pub fn
generate_transaction(payload: Vec<u8>, private_key : Box<PrivateKey>, batcher_key : Box<PublicKey>) -> Transaction {

    let context = create_context("secp256k1") .expect("Unsupported algorithm");
    let signer = Signer::new(context.as_ref(), private_key.as_ref());
    let pubkey = signer.get_public_key().expect("Something went really wrong");
    let address = get_addresses(&pubkey.as_hex());
    // Create Transactio Header 
    let transaction_header = create_transaction_header( &address, &address, payload.clone(), pubkey, batcher_key);

    // Create Transaction
    let transaction = create_transaction(
        &signer,
        transaction_header,
        payload,
        );

    transaction
}

pub fn
vote( private_key : Box<PrivateKey>, batcher_key : Box<PublicKey>, vote_id: String, value: i64, client: &reqwest::Client){
    let value_str = value.to_string();

    let mut payload = BTreeMap::new();
    payload.insert("action", "vote");
    payload.insert("voteID",&vote_id);
    payload.insert("value", &value_str);
    let payload_bytes = to_vec(&payload).expect("Encoding Went Wrong");

    let transaction = generate_transaction(payload_bytes, private_key, batcher_key);

    println!("{:#?}", transaction);

    println!("Sending Vote to OBU...");
    submit_transaction_to_obu_api(transaction, client);
    println!("Done!");
}

pub fn create_vote(private_key : Box<PrivateKey>,
                   batcher_key : Box<PublicKey>,
                   title: String,
                   info: String,
                   lat:f64,
                   lng:f64,
                   dir:f64,
                   client: &reqwest::Client
                   )
{
    let timestamp_str = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Something Really weird Happened")
        .as_secs()
        .to_string();
    let lat_str = lat.to_string();
    let lng_str = lng.to_string();
    let dir_str = dir.to_string();

    let mut payload = BTreeMap::new();
    payload.insert("action", "create");
    payload.insert("title", &title);
    payload.insert("info",  &info);
    payload.insert("lat", &lat_str);
    payload.insert("lng", &lng_str);
    payload.insert("dir", &dir_str);
    payload.insert("timestamp", &timestamp_str);

    let payload_bytes = to_vec(&payload).expect("Encoding Went Wrong");

    let transaction = generate_transaction(payload_bytes, private_key, batcher_key);

    //println!("Sending Vote Creation to OBU...");
    submit_transaction_to_obu_api(transaction, client);
    //println!("Done!");
}

pub fn get_sw_prefix() -> String {
    "ce9618".to_string()
}

pub fn get_wallets_prefix() -> String {
    get_sw_prefix() + &"00".to_string()
}

pub fn get_votes_prefix() -> String {
    get_sw_prefix() + &"01".to_string()
}

fn calculate_address_wallets( pubkey: &str ) -> String{
    let mut sha = Sha512::new();
    sha.input_str(pubkey);
    get_wallets_prefix() + &sha.result_str()[..62].to_string()
}

fn get_addresses(pubkey: &str) -> Box<[String]> {
    let address_vote = get_votes_prefix();
    let address_wallet = calculate_address_wallets(pubkey);

    let array = [address_vote, address_wallet];
    Box::new(array)
}

fn to_hex_string(bytes: &Vec<u8>) -> String {
    let strs: Vec<String> = bytes.iter().map(|b| format!("{:02x}", b)).collect();
    strs.join("")
}


fn submit_transaction_to_obu_api(transaction: Transaction, client: &reqwest::Client) {
    // Create request body, which in this case is batch list
    let raw_bytes = transaction
        .write_to_bytes()
        .expect("Unable to write batch list as bytes");

    let _res = client
        .post("http://127.0.0.1:8000/api/v1/transaction")
        .header("Content-Type", "application/octet-stream")
        .body(raw_bytes)
        .send();
}

pub fn create_transaction_header(
    input_addresses:    &[String],
    output_addresses:   &[String],
    payload:            Vec<u8>,
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
    transaction_header.set_payload_sha512(to_hex_string(&sha512(&payload).to_vec()));
    transaction_header.set_signer_public_key(public_key.as_hex());
    transaction_header.set_batcher_public_key(batcher_public_key.as_hex());
    transaction_header.set_inputs(RepeatedField::from_vec(input_addresses.to_vec()));
    transaction_header.set_outputs(RepeatedField::from_vec(output_addresses.to_vec()));

    transaction_header
}

pub fn create_transaction(
    signer: &Signer,
    transaction_header: TransactionHeader,
    payload: Vec<u8>,
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
    transaction.set_payload(payload);
    transaction
}
