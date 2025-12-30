use std::fs;
use std::process::Command;

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let descriptor_path = format!("{}/person.pb", out_dir);

    // Run protoc to generate descriptor set
    let status = Command::new("protoc")
        .args(["--descriptor_set_out", &descriptor_path, "proto/person.proto"])
        .status()
        .expect("failed to run protoc");
    assert!(status.success(), "protoc failed");

    // Read descriptor and generate Rust code
    let descriptor_bytes = fs::read(&descriptor_path).expect("failed to read descriptor");
    let generated = protocrap::codegen::generate(&descriptor_bytes).expect("codegen failed");

    // Write generated code
    let generated_path = format!("{}/person.pc.rs", out_dir);
    fs::write(&generated_path, generated).expect("failed to write generated code");

    println!("cargo::rerun-if-changed=proto/person.proto");
    println!("cargo::rerun-if-changed=build.rs");
}
