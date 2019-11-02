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

use failure::{format_err, Error, ResultExt};
use log::debug;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{exit, Command, Stdio};
use std::str::FromStr;

mod cfg;
mod opt;
mod templates;

fn build_cache_path(script_path: &Path) -> Result<PathBuf, Error> {
    let script_path = script_path
        .canonicalize()
        .context("Cannot build full path from given script path")?;
    let script_path_rel = script_path
        .strip_prefix("/")
        .context(format!("Could not strip '/' prefix from {:?}", script_path))?;

    let mut user_cache =
        dirs::cache_dir().ok_or_else(|| format_err!("Cannot compute user's cache dir"))?;
    user_cache.push("scriptisto/bin");
    user_cache.push(script_path_rel);
    Ok(user_cache)
}

fn write_bytes(cache_path: &Path, rel_path: &Path, data: &[u8]) -> Result<(), Error> {
    let mut path = cache_path.to_path_buf();
    path.push(rel_path);
    debug!("Writing {} bytes to {:?}", data.len(), path);
    let parent = path
        .parent()
        .ok_or_else(|| format_err!("Cannot compute parent path of {:?}", path))?;
    std::fs::create_dir_all(parent).context(format!(
        "Cannot create cache directory for script, dir path: {:?}",
        parent
    ))?;
    let mut file = File::create(path).context("Cannot output extra file")?;
    file.write_all(data).context("Cannot write bytes to file")?;
    Ok(())
}

fn file_modified(p: &Path) -> Result<std::time::SystemTime, std::io::Error> {
    let meta = std::fs::metadata(p)?;
    let modified = meta.modified()?;
    Ok(modified)
}

fn run_command(
    current_directory: &Path,
    mut cmd: Command,
    stderr_mode: Stdio,
) -> Result<std::process::Output, Error> {
    cmd.stdout(Stdio::piped())
        .stderr(stderr_mode)
        .current_dir(current_directory);

    debug!("Running command: {:?}", cmd);

    let out = cmd.output().context(format!("Cannot run: {:?}", cmd))?;

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
            Some(code) => format_err!("Command {:?} failed. Exit code: {}.", cmd, code,),
            None => format_err!("Child build process terminated by signal"),
        };
        return Err(error);
    }

    Ok(out)
}

fn run_build_command<F>(
    cfg: &cfg::BuildSpec,
    script_cache_path: &Path,
    first_run: bool,
    build_mode: opt::BuildMode,
    stderr_mode: F,
) -> Result<(), Error>
where
    F: Fn() -> Stdio,
{
    if first_run || build_mode == opt::BuildMode::Full {
        if let Some(build_once_cmd) = &cfg.build_once_cmd {
            let mut cmd = Command::new("/bin/sh");
            cmd.arg("-c").arg(build_once_cmd);
            run_command(&script_cache_path, cmd, stderr_mode())?;
        }
    }

    if let Some(build_cmd) = &cfg.build_cmd {
        match &cfg.docker_build {
            // TODO: Do better validation for empty dockerfile, but not-empty docker_build.
            Some(docker_build) if docker_build.dockerfile.is_some() => {
                // Write Dockerfile.
                let tmp_dockerfile_name = "Dockerfile.scriptisto";
                write_bytes(
                    &script_cache_path,
                    &PathBuf::from(&tmp_dockerfile_name),
                    docker_build.dockerfile.clone().unwrap().as_bytes(),
                )?;

                // Build temporary image.
                let tmp_docker_image = format!(
                    "scriptisto-{}-{:x}",
                    script_cache_path
                        .file_name()
                        .ok_or_else(|| format_err!(
                            "BUG: invalid script_cache_path={:?}",
                            script_cache_path
                        ))?
                        .to_string_lossy(),
                    md5::compute(script_cache_path.to_string_lossy().as_bytes())
                )
                .to_string();

                let mut build_im_cmd = Command::new("docker");
                build_im_cmd.arg("build");

                if build_mode == opt::BuildMode::Full {
                    build_im_cmd.arg("--no-cache");
                }

                build_im_cmd
                    .arg("-t")
                    .arg(&tmp_docker_image)
                    .arg("--label")
                    .arg(format!(
                        "scriptisto-cache-path={}",
                        script_cache_path.to_string_lossy()
                    ))
                    .arg("-f")
                    .arg(&tmp_dockerfile_name)
                    .arg(".");

                run_command(&script_cache_path, build_im_cmd, stderr_mode())?;

                // Build binary in Docker.
                let mut cmd = Command::new("docker");
                cmd.arg("run").arg("-t").arg("--rm");

                if let Some(src_mount_dir) = &docker_build.src_mount_dir {
                    cmd.arg("-v").arg(format!(
                        "{}:{}",
                        script_cache_path.to_string_lossy(),
                        src_mount_dir
                    ));
                }

                cmd.args(docker_build.extra_args.iter())
                    .arg(tmp_docker_image)
                    .arg("sh")
                    .arg("-c")
                    .arg(build_cmd);

                run_command(&script_cache_path, cmd, stderr_mode())?;
            }

            _ => {
                let mut cmd = Command::new("/bin/sh");
                cmd.arg("-c").arg(build_cmd);

                run_command(&script_cache_path, cmd, stderr_mode())?;
            }
        }
    }

    write_bytes(
        &script_cache_path,
        &PathBuf::from("scriptisto.metadata"),
        String::new().as_bytes(),
    )
    .context("Cannot write metadata file")?;

    Ok(())
}

fn build(
    build_mode: opt::BuildMode,
    script_path: &str,
) -> Result<(cfg::BuildSpec, PathBuf), Error> {
    let script_path = Path::new(script_path);

    let script_body = std::fs::read(&script_path).context("Cannot read script file")?;
    let script_cache_path = build_cache_path(script_path).context(format!(
        "Cannot build cache path for script: {:?}",
        script_path
    ))?;
    debug!("Path: {:?}", script_path);
    debug!("Cache path: {:?}", script_cache_path);
    let cfg = cfg::BuildSpec::new(&script_body)?;

    let mut metadata_path = script_cache_path.clone();
    metadata_path.push("scriptisto.metadata");
    let metadata_modified = file_modified(&metadata_path).ok();
    let script_modified = file_modified(&script_path).ok();

    let first_run = metadata_modified.is_none();
    let skip_rebuild = metadata_modified > script_modified && build_mode == opt::BuildMode::Default;

    if skip_rebuild {
        debug!("Already compiled, skipping compilation");
    } else {
        for file in cfg.files.iter() {
            write_bytes(
                &script_cache_path,
                &PathBuf::from(&file.path),
                &file.content.as_bytes(),
            )?;
        }

        run_build_command(&cfg, &script_cache_path, first_run, build_mode, || {
            Stdio::piped()
        })?;
    }

    Ok((cfg, script_cache_path))
}

fn default_main(script_path: &str, args: &[String]) -> Result<(), Error> {
    let build_mode_env = std::env::var_os("SCRIPTISTO_BUILD").unwrap_or_default();
    let build_mode = opt::BuildMode::from_str(&build_mode_env.to_string_lossy())?;

    let (cfg, script_cache_path) = build(build_mode, &script_path)?;

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
            default_main(&script_src, &opts.args)
        }
        Some(opt::Command::New { template_name }) => templates::command_new(template_name),
        Some(opt::Command::Template { cmd }) => templates::command_template(cmd),
        Some(opt::Command::Build {
            script_src,
            build_mode,
        }) => {
            let _ = build(build_mode.unwrap_or_default(), &script_src);
            Ok(())
        }
    }
}

fn main() {
    #[cfg(debug_assertions)]
    {
        simple_logger::init_with_level(log::Level::Debug).expect("Cannot init simple logger");
    }
    #[cfg(not(debug_assertions))]
    {
        simple_logger::init_with_level(log::Level::Info).expect("Cannot init simple logger");
    }

    if let Err(e) = main_err() {
        eprintln!("Error: {:?}", e);
        exit(1);
    }
}
