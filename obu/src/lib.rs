#![feature(proc_macro_hygiene, decl_macro)] // COMNS API
#[macro_use] extern crate rocket;           // COMNS API

extern crate sawtooth_sdk;
extern crate protobuf;
extern crate crypto;
extern crate base64;

use sawtooth_sdk::signing::create_context;
use sawtooth_sdk::signing::secp256k1::Secp256k1PrivateKey;
use sawtooth_sdk::signing::PrivateKey;
use sawtooth_sdk::signing::Signer;

use sawtooth_sdk::messages::transaction::Transaction;

use protobuf::Message;

use base64::decode;

use std::fs::File;
use std::io::Read;
use std::str;

mod util;
use util::transaction_helper as tp;

mod data;
use data::votes::{Vote};
use data::balance::{Balance};

mod comns;
use comns::api::start_server as start_api;
use comns::out;

const OBU_KEY_FILE_NAME : &str = "client.key";

pub fn start_server(){
    start_api();
}

pub fn proxy_transaction( payload: String )
{
    println!("Going to Create a Batch For the Received Transaction");

    // Read private key of OBU
    let private_key = key_from_file(OBU_KEY_FILE_NAME);

    let context = create_context("secp256k1")
        .expect("Unsupported algorithm");

    let signer = Signer::new(context.as_ref(), private_key.as_ref());

    let transaction: Transaction =  protobuf::parse_from_bytes(&decode(&payload).unwrap()[..]).expect("omg, yes");

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

    match r {
        Ok(t) => {
            for data in &t.data{
                let vote_data = data.data.clone();
                list_of_votes.push(Vote::from_tp_response(vote_data));
            }
        }
        Err(e) => {
            println!("{}", e)
        }

    }
    list_of_votes
}

pub fn retrieve_dignitas(wallet : &str) -> Balance{

    let r = out::get_state_address(wallet);
    let mut balance = Balance{value: -1};

    match r {
        Ok(t) => {
            if t.data.len() != 0 {
                balance = Balance::from_tp_response(t.data[0].data.clone());
            }
        }
        Err(e) => {
            println!("{}", e)
        }
    }

    return balance
}
