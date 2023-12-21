extern crate protoc_rust;

fn main() {
    let result = protoc_rust::Codegen::new()
        .out_dir("src/protos")
        .inputs(&["src/spb.proto", "src/spb.proto"]) // Ensure these paths are correct
        .include("protos") // Ensure this path is correct
        .run();

    match result {
        Ok(_) => println!("Hooray! Protoc code generation succeeded."),
        Err(e) => eprintln!("Error during protoc code generation: {}", e),
    }
}

