use std::env;
use std::sync::Arc;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate rmp_serde as rmps;

use chrono;
extern crate rpassword;
use rpassword::read_password;
use std::collections::HashMap;
mod keys;
use clap::Parser;
use crypto::buffer::ReadBuffer;
use crypto::buffer::{BufferResult, RefReadBuffer, RefWriteBuffer, WriteBuffer};
use crypto::rc4::Rc4;
use crypto::symmetriccipher::{Decryptor, Encryptor};
use keys::{KeyMaster, KeyPair, RootCert, RootCerts};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

extern crate getopts;
use getopts::Options;

#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    passphrase: String,
    /// The path to the file to store
    path: String,
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();

    let root_cert_name = RootCerts::get_filename(); 

    opts.optopt(
        "i",
        "input",
        "set input keys file to be added to root cert",
        "name",
    );
    opts.optopt("e", "era", "era the input file belongs to", "id");

    opts.optflag("p", "print", "print public keys in root certificate");
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            panic!("{}", f.to_string())
        }
    };

    let rs: bool = Path::new(&root_cert_name).exists();

    if matches.opt_present("p") {
        if rs {
            let rc: RootCerts = RootCerts::from_file(&root_cert_name);
            rc.print();
            return;
        } else {
            println!("No root cert file has been created");
            print_usage(&args[0], opts);
            return;
        }
    }

    if matches.opt_present("h") {
        print_usage(&args[0], opts);
        return;
    }

    if !matches.opt_present("i") && !matches.opt_present("e") {
        print_usage(&args[0], opts);
        return;
    }
    let era: u32 = matches.opt_str("e").unwrap().parse::<u32>().unwrap();
    let keys_file: String = matches.opt_str("i").unwrap();

    print!("Enter passphrase for {keys_file}: ");
    std::io::stdout().flush().unwrap();
    let passphrase = read_password().unwrap();

    let key_pair: KeyPair = KeyPair::from_file(keys_file.as_str(), passphrase.as_str());

    let mut rc: RootCerts = RootCerts {
        certs: Vec::<RootCert>::new(),
    };

    if rs == true {
        rc = RootCerts::from_file(&root_cert_name);
    }

    rc.add_pub_key(key_pair.public_key.as_str(), era);
    rc.to_file(&root_cert_name);

    println!("Root cert: {root_cert_name}");
}
