extern crate rmp_serde as rmps;
extern crate serde;
extern crate serde_derive;

mod keys;
use keys::KeyPair;

use clap::Parser;

#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    passphrase: String,
    /// The path to the file to store
    path: String,
}

fn main() {
    let args = Cli::parse();

    let keypair: KeyPair = KeyPair::from_file(args.path.as_str(), args.passphrase.as_str());
    println!(
        "{}",
        format!(
            "Inspect file:\npublic key: {}\nsecret key (not so by now): {}\n",
            keypair.public_key, keypair.secret_key
        )
    );
}
