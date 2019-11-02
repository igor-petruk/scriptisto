use ascii_table::{print_table, Align, ColumnConfig, TableConfig};
use failure::{format_err, Error};
use include_dir::Dir;
use log::debug;
use std::collections::BTreeMap;
use std::path::PathBuf;

const TEMPLATES: Dir = include_dir!("./data/templates/");

#[derive(Debug)]
enum Source {
    BuiltIn,
    Custom,
}

#[derive(Debug)]
struct Template {
    source: Source,
    contents: String,
}

type TemplateMap = BTreeMap<String, Template>;

fn get_built_in_templates() -> Result<TemplateMap, Error> {
    let mut templates = TemplateMap::new();
    for file in TEMPLATES.files() {
        let path = PathBuf::from(file.path());
        let file_stem = path
            .file_stem()
            .ok_or_else(|| format_err!("Cannot strip extension from {:?}", path))?;
        templates.insert(
            file_stem.to_string_lossy().to_string(),
            Template {
                source: Source::BuiltIn,
                contents: file
                    .contents_utf8()
                    .ok_or_else(|| format_err!("File {:?} is not UTF-8", file))?
                    .to_string(),
            },
        );
    }
    Ok(templates)
}

fn get_custom_templates() -> Result<TemplateMap, Error> {
    let mut templates = TemplateMap::new();

    let templates_dir = {
        let mut p =
            dirs::config_dir().ok_or_else(|| format_err!("Cannot compute user's config dir"))?;
        p.push("scriptisto/templates");
        p
    };

    debug!("Scanning for custom templates at {:?};", templates_dir);
    match std::fs::read_dir(&templates_dir) {
        Ok(dir_iter) => {
            debug!("Custom templates directory found");
            for template_file in dir_iter {
                let template_file = template_file?;
                let name = template_file
                    .path()
                    .file_stem()
                    .map(|x| x.to_string_lossy().to_string())
                    .unwrap_or_else(|| template_file.file_name().to_string_lossy().to_string());
                let contents = std::fs::read_to_string(template_file.path())?;
                templates.insert(
                    name,
                    Template {
                        source: Source::Custom,
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
            header: "Size".into(),
        },
    );
    config
}

fn get_templates() -> Result<TemplateMap, Error> {
    let mut templates = get_built_in_templates()?;
    templates.append(&mut get_custom_templates()?);
    Ok(templates)
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
            ]
        })
        .collect();

    print_table(table, &table_config());
}

pub fn command_new(name: Option<String>) -> Result<(), Error> {
    let templates = get_templates()?;

    if let Some(name) = name {
        if let Some(template) = templates.get(&name) {
            println!("{}", template.contents);
        } else {
            println!("Template '{}' is not found!", name);
            println!("Available templates in the table below:");
            print_templates(&templates);
        }
    } else {
        println!("Usage:\n\t$ scriptisto new <template> | tee ./new-script");
        println!("Available templates in the table below:");
        print_templates(&templates);
    }
    Ok(())
}
