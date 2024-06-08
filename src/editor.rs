use std::{
    env, fs,
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::Context;

use crate::common;

pub fn edit<P: AsRef<Path>>(source_file_path: P, content: &str) -> anyhow::Result<Option<String>> {
    use std::env::var;

    let tmp_file = {
        let mut path = env::temp_dir();
        path.push("scriptisto");
        path.push(format!("{}", std::process::id()));
        if !path.exists() {
            fs::create_dir_all(&path).with_context(|| format!("Unable to make dir: {:?}", path))?
        }
        let mut filename = String::from("script");
        if let Some(ext) = source_file_path.as_ref().extension() {
            filename.push('.');
            filename.push_str(&ext.to_string_lossy());
        }
        path.push(filename);
        path
    };
    log::info!("{:?}", tmp_file);

    common::write_bytes(&PathBuf::from("/"), &tmp_file, content.as_bytes())?;

    let editors = [
        var("VISUAL"),
        var("EDITOR"),
        Ok("vim".into()),
        Ok("vi".into()),
    ];
    for editor in editors.iter().flatten() {
        if Command::new(editor).arg(tmp_file.clone()).status().is_ok() {
            let content_after_editor = fs::read_to_string(tmp_file)?;
            if content_after_editor == content {
                return Ok(None);
            } else {
                return Ok(Some(content_after_editor));
            }
        }
    }
    Err(anyhow::anyhow!(
        "Unable to open editor, specify valid editor via EDITOR or VISUAL environment variables"
    ))
}
