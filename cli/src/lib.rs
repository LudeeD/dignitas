// sawtooth sdk
extern crate rand;
extern crate sawtooth_sdk;
extern crate reqwest;

use sawtooth_sdk::signing::create_context;
use sawtooth_sdk::signing::secp256k1::Secp256k1PrivateKey;
use sawtooth_sdk::signing::PrivateKey;
use sawtooth_sdk::signing::Signer;

use crypto::digest::Digest;
use crypto::sha2::Sha512;

use std::fs::File;
use std::io::Read;
use std::io::Write;

mod tp_helper;

pub fn generate_key() -> Box<PrivateKey> {
    println!("Creating and Storing a Key");

    let context = create_context("secp256k1").expect("Failed creating context");

    let private_key = context
        .new_random_private_key()
        .expect("Failed creating private key");
    private_key
}

pub fn key_to_file(private_key: &PrivateKey, file_name: String) {
    let mut file = File::create(file_name).expect("Failed creating file");

    file.write_all(private_key.as_hex().as_bytes())
        .expect("Failed writing to file");
}

pub fn key_from_file(file_name: &str) -> Box<PrivateKey> {

    let mut key_hex_data = String::new();

    let mut file = File::open(file_name).expect("Failed opening file");

    file.read_to_string(&mut key_hex_data)
        .expect("Unable to read string");

    let private_key =
        Secp256k1PrivateKey::from_hex(&key_hex_data).expect("Unable to generate private key");

    Box::new(private_key)
}

pub fn create_vote( private_key : Box<PrivateKey>, vote_id : u32, optional_file : Option<File>) {

    let context = create_context("secp256k1")
        .expect("Unsupported algorithm");

    let signer = Signer::new(context.as_ref(), private_key.as_ref());

    //Construct Payload
    let payload = vec![String::from("CreateVote"), vote_id.to_string()];
    let payload_string : String = payload.join(",");

    let address = get_addresses(2, "pila");

    let nonce = String::from("grrr");

    let pubkey = signer.get_public_key().expect("Something went really wrong");

    // Create Transactio Header
    let transaction_header = tp_helper::create_transaction_header(
        &address,
        &address,
        payload_string.clone(),
        pubkey,
        nonce
        );

    // Create Transaction
    let transaction = tp_helper::create_transaction(
        &signer,
        transaction_header,
        payload_string,
        );

    // Create Batch Header / Batch
    let batch = tp_helper::create_batch(
        &signer,
        transaction
        );

    // Create Batch List
    let batch_list = tp_helper::create_batch_list(
        batch
        );


    // Handle BatchList

    match optional_file {
        Some(f) => {
            println!("Going to write a file with this Batch List");
            tp_helper::create_batchlist_file(batch_list, f);
        },
        None    => {
            println!("Going to directly send to the API");
            tp_helper::submit_batchlist_to_rest_api(batch_list);
        }
    }
}

fn get_addresses(vote_id: u32, pubkey: &str) -> Box<[String]> {
    // Get Addresses That Input depends On
    // Namely, the vote addres and user balance

    // Vote Address

    let address_vote = calculate_address_votes(&vote_id.to_string());
    let address_wallet = calculate_address_wallets(pubkey);

    let array = [address_vote, address_wallet];
    Box::new(array)
}

// REFACTOR XD
pub fn get_sw_prefix() -> String {
    let mut sha = Sha512::new();
    sha.input_str("dignitas");
    sha.result_str()[..6].to_string()
}

pub fn get_wallets_prefix() -> String {
    let mut sha = Sha512::new();
    sha.input_str("wallets");
    get_sw_prefix() + &sha.result_str()[..2].to_string()
}

pub fn get_votes_prefix() -> String {
    let mut sha = Sha512::new();
    sha.input_str("votes");
    get_sw_prefix() + &sha.result_str()[..2].to_string()
}

fn calculate_address_wallets(name: &str) -> String {
    let mut sha = Sha512::new();
    sha.input_str(name);
    get_wallets_prefix() + &sha.result_str()[..62].to_string()
}

fn calculate_address_votes(name: &str) -> String {
    let mut sha = Sha512::new();
    sha.input_str(name);
    get_votes_prefix() + &sha.result_str()[..62].to_string()
}

#[cfg(test)] // TODO Unit Tests
mod tests {}
