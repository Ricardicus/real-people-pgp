
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate rmp_serde as rmps;
extern crate rpassword;
use clap::Parser;
use rpassword::read_password;

use std::io::Write;
mod keys;
use keys::{KeyMaster, KeyPair};

#[derive(Parser)]
struct Cli {
    /// The path to the file to store
    path: String,
}

fn main() {
    let args = Cli::parse();
    print!("Enter passphrase for the new key: ");
    std::io::stdout().flush().unwrap();
    let passphrase = read_password().unwrap();

    let keys = KeyMaster::new(Some(passphrase.as_str()));
    let key_pair: KeyPair = KeyPair {
        public_key: keys.public_key.to_string(),
        secret_key: keys.secret_key.to_string(),
    };

    key_pair.to_file(args.path.as_str(), passphrase.as_str());
    println!("{}", format!("Keys exported to file: {}", args.path));
}
