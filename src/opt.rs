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

use failure::format_err;
use log::debug;
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
    args_iter.next(); // Skip self

    let opts = match args_iter.next() {
        None => {
            display_help();
            unreachable!();
        }
        Some(ref script_src) if script_src.starts_with('.') || script_src.starts_with('/') => {
            // Do not use structopt, because it will try to recognize subcommands in all args.
            let extra_args: Vec<_> = args_iter.cloned().collect();
            Opt {
                script_src: Some((*script_src).to_string()),
                args: extra_args,
                cmd: None,
            }
        }
        _ => match Opt::from_iter(args.iter()) {
            ref opts if opts.script_src.is_some() => {
                // Sublte case when the first argument is so different from any subcommand
                // that autosuggestion does not kick in an it's value ends up in script_src.
                // We have manually checked for a valid script_src in a differen branch, so
                // basically this is strongly misspelled subcommand.
                // I could not figure out how to avoid this via structopt validators.
                display_help();
                unreachable!();
            }
            opts => opts,
        },
    };
    debug!("Parsed command line options: {:#?}", opts);
    opts
}

// Need to extend these tests with covering early exits.
#[cfg(test)]
mod tests {
    use super::*;

    fn args(slice: &[&str]) -> Vec<String> {
        let mut v = vec!["scriptisto".to_string()];
        v.extend(slice.iter().map(|s| s.to_string()));
        v
    }

    #[test]
    fn test_script_args() {
        // Includes a normal arg and an arg from a subcommand.
        let opts = from_args(&args(&vec!["./foo", "arg", "new"]));
        assert_eq!(
            opts,
            Opt {
                script_src: Some(String::from("./foo")),
                args: vec!["arg".into(), "new".into()],
                cmd: None,
            }
        );
    }

    #[test]
    fn test_new() {
        let opts = from_args(&args(&vec!["new", "rust"]));
        assert_eq!(
            opts,
            Opt {
                script_src: None,
                args: vec![],
                cmd: Some(Command::New {
                    template_name: Some("rust".into()),
                }),
            }
        );
    }
}
