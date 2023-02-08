use chrono;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate rmp_serde as rmps;
use clap::Parser;
use crypto::buffer::ReadBuffer;
use crypto::buffer::{BufferResult, RefReadBuffer, RefWriteBuffer, WriteBuffer};
use crypto::rc4::Rc4;
use crypto::symmetriccipher::Encryptor;
use rmps::Serializer;
use serde::Serialize;
use std::fs::File;
use std::io::Write;

mod keys;
use keys::{KeyMaster, KeyPair};

#[derive(Parser)]
struct Cli {
    /// Input master key pair 
    input: String,
    passphrase: String,
    /// The path to the file to store
    output: String,
}

fn main() {
    let args = Cli::parse();

    let key_pair: KeyPair = KeyPair::from_file(args.input, args.passphrase);
    print!("Enter passphrase for {keys_file}: ");
    std::io::stdout().flush().unwrap();
    let keys = KeyMaster::new(
        Some(format!("A pair of keys generated {} for certificate", chrono::offset::Local::now()).as_str()),
        Some(args.passphrase_out.as_str()),
    );

    let new_key_pair: KeyPair = KeyPair::
    key_pair.to_file(args.path.as_str(), args.passphrase.as_str());
    println!("{}", format!("Keys exported to file: {}", args.path));
}
