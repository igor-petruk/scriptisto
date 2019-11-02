use ascii_table::{print_table, Align, ColumnConfig, TableConfig};
use failure::{format_err, Error, ResultExt};
use include_dir::Dir;
use log::debug;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use structopt::StructOpt;

const TEMPLATES: Dir = include_dir!("./data/templates/");
const NEW_TEMPLATE: &str = include_str!("../data/new-template");

#[derive(Debug, StructOpt, PartialEq)]
pub enum Command {
    /// Adds a new template.
    Add {
        #[structopt(
            help = "A filename of the script file. Extension will be stripped for the template name."
        )]
        filename: String,
    },
    /// Opens and editor to modify an existing template.
    Edit {
        #[structopt(help = "A name of the template to edit")]
        template_name: String,
    },
    /// Remove a custom template or reset it to built-in contents.
    Rm {
        #[structopt(help = "A name of the template to remove")]
        template_name: String,
    },
    /// List all templates. Alias: ls.
    #[structopt(alias = "ls")]
    List {},
}

#[derive(Debug, PartialEq)]
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

fn filename_to_template_name<T: AsRef<Path>>(p: T) -> Result<String, Error> {
    let p: PathBuf = p.as_ref().into();
    let file_name = p
        .file_name()
        .ok_or_else(|| format_err!("Cannot extract filename from {:?}", p))?;
    Ok(p.file_stem()
        .unwrap_or(file_name)
        .to_string_lossy()
        .to_string())
}

fn get_built_in_templates() -> Result<TemplateMap, Error> {
    let mut templates = TemplateMap::new();
    for file in TEMPLATES.files() {
        let path = PathBuf::from(file.path());
        templates.insert(
            filename_to_template_name(&path)?,
            Template {
                source: Source::BuiltIn,
                filename: path
                    .file_name()
                    .ok_or_else(|| format_err!("Cannot extract filename from {:?}", path))?
                    .to_string_lossy()
                    .to_string(),
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
                let filename = template_file.file_name().to_string_lossy().to_string();
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

fn table_config() -> TableConfig {
    let mut config = TableConfig::default();
    config.columns.insert(
        0,
        ColumnConfig {
            align: Align::Left,
            header: "Template Name".into(),
        },
    );
    config.columns.insert(
        1,
        ColumnConfig {
            align: Align::Left,
            header: "Custom".into(),
        },
    );
    config.columns.insert(
        2,
        ColumnConfig {
            align: Align::Left,
            header: "Extension".into(),
        },
    );
    config
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

    print_table(table, &table_config());
}

fn template_not_found(name: &str, templates: &TemplateMap) -> ! {
    println!("Template '{}' is not found!", name);
    println!("Available templates in the table below:");
    print_templates(&templates);
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

pub fn edit(initial_value: &str, filename: &str) -> Result<(), Error> {
    let extension = filename_extension(filename);
    let mut editor = scrawl::editor::Editor::new();
    editor.contents(initial_value).extension(&extension);

    let new_value = editor.edit().unwrap();

    if new_value.trim() == initial_value.trim() {
        println!("No changes was made during editing.");
    } else {
        let templates_directory = get_templates_directory()?;
        std::fs::create_dir_all(&templates_directory)?;
        let mut script_path = templates_directory.clone();
        script_path.push(filename);
        let mut file = File::create(&script_path).context("Cannot create script file")?;
        let bytes = new_value.as_bytes();
        file.write_all(bytes)
            .context("Cannot write bytes to file")?;
        debug!("Wrote {} bytes to {:?}", bytes.len(), script_path);
    }

    Ok(())
}

pub fn command_template_add(filename: &str) -> Result<(), Error> {
    let template_name = filename_to_template_name(filename)?;
    let templates = get_templates()?;

    if templates.contains_key(&template_name) {
        println!(
            "Cannot add new template '{}', already exits.\nMaybe \"scriptisto template edit {}\"?",
            template_name, template_name
        );
        std::process::exit(1);
    }

    edit(NEW_TEMPLATE, filename)
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

pub fn command_template(cmd: Command) -> Result<(), Error> {
    let templates = get_templates()?;

    match cmd {
        Command::List {} => {
            print_templates(&templates);
            Ok(())
        }
        Command::Add { filename } => command_template_add(&filename),
        Command::Edit { template_name } => command_template_edit(template_name),
        Command::Rm { template_name } => command_template_rm(template_name),
    }
}
