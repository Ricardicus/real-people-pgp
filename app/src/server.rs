mod keys;
use keys::{hash_string, KeyMaster, RootCerts};

use grpc::{ServerHandlerContext, ServerRequestSingle, ServerResponseUnarySink};
// importing generated gRPC code
use poh_grpc::*;
// importing types for messages
use poh::*;
mod poh;
mod poh_grpc;

struct MyPoH {
    rootcerts: RootCerts,
    keymaster: KeyMaster,
}

impl PoH for MyPoH {
    // rpc for service
    fn check(
        &self,
        _: ServerHandlerContext,
        req: ServerRequestSingle<CheckRequest>,
        resp: ServerResponseUnarySink<CheckResponse>,
    ) -> grpc::Result<()> {
        // create Response
        let mut r = CheckResponse::new();
        let msg = req.message.get_msg();
        let msg_signature = req.message.get_msg_sig();
        let cert = req.message.get_cert();
        let pub_key = req.message.get_pub_key();
        let mut valid: bool = false;
        let mut cert_era: u64 = 0;

        for rootcert in &self.rootcerts.certs {
            if self
                .keymaster
                .verify_with_public_key(&rootcert.public_key, &pub_key, cert)
            {
                valid = true;
                cert_era = rootcert.era;
                break;
            }
        }

        if valid {
            valid =
                self.keymaster
                    .verify_with_public_key(&pub_key, &hash_string(msg), msg_signature);
            if !valid {
                println!("Message signature didn't match");
            }
        }

        // sent the response
        println!("Received message {}, valid: {}", msg, valid);
        r.set_valid(valid);
        r.set_msg("Checked client validity".to_string());
        resp.finish(r)
    }
}

fn main() {
    let port = 50051;
    // creating server
    let mut server = grpc::ServerBuilder::new_plain();
    // adding port to server for http
    server.http.set_port(port);
    // adding say service to server
    server.add_service(PoHServer::new_service_def(MyPoH {
        rootcerts: RootCerts::read_rootcert(),
        keymaster: KeyMaster::new(None),
    }));
    // running the server
    let _server = server.build().expect("server");
    println!("greeter server started on port {}", port,);
    // stopping the program from finishing
    loop {
        std::thread::park();
    }
}
