#![feature(proc_macro_hygiene, decl_macro)] // COMNS API
#[macro_use] extern crate rocket;           // COMNS API
#[macro_use] extern crate rocket_contrib;   // COMNS API

extern crate sawtooth_sdk;
extern crate protobuf;
extern crate crypto;
extern crate base64;

use sawtooth_sdk::signing::create_context;
use sawtooth_sdk::signing::secp256k1::Secp256k1PrivateKey;
use sawtooth_sdk::signing::PrivateKey;
use sawtooth_sdk::signing::Signer;

use sawtooth_sdk::messages::transaction::Transaction;
use sawtooth_sdk::messages::transaction::TransactionHeader;

use protobuf::Message;

use crypto::digest::Digest;
use crypto::sha2::Sha512;

use base64::decode;

use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::str;

mod util;
use util::transaction_helper as tp;

mod data;
use data::votes::{Vote};
use data::balance::{Balance};

mod comns;
use comns::api::start_server as start_api;
use comns::out;

pub fn start_server(){
    start_api();
}



pub fn create_vote( private_key_file : &str,
                    payload: String )
{
    println!("Going to Create a Vote");

    // Read private key of OBU
    let private_key = key_from_file(private_key_file);

    let context = create_context("secp256k1")
        .expect("Unsupported algorithm");

    let signer = Signer::new(context.as_ref(), private_key.as_ref());

    // Create Transaction
    // let mut transaction = Transaction::new();
    // transaction.set_header(decode(&header).unwrap()[..].to_vec());
    // transaction.set_header_signature(header_signature);
    // transaction.set_payload(decode(&payload).unwrap()[..].to_vec());
    //
    let mut transaction: Transaction =  protobuf::parse_from_bytes(&decode(&payload).unwrap()[..]).expect("omg, yes");
    let transaction_header : TransactionHeader = protobuf::parse_from_bytes(transaction.get_header()).expect("vai bater");

    println!("transaction header \n{:?}", transaction_header);
    println!(" ");
    println!("transaction \n{:?}", transaction);
    println!(" ");

    // Create Batch Header / Batch
    let batch = tp::create_batch(
        &signer,
        transaction
        );

    println!("batch \n{:?}", batch);

    // Create Batch List
    let batch_list = tp::create_batch_list(
        batch
        );

    let raw_bytes = batch_list
        .write_to_bytes()
        .expect("Unable to write batch list as bytes");


    out::send(raw_bytes);
}

pub fn vote( private_key_file : &str, payload: String){
    println!("Going to Vote");

    // Read private key
    let private_key = key_from_file(private_key_file);

    let context = create_context("secp256k1")
        .expect("Unsupported algorithm");


    let signer = Signer::new(context.as_ref(), private_key.as_ref());

    let mut transaction: Transaction =  protobuf::parse_from_bytes(&decode(&payload).unwrap()[..]).expect("omg, yes");
    let transaction_header : TransactionHeader = protobuf::parse_from_bytes(transaction.get_header()).expect("vai bater");


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


    out::send(raw_bytes);

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

pub fn get_list_votes()-> Vec<Vote> {
    let mut list_of_votes =  Vec::new();

    let r = out::get_state_address("ce961801");

    for data in &r.data{
        let vote_data = data.data.clone();
        list_of_votes.push(Vote::from_tp_response(vote_data));
    }

    list_of_votes
}

pub fn retrieve_dignitas(wallet : &str) -> Balance{

    let r = out::get_state_address(wallet);
    let mut balance = Balance{value: 0};

    if(r.data.len() != 0){
        balance = Balance::from_tp_response(r.data[0].data.clone());
    }

    return balance
}
