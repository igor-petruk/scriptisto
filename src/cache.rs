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

use crate::opt::CacheCommand;
use anyhow::{anyhow, Context, Result};
use number_prefix::NumberPrefix;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process;

use crate::*;

fn print_item(name: &str, value: &str) {
    println!("{:20} {}", format!("{}:", name), value);
}

fn get_dir_size_lossy(path: &Path) -> String {
    let size: u64 = walkdir::WalkDir::new(path)
        .into_iter()
        .map(|r| {
            r.map(|e| e.metadata().map(|m| m.len()).unwrap_or_default())
                .unwrap_or_default()
        })
        .sum();

    match NumberPrefix::binary(size as f64) {
        NumberPrefix::Standalone(bytes) => format!("{} bytes", bytes),
        NumberPrefix::Prefixed(prefix, n) => format!("{:.0} {}B", n, prefix),
    }
}

fn collect_info(script_path: &Path) -> Result<BTreeMap<String, String>> {
    let script_body = std::fs::read(script_path).context("Cannot read script file")?;
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

pub fn command_get(name: &str, script_path: &Path) -> Result<()> {
    let items = collect_info(script_path)?;

    if let Some(value) = items.get(name) {
        println!("{}", value);
        Ok(())
    } else {
        Err(anyhow!(
            "'{}' is not found. Available items: {:?}.",
            name,
            items.keys()
        ))
    }
}

pub fn command_info(script_path: &Path) -> Result<()> {
    let items = collect_info(script_path)?;

    for (k, v) in items.iter() {
        print_item(k, v);
    }

    Ok(())
}

pub fn command_clean(script_path: &Path) -> Result<()> {
    let items = collect_info(script_path)?;
    let cache_path = items.get("cache_path").expect("cache_path to exist");

    let _ = std::fs::remove_dir_all(cache_path);

    if let Some(docker_image) = items.get("docker_image") {
        let mut cmd = process::Command::new("docker");
        cmd.arg("image").arg("rm").arg(docker_image);
        let _ = common::run_command(&PathBuf::from("/"), cmd, process::Stdio::piped());
    }

    if let Some(docker_volume) = items.get("docker_src_volume") {
        let mut cmd = process::Command::new("docker");
        cmd.arg("volume").arg("rm").arg(docker_volume);
        let _ = common::run_command(&PathBuf::from("/"), cmd, process::Stdio::piped());
    }

    Ok(())
}

pub fn command_cache(cmd: CacheCommand) -> Result<()> {
    match cmd {
        CacheCommand::Clean { file } => command_clean(&file),
        CacheCommand::Get { name, file } => command_get(&name, &file),
        CacheCommand::Info { file } => command_info(&file),
    }
}
