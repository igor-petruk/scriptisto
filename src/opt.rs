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

use super::common;
use failure::format_err;
use log::debug;
use std::path::Path;
use std::str::FromStr;
use structopt::StructOpt;

#[derive(Debug, PartialEq, StructOpt)]
#[structopt(rename_all = "snake-case")]
pub enum BuildMode {
    Default,
    Source,
    Full,
}

impl Default for BuildMode {
    fn default() -> Self {
        BuildMode::Default
    }
}

impl FromStr for BuildMode {
    type Err = failure::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use BuildMode::*;
        Ok(match s {
            "" => Default,
            "source" => Source,
            "full" => Full,
            _ => {
                return Err(format_err!(
                    "Incorrect build mode value. Available values: <unset>, source, full."
                ))
            }
        })
    }
}

#[derive(Debug, StructOpt, PartialEq)]
#[structopt(
    name = "scriptisto",
    about = "A 'shebang-interpreter' for compiled languages"
)]
pub struct Opt {
    /// A path to a script to run. If specified, first character must be "." or "/".
    #[structopt()]
    pub script_src: Option<String>,

    /// Additional arguments passed to a script.
    #[structopt()]
    pub args: Vec<String>,

    #[structopt(subcommand)]
    pub cmd: Option<Command>,
}

#[derive(Debug, StructOpt, PartialEq)]
pub enum Command {
    /// Build cache operations.
    Cache {
        #[structopt(subcommand)]
        cmd: crate::cache::Command,
    },
    /// Prints an example starting script in a programming language of your
    /// choice.
    New {
        #[structopt(
            help = "If specified, determines a language. Example usage: \"scriptisto new <template_name> | tee new-script\".\nIf not specified, \"new\" lists available templates."
        )]
        template_name: Option<String>,
    },
    /// Manage custom script templates.
    Template {
        #[structopt(subcommand)]
        cmd: crate::templates::Command,
    },
    /// Build a script without running.
    Build {
        /// A path to a script to build.
        #[structopt()]
        script_src: String,
        /// Build mode. If unset, only builds if necessary. "source" - to rebuild each time. "full" to fully re-fetch Docker image and run `build_once_cmd`.
        #[structopt(short, long)]
        build_mode: Option<BuildMode>,
    },
}

fn display_help() {
    Opt::from_iter(vec!["", "help"]);
}

pub fn from_args(args: &[String]) -> Opt {
    let mut args_iter = args.iter();
    args_iter.next();

    if let Some(script_src) = args_iter.next() {
        let absolute_script_src = common::script_src_to_absolute(Path::new(&script_src));
        if let Ok(absolute_script_src) = absolute_script_src {
            if absolute_script_src.exists() {
                return Opt {
                    script_src: Some(absolute_script_src.to_string_lossy().into()),
                    args: args_iter.cloned().collect(),
                    cmd: None,
                };
            }
        }
    }

    let opts = Opt::from_iter(args.iter());
    debug!("Parsed command line options: {:#?}", opts);

    if opts.cmd.is_none() && opts.script_src.is_none() {
        display_help();
    };
    opts
}
