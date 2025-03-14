use clap::Parser;
use clio::ClioPath;
use std::io;

use doctor::tree::FileTree;

/// Clean, validate, and aggregate file trees.
#[derive(Parser, Debug)]
#[command(name="doctor", version, about, long_about=None)]
struct Args {
    /// directory whose file tree is to be created
    #[clap(value_parser = clap::value_parser!(ClioPath).exists().is_dir())]
    dir: ClioPath,
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let tree: FileTree = args.dir.canonicalize()?.try_into()?;
    println!("{}", serde_json::to_string_pretty(&tree)?);
    Ok(())
}
