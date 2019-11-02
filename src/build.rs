use failure::{format_err, Error, ResultExt};
use log::debug;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use crate::cfg;
use crate::common;
use crate::opt;

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
            common::run_command(&script_cache_path, cmd, stderr_mode())?;
        }
    }

    if let Some(build_cmd) = &cfg.build_cmd {
        match &cfg.docker_build {
            // TODO: Do better validation for empty dockerfile, but not-empty docker_build.
            Some(docker_build) if docker_build.dockerfile.is_some() => {
                // Write Dockerfile.
                let tmp_dockerfile_name = "Dockerfile.scriptisto";
                common::write_bytes(
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

                common::run_command(&script_cache_path, build_im_cmd, stderr_mode())?;

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

                common::run_command(&script_cache_path, cmd, stderr_mode())?;
            }

            _ => {
                let mut cmd = Command::new("/bin/sh");
                cmd.arg("-c").arg(build_cmd);

                common::run_command(&script_cache_path, cmd, stderr_mode())?;
            }
        }
    }

    common::write_bytes(
        &script_cache_path,
        &PathBuf::from("scriptisto.metadata"),
        String::new().as_bytes(),
    )
    .context("Cannot write metadata file")?;

    Ok(())
}

pub fn perform(
    build_mode: opt::BuildMode,
    script_path: &str,
    show_logs: bool,
) -> Result<(cfg::BuildSpec, PathBuf), Error> {
    let script_path = Path::new(script_path);

    let script_body = std::fs::read(&script_path).context("Cannot read script file")?;
    let script_cache_path = common::build_cache_path(script_path).context(format!(
        "Cannot build cache path for script: {:?}",
        script_path
    ))?;
    debug!("Path: {:?}", script_path);
    debug!("Cache path: {:?}", script_cache_path);
    let cfg = cfg::BuildSpec::new(&script_body)?;

    let mut metadata_path = script_cache_path.clone();
    metadata_path.push("scriptisto.metadata");
    let metadata_modified = common::file_modified(&metadata_path).ok();
    let script_modified = common::file_modified(&script_path).ok();

    let first_run = metadata_modified.is_none();
    let skip_rebuild = metadata_modified > script_modified && build_mode == opt::BuildMode::Default;

    if skip_rebuild {
        debug!("Already compiled, skipping compilation");
    } else {
        for file in cfg.files.iter() {
            common::write_bytes(
                &script_cache_path,
                &PathBuf::from(&file.path),
                &file.content.as_bytes(),
            )?;
        }

        run_build_command(&cfg, &script_cache_path, first_run, build_mode, || {
            if show_logs {
                Stdio::inherit()
            } else {
                Stdio::piped()
            }
        })?;
    }

    Ok((cfg, script_cache_path))
}
