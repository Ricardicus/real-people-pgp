extern crate rand;
extern crate secp256k1;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

use secp256k1::bitcoin_hashes::sha256;
use secp256k1::rand::rngs::OsRng;
use secp256k1::{All, Message, PublicKey, Secp256k1, SecretKey, Signature};

use serde_derive::Deserialize;
use sha2::{Digest, Sha256};

use rmp_serde::Serializer;
use serde::Serialize;
use std::path::Path;
use std::str::{from_utf8, FromStr};

use crypto::buffer::ReadBuffer;
use crypto::buffer::{BufferResult, RefReadBuffer, RefWriteBuffer, WriteBuffer};
use crypto::rc4::Rc4;
use crypto::symmetriccipher::{Decryptor, Encryptor};
use ecies::{decrypt, encrypt};
use std::fs::{create_dir, create_dir_all, File};
use std::io::{Read, Write};

use chrono::offset::Local;
use std::collections::HashMap;

pub struct KeyMaster {
    pub secp: Secp256k1<All>,
    pub public_key: String,
    pub secret_key: String,
    pub passphrase: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct KeyPair {
    pub public_key: String,
    pub secret_key: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Cert {
    pub public_key: String,
    pub signature: String,
    pub issuer: String,
    pub time: String,
    pub ca_public_key: String,
}

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct RootCert {
    pub public_key: String,
    pub time: String,
    pub issuer: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct RootCerts {
    pub certs: Vec<RootCert>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct DatabaseEntry {
    pub time: String,
    pub issuer: String,
    pub public_key: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Database {
    pub entries: HashMap<String, DatabaseEntry>,
}

/* Keymaster holds the keys */
impl KeyMaster {
    pub fn new(passphrase: Option<&str>) -> Self {
        let secp = Secp256k1::new();
        let mut rng = OsRng::new().expect("OsRng");
        let secret_key = SecretKey::new(&mut rng);
        let public_key = PublicKey::from_secret_key(&secp, &secret_key);
        Self {
            secp: secp,
            secret_key: secret_key.to_string(),
            public_key: public_key.to_string(),
            passphrase: passphrase.unwrap_or("Default passphrase").to_string(),
        }
    }

    /* To start it from already generated values */
    pub fn holding_these(&mut self, secret_key: &str, public_key: &str) {
        let secp = Secp256k1::new();
        let secret_key = SecretKey::from_str(secret_key).expect("Invalid secret key");
        let public_key = PublicKey::from_str(public_key).expect("Invalid public key");
        self.secp = secp;
        self.secret_key = secret_key.to_string();
        self.public_key = public_key.to_string();
    }

    /* Sign a message */
    pub fn sign(&self, message: String) -> String {
        let message_ = Message::from_hashed_data::<sha256::Hash>(message.as_bytes());
        return self
            .secp
            .sign(
                &message_,
                &SecretKey::from_str(&self.secret_key[..]).unwrap(),
            )
            .to_string();
    }

    /* Verify a message */
    pub fn verify(&self, message: String, signature: String) -> bool {
        let message_ = Message::from_hashed_data::<sha256::Hash>(message.as_bytes());
        return self
            .secp
            .verify(
                &message_,
                &Signature::from_str(&signature[..]).unwrap(),
                &PublicKey::from_str(&self.public_key[..]).unwrap(),
            )
            .is_ok();
    }

    /* Verify a message using another public key */
    pub fn verify_with_public_key(&self, public_key: &str, message: &str, signature: &str) -> bool {
        let message_ = Message::from_hashed_data::<sha256::Hash>(message.as_bytes());
        return self
            .secp
            .verify(
                &message_,
                &Signature::from_str(signature).unwrap(),
                &PublicKey::from_str(public_key).unwrap(),
            )
            .is_ok();
    }

    /* Generate a certificate, a bundle of a signature and a certificate */
    pub fn new_certificate(&self) -> (String, String) {
        let message: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(100)
            .map(char::from)
            .collect();
        (message.to_owned(), self.sign(message))
    }

    /* Store keys to file */
    pub fn export_to_file(&self, file: &str) -> String {
        let key_pair: KeyPair = KeyPair {
            secret_key: self.secret_key.to_string(),
            public_key: self.public_key.to_string(),
        };

        key_pair.to_file(file, &self.passphrase);
        return format!("Keys exported to file: {file}");
    }

    pub fn import_from_file(file: &str, passphrase: &str) -> KeyMaster {
        let key_pair: KeyPair = KeyPair::from_file(file, passphrase);

        let mut km: KeyMaster = KeyMaster::new(None);
        km.passphrase = passphrase.to_string();

        km.holding_these(&key_pair.secret_key, &key_pair.public_key);
        return km;
    }
}

impl RootCerts {
    pub fn from_file(file: &str) -> RootCerts {
        let mut filecheck = File::open(file).expect(&format!("Failed to open file: {}", file));
        let mut data: Vec<u8> = Vec::<u8>::new();
        filecheck
            .read_to_end(&mut data)
            .expect("Failed to read data");

        return rmp_serde::from_slice(&data).unwrap();
    }

    pub fn print(&self) {
        for cert in &self.certs {
            println!(
                "public_key:{}\nissuer: {}\ntime: {}",
                cert.public_key, cert.issuer, cert.time
            );
        }
    }

    pub fn add_rootcert(&mut self, root_cert: &RootCert) {
        self.certs.push(root_cert.clone());
    }

    pub fn get_filename() -> String {
        return "poh_rootcert.pohrc".to_string();
    }

    pub fn read_rootcert() -> RootCerts {
        let root_cert_name = RootCerts::get_filename();
        let rs: bool = Path::new(&root_cert_name).exists();
        // Check that file 'name' exists
        if !rs {
            panic!("There is not rootcert file");
        }
        return RootCerts::from_file(&root_cert_name);
    }

    pub fn to_file(&self, file: &str) {
        let mut buf = Vec::new();

        self.serialize(&mut Serializer::new(&mut buf)).unwrap();

        let mut f = File::create(file).expect(&format!("Failed to open file: {}", file)[..]);

        /* Writing to file */
        f.write(&buf[..]).expect("Failed to write bytes");
    }
}

impl KeyPair {
    pub fn to_file(&self, file: &str, passphrase: &str) {
        let mut buf = Vec::new();

        self.serialize(&mut Serializer::new(&mut buf)).unwrap();

        let mut f = File::create(file).expect(&format!("Failed to open file: {}", file)[..]);

        let mut out_bytes: &mut [u8] = &mut [0; 1024];
        let rc4_key: &[u8] = passphrase.as_bytes();
        let mut rc4_crypto: Rc4 = Rc4::new(&rc4_key);
        let mut incoming_buf: RefReadBuffer = RefReadBuffer::new(&buf);
        let mut out_buf: RefWriteBuffer = RefWriteBuffer::new(&mut out_bytes);

        let mut final_result = Vec::<u8>::new();
        loop {
            let result = rc4_crypto
                .encrypt(&mut incoming_buf, &mut out_buf, true)
                .unwrap();
            final_result.extend(
                out_buf
                    .take_read_buffer()
                    .take_remaining()
                    .iter()
                    .map(|&i| i),
            );
            match result {
                BufferResult::BufferUnderflow => break,
                BufferResult::BufferOverflow => {}
            }
        }

        /* Writing to file */
        f.write(&final_result[..]).expect("Failed to write bytes");
    }

    pub fn from_file(file: &str, passphrase: &str) -> KeyPair {
        let mut out_bytes: &mut [u8] = &mut [0; 1024];
        let rc4_key: &[u8] = passphrase.as_bytes();
        let mut rc4_crypto: Rc4 = Rc4::new(&rc4_key);
        let mut out_buf: RefWriteBuffer = RefWriteBuffer::new(&mut out_bytes);
        let mut filecheck = File::open(file).expect("failed to open file");
        let mut data: Vec<u8> = Vec::<u8>::new();
        filecheck
            .read_to_end(&mut data)
            .expect("Failed to read data");

        let mut final_result = Vec::<u8>::new();
        let mut incoming_buf: RefReadBuffer = RefReadBuffer::new(&data[..]);

        loop {
            let result = rc4_crypto
                .decrypt(&mut incoming_buf, &mut out_buf, true)
                .unwrap();
            final_result.extend(
                out_buf
                    .take_read_buffer()
                    .take_remaining()
                    .iter()
                    .map(|&i| i),
            );
            match result {
                BufferResult::BufferUnderflow => break,
                BufferResult::BufferOverflow => {}
            }
        }

        return rmp_serde::from_slice(&final_result).unwrap();
    }
}

impl Cert {
    pub fn generate(keys: KeyMaster, issuer: &str, passphrase: &str, out_dir: &str) -> Self {
        let cert_keys = KeyMaster::new(Some(passphrase));
        let signature: String = keys.sign(cert_keys.public_key.to_string());

        create_dir_all(out_dir).expect("Failed to create directory {out_dir}");

        let keys_filename = "keys.poh";
        let keys_path = format!("{}/{}", out_dir, keys_filename);

        println!("keys_path: {}", keys_path);
        cert_keys.export_to_file(&keys_path);

        /* Current date and time in Rust to string */
        let new_cert = Cert {
            public_key: cert_keys.public_key,
            signature: signature,
            ca_public_key: keys.public_key,
            time: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            issuer: issuer.to_string(),
        };

        new_cert.store(out_dir);
        return new_cert;
    }

    pub fn store_database_entry(&self, out_dir: &str) {
        let entry: DatabaseEntry = DatabaseEntry {
            time: chrono::Local::now().to_rfc3339(),
            issuer: self.issuer.clone(),
            public_key: self.public_key.clone(),
        };
        entry.store(out_dir);
    }

    pub fn store(&self, out_dir: &str) {
        let mut buf = Vec::new();
        let filename = "cert.pohcert";
        let filepath = format!("{}/{}", out_dir, filename);

        self.serialize(&mut Serializer::new(&mut buf)).unwrap();
        let mut f = File::create(filepath.to_string())
            .expect(&format!("Failed to create file: {}", filepath)[..]);

        /* Writing to file */
        f.write(&buf[..]).expect("Failed to write bytes");
        println!("Stored new certificate file {filepath}");
    }

    pub fn from_file(file: &str) -> Cert {
        let mut filecheck = File::open(file).expect(&format!("Failed to open file: {}", file));
        let mut data: Vec<u8> = Vec::<u8>::new();
        filecheck
            .read_to_end(&mut data)
            .expect("Failed to read data");

        return rmp_serde::from_slice(&data).unwrap();
    }
}

impl DatabaseEntry {
    pub fn store(&self, out_dir: &str) {
        let mut buf = Vec::new();
        let filename = "entry.pohdbe";
        let filepath = format!("{}/{}", out_dir, filename);

        self.serialize(&mut Serializer::new(&mut buf)).unwrap();
        let mut f = File::create(filepath.to_string())
            .expect(&format!("Failed to create file: {}", filepath)[..]);

        /* Writing to file */
        f.write(&buf[..]).expect("Failed to write bytes");
        println!("Stored new database entry file {filepath}");
    }
    pub fn from_file(file: &str) -> DatabaseEntry {
        let mut filecheck = File::open(file).expect(&format!("Failed to open file: {}", file));
        let mut data: Vec<u8> = Vec::<u8>::new();
        filecheck
            .read_to_end(&mut data)
            .expect("Failed to read file {file}");

        return rmp_serde::from_slice(&data).unwrap();
    }
    pub fn print(&self) {
        println!("[issed by {} at {}]", self.issuer, self.time);
    }
}

impl Database {
    pub fn get_std_db() -> &'static str {
        return "databases/db.pohd";
    }
    pub fn store(&self, out_dir: &str) {
        let mut buf = Vec::new();
        let filename = "db.pohd";

        if !Path::new(out_dir).exists() {
            create_dir(out_dir).expect("Unable to create directory {out_dir}");
        }

        let filepath = format!("{}/{}", out_dir, filename);

        self.serialize(&mut Serializer::new(&mut buf)).unwrap();
        let mut f = File::create(filepath.to_string())
            .expect(&format!("Failed to create file: {}", filepath)[..]);

        /* Writing to file */
        f.write(&buf[..]).expect("Failed to write bytes");
        println!("Stored new database file {filepath}");
    }
    pub fn from_file(file: &str) -> Database {
        let mut filecheck = File::open(file).expect(&format!("Failed to open file: {}", file));
        let mut data: Vec<u8> = Vec::<u8>::new();
        filecheck
            .read_to_end(&mut data)
            .expect("Failed to open file {file}");

        return rmp_serde::from_slice(&data).unwrap();
    }
    pub fn print(&self) {
        for (key, val) in &self.entries {
            print!("{}: ", key);
            val.print();
        }
    }
}

/* Quick and dirty secp encrypt/decrypt */
pub fn secp256k1_encrypt(pk: &str, msg: &str) -> String {
    let pk_vec: &[u8] = &hex::decode(pk).unwrap();
    let msg_vec: &[u8] = &msg.as_bytes();
    let enc_vec: &[u8] = &encrypt(pk_vec, msg_vec).unwrap();
    return hex::encode(enc_vec);
}
pub fn secp256k1_decrypt(sk: &str, msg: &str) -> String {
    let sk_vec: &[u8] = &hex::decode(sk).unwrap();
    let msg_vec: &[u8] = &hex::decode(msg).unwrap();
    let enc_vec: &[u8] = &decrypt(sk_vec, msg_vec).unwrap();
    return from_utf8(enc_vec).unwrap().to_string();
}

/* sha256 */
pub fn hash_string(in_str: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(in_str);
    return format!("{:x}", hasher.finalize());
}
