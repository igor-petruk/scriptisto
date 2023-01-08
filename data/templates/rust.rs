#!/usr/bin/env scriptisto

// scriptisto-begin
// script_src: src/main.rs
// build_cmd: cargo build --release && strip ./target/release/script
// target_bin: ./target/release/script
// files:
//  - path: Cargo.toml
//    content: |
//     package = { name = "script", version = "0.1.0", edition = "2018"}
//     [dependencies]
//     clap={version="4", features=["derive"]}
// scriptisto-end

use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "script", about = "A script.")]
struct Opt {
    /// Example input
    #[arg(short, long)]
    input: Option<u32>,
}

fn main() {
    let opt = Opt::parse();
    println!("Hello, Rust! Command line options: {:?}", opt);
}
