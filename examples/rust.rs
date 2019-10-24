#!/usr/bin/env scriptisto

// scriptisto-begin
// script_src: src/main.rs
// build_cmd: cargo build --release && strip ./target/release/script
// target_bin: ./target/release/script
// files:
//  - path: Cargo.toml
//    content: |
//     [package]
//     name = "script"
//     version = "0.1.0"
//     edition = "2018"
//     [dependencies]
//     rand="*"
// scriptisto-end

fn main() {
    println!("Hello, Rust! Random: {}", rand::random::<u64>());
}
