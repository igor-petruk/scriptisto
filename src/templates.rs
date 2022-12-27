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

use crate::opt::TemplatesCommand;
use failure::{format_err, Error, ResultExt};
use include_dir::Dir;
use log::debug;
use std::collections::BTreeMap;
use std::fmt::Debug;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

const TEMPLATES: Dir = include_dir!("./data/templates/");

#[derive(Debug, PartialEq, Eq)]
enum Source {
    BuiltIn,
    Custom,
}

#[derive(Debug)]
struct Template {
    source: Source,
    filename: String,
    contents: String,
}

type TemplateMap = BTreeMap<String, Template>;

fn path_to_file_name<T: AsRef<Path> + Debug>(p: T) -> Result<String, Error> {
    let p: PathBuf = p.as_ref().into();
    Ok(p.file_name()
        .ok_or_else(|| format_err!("Cannot extract filename from {:?}", p))?
        .to_string_lossy()
        .to_string())
}

fn filename_to_template_name<T: AsRef<Path>>(p: T) -> Result<String, Error> {
    let p: PathBuf = p.as_ref().into();
    let file_name = path_to_file_name(&p)?;
    Ok(p.file_stem()
        .map(|f| f.to_string_lossy().to_string())
        .unwrap_or(file_name))
}

// Also creates the directory, this ok for now.
fn filename_to_template_path<T: AsRef<Path> + Debug>(p: T) -> Result<PathBuf, Error> {
    let file_name = path_to_file_name(&p)?;
    let templates_directory = get_templates_directory()?;
    std::fs::create_dir_all(&templates_directory)?;
    let mut template_path = templates_directory;
    template_path.push(file_name);
    Ok(template_path)
}

fn get_built_in_templates() -> Result<TemplateMap, Error> {
    let mut templates = TemplateMap::new();
    for file in TEMPLATES.files() {
        let path = PathBuf::from(file.path());
        templates.insert(
            filename_to_template_name(&path)?,
            Template {
                source: Source::BuiltIn,
                filename: path_to_file_name(path)?,
                contents: file
                    .contents_utf8()
                    .ok_or_else(|| format_err!("File {:?} is not UTF-8", file))?
                    .to_string(),
            },
        );
    }
    Ok(templates)
}

fn get_templates_directory() -> Result<PathBuf, Error> {
    let mut p =
        dirs::config_dir().ok_or_else(|| format_err!("Cannot compute user's config dir"))?;
    p.push("scriptisto/templates");
    Ok(p)
}

fn get_custom_templates() -> Result<TemplateMap, Error> {
    let mut templates = TemplateMap::new();

    let templates_dir = get_templates_directory()?;

    debug!("Scanning for custom templates at {:?};", templates_dir);
    match std::fs::read_dir(&templates_dir) {
        Ok(dir_iter) => {
            debug!("Custom templates directory found");
            for template_file in dir_iter {
                let template_file = template_file?;
                let name = filename_to_template_name(template_file.path())?;
                let filename = path_to_file_name(&template_file.path())?;
                let contents = std::fs::read_to_string(template_file.path())?;
                templates.insert(
                    name,
                    Template {
                        source: Source::Custom,
                        filename,
                        contents,
                    },
                );
            }
        }
        Err(e) => {
            debug!("The custom templates directory skipped, reason: {:?}.", e);
        }
    }

    Ok(templates)
}

fn build_ascii_table() -> ascii_table::AsciiTable {
    let mut table = ascii_table::AsciiTable::default();
    table
        .column(0)
        .set_header("Template Name")
        .set_align(ascii_table::Align::Left);
    table
        .column(1)
        .set_header("Custom")
        .set_align(ascii_table::Align::Left);
    table
        .column(2)
        .set_header("Extension")
        .set_align(ascii_table::Align::Left);
    table
}

fn get_templates() -> Result<TemplateMap, Error> {
    let mut templates = get_built_in_templates()?;
    templates.append(&mut get_custom_templates()?);
    Ok(templates)
}

fn filename_extension(filename: &str) -> String {
    Path::new(filename)
        .extension()
        .map(|e| format!(".{}", e.to_string_lossy()))
        .unwrap_or_default()
}

fn print_templates(templates: &TemplateMap) {
    let table: Vec<_> = templates
        .iter()
        .map(|(k, v)| {
            vec![
                k.clone(),
                match v.source {
                    Source::BuiltIn => "",
                    Source::Custom => "yes",
                }
                .to_string(),
                filename_extension(&v.filename),
            ]
        })
        .collect();

    build_ascii_table().print(table);
}

fn template_not_found(name: &str, templates: &TemplateMap) -> ! {
    println!("Template '{}' is not found!", name);
    println!("Available templates in the table below:");
    print_templates(templates);
    std::process::exit(1);
}

pub fn command_new(name: Option<String>) -> Result<(), Error> {
    let templates = get_templates()?;

    if let Some(name) = name {
        if let Some(template) = templates.get(&name) {
            println!("{}", template.contents);
        } else {
            template_not_found(&name, &templates);
        }
    } else {
        println!("Usage:\n$ scriptisto new <template> | tee ./new-script");
        println!("Available templates in the table below:");
        print_templates(&templates);
    }
    Ok(())
}

pub fn write_template(filename: &str, content: &str) -> Result<(), Error> {
    let template_path = filename_to_template_path(filename)?;
    let mut file = File::create(&template_path).context("Cannot create script file")?;
    let bytes = content.as_bytes();
    file.write_all(bytes)
        .context("Cannot write bytes to file")?;
    debug!("Wrote {} bytes to {:?}", bytes.len(), template_path);
    Ok(())
}

pub fn edit(initial_value: &str, filename: &str) -> Result<(), Error> {
    let extension = filename_extension(filename);
    let editor = scrawl::editor::new()
        .extension(&extension)
        .contents(initial_value);

    let new_value = editor.open().unwrap();

    if new_value.trim() == initial_value.trim() {
        println!("No changes were made during editing.");
    } else {
        write_template(filename, &new_value)?;
    }

    Ok(())
}

pub fn command_template_import(path: &Path) -> Result<(), Error> {
    let file_name = path_to_file_name(path)?;
    let templates = get_templates()?;
    let template_name = filename_to_template_name(path)?;

    let old_file_to_remove: Option<_> = templates
        .get(&template_name)
        .iter()
        .flat_map(|template| {
            if file_name != template.filename {
                Some(template.filename.clone())
            } else {
                None
            }
        })
        .next();

    let template_path = filename_to_template_path(file_name)?;
    std::fs::copy(path, template_path).context("Cannot copy file to the template directory")?;

    // Not that copy was successful.
    if let Some(old_file) = old_file_to_remove {
        let path = filename_to_template_path(old_file)?;
        std::fs::remove_file(&path)
            .context("Failed to remove old template, template directory may be insonsistent!")?;
    }
    Ok(())
}

pub fn command_template_edit(template_name: String) -> Result<(), Error> {
    let templates = get_templates()?;

    match templates.get(&template_name) {
        Some(template) => edit(&template.contents, &template.filename),
        None => {
            template_not_found(&template_name, &templates);
        }
    }
}

pub fn command_template_rm(template_name: String) -> Result<(), Error> {
    let templates = get_templates()?;

    match templates.get(&template_name) {
        Some(template) if template.source == Source::BuiltIn => {
            println!(
                "Cannot remove custom '{}' script. It is already reset to the built-in content.",
                template_name
            );
            std::process::exit(1);
        }
        Some(template) => {
            let mut script_path = get_templates_directory()?;
            script_path.push(&template.filename);
            std::fs::remove_file(&script_path).context("Cannot remove script file")?;
            Ok(())
        }
        None => {
            template_not_found(&template_name, &templates);
        }
    }
}

pub fn command_template(cmd: TemplatesCommand) -> Result<(), Error> {
    let templates = get_templates()?;

    match cmd {
        TemplatesCommand::List {} => {
            print_templates(&templates);
            Ok(())
        }
        TemplatesCommand::Import { file } => command_template_import(&file),
        TemplatesCommand::Edit { template_name } => command_template_edit(template_name),
        TemplatesCommand::Remove { template_name } => command_template_rm(template_name),
    }
}
