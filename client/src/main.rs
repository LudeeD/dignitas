// our own crate

// command line parsing
extern crate clap;

use clap::{App, Arg, SubCommand};

use clignitas;

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

    let file = arguments.value_of("key").unwrap_or("client.key");

    if let Some(arguments) = arguments.subcommand_matches("genkey"){
        // For now generating a key also ends the program
        let key = clignitas::generate_key();
        clignitas::key_to_file(key.as_ref(), file.to_string());
        return
    }


    let private_key = clignitas::key_from_file(file);

    println!("Done!");
}


