extern crate rand;
extern crate secp256k1;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

use pyo3::prelude::*;
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
use std::fs::{create_dir_all, File};
use std::io::{Read, Write};

#[pyclass]
#[pyo3(text_signature = "(/)")]
pub struct KeyMaster {
    pub secp: Secp256k1<All>,
    #[pyo3(get)]
    pub public_key: String,
    #[pyo3(get)]
    pub secret_key: String,
    #[pyo3(get)]
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
#[pymethods]
impl KeyMaster {
    #[new]
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
    #[pyo3(text_signature = "($self, secret_key, public_key /)")]
    pub fn holding_these(&mut self, secret_key: &str, public_key: &str) {
        let secp = Secp256k1::new();
        let secret_key = SecretKey::from_str(secret_key).expect("Invalid secret key");
        let public_key = PublicKey::from_str(public_key).expect("Invalid public key");
        self.secp = secp;
        self.secret_key = secret_key.to_string();
        self.public_key = public_key.to_string();
    }

    /* Sign a message */
     #[pyo3(text_signature = "($self, message /)")]
    pub fn sign(&self, message: String) -> PyResult<String> {
        let message_ = Message::from_hashed_data::<sha256::Hash>(message.as_bytes());
        return Ok(self
            .secp
            .sign(
                &message_,
                &SecretKey::from_str(&self.secret_key[..]).unwrap(),
            )
            .to_string());
    }

    /* Verify a message */
    #[pyo3(text_signature = "($self, message, signature /)")]
    pub fn verify(&self, message: String, signature: String) -> PyResult<bool> {
        let message_ = Message::from_hashed_data::<sha256::Hash>(message.as_bytes());
        return Ok(self
            .secp
            .verify(
                &message_,
                &Signature::from_str(&signature[..]).unwrap(),
                &PublicKey::from_str(&self.public_key[..]).unwrap(),
            )
            .is_ok());
    }

    /* Verify a message using another public key */
    #[pyo3(text_signature = "($self, public_key, message, signature /)")]
    pub fn verify_with_public_key(
        &self,
        public_key: String,
        message: String,
        signature: String,
    ) -> PyResult<bool> {
        let message_ = Message::from_hashed_data::<sha256::Hash>(message.as_bytes());
        return Ok(self
            .secp
            .verify(
                &message_,
                &Signature::from_str(&signature[..]).unwrap(),
                &PublicKey::from_str(&public_key[..]).unwrap(),
            )
            .is_ok());
    }

    /* Generate a certificate, a bundle of a signature and a certificate */
    #[pyo3(text_signature = "($self /)")]
    pub fn new_certificate(&self) -> PyResult<(String, String)> {
        let message: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(100)
            .map(char::from)
            .collect();
        Ok((message.to_owned(), self.sign(message).unwrap()))
    }

    /* Store keys to file */
    #[pyo3(text_signature = "($self, file /)")]
    pub fn export_to_file(&self, file: &str) -> PyResult<String> {
        let key_pair: KeyPair = KeyPair {
            secret_key: self.secret_key.to_string(),
            public_key: self.public_key.to_string(),
        };

        key_pair.to_file(file, &self.passphrase);
        return Ok(format!("Keys exported to file: {file}"));
    }

    #[pyo3(text_signature = "($self, file, passphrase /)")]
    pub fn import_from_file(&mut self, file: &str, passphrase: &str) -> PyResult<String> {
        let key_pair: KeyPair = KeyPair::from_file(file, passphrase);

        self.passphrase = passphrase.to_string();

        self.holding_these(&key_pair.secret_key, &key_pair.public_key);
        return Ok(format!("Keys imported from file: {file}"));
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

    pub fn get_filename() -> String {
        return "poh_rootcert.pohrc".to_string();
    }

    pub fn read_rootcert(&self) -> RootCerts {
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
    pub fn new(keys: KeyMaster, passphrase: &str, out_dir: &str) -> Self {
        let cert_keys = KeyMaster::new(Some(passphrase));

        create_dir_all(out_dir).expect("Failed to create directory {out_dir}");

        let keys_filename = "keys.poh";
        let keys_path = format!("{}/{}", out_dir, keys_filename);

        cert_keys.export_to_file(&keys_path).unwrap();

        let signature: String = keys.sign(cert_keys.public_key.to_string()).unwrap();

        let new_cert = Cert {
            public_key: cert_keys.public_key,
            signature: signature,
        };

        new_cert.store(out_dir);
        return new_cert;
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
}

/* sha256 */
#[pyfunction]
pub fn hash_string(in_str: &str) -> PyResult<String> {
    let mut hasher = Sha256::new();
    hasher.update(in_str);
    return Ok(format!("{:x}", hasher.finalize()));
}




/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn pohlib(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<KeyMaster>()?;
    m.add_function(wrap_pyfunction!(hash_string, m)?).unwrap();
    Ok(())
}


