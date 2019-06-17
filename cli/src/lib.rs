// sawtooth sdk
extern crate base64;
extern crate rand;
extern crate sawtooth_sdk;
extern crate reqwest;

use base64::decode;

use sawtooth_sdk::signing::create_context;
use sawtooth_sdk::signing::secp256k1::Secp256k1PrivateKey;
use sawtooth_sdk::signing::PrivateKey;
use sawtooth_sdk::signing::Signer;

use crypto::digest::Digest;
use crypto::sha2::Sha512;

use std::fs::File;
use std::io::Read;
use std::io::Write;

use geohash_16::{encode, Coordinate};

mod tp_helper;

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

pub fn create_vote(     private_key : Box<PrivateKey>,
                        title: String,
                        info: String,
                        lat:f64,
                        lng:f64,
                        dir:f64,
                        optional_file : Option<&str>){

    let context = create_context("secp256k1")
        .expect("Unsupported algorithm");

    let signer = Signer::new(context.as_ref(), private_key.as_ref());

    //Construct Payload
    let payload = vec![ String::from("CreateVote"),
    String::from(""),
    String::from(""),
    title,
    info,
    lat.to_string(),
    lng.to_string(),
    dir.to_string()
    ];

    let payload_string : String = payload.join(",");


    let nonce = String::from("grrr");

    let pubkey = signer.get_public_key().expect("Something went really wrong");

    let address = get_addresses(generate_id(lat, lng), "damn");

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

fn get_addresses(vote_id: String, pubkey: &str) -> Box<[String]> {
    // Get Addresses That Input depends On
    // Namely, the vote addres and user balance

    // Vote Address

    let address_vote = calculate_address_votes(vote_id);
    let address_wallet = calculate_address_wallets(pubkey);

    let array = [address_vote, address_wallet];
    Box::new(array)
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

fn generate_id(lat: f64, lng:f64) -> String {
    let c = Coordinate {x: lng, y: lat};
    let c = Coordinate {x: lng, y: lat};
    let encoded : String = encode(c, 12usize)
        .expect("Generating ID");
    encoded
}

fn calculate_address_votes( vote_id: String ) -> String{
    let zero_vec : String = vec!['0';50].into_iter().collect();
    let address = get_votes_prefix() + &vote_id + &zero_vec;
    address
}

#[cfg(test)] // TODO Unit Tests
mod tests {}
