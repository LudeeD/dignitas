// our own crate

// command line parsing
extern crate clap;

use clap::{App, Arg, SubCommand};

use clignitas;

use std::fs::File;

fn main() {

    // Available Arguments
    let arg_key_file = Arg::with_name("key")
        .short("k")
        .long("key")
        .takes_value(true)
        .help("key file");

    let arg_action = Arg::with_name("action")
        .short("a")
        .long("action")
        .takes_value(true)
        .possible_values(&["CreateVote", "Vote"])
        .help("action to send to the DL");

    let arg_value = Arg::with_name("value")
        .short("v")
        .long("value")
        .takes_value(true)
        .help("value to send to the DL");

    // Available Subcommands
    let genkey_subcmd = SubCommand::with_name("genkey")
        .about("generates a key and writes to client.key");



    // Argument Parsing
    let arguments = App::new("dignitas")
        .version("0.1")
        .author("Luís Silva")
        .about("#TODO")
        .arg(arg_key_file)
        .arg(arg_action)
        .arg(arg_value)
        .subcommand(genkey_subcmd)
        .get_matches();

    let action  = arguments.value_of("action").unwrap_or("CreateVote");
    let value : u32 = arguments.value_of("value").unwrap_or("1234").parse().expect("Failed Parsing Number");
    let file    = arguments.value_of("key").unwrap_or("client.key");

    if let Some(_arguments) = arguments.subcommand_matches("genkey"){
        // For now generating a key also ends the program
        let key = clignitas::generate_key();
        clignitas::key_to_file(key.as_ref(), file.to_string());
        return
    }


    let private_key = clignitas::key_from_file(file);

    let file_batches = File::open("dignitas.batches").expect("Failed opening file");
    clignitas::create_vote(private_key, value, Some(file_batches));

    println!("Done!");
}
