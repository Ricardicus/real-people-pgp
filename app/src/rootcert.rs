use std::env;
use std::sync::Arc;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate rmp_serde as rmps;

extern crate rpassword;
use rpassword::read_password;
mod keys;
use clap::Parser;
use keys::{KeyPair, RootCert, RootCerts};
use std::io::Write;
use std::path::Path;

extern crate getopts;
use getopts::Options;

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
    opts.optopt("is", "issuer", "issuer id", "id");
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

    if !matches.opt_present("i") && !matches.opt_present("is") {
        print_usage(&args[0], opts);
        return;
    }
    let issuer: String = matches.opt_str("e").unwrap().parse::<String>().unwrap();
    let keys_file: String = matches.opt_str("i").unwrap();

    print!("Enter passphrase for {keys_file}: ");
    std::io::stdout().flush().unwrap();
    let passphrase = read_password().unwrap();

    let key_pair: KeyPair = KeyPair::from_file(keys_file.as_str(), passphrase.as_str());

    let mut rcs: RootCerts = RootCerts {
        certs: Vec::<RootCert>::new(),
    };

    if rs == true {
        rcs = RootCerts::from_file(&root_cert_name);
    }

    let rc: RootCert = RootCert {
        public_key: key_pair.public_key.clone(),
        time: chrono::Local::now().to_rfc3339(),
        issuer: issuer.clone(),
    };
    rcs.add_rootcert(&rc);
    rcs.to_file(&root_cert_name);

    println!("Root cert: {root_cert_name}");
}
