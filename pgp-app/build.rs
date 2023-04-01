use std::path::{Path, PathBuf};
use std::{env, fs};

fn main() {
    // compile protocol buffer using protoc
    protoc_rust_grpc::Codegen::new()
        .out_dir("src")
        .input("./proto/poh.proto")
        .rust_protobuf(true)
        .run()
        .expect("error compiling protocol buffer");
}
