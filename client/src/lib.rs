// sawtooth sdk
extern crate sawtooth_sdk;

use sawtooth_sdk::signing::CryptoFactory;
use sawtooth_sdk::signing::PrivateKey;
use sawtooth_sdk::signing::Signer;
use sawtooth_sdk::signing::create_context;
use sawtooth_sdk::signing::secp256k1::Secp256k1PrivateKey;

use std::fs::File;
use std::io::Read;
use std::io::Write;

pub fn generate_key() -> Box<PrivateKey>{
    println!("Creating and Storing a Key");

    let context = create_context("secp256k1")
        .expect("Failed creating context");

    let private_key = context.new_random_private_key()
        .expect("Failed creating private key");
    private_key
}

pub fn key_to_file( private_key : &PrivateKey, file_name : String){

    let mut file = File::create(file_name)
        .expect("Failed creating file");

    file.write_all(private_key.as_hex().as_bytes())
        .expect("Failed writing to file");
}

pub fn key_from_file( file_name : &str) -> Box<PrivateKey>{

    let context = create_context("secp256k1")
        .expect("Failed creating context");

    let mut key_hex_data = String::new();

    let mut file = File::open(file_name)
        .expect("Failed opening file");

    file.read_to_string(&mut key_hex_data)
        .expect("Unable to read string");

    let private_key = Secp256k1PrivateKey::from_hex(&key_hex_data)
        .expect("Unable to generate private key");

    Box::new(private_key)
}

#[cfg(test)] // TODO Unit Tests
mod tests {

}
