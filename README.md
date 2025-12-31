# pc-example

A minimal example showing how to use [protocrap](https://crates.io/crates/protocrap) for protobuf serialization in Rust.

## What this demonstrates

- Code generation from `.proto` files via `build.rs`
- Creating and populating protobuf messages
- Serializing to bytes with `encode_vec`
- Deserializing with `decode_flat`
- Working with repeated fields
- Arena-based memory allocation

## Prerequisites

- Rust 1.85+ (edition 2024)
- `protoc` (Protocol Buffers compiler) in your PATH

## Running

```bash
cargo run
```

## Project structure

```
├── proto/
│   └── person.proto      # Protobuf schema
├── src/
│   ├── lib.rs            # Includes generated code
│   └── main.rs           # Example usage
└── build.rs              # Runs protoc + protocrap codegen
```

## How it works

1. `build.rs` invokes `protoc` to create a descriptor set from `person.proto`
2. `protocrap::codegen::generate()` produces Rust code from the descriptor
3. The generated code is included via `include!()` in `lib.rs`
4. `main.rs` demonstrates creating, encoding, and decoding messages

## License

MIT
