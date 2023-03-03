use std::path::{Path, PathBuf};
use std::{env, fs};

fn get_output_path() -> PathBuf {
    //<root or manifest path>/target/<profile>/
    let manifest_dir_string = env::var("CARGO_MANIFEST_DIR").unwrap();
    let build_type = env::var("PROFILE").unwrap();
    let path = Path::new(&manifest_dir_string)
        .join("target")
        .join(build_type);
    return PathBuf::from(path);
}

fn main() {
    // compile protocol buffer using protoc
    protoc_rust_grpc::Codegen::new()
        .out_dir("src")
        .input("./proto/poh.proto")
        .rust_protobuf(true)
        .run()
        .expect("error compiling protocol buffer");
    // Moving rootcert file
    let rootcert_file = "poh_rootcert.pohrc";
    let target_dir = get_output_path();
    let src = Path::join(&env::current_dir().unwrap(), &rootcert_file);
    let dest = Path::join(Path::new(&target_dir), Path::new(&rootcert_file));
    fs::copy(src, dest).unwrap();
    // Moving database file
    let database_file = "db.pohdb";
    let mut target_dir = get_output_path();
    target_dir.push("databases");
    if !target_dir.exists() {
        fs::create_dir(&target_dir).expect("Failed to create database directory");
    }
    let src = Path::join(&env::current_dir().unwrap(), &database_file);
    let dest = Path::join(Path::new(&target_dir), Path::new(&database_file));
    println!("src: {:?}, dest: {:?}", src, dest);
    fs::copy(src, dest).unwrap();
}
