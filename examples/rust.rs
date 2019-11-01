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
//     structopt="*"
// scriptisto-end

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "script", about = "A script.")]
struct Opt {
    /// Example input
    #[structopt(short, long)]
    input: Option<String>,
}

fn main() {
    let opt = Opt::from_args();
    println!("Hello, Rust! Command line options: {:?}", opt);
}
