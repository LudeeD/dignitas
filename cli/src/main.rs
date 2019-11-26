// command line parsing
extern crate clap;

use std::thread;

use clap::{App, Arg, SubCommand};
use std::time::{Duration, Instant};

use clignitas;

fn main() {

    // Available Arguments
    let arg_key_file = Arg::with_name("key")
        .short("k")
        .long("key")
        .takes_value(true)
        .help("key file");

    let arg_obu_pubk = Arg::with_name("obu")
        .long("obu")
        .takes_value(true)
        .help("obu hex pub key");

    let arg_output_file = Arg::with_name("output")
        .short("o")
        .long("output")
        .takes_value(true)
        .help("output file for batches");

    let arg_action = Arg::with_name("action")
        .short("a")
        .long("action")
        .takes_value(true)
        .possible_values(&["CreateVote", "Vote", "stress"])
        .help("action to send to the DL");

    let arg_vote_id = Arg::with_name("voteID")
        .long("voteid")
        .takes_value(true)
        .help("Vote Id to vote for");

    let arg_value = Arg::with_name("value")
        .short("v")
        .long("value")
        .takes_value(true)
        .allow_hyphen_values(true)
        .help("value to send to the DL");

    let arg_lat = Arg::with_name("lat")
        .long("lat")
        .takes_value(true)
        .allow_hyphen_values(true)
        .help("latitude");

    let arg_lng = Arg::with_name("lng")
        .long("lng")
        .takes_value(true)
        .allow_hyphen_values(true)
        .help("longitude");

    let arg_dir = Arg::with_name("dir")
        .long("dir")
        .takes_value(true)
        .help("longitude");

    let arg_title = Arg::with_name("title")
        .short("t")
        .takes_value(true)
        .help("title");

    let arg_info = Arg::with_name("info")
        .short("i")
        .takes_value(true)
        .help("more info");

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


    let unwrap_subcmd = SubCommand::with_name("unwrap")
        .about("unwraps the content that are in the leaves")
        .arg(arg_unwallet)
        .arg(arg_unvotes);

    let number_testng = Arg::with_name("number")
        .short("n")
        .takes_value(true)
        .help("number of requests to send");

    let tmax = Arg::with_name("tmax")
        .short("m")
        .takes_value(true)
        .help("number of max threads");



    // Argument Parsing
    let arguments = App::new("dignitas")
        .version("0.1")
        .author("Luís Silva")
        .about("#TODO")
        .arg(number_testng)
        .arg(tmax)
        .arg(arg_key_file)
        .arg(arg_action)
        .arg(arg_value)
        .arg(arg_vote_id)
        .arg(arg_output_file)
        .arg(arg_lat)
        .arg(arg_lng)
        .arg(arg_title)
        .arg(arg_info)
        .arg(arg_dir)
        .arg(arg_obu_pubk)
        .subcommand(genkey_subcmd)
        .subcommand(unwrap_subcmd)
        .get_matches();

    let file = arguments.value_of("key").unwrap_or("client.key");

    if let Some(_arguments) = arguments.subcommand_matches("genkey"){
        // Generating a key also ends the program
        let key = clignitas::generate_key();
        clignitas::key_to_file(key.as_ref(), file.to_string());
        println!("=END=");
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

    let action  = arguments.value_of("action").unwrap_or("CreateVote");

    let private_key = clignitas::priv_key_from_file(file);

    let obu_pk = arguments.value_of("obu")
        .unwrap_or("02381caa0892d913daa3c4856a4f9b665931964b3fc630ef9dd5edbd8a27952f7e");

    let batcher_key = clignitas::pub_key_from_hex(obu_pk);


    let client = reqwest::Client::new();
    match action{
        "CreateVote" => {

            let title = arguments.value_of("title")
                .unwrap_or("Title").to_string();
            let info = arguments.value_of("info")
                .unwrap_or("Info").to_string();
            let lat = arguments.value_of("lat")
                .unwrap_or("40.6405").parse().expect("Failed Parsing Lat");
            let lng = arguments.value_of("lng")
                .unwrap_or("8.6538").parse().expect("Failed Parsing Lng");
            let dir = arguments.value_of("dir")
                .unwrap_or("0.0").parse().expect("dir");;

            clignitas::create_vote( private_key, batcher_key, title, info, lat, lng, dir, &client);
        },
        "Vote" => {

            let vote_id = arguments.value_of("voteID").expect("Failed Parsing Vote ID").to_string();
            let value : i64 = arguments.value_of("value").unwrap_or("1").parse().expect("Failed Parsing Value");

            clignitas::vote( private_key, batcher_key, vote_id, value, &client);
        },
        "stress" => {
            let number : u32 = arguments.value_of("number").unwrap_or("10").parse().expect("upsi");
            let tmax = arguments.value_of("tmax").unwrap_or("3").parse().expect("upsi");
            let title = "demo_title".to_string();
            let info = "demo_info".to_string();
            let lat = 40.6405;
            let lng = 8.6538;
            let dir = 0.0;

            let start = Instant::now();
            let mut handle_vec = Vec::new();

            let mut thread_request_number = number/tmax;
            let remaining = number - (thread_request_number * tmax);

            let mut i = 0;
            for x in 0..tmax {

                thread_request_number = if x == tmax-1 { thread_request_number + remaining }else {thread_request_number };
                i = i + thread_request_number;

                handle_vec.push(
                    thread::spawn(move || {
                        let thread_client = reqwest::Client::new();
                        let client = reqwest::Client::new();
                        for n in 0..thread_request_number{
                            let private_key = clignitas::priv_key_from_file("./keys/key_1.file");
                            let batcher_key 
                                = clignitas::pub_key_from_hex("02381caa0892d913daa3c4856a4f9b665931964b3fc630ef9dd5edbd8a27952f7e");
                            clignitas::create_vote(
                                private_key,
                                batcher_key,
                                "demo_title".to_string(),
                                "demo_info_blaslka wkl knwa lknlkwn alwn dinalcneb adb aw".to_string(),
                                40.6405, 8.6538, 0.0, &thread_client);
                        }
                    }));
            }

            for child in handle_vec {
                // Wait for the thread to finish. Returns a result.
                let _ = child.join();
            }

            let duration = start.elapsed();
            println!("Sent {} : Time elapsed {:?}", i, duration);

        },
        _ => println!(" Action not recognised! Use -h to see Options")
    }


    println!("=END=");
}
