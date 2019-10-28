#!/usr/bin/env scriptisto

// scriptisto-begin
// script_src: src/main.rs
// build_cmd: cargo build --release
// target_bin: ./target/x86_64-unknown-linux-musl/release/script
// docker_build:
//    dockerfile: FROM clux/muslrust
//    src_mount_dir: /volume
//    extra_args: [-v,cargo-cache:/root/.cargo/registry]
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
    println!(
        "Hello, Rust built in Docker! Random: {}",
        rand::random::<u64>()
    );
}
