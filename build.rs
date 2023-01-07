#[path = "src/opt.rs"]
#[allow(dead_code)]
mod opt;

use clap::CommandFactory;

fn main() -> std::io::Result<()> {
    let current_dir = std::env::current_dir()?;
    let man_path = current_dir.join("./target/man");
    std::fs::create_dir_all(&man_path)?;

    let cmd = opt::Opt::command();

    let man = clap_mangen::Man::new(cmd);
    let mut buffer: Vec<u8> = Default::default();
    man.render(&mut buffer)?;

    std::fs::write(man_path.join("scriptisto.1"), buffer)?;

    Ok(())
}
