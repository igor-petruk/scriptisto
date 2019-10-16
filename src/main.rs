// Copyright 2019 Google LLC
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
use include_dir::Dir;
use log::debug;
use std::cmp::min;
use std::collections::{BTreeSet, HashMap};
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Write};
use std::path::{Path, PathBuf};
use std::process::{exit, Command, Stdio};

const EXAMPLES: Dir = include_dir!("./examples/");

fn file_bytes(path: &Path) -> Result<Vec<u8>, Error> {
    let mut file = File::open(path).context(format!("Cannot open file: {:?}", path))?;
    let initial_cap = file
        .metadata()
        .map(|m| m.len() as usize + 1)
        .unwrap_or(4096);
    let mut vec = Vec::with_capacity(initial_cap);
    file.read_to_end(&mut vec)
        .context(format!("Cannot read file: {:?}", path))?;
    Ok(vec)
}

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

#[derive(Debug)]
struct Config {
    build_cmd: String,
    target_bin: String,
    extra_files: HashMap<String, Vec<u8>>,
}

#[derive(Clone, Debug)]
enum ParserState {
    ScriptSource,
    ConfigSource { prefix_len: usize },
    InputFile { path: String, prefix_len: usize },
}

impl Config {
    fn new(script_body: &[u8]) -> Result<Self, Error> {
        let mut build_cmd: Option<String> = None;
        let mut target_bin: Option<String> = None;
        let mut script_src_path: Option<String> = None;
        let mut replace_shebang_with = String::new();

        let mut script_src = Vec::new();
        let mut extra_files: HashMap<String, Vec<String>> = Default::default();
        let reader = BufReader::new(script_body);

        use ParserState::*;
        let mut state = ParserState::ScriptSource;
        let mut line_num = 0;

        for line in reader.lines() {
            let mut line = line.context(format!("Cannot parse script line: {}", line_num))?;
            
            // Artisan parser here, needs to be rewritten cleanly.
        
            script_src.push(line.clone());
            let old_state = state.clone();
            state = match old_state {
                ScriptSource => {
                    let sb_start = line.find("scriptisto-begin");
                    if let Some(pos) = sb_start {
                        ConfigSource { prefix_len: pos }
                    } else {
                        ScriptSource
                    }
                }
                ConfigSource { prefix_len } => {
                    line.drain(..min(prefix_len, line.len()));
                    let split: Vec<&str> = line.splitn(2, ':').collect();
                    let config_opt = split[0];
                    let config_val = split.get(1);
                    if config_opt.contains("scriptisto-end") {
                        ScriptSource
                    } else if let Some(val) = config_val {
                        let val = val.trim().to_string();
                        if config_opt.contains("file-begin") {
                            InputFile {
                                path: val,
                                prefix_len,
                            }
                        } else {
                            if config_opt == "build_cmd" {
                                build_cmd = Some(val);
                            } else if config_opt == "target_bin" {
                                target_bin = Some(val);
                            } else if config_opt == "script_src" {
                                script_src_path = Some(val);
                            } else if config_opt == "replace_shebang_with" {
                                replace_shebang_with = val;
                            } else if !config_opt.trim().is_empty() {
                                return Err(format_err!(
                                    "Unknown option: {}, line num: {}",
                                    config_opt,
                                    line_num
                                ));
                            }
                            old_state
                        }
                    } else {
                        old_state
                    }
                }
                InputFile {
                    ref path,
                    prefix_len,
                } => {
                    line.drain(..min(prefix_len, line.len()));
                    if line.contains("file-end") {
                        ConfigSource { prefix_len }
                    } else {
                        let lines = extra_files.entry(path.clone()).or_default();
                        lines.push(line.clone());
                        old_state
                    }
                }
            };
            debug!("{}", line);
            debug!("######## {:?}", state);
            line_num += 1;
        }

        if !script_src.is_empty() {
            script_src[0] = replace_shebang_with;
        }

        extra_files.insert(
            script_src_path.ok_or_else(|| format_err!("Please specify script_src"))?,
            script_src,
        );
        let extra_files = extra_files
            .iter()
            .map(|(k, v)| (k.clone(), v.join("\n").into_bytes()))
            .collect();
        Ok(Config {
            build_cmd: build_cmd.ok_or_else(|| format_err!("Please specify build_cmd"))?,
            target_bin: target_bin.ok_or_else(|| format_err!("Please specfy target_bin"))?,
            extra_files,
        })
    }
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

fn default_main(mut args: Vec<String>) -> Result<(), Error> {
    let script_path_str = args
        .get(1)
        .ok_or_else(|| format_err!("Please specify script path as first argument"))?;
    let script_path = Path::new(script_path_str);
    let script_body = file_bytes(&script_path).context("Cannot read script file")?;
    let script_cache_path = build_cache_path(script_path).context(format!(
        "Cannot build cache path for script: {:?}",
        script_path
    ))?;
    debug!("Path: {:?}", script_path);
    debug!("Cache path: {:?}", script_cache_path);
    let cfg = Config::new(&script_body)?;

    let mut metadata_path = script_cache_path.clone();
    metadata_path.push("scriptisto.metadata");
    let metadata_modified = file_modified(&metadata_path).ok();
    let script_modified = file_modified(&script_path).ok();
    let already_compiled = metadata_modified > script_modified;

    if already_compiled {
        debug!("Already compiled, skipping compilation");
    } else {
        for (extra_file_path, data) in cfg.extra_files.iter() {
            write_bytes(&script_cache_path, &PathBuf::from(extra_file_path), &data)?;
        }

        let out = Command::new("bash")
            .arg("-c")
            .arg(cfg.build_cmd.clone())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .current_dir(&script_cache_path)
            .output()
            .context(format!("Could not run build command: {}", cfg.build_cmd))?;

        if !out.status.success() {
            let stderr = String::from_utf8_lossy(&out.stderr);
            let stdout = String::from_utf8_lossy(&out.stdout);
            eprintln!("{}", stderr);
            eprintln!("{}", stdout);
            let error = match out.status.code() {
                Some(code) => format_err!("Build failed. Exit code: {}.", code,),
                None => format_err!("Child build process terminated by signal"),
            };
            return Err(error);
        }
        write_bytes(
            &script_cache_path,
            &PathBuf::from("scriptisto.metadata"),
            String::new().as_bytes(),
        )
        .context("Cannot write metadata file")?;
    }

    let bin_cmd =
        PathBuf::from(cfg.target_bin.replace(
            "@@@",
            script_cache_path.to_str().ok_or_else(|| {
                format_err!("Cannot turn path to string: {:?}", script_cache_path)
            })?,
        ));
    debug!("Bin cmd path: {:?}", bin_cmd);
    let mut bin_cmd_segments: Vec<String> = bin_cmd
        .to_string_lossy()
        .split_ascii_whitespace()
        .map(|s| s.to_string())
        .collect();
    let binary = bin_cmd_segments
        .drain(..1)
        .next()
        .ok_or_else(|| format_err!("Could not remove first segment from: {:?}", bin_cmd,))?
        .to_string();
    args.drain(..2);
    bin_cmd_segments.extend(args);

    let mut target_argv: Vec<String> = Vec::new();
    target_argv.push(binary.clone());
    target_argv.extend(bin_cmd_segments);

    let error = match exec::execvp(&binary, &target_argv) {
        exec::Error::Errno(e) => {
            format_err!("Cannot execute target binary '{:?}': {:#?}", binary, e)
        }
        _ => format_err!("Cannot exec"),
    };
    Err(error)
}

fn gen_main(args: Vec<String>) -> Result<(), Error> {
    let lang = args.get(2).map(|s| s.to_string()).unwrap_or_default();
    let mut langs = BTreeSet::new();
    for file in EXAMPLES.files() {
        let path = PathBuf::from(file.path());
        let file_stem = path
            .file_stem()
            .ok_or_else(|| format_err!("Cannot strip extension from {:?}", path))?;
        let current_lang = file_stem.to_string_lossy().into_owned();
        if lang == current_lang {
            print!(
                "{}",
                file.contents_utf8()
                    .ok_or_else(|| format_err!("File {:?} is not UTF-8", file))?
            );
            return Ok(());
        }
        langs.insert(current_lang);
    }
    // Not found
    let langs: Vec<_> = langs.iter().collect();
    eprintln!("Usage: -g <lang>. Available:\n{:#?}", langs);
    Ok(())
}

fn main_err() -> Result<(), Error> {
    #[cfg(debug_assertions)]
    {
        simple_logger::init_with_level(log::Level::Debug).context("Cannot init simple logger")?;
    }
    #[cfg(not(debug_assertions))]
    {
        simple_logger::init_with_level(log::Level::Info).context("Cannot init simple logger")?;
    }

    let args: Vec<String> = std::env::args().collect();
    debug!("Args: {:?}", args);
    let first_arg = args
        .get(1)
        .ok_or_else(|| format_err!("Please specify script path as first argument or '-g'"))?;

    if first_arg == "-g" {
        gen_main(args)
    } else {
        default_main(args)
    }
}

fn main() {
    if let Err(e) = main_err() {
        eprintln!("Error: {:?}", e);
        exit(1);
    }
}
