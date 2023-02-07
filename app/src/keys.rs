extern crate rand;
extern crate secp256k1;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

use secp256k1::bitcoin_hashes::sha256;
use secp256k1::rand::rngs::OsRng;
use secp256k1::{All, Message, PublicKey, Secp256k1, SecretKey, Signature};

use serde_derive::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use rmp_serde::Serializer;
use serde::Serialize;
use std::path::Path;
use std::str::FromStr;

use crypto::buffer::ReadBuffer;
use crypto::buffer::{BufferResult, RefReadBuffer, RefWriteBuffer, WriteBuffer};
use crypto::rc4::Rc4;
use crypto::symmetriccipher::{Decryptor, Encryptor};
use std::fs::File;
use std::io::{Read, Write};

pub struct KeyMaster {
    pub secp: Secp256k1<All>,
    pub public_key: String,
    pub secret_key: String,
    pub name: String,
    pub passphrase: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct KeyPair {
    pub public_key: String,
    pub secret_key: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct RootCert {
    pub public_key: String,
    pub era: u32,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct RootCerts {
    pub certs: Vec<RootCert>,
}

/* Keymaster holds the keys */
impl KeyMaster {
    pub fn new(name: Option<&str>, passphrase: Option<&str>) -> Self {
        let secp = Secp256k1::new();
        let mut rng = OsRng::new().expect("OsRng");
        let secret_key = SecretKey::new(&mut rng);
        let public_key = PublicKey::from_secret_key(&secp, &secret_key);
        Self {
            secp: secp,
            secret_key: secret_key.to_string(),
            public_key: public_key.to_string(),
            name: name.unwrap_or("Default name").to_string(),
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
    pub fn verify_with_public_key(
        &self,
        public_key: String,
        message: String,
        signature: String,
    ) -> bool {
        let message_ = Message::from_hashed_data::<sha256::Hash>(message.as_bytes());
        return self
            .secp
            .verify(
                &message_,
                &Signature::from_str(&signature[..]).unwrap(),
                &PublicKey::from_str(&public_key[..]).unwrap(),
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
        let mut f = File::create(file).expect(&format!("Failed to open file: {}", file)[..]);

        let v = vec![
            format!("{:width$}", &self.public_key.clone(), width = 66),
            format!("{:width$}", &self.secret_key.clone(), width = 64),
            format!("{}", &self.name.clone()),
        ];
        let export_content: String = v.concat();

        let mut out_bytes: &mut [u8] = &mut [0; 1024];
        let rc4_key: &[u8] = self.passphrase.as_bytes();
        let mut rc4_crypto: Rc4 = Rc4::new(&rc4_key);
        let mut incoming_buf: RefReadBuffer = RefReadBuffer::new(&export_content.as_bytes());
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
        return format!("Keys exported to file: {file}");
    }

    pub fn import_from_file(&mut self, file: &str, passphrase: &str) -> String {
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

        let contents = std::str::from_utf8(&final_result[..]).unwrap();
        let content_itr = || contents.chars().into_iter();
        assert!(contents.len() >= 66 + 64 + 1);

        let public_key: String = content_itr().take(66).collect();
        let secret_key: String = content_itr().skip(66).take(64).collect();
        self.name = content_itr().skip(66 + 64).collect();
        self.passphrase = passphrase.to_string();

        self.holding_these(&secret_key, &public_key);
        return format!("Wallet imported from file: {file}");
    }
}

impl RootCerts {
    pub fn from_file(file: &str) -> RootCerts {
        let mut filecheck = File::open(file).expect("failed to open file");
        let mut data: Vec<u8> = Vec::<u8>::new();
        filecheck
            .read_to_end(&mut data)
            .expect("Failed to read data");

        return rmp_serde::from_slice(&data).unwrap();
    }

    pub fn print(&self) {
        for cert in &self.certs {
            println!("{} ({})", cert.public_key, cert.era);
        }
    }

    pub fn add_pub_key(&mut self, pub_key: &str, era: u32) {
        let rc: RootCert = RootCert {
            public_key: pub_key.to_string(),
            era: era,
        };
        self.certs.push(rc);
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

/* sha256 */
pub fn hash_string(in_str: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(in_str);
    return format!("{:x}", hasher.finalize());
}
