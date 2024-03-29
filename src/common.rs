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

use anyhow::{anyhow, Context, Result};
use log::debug;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

pub fn script_src_to_absolute(script_src: &Path) -> Result<PathBuf> {
    let script_src_str = script_src.to_string_lossy();
    if !script_src_str.starts_with(['.', '/']) {
        return Err(anyhow!(
            "Script path {:?} must start with '.' or '/'",
            script_src
        ));
    }
    Ok(script_src.canonicalize()?)
}

pub fn build_cache_path(script_path: &Path) -> Result<PathBuf> {
    let script_path = script_src_to_absolute(script_path)?;
    let script_path_rel = script_path
        .strip_prefix("/")
        .context(format!("Could not strip '/' prefix from {:?}", script_path))?;

    let mut user_cache =
        dirs::cache_dir().ok_or_else(|| anyhow!("Cannot compute user's cache dir"))?;
    user_cache.push("scriptisto/bin");
    user_cache.push(script_path_rel);
    Ok(user_cache)
}

pub fn write_bytes(cache_path: &Path, rel_path: &Path, data: &[u8]) -> Result<()> {
    let mut path = cache_path.to_path_buf();
    path.push(rel_path);
    debug!("Writing {} bytes to {:?}", data.len(), path);
    let parent = path
        .parent()
        .ok_or_else(|| anyhow!("Cannot compute parent path of {:?}", path))?;
    std::fs::create_dir_all(parent).context(format!(
        "Cannot create cache directory for script, dir path: {:?}",
        parent
    ))?;
    let mut file = File::create(path).context("Cannot output extra file")?;
    file.write_all(data).context("Cannot write bytes to file")?;
    Ok(())
}

pub fn file_modified(p: &Path) -> Result<std::time::SystemTime, std::io::Error> {
    let meta = std::fs::metadata(p)?;
    let modified = meta.modified()?;
    Ok(modified)
}

pub fn run_command(
    current_directory: &Path,
    mut cmd: Command,
    stderr_mode: Stdio,
) -> Result<std::process::Output> {
    cmd.stdout(Stdio::piped())
        .stderr(stderr_mode)
        .current_dir(current_directory);

    debug!("Running command: {:?}", cmd);

    let out = cmd.output().context(format!(
        "Cannot run cmd={:?} in current_directory={:?}",
        cmd, current_directory
    ))?;

    let stderr = String::from_utf8_lossy(&out.stderr);
    let stdout = String::from_utf8_lossy(&out.stdout);
    debug!(
        "Command result: {:?}\nstderr:\n{}\nstdout:\n{}",
        out.status.code(),
        stderr,
        stdout
    );

    if !out.status.success() {
        eprintln!("{}", stderr);
        eprintln!("{}", stdout);
        let error = match out.status.code() {
            Some(code) => anyhow!("Command {:?} failed. Exit code: {}.", cmd, code,),
            None => anyhow!("Child build process terminated by signal"),
        };
        return Err(error);
    }

    Ok(out)
}
