// Copyright 2019 The Scriptisto Authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#[macro_use]
extern crate include_dir;

use failure::{format_err, Error};
use log::debug;
use std::env;
use std::path::{Path, PathBuf};
use std::process::exit;
use std::str::FromStr;

mod build;
mod cache;
mod cfg;
mod common;
mod opt;
mod templates;

fn default_main(script_path: &str, args: &[String]) -> Result<(), Error> {
    let build_mode_env = std::env::var_os("SCRIPTISTO_BUILD").unwrap_or_default();
    let build_mode = opt::BuildMode::from_str(&build_mode_env.to_string_lossy())?;
    let show_logs = std::env::var_os("SCRIPTISTO_BUILD_LOGS").is_some();

    let (cfg, script_cache_path) = build::perform(build_mode, &script_path, show_logs)?;

    let mut full_target_bin = script_cache_path.clone();
    full_target_bin.push(PathBuf::from(cfg.target_bin));
    let full_target_bin = full_target_bin
        .canonicalize()?
        .to_string_lossy()
        .to_string();
    debug!("Full target_bin path: {:?}", full_target_bin);

    let (binary, mut target_argv) = match cfg.target_interpreter {
        Some(ref target_interpreter) if !target_interpreter.is_empty() => {
            let mut seq: Vec<String> = target_interpreter
                .split_ascii_whitespace()
                .map(|s| s.to_string())
                .collect();
            let binary = seq
                .first()
                .expect("first() should work as we checked the guard above")
                .clone();
            seq.drain(..1);
            seq.push(full_target_bin);
            (binary, seq)
        }
        _ => (full_target_bin, vec![]),
    };
    target_argv.insert(0, binary.clone());
    // args.drain(..2);
    target_argv.extend_from_slice(args);
    debug!("Running exec {:?}, Args: {:?}", binary, target_argv);

    // Scripts can use this to find other build artifacts
    env::set_var("SCRIPTISTO_CACHE_DIR", script_cache_path);

    let error = match exec::execvp(&binary, &target_argv) {
        exec::Error::Errno(e) => {
            format_err!("Cannot execute target binary '{:?}': {:#?}", binary, e)
        }
        _ => format_err!("Cannot exec"),
    };
    Err(error)
}

fn main_err() -> Result<(), Error> {
    let args: Vec<String> = std::env::args().collect();
    let opts = opt::from_args(&args);
    debug!("Parsed options: {:?}", opts);

    match opts.cmd {
        None => {
            let script_src = opts.script_src.ok_or_else(|| {
                format_err!("PROBABLY A BUG: script_src must be non-empty if no subcommand found.")
            })?;
            let script_src = common::script_src_to_absolute(Path::new(&script_src))?;
            default_main(&script_src.to_string_lossy(), &opts.args)
        }
        Some(opt::Command::Cache { cmd }) => cache::command_cache(cmd),
        Some(opt::Command::New { template_name }) => templates::command_new(template_name),
        Some(opt::Command::Template { cmd }) => templates::command_template(cmd),
        Some(opt::Command::Build {
            script_src,
            build_mode,
        }) => {
            let _ = build::perform(build_mode.unwrap_or_default(), &script_src, true);
            Ok(())
        }
    }
}

fn main() {
    env_logger::init();

    if let Err(e) = main_err() {
        eprintln!("Error: {:?}", e);
        exit(1);
    }
}
