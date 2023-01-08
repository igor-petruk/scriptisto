#[path = "src/opt.rs"]
#[allow(dead_code)]
mod opt;

use clap::CommandFactory;
use std::path::Path;

fn generate_command(man_path: &Path, cmd: &clap::Command) -> std::io::Result<()> {
    let man = clap_mangen::Man::new(cmd.clone());
    let mut buffer: Vec<u8> = Default::default();
    man.render(&mut buffer)?;

    std::fs::write(man_path.join(format!("{}.1", cmd.get_name())), buffer)?;

    for sub in cmd.get_subcommands() {
        let sub_name: String = format!("{}-{}", cmd.get_name(), sub.get_name()).to_string();
        generate_command(man_path, &sub.clone().name(sub_name))?;
    }

    Ok(())
}

fn generate_man_page() -> std::io::Result<()> {
    let current_dir = std::env::current_dir()?;
    let man_path = current_dir.join("./target/man");
    std::fs::create_dir_all(&man_path)?;

    let cmd = opt::Opt::command();
    generate_command(&man_path, &cmd)?;

    Ok(())
}

fn main() -> std::io::Result<()> {
    generate_man_page()?;

    Ok(())
}
