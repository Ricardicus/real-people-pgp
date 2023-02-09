extern crate rmp_serde as rmps;
extern crate serde;
extern crate serde_derive;

mod keys;
use keys::KeyPair;
extern crate rpassword;
use clap::Parser;
use rpassword::read_password;
use std::io::Write;
#[derive(Parser)]
struct Cli {
    /// The path to the file to store
    path: String,
}

fn main() {
    let args = Cli::parse();
    print!("Enter passphrase: ");
    std::io::stdout().flush().unwrap();
    let passphrase = read_password().unwrap();
    let keypair: KeyPair = KeyPair::from_file(args.path.as_str(), passphrase.as_str());
    println!(
        "{}",
        format!(
            "Inspect file:\npublic key: {}\nsecret key (not so by now): {}\n",
            keypair.public_key, keypair.secret_key
        )
    );
}
