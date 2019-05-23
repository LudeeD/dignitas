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

    let arg_output_file = Arg::with_name("output")
        .short("o")
        .long("output")
        .takes_value(true)
        .help("output file for batches");

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

    let arg_unwallet = Arg::with_name("wallet")
        .short("w")
        .long("wallet")
        .takes_value(true)
        .help("wallet unwrapper");

    let arg_unvotes = Arg::with_name("vote")
        .short("v")
        .long("votes")
        .takes_value(true)
        .help("votes unwrapper");


    let genkey_subcmd = SubCommand::with_name("unwrap")
        .about("unwraps the content that are in the leaves")
        .arg(arg_unwallet)
        .arg(arg_unvotes);


        // Argument Parsing
    let arguments = App::new("dignitas")
        .version("0.1")
        .author("Luís Silva")
        .about("#TODO")
        .arg(arg_key_file)
        .arg(arg_action)
        .arg(arg_value)
        .arg(arg_output_file)
        .subcommand(genkey_subcmd)
        .get_matches();

    let action  = arguments.value_of("action").unwrap_or("CreateVote");
    let value : u32 = arguments.value_of("value").unwrap_or("1234").parse().expect("Failed Parsing Number");
    let file    = arguments.value_of("key").unwrap_or("client.key");

    if let Some(_arguments) = arguments.subcommand_matches("genkey"){
        println!("NÂO chegou Aqui");
        // For now generating a key also ends the program
        let key = clignitas::generate_key();
        clignitas::key_to_file(key.as_ref(), file.to_string());
        return
    }

    if let Some(arguments) = arguments.subcommand_matches("unwrap"){
        println!("UnWrapper Subcommand");

        if arguments.is_present("wallet") {
            let value = arguments.value_of("wallet").unwrap();
            println!("UnWrap wallets: {}", &value);
            clignitas::unwrap_balance(value);
        } else if arguments.is_present("vote"){
            let value  = arguments.value_of("vote").unwrap();
            println!("UnWrap votes: {}", &value);
            clignitas::unwrap_votes(value);
        }
        return
    }

    let private_key = clignitas::key_from_file(file);

    let file = arguments.value_of("output");

    clignitas::create_vote(private_key, value, file);

    println!("Done!");
}
