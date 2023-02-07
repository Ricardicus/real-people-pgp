use std::env;
use std::sync::Arc;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate rmp_serde as rmps;

use chrono;

use std::collections::HashMap;

mod keys;
use keys::{KeyMaster, KeyPair};

use clap::Parser;

use crypto::buffer::ReadBuffer;
use crypto::buffer::{BufferResult, RefReadBuffer, RefWriteBuffer, WriteBuffer};
use crypto::rc4::Rc4;
use crypto::symmetriccipher::{Decryptor, Encryptor};
use std::fs::File;
use std::io::{Read, Write};

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
    opts.optopt("i", "input", "set input keys file to be added to root cert", "NAME");
    opts.optopt("e", "era", "era the input file belongs to", "NAME");

    opts.optopt("r", "root", "root cert file to be modified", "NAME");
    opts.optflag("h", "help", "print this help menu");
    opts.optflag("a", "", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

}
