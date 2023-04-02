/// Generates a key, then signs and verifies a message.
use std::io::{self, Write};

use crate::openpgp::cert::prelude::*;
use crate::openpgp::parse::{stream::*, Parse};
use crate::openpgp::policy::Policy;
use crate::openpgp::policy::StandardPolicy as P;
use crate::openpgp::serialize::stream::*;
use anyhow::*;
use sequoia_openpgp as openpgp;

use std::env;

use std::result::Result::Ok;

// importing generated gRPC code
use poh_grpc::*;
// importing types for messages
use poh::*;
mod poh;
mod poh_grpc;
use grpc::{ServerHandlerContext, ServerRequestSingle, ServerResponseUnarySink};

use std::cell::RefCell;
use std::sync::{Arc, Mutex};

const MESSAGE: &str = "дружба";

struct MyPoH {
    keyring: Arc<Mutex<Vec<Cert>>>,
}

impl MyPoH {
    fn process_attached_signature(&self, signature: &str) -> Result<(), ()> {
        let mut good = false;
        let binding = self.keyring.lock().unwrap();
        for cert in binding.iter() {
            let p = &P::new();
            let mut plaintext = Vec::new();
            match verify(p, &mut plaintext, &signature.as_bytes(), &cert) {
                Ok(()) => {
                    good = true;
                }
                Err(_s) => {}
            }
        }
        if good {
            Ok(())
        } else {
            Err(())
        }
    }
}

impl PoH for MyPoH {
    fn verify_attached_signature(
        &self,
        _: ServerHandlerContext,
        req: ServerRequestSingle<VerifyAttachedSignatureRequest>,
        resp: ServerResponseUnarySink<VerifyResponse>,
    ) -> grpc::Result<()> {
        let mut r = VerifyResponse::new();
        if self
            .process_attached_signature(&req.message.file_attached_signature)
            .is_ok()
        {
            r.set_valid(true);
            r.set_info("Valid signature".to_string());
        } else {
            r.set_valid(false);
            r.set_info("Invalid signature".to_string());
        }
        resp.finish(r)
    }
    fn verify_detached_signature(
        &self,
        _: ServerHandlerContext,
        _req: ServerRequestSingle<VerifyDetachedSignatureRequest>,
        resp: ServerResponseUnarySink<VerifyResponse>,
    ) -> grpc::Result<()> {
        let mut r = VerifyResponse::new();
        r.set_info("Not implemented yet".to_string());
        r.set_valid(false);
        resp.finish(r)
    }
}

fn main() -> openpgp::Result<()> {
    let mut port = 50051;

    let args: Vec<String> = env::args().collect();
    let mut key = None;

    if args.len() == 3 {
        key = Some(&args[1]);
        port = args[2].parse::<u16>().expect("invalid port number");
        println!("key: {}, port: {}", key.unwrap(), port);
    } else {
        return Err(anyhow!("usage: {} key-ring port", args[0]));
    }

    let key_raw = std::fs::read_to_string(key.unwrap()).expect("Unable to read file");
    let key_static: &'static str = Box::leak(key_raw.into_boxed_str());
    let mut cert_parse = CertParser::from_bytes(key_static).expect("Failed to read keys");
    let mut certs = Vec::<Cert>::new();
    while let Some(packet) = cert_parse.next() {
        match packet {
            Ok(cert) => {
                certs.push(cert);
            }
            Err(_err) => {}
        }
    }
    let mut server = grpc::ServerBuilder::new_plain();

    server.http.set_port(port);

    server.add_service(PoHServer::new_service_def(MyPoH {
        keyring: Arc::new(Mutex::new(certs)),
    }));
    // running the server
    let _server = server.build().expect("server");
    println!("proof of human server started on port {}", port);
    // stopping the program from finishing
    loop {
        std::thread::park();
    }
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
