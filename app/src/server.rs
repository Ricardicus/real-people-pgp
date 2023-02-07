use grpc::{ServerHandlerContext, ServerRequestSingle, ServerResponseUnarySink};
// importing generated gRPC code
use poh_grpc::*;
// importing types for messages
use poh::*;
mod poh;
mod poh_grpc;
struct MyPoH;
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
        let message = if req.message.get_msg().is_empty() {
            "world"
        } else {
            req.message.get_msg()
        };
        // sent the response
        println!("Received message {}", message);
        r.set_valid(true);
        r.set_msg("Not checked validity".to_string());
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
    server.add_service(PoHServer::new_service_def(MyPoH));
    // running the server
    let _server = server.build().expect("server");
    println!("greeter server started on port {}", port,);
    // stopping the program from finishing
    loop {
        std::thread::park();
    }
}
