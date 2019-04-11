// command line parsing
extern crate clap;

// sawtooth sdk
extern crate sawtooth_sdk;

use clap::{App, Arg, SubCommand};

use sawtooth_sdk::signing::CryptoFactory;
use sawtooth_sdk::signing::create_context;

use std::fs::File;
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
    let generate_key = SubCommand::with_name("genkey")
                            .about("generates a key and writes to client.key");

    let arguments = App::new("dignitas")
                            .version("0.1")
                            .author("Luís Silva")
                            .about("#TODO")
                            .arg(arg_key_file)
                            .subcommand(generate_key)
                            .get_matches();

    if let Some(arguments) = arguments.subcommand_matches("genkey"){
        genkey();
    }


    println!("Done!");
}

fn genkey(){
    println!("Creating and Storing a Key");

    let context = create_context("secp256k1")
                            .expect("Failed creating context");

    let private_key = context.new_random_private_key()
                            .expect("Failed creating private key");

    let mut file = File::create("client.key")
                            .expect("Failed creating file");

    file.write_all(private_key.as_hex().as_bytes())
                            .expect("Failed writing to file");
}
