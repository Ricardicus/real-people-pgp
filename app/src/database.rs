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
use keys::{Database, DatabaseEntry, KeyPair};
use std::io::Write;
use std::path::Path;

extern crate getopts;
use getopts::Options;
use std::collections::HashMap;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();

    opts.optopt("d", "database", "database file to open", "file");
    opts.optopt("e", "entry", "database entry", "file");
    opts.optopt(
        "c",
        "command",
        "what to do:\n\
   print - print database entries\n\
   append - append entry to database\n\
   create - create a new database file",
        "command",
    );
    opts.optflag("h", "help", "print this help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            panic!("{}", f.to_string())
        }
    };

    if matches.opt_present("h") {
        print_usage(&args[0], opts);
        return;
    }

    if !matches.opt_present("c") {
        print_usage(&args[0], opts);
        return;
    }

    let cmd: String = matches.opt_str("c").unwrap().parse::<String>().unwrap();
    if cmd == "print" {
        let db_path: String = matches.opt_str("d").unwrap().parse::<String>().unwrap();
        let db: Database = Database::from_file(&db_path);
        db.print();
        return;
    }
    if cmd == "create" {
        let db: Database = Database {
            entries: HashMap::new(),
        };
        db.store("databases");
        return;
    }
    if cmd == "append" {
        let db_path: String = matches.opt_str("d").unwrap().parse::<String>().unwrap();
        let mut db: Database = Database::from_file(&db_path);
        if !matches.opt_present("e") {
            print_usage(&args[0], opts);
            return;
        }
        let entry: String = matches.opt_str("e").unwrap().parse::<String>().unwrap();
        let dbe: DatabaseEntry = DatabaseEntry::from_file(&entry);
        db.entries.insert(dbe.public_key.clone(), dbe);

        db.store("databases");
        println!("Added entry {entry} to database file {db_path}");
        return;
    }

    print_usage(&args[0], opts);
}
