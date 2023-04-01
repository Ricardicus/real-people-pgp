/// Generates a key, then signs and verifies a message.
use std::io::{self, Read, Write};

use crate::openpgp::cert::prelude::*;
use crate::openpgp::parse::{stream::*, Parse};
use crate::openpgp::policy::Policy;
use crate::openpgp::policy::StandardPolicy as P;
use crate::openpgp::serialize::stream::*;
use anyhow::*;
use sequoia_openpgp as openpgp;
use sequoia_openpgp::packet::key::*;
use std::env;
use std::env::args;
use std::fs::File;
use std::result::Result::Ok;

const MESSAGE: &str = "дружба";

fn main() -> openpgp::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut key = None;
    let mut option = None;

    if args.len() == 3 {
        key = Some(&args[1]);
        option = Some(&args[2]);
        println!("key: {}, option: {}", key.unwrap(), option.unwrap());
    } else {
        return Err(anyhow!("usage: {} key-ring signature", args[0]));
    }

    let key_raw = std::fs::read_to_string(key.unwrap()).expect("Unable to read file");
    let certs = CertParser::from_bytes(&key_raw).expect("Failed to read keys");
    let signature_raw = std::fs::read_to_string(option.unwrap()).expect("Unable to read file");

    let mut good = false;
    for cert in certs {
        let p = &P::new();
        let mut plaintext = Vec::new();
        match verify(p, &mut plaintext, &signature_raw.as_bytes(), &cert.unwrap()) {
            Ok(()) => {
                good = true;
            }
            Err(s) => {
                println!("error: {:?}", s);
            }
        }
    }

    if good {
        println!("Valid signature!");
    } else {
        println!("Invalid signature");
    }
    /*
        let ppr = PacketParser::from_bytes(&keyring)?;
        for certo in CertParser::from(ppr) {
            match certo {
                Ok(cert) => {
                    println!("Key: {}", cert.fingerprint());
                    for ua in cert.userids() {
                        println!("  User ID: {}", ua.userid());
                    }
                }
                Err(err) => {
                    eprintln!("Error reading keyring: {}", err);
                }
            }
        }
    */

    /*    let args: Vec<String> = env::args().collect();
        let mut key = None;
        let mut option = None;

        if args.len() == 3 {
            key = Some(&args[1]);
            option = Some(&args[2]);
            println!("key: {}, option: {}", key.unwrap(), option.unwrap());
        } else {
            return Err(anyhow!("usage: {} key option", args[0]));
        }

        let p = &P::new();
        // read from file key, store as a Vec<u8>
        let mut key_file = File::open(key.unwrap()).expect("Unable to open key file");
        let mut key_vec = Vec::new();
        key_file.read_to_end(&mut key_vec).expect("Unable to read key file");

        // Generate a key.
        let key: sequoia_openpgp::packet::prelude::Key4<PublicParts, PrimaryRole> = Key4::import_public_ed25519(&key_vec[..], None).unwrap();

        // print fingerprint
        println!("key fingerprint: {}", key.fingerprint());
    */
    /*
        // Sign the message.
        let mut signed_message = Vec::new();
        sign(p, &mut signed_message, MESSAGE, &key)?;

        // Verify the message.
        let mut plaintext = Vec::new();
        verify(p, &mut plaintext, &signed_message, &key)?;

        assert_eq!(MESSAGE.as_bytes(), &plaintext[..]);
    */
    Ok(())
}

/// Generates an signing-capable key.
fn generate() -> openpgp::Result<openpgp::Cert> {
    let (cert, _revocation) = CertBuilder::new()
        .add_userid("someone@example.org")
        .add_signing_subkey()
        .generate()?;

    // Save the revocation certificate somewhere.

    Ok(cert)
}

/// Signs the given message.
fn sign(
    p: &dyn Policy,
    sink: &mut (dyn Write + Send + Sync),
    plaintext: &str,
    tsk: &openpgp::Cert,
) -> openpgp::Result<()> {
    // Get the keypair to do the signing from the Cert.
    let keypair = tsk
        .keys()
        .unencrypted_secret()
        .with_policy(p, None)
        .supported()
        .alive()
        .revoked(false)
        .for_signing()
        .next()
        .unwrap()
        .key()
        .clone()
        .into_keypair()?;

    // Start streaming an OpenPGP message.
    let message = Message::new(sink);

    // We want to sign a literal data packet.
    let signer = Signer::new(message, keypair).build()?;

    // Emit a literal data packet.
    let mut literal_writer = LiteralWriter::new(signer).build()?;

    // Sign the data.
    literal_writer.write_all(plaintext.as_bytes())?;

    // Finalize the OpenPGP message to make sure that all data is
    // written.
    literal_writer.finalize()?;

    Ok(())
}

/// Verifies the given message.
fn verify(
    p: &dyn Policy,
    sink: &mut dyn Write,
    signed_message: &[u8],
    sender: &openpgp::Cert,
) -> openpgp::Result<()> {
    // Make a helper that that feeds the sender's public key to the
    // verifier.
    let helper = Helper { cert: sender };

    // Now, create a verifier with a helper using the given Certs.
    let mut verifier = VerifierBuilder::from_bytes(signed_message)?.with_policy(p, None, helper)?;

    // Verify the data.
    io::copy(&mut verifier, sink)?;

    Ok(())
}

struct Helper<'a> {
    cert: &'a openpgp::Cert,
}

impl<'a> VerificationHelper for Helper<'a> {
    fn get_certs(&mut self, _ids: &[openpgp::KeyHandle]) -> openpgp::Result<Vec<openpgp::Cert>> {
        // Return public keys for signature verification here.
        Ok(vec![self.cert.clone()])
    }

    fn check(&mut self, structure: MessageStructure) -> openpgp::Result<()> {
        // In this function, we implement our signature verification
        // policy.

        let mut good = false;
        for (i, layer) in structure.into_iter().enumerate() {
            match (i, layer) {
                // First, we are interested in signatures over the
                // data, i.e. level 0 signatures.
                (_, MessageLayer::SignatureGroup { results }) => {
                    // Finally, given a VerificationResult, which only says
                    // whether the signature checks out mathematically, we apply
                    // our policy.
                    match results.into_iter().next() {
                        Some(Ok(_)) => good = true,
                        Some(Err(e)) => return Err(openpgp::Error::from(e).into()),
                        None => return Err(anyhow::anyhow!("No signature")),
                    }
                }
                (_o, _b) => {
                    // pass
                }
            }
        }

        if good {
            Ok(()) // Good signature.
        } else {
            Err(anyhow::anyhow!("Signature verification failed"))
        }
    }
}
