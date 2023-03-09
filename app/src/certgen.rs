extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate rmp_serde as rmps;
use clap::Parser;
extern crate rpassword;
use rpassword::read_password;
mod keys;
use keys::{Cert, KeyMaster};
use std::io::{self, BufRead, Write};

#[derive(Parser)]
struct Cli {
    /// Input master key pair
    ca_keys: String,
    /// Issuer
    issuer: String,
}

fn main() {
    let args = Cli::parse();

    print!("Enter CA passphrase: ");
    std::io::stdout().flush().unwrap();
    let ca_passphrase = read_password().unwrap();

    print!("Enter name for the new human client: ");
    std::io::stdout().flush().unwrap();
    let mut name = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut name).unwrap();

    name = name.replace(" ", "_");
    name = name.replace("\n", "");
    println!("name: '{}'", name);
    print!("Enter passphrase for the new human client: ");
    std::io::stdout().flush().unwrap();
    let passphrase = read_password().unwrap();
    let keys = KeyMaster::import_from_file(&args.ca_keys, &ca_passphrase);

    let cert: Cert = Cert::generate(keys, &args.issuer, &passphrase, &name);
    cert.store_database_entry(&name);
}
