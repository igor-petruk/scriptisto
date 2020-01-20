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

// use ascii_table::{print_table, Align, ColumnConfig, TableConfig};
use failure::{Error, ResultExt};
// use log::debug;
use number_prefix::{NumberPrefix, Prefixed, Standalone};
use std::collections::BTreeMap;
use std::fmt::Debug;
use std::path::{Path, PathBuf};
use std::process;
use structopt::StructOpt;

use crate::*;

#[derive(Debug, StructOpt, PartialEq)]
pub enum Command {
    /// Shows information about the cache directory for the script.
    Info {
        #[structopt(help = "A filename of the script file.")]
        file: PathBuf,
    },
    /// Clean the cache for a particular script. Removes the cache directory. Removes the Docker image/volume if
    /// they exist, but does not prune.
    #[structopt(visible_alias = "clear")]
    Clean {
        #[structopt(help = "A filename of the script file.")]
        file: PathBuf,
    },
    /// Shows a particular item from "info" by name.
    Get {
        #[structopt(help = "An item name, e.g. cache_path.")]
        name: String,
        #[structopt(help = "A filename of the script file.")]
        file: PathBuf,
    },
}

fn print_item(name: &str, value: &str) {
    println!("{:20} {}", format!("{}:", name), value);
}

fn get_dir_size_lossy(path: &Path) -> String {
    let size: u64 = walkdir::WalkDir::new(&path)
        .into_iter()
        .map(|r| {
            r.map(|e| e.metadata().map(|m| m.len()).unwrap_or_default())
                .unwrap_or_default()
        })
        .sum();

    match NumberPrefix::binary(size as f64) {
        Standalone(bytes) => format!("{} bytes", bytes),
        Prefixed(prefix, n) => format!("{:.0} {}B", n, prefix),
    }
}

fn collect_info(script_path: &Path) -> Result<BTreeMap<String, String>, Error> {
    let script_body = std::fs::read(&script_path).context("Cannot read script file")?;
    let script_cache_path = common::build_cache_path(script_path).context(format!(
        "Cannot build cache path for script: {:?}",
        script_path
    ))?;
    let cfg = cfg::BuildSpec::new(&script_body)?;

    let mut items = BTreeMap::new();

    items.insert(
        "cache_path".into(),
        script_cache_path.to_string_lossy().to_string(),
    );

    if cfg.docker_build.is_some() {
        items.insert(
            "docker_image".into(),
            build::docker_image_name(&script_cache_path)?,
        );
        items.insert(
            "docker_src_volume".into(),
            build::docker_volume_name(&script_cache_path)?,
        );
        items.insert("dir_size".into(), get_dir_size_lossy(&script_cache_path));
    }

    Ok(items)
}

pub fn command_get(name: &str, script_path: &Path) -> Result<(), Error> {
    let items = collect_info(&script_path)?;

    if let Some(value) = items.get(name) {
        println!("{}", value);
        Ok(())
    } else {
        Err(format_err!(
            "'{}' is not found. Available items: {:?}.",
            name,
            items.keys()
        ))
    }
}

pub fn command_info(script_path: &Path) -> Result<(), Error> {
    let items = collect_info(&script_path)?;

    for (k, v) in items.iter() {
        print_item(k, &v);
    }

    Ok(())
}

pub fn command_clean(script_path: &Path) -> Result<(), Error> {
    let items = collect_info(&script_path)?;
    let cache_path = items.get("cache_path").expect("cache_path to exist");

    let _ = std::fs::remove_dir_all(cache_path);

    if let Some(docker_image) = items.get("docker_image") {
        let mut cmd = process::Command::new("docker");
        cmd.arg("image").arg("rm").arg(&docker_image);
        let _ = common::run_command(&PathBuf::from("/"), cmd, process::Stdio::piped());
    }

    if let Some(docker_volume) = items.get("docker_src_volume") {
        let mut cmd = process::Command::new("docker");
        cmd.arg("volume").arg("rm").arg(&docker_volume);
        let _ = common::run_command(&PathBuf::from("/"), cmd, process::Stdio::piped());
    }

    Ok(())
}

pub fn command_cache(cmd: Command) -> Result<(), Error> {
    match cmd {
        Command::Clean { file } => command_clean(&file),
        Command::Get { name, file } => command_get(&name, &file),
        Command::Info { file } => command_info(&file),
    }
}
