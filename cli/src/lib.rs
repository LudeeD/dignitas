// sawtooth sdk
extern crate base64;
extern crate sawtooth_sdk;
extern crate reqwest;

use base64::decode;

use sawtooth_sdk::signing::create_context;
use sawtooth_sdk::signing::secp256k1::Secp256k1PrivateKey;
use sawtooth_sdk::signing::secp256k1::Secp256k1PublicKey;
use sawtooth_sdk::signing::PublicKey;
use sawtooth_sdk::signing::PrivateKey;
use sawtooth_sdk::signing::Signer;

use crypto::digest::Digest;
use crypto::sha2::Sha512;

use std::fs::File;
use std::io::Read;
use std::io::Write;

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

pub fn create_vote_obu(private_key : Box<PrivateKey>,
                       batcher_key : Box<PublicKey>,
                       title: String,
                       info: String,
                       lat:f64,
                       lng:f64,
                       dir:f64,
                       optional_file : Option<&str>)
{

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

    let context = create_context("secp256k1")
        .expect("Unsupported algorithm");

    let signer = Signer::new(context.as_ref(), private_key.as_ref());

    let pubkey = signer.get_public_key().expect("Something went really wrong");

    let address = get_addresses(&pubkey.as_hex());

    // Create Transactio Header
    let transaction_header = tp_helper::create_transaction_header(
        &address,
        &address,
        payload_string.clone(),
        pubkey,
        batcher_key
    );

    // Create Transaction
    let transaction = tp_helper::create_transaction(
        &signer,
        transaction_header,
        payload_string,
        );

    match optional_file {
        Some(f) => {
            println!("Going to write a file with this Batch List");
            tp_helper::create_transaction_file(transaction, f);
        },
        None    => {
            println!("Sending Vote to OBU...");
            tp_helper::submit_transaction_to_obu_api(transaction);
            println!("Done!");
        }
    }
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
