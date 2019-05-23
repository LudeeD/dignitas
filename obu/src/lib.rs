extern crate sawtooth_sdk;
extern crate protobuf;
extern crate crypto;

use sawtooth_sdk::signing::create_context;
use sawtooth_sdk::signing::secp256k1::Secp256k1PrivateKey;
use sawtooth_sdk::signing::PrivateKey;
use sawtooth_sdk::signing::Signer;

use protobuf::Message;

use crypto::digest::Digest;
use crypto::sha2::Sha512;

use std::fs::File;
use std::io::Read;
use std::io::Write;

mod comns;
mod transaction_helper;

use transaction_helper as tp;

pub fn retrieve_dignitas(private_key_file : &str){
    let private_key = key_from_file(private_key_file);

    let context = create_context("secp256k1")
        .expect("Unsupported algorithm");

    let signer = Signer::new(context.as_ref(), private_key.as_ref());

    let pubkey = signer.get_public_key().expect("Something went really wrong");

    let address_wallet = get_addresses(1, &pubkey.as_hex()).get(1).expect("Impossible").clone();

    comns::out::get_state(&address_wallet);

}

pub fn create_vote( private_key_file : &str, vote_id : u32) {
    println!("Going to Create a Vote");

    // Read private key
    let private_key = key_from_file(private_key_file);

    let context = create_context("secp256k1")
        .expect("Unsupported algorithm");


    let signer = Signer::new(context.as_ref(), private_key.as_ref());

    let pubkey = signer.get_public_key().expect("Something went really wrong");

    //Construct Payload
    let payload = vec![String::from("CreateVote"), vote_id.to_string()];
    let payload_string : String = payload.join(",");

    let address = get_addresses(vote_id, &pubkey.as_hex());

    let nonce = String::from("grrr");

    // Create Transactio Header
    let transaction_header = tp::create_transaction_header(
        &address,
        &address,
        payload_string.clone(),
        pubkey,
        nonce
        );

    // Create Transaction
    let transaction = tp::create_transaction(
        &signer,
        transaction_header,
        payload_string,
        );

    // Create Batch Header / Batch
    let batch = tp::create_batch(
        &signer,
        transaction
        );

    // Create Batch List
    let batch_list = tp::create_batch_list(
        batch
        );

    let raw_bytes = batch_list
        .write_to_bytes()
        .expect("Unable to write batch list as bytes");


    comns::out::send(raw_bytes);
}

pub fn vote( private_key_file : &str, vote_id : u32, value: i32){
    println!("Going to Vote");

    // Read private key
    let private_key = key_from_file(private_key_file);

    let context = create_context("secp256k1")
        .expect("Unsupported algorithm");


    let signer = Signer::new(context.as_ref(), private_key.as_ref());

    let pubkey = signer.get_public_key().expect("Something went really wrong");

    //Construct Payload
    let payload = vec![String::from("Vote"), vote_id.to_string(), value.to_string()];
    let payload_string : String = payload.join(",");

    let address = get_addresses(vote_id, &pubkey.as_hex());

    let nonce = String::from("grrr");

    // Create Transactio Header
    let transaction_header = tp::create_transaction_header(
        &address,
        &address,
        payload_string.clone(),
        pubkey,
        nonce
        );

    // Create Transaction
    let transaction = tp::create_transaction(
        &signer,
        transaction_header,
        payload_string,
        );

    // Create Batch Header / Batch
    let batch = tp::create_batch(
        &signer,
        transaction
        );

    // Create Batch List
    let batch_list = tp::create_batch_list(
        batch
        );

    let raw_bytes = batch_list
        .write_to_bytes()
        .expect("Unable to write batch list as bytes");


    comns::out::send(raw_bytes);

}

fn get_addresses(vote_id: u32, pubkey: &str) -> Vec<String> {
    // Get Addresses That Input depends On
    // Namely, the vote addres and use

    //let mut sha = Sha512::new();
    //sha.input_str("dignitas");
    //sha.result_str()[..6].to_string()
    let prefix = "ce9618".to_string();

    let wallet_prefix = "00".to_string();
    let votes_prefix = "01".to_string();

    let pubkeysha = calculate_sha_first_62(pubkey);
    let voteidsha = calculate_sha_first_62(&vote_id.to_string());

    let address_vote = prefix.clone()+&votes_prefix+&voteidsha;
    println!("Address Vote:   {}", address_vote);
    let address_wallet = prefix+&wallet_prefix+&pubkeysha;
    println!("Address Wallet: {}", address_wallet);

    let array = [address_vote, address_wallet];
    array.to_vec()
}


fn calculate_sha_first_62( data : &str ) -> String{
    let mut sha = Sha512::new();
    sha.input_str(data);
    sha.result_str()[..62].to_string()
}

fn key_from_file(file_name: &str) -> Box<PrivateKey> {
    let mut key_hex_data = String::new();
    let mut file = File::open(file_name).expect("Failed opening file");

    file.read_to_string(&mut key_hex_data)
        .expect("Unable to read string");

    let private_key =
        Secp256k1PrivateKey::from_hex(&key_hex_data).expect("Unable to generate private key");

    Box::new(private_key)
}
