extern crate clap;
extern crate hex;
extern crate secp256k1;
use bitcoin::network::constants::Network;
use bitcoin::secp256k1::Secp256k1;
use bitcoin::util::address::Address;
use bitcoin::util::ecdsa::{PrivateKey, PublicKey};
use clap::{App, Arg};
use std::process;
use std::str::FromStr;
use std::thread;
use std::time::SystemTime;

fn scan_designated_space(mut init: u64, end_limit: u64, input_address: String) {
    let secp256k1 = Secp256k1::new();
    let mut key_bytes: [u8; 32] = [0; 32];
    let mut now = SystemTime::now();
    let network: Network = Network::Bitcoin;
    while init < end_limit {
        let interger_key: u64 = init;
        init += 1;
        if init % 1000000 == 0 {
            let current = SystemTime::now();
            let difference = current
                .duration_since(now)
                .expect("Clock may have gone backwards");
            println!("time taken in one million keys {:?}", difference);
            now = current;
        }
        let bytes: [u8; 8] = interger_key.to_ne_bytes();
        {
            let mut i: usize = 0;
            while i < 8 {
                key_bytes[i + 24] = bytes[i];
                i = i + 1;
            }
        }
        let private_key_result = PrivateKey::from_slice(&key_bytes, network);
        let private_key = match private_key_result {
            Ok(private_key) => private_key,
            Err(error) => panic!("{:?}", error),
        };
        let public_key: PublicKey = private_key.public_key(&secp256k1);
        let address: Address = Address::p2pkh(&public_key, Network::Bitcoin);
        if address.to_string() == input_address {
            println!("{:?} key is {:?}", input_address, interger_key);
            process::exit(1);
        }
    }
}

fn main() {
    let matches = App::new("Find Private Key Program")
        .version("1.0")
        .author("Gaurav Agarwal <agagaurav96@gmail.com>")
        .about("------------")
        .arg(
            Arg::with_name("address")
                .long("address")
                .help("Wallet address")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("start")
                .long("start")
                .help("Initial point for Search(int in between 0-63)")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("end")
                .long("end")
                .help("End point for Search(int in between 0-63)")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("threads")
                .long("threads")
                .help("Threads count")
                .takes_value(true),
        )
        .get_matches();
    let address: String = matches
        .value_of("address")
        .unwrap_or("1Q5nyJU1xj7vz4oNRKrvRxDgeGuKJYmbBB")
        .to_string();

    match Address::from_str(&address) {
        Result::Ok(_) => println!("Input Address is a valid Bitcoin Address"),
        Result::Err(_) => panic!("Input Address is not a valid Bitcoin Address"),
    }
    let start: u32 = match matches.value_of("start").unwrap_or("42").parse::<u32>() {
        Result::Ok(num) => num,
        Result::Err(_) => panic!("Please provide correct inputs"),
    };
    let end: u32 = match matches.value_of("end").unwrap_or("43").parse::<u32>() {
        Result::Ok(num) => num,
        Result::Err(_) => panic!("Please provide correct inputs"),
    };
    let threads: u64 = match matches.value_of("threads").unwrap_or("4").parse::<u64>() {
        Result::Ok(num) => num,
        Result::Err(_) => panic!("Please provide correct inputs"),
    };
    if start >= end {
        panic!("End input should be greater than start input");
    }

    if 1 > threads || threads > 20 {
        panic!("Threads count should in 1..=20");
    }
    let base: u64 = 2;
    let init: u64 = base.pow(start);
    let end_limit: u64 = base.pow(end);

    println!(
        "Searching private key for address {:?} in given range {:?} - {:?}",
        address, init, end_limit
    );
    let mut handles = Vec::with_capacity(10);
    for i in 1..=threads {
        let diff: u64 = end_limit - init;
        let start: u64 = init + (i - 1) * (diff / threads);
        let end: u64 = init + i * (diff / threads);
        println!("Hello!, {} thread spawned! {:?} {:?}", i, start, end);
        let cloned_address: String = address.clone();
        handles.push(thread::spawn(move || {
            scan_designated_space(start, end, cloned_address);
        }));
    }
    for handle in handles {
        handle.join().unwrap();
    }
}
