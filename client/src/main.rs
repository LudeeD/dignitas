// command line parsing
extern crate clap;

// sawtooth sdk
extern crate sawtooth_sdk;

use clap::{App, Arg, SubCommand};

use sawtooth_sdk::signing::CryptoFactory;
use sawtooth_sdk::signing::PrivateKey;
use sawtooth_sdk::signing::Signer;
use sawtooth_sdk::signing::create_context;
use sawtooth_sdk::signing::secp256k1::Secp256k1PrivateKey;

use std::fs::File;
use std::io::Read;
use std::io::Write;

fn main() {

    // Available Arguments
    //
    let arg_key_file = Arg::with_name("key")
                            .short("k")
                            .long("key")
                            .takes_value(true)
                            .help("key file");

    // Available Subcommands
    //
    let genkey_subcmd = SubCommand::with_name("genkey")
                            .about("generates a key and writes to client.key");

    let arguments = App::new("dignitas")
                            .version("0.1")
                            .author("Luís Silva")
                            .about("#TODO")
                            .arg(arg_key_file)
                            .subcommand(genkey_subcmd)
                            .get_matches();

    if let Some(arguments) = arguments.subcommand_matches("genkey"){
        // For now generating a key also ends the program
        let key = generate_key();
        key_to_file(key.as_ref());
        return
    }

    let file = arguments.value_of("key").unwrap_or("client.key");

    let private_key = key_from_file(file);


    println!("Done!");
}

fn generate_key() -> Box<PrivateKey>{
    println!("Creating and Storing a Key");

    let context = create_context("secp256k1")
                            .expect("Failed creating context");

    let private_key = context.new_random_private_key()
                            .expect("Failed creating private key");
    private_key
}

fn key_to_file( private_key : &PrivateKey){

    let mut file = File::create("client.key")
                            .expect("Failed creating file");

    file.write_all(private_key.as_hex().as_bytes())
                            .expect("Failed writing to file");
}

fn key_from_file( file_name : &str) -> Box<PrivateKey>{

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
